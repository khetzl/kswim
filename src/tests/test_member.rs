use crate::peer::Peer;
use crate::ping::PingResult;
use rand::Rng;
use std::error::Error;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct Member {
    id: usize,
    name: String,
    pong_rate: usize,
}

impl Member {
    pub fn new(id: usize, name: &str, pong_rate: usize) -> Member {
        Member {
            id,
            name: name.to_owned(),
            pong_rate,
        }
    }
}

impl Peer for Member {
    fn ping(&mut self, _timeout: Duration) -> PingResult {
        let mut rng = rand::thread_rng();
        let sr: usize = rng.gen_range(0..100);
        if sr >= self.pong_rate {
            PingResult::Pang
        } else {
            PingResult::Pong
        }
    }
    fn ping_request<M>(&mut self, target: &mut M, timeout: Duration) -> PingResult
    where
        M: Peer,
    {
        target.ping(timeout)
    }

    fn announce_failure<M: Peer>(&mut self, target: &M) -> Result<(), Box<dyn Error>> {
        println!("we reported {} as faulty to {}", target.name(), self.name());
        Ok(())
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
