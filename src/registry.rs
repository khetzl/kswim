use crate::peer::Peer;
use crate::ping::PingResult;
use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;
use tokio::time::Duration;

#[derive(Clone)]
pub struct Registry<I, M>
where
    I: Clone + Hash + Eq + std::fmt::Debug,
    M: Clone + Peer + std::fmt::Debug,
{
    members: HashMap<I, M>,
    ping_timeout: Duration,
    k: usize,
}

impl<I, M> Registry<I, M>
where
    I: Clone + Hash + Eq + std::fmt::Debug,
    M: Clone + Peer + std::fmt::Debug,
{
    pub fn new(ping_timeout: Duration, k: usize) -> Registry<I, M> {
        Registry {
            members: HashMap::new(),
            ping_timeout,
            k,
        }
    }

    pub fn add(&mut self, id: I, member: M) {
        self.members.insert(id, member);
    }

    pub fn remove(&mut self, id: &I) {
        self.members.remove(id);
    }

    pub fn remove_and_broadcast(&mut self, id: &I) {
        let failed_member = self.members[id].clone();
        self.remove(id);
        for (_i, m) in &self.members {
            if let Err(error) = m.announce_failure(&failed_member) {
                // FIXME: LOG error
                println!("Error: {}", error);
            }
        }
    }
    pub fn pick_random(&self) -> Option<(&I, &M)> {
        //    pub fn pick_random<'a>(&'a self) -> Option<(&I, &'a M)> {
        // Not a very nice solution, but will be fine for now.
        // hashmap.iter().next() will give a random enough value,
        // since it's not deterministic.
        if self.members.is_empty() {
            None
        } else {
            //FIXME: nicer way of finding a random member
            let mut rng = rand::thread_rng();
            let n = rng.gen_range(0..(self.members.len()));
            let mut it = self.members.iter();
            for _ in 1..n {
                it.next();
            }
            it.next()
        }
    }

    pub fn pick_k_random<'a>(&'a self, k: usize) -> Vec<(&I, &'a M)> {
        let mut v: Vec<(&I, &'a M)> = Vec::new();
        // Again not a very nice way to get random elements... but it might work
        let mut iter = self.members.iter();
        for _i in 0..k {
            match iter.next() {
                None => continue,
                Some((id, member)) => v.push((id, member)),
            }
        }
        v
    }

    pub fn perform_periodic_check(&mut self) {
        // Member Mi (self) selects a random member (Mj) to ping, in every period.
        if let Some((id, mj)) = self.pick_random() {
            let id = id.clone();
            let r = self.test_member(&mj);
            println!("{:?} tested {:?}", mj, r);
            match r {
                PingResult::Pong => return (),
                PingResult::Pang => self.remove_and_broadcast(&id),
            }
        }
    }

    fn test_member(&self, mj: &M) -> PingResult {
        match mj.ping(self.ping_timeout) {
            PingResult::Pong => PingResult::Pong,
            PingResult::Pang => self.try_neighbours(mj),
        }
    }

    fn try_neighbours(&self, mj: &M) -> PingResult {
        println!("neighbour");
        let neighbours = self.pick_k_random(self.k);
        let mut responses: Vec<PingResult> = vec![];
        for (_, n) in neighbours {
            let r = n.ping_request(mj, self.ping_timeout);
            responses.push(r);
        }
        println!("responses: {:?}", responses);
        //FIXME: nicer patterns
        if responses.iter().any(|ping_result| match ping_result {
            PingResult::Pong => true,
            PingResult::Pang => false,
        }) {
            PingResult::Pong
        } else {
            PingResult::Pang
        }
    }
}
