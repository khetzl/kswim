use crate::ping::PingResult;
use std::error::Error;
use tokio::time::Duration;

pub trait Peer {
    fn ping(&mut self, _timeout: Duration) -> PingResult {
        PingResult::Pong
    }

    fn ping_request<M: Peer>(&mut self, _target: &mut M, _timeout: Duration) -> PingResult {
        PingResult::Pong
    }

    fn announce_failure<M: Peer>(&mut self, _target: &M) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn announce_remove<M: Peer>(&mut self, _target: &M) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn announce_add<M: Peer>(&mut self, _target: &M) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn name(&self) -> String;
}
