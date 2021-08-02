use crate::peer::Peer;
use crate::ping::PingResult;
use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::time::Duration;

#[derive(Clone, Debug)]
pub struct Registry<I, M>
where
    I: Send + Clone + Hash + Eq + std::fmt::Debug,
    M: Send + Peer + std::fmt::Debug,
{
    members: HashMap<I, Arc<M>>,
    ping_timeout: Duration,
    k: usize,
}

unsafe impl<I, M> Send for Registry<I, M>
where
    I: Send + Clone + Hash + Eq + std::fmt::Debug,
    M: Send + Peer + std::fmt::Debug,
{
}

impl<I, M> Registry<I, M>
where
    I: Send + Clone + Hash + Eq + std::fmt::Debug,
    M: Send + Peer + std::fmt::Debug,
{
    pub fn new(ping_timeout: Duration, k: usize) -> Registry<I, M> {
        Registry {
            members: HashMap::new(),
            ping_timeout,
            k,
        }
    }

    pub fn add(&mut self, id: I, member: M) {
        self.members.insert(id, Arc::new(member));
    }

    pub fn remove(&mut self, id: &I) -> Option<Arc<M>> {
        self.members.remove(id)
    }

    pub unsafe fn broadcast_failure(&mut self, failed_member: &mut M) {
        for (_i, narc) in &mut self.members {
            let m = Arc::get_mut_unchecked(narc);
            if let Err(error) = m.announce_failure(failed_member) {
                println!("ERROR:{:?}", error);
            }
        }
    }
    pub fn pick_random(&self) -> Option<I> {
        // Not a very nice solution, but will be fine for now.
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
            let (id, _) = it.next().unwrap();
            Some(id.clone())
        }
    }

    pub fn pick_k_random(&mut self) -> Vec<Arc<M>> {
        let mut v: Vec<Arc<M>> = Vec::new();
        // Again not a very nice way to get random elements... but it might work
        let mut reg = self.members.clone();
        let mut iter = reg.iter_mut();

        for _i in 0..self.k {
            match iter.next() {
                None => continue,
                Some((_id, m)) => {
                    let n = Arc::clone(m);
                    v.push(n)
                }
            }
        }
        v
    }

    pub unsafe fn perform_periodic_check(&mut self) {
        // Member Mi (self) selects a random member (Mj) to ping, in every period.
        if let Some(id) = self.pick_random() {
            let id = id.clone();
            let mut removed = self.members.remove(&id).unwrap();
            let mj: &mut M = Arc::get_mut(&mut removed).unwrap();
            let r = self.test_member(mj);
            println!("{:?} tested {:?}", mj, r);
            match r {
                PingResult::Pong => {
                    self.members.insert(id, removed);
                    ()
                }
                PingResult::Pang => self.broadcast_failure(mj),
            }
        }
    }

    unsafe fn test_member(&mut self, mj: &mut M) -> PingResult {
        println!("testing {:?}", mj.name());
        match mj.ping(self.ping_timeout) {
            PingResult::Pong => PingResult::Pong,
            PingResult::Pang => self.try_neighbours(mj),
        }
    }

    unsafe fn try_neighbours(&mut self, mj: &mut M) -> PingResult {
        let timeout = self.ping_timeout.clone();
        let mut neighbours = self.pick_k_random();

        let mut responses: Vec<PingResult> = vec![];
        println!("neightbours");
        for n in &mut neighbours {
            let mn = Arc::get_mut_unchecked(n);
            let r = mn.ping_request(mj, timeout);
            responses.push(r);
        }
        if responses
            .iter()
            .any(|ping_result| matches!(ping_result, PingResult::Pong))
        {
            PingResult::Pong
        } else {
            PingResult::Pang
        }
    }
}
