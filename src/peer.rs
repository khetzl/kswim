use crate::ping::PingResult;
use std::error::Error;
use tokio::time::Duration;

pub trait Peer {
    fn ping(&self, _timeout: Duration) -> PingResult {
        PingResult::Pong
    }

    fn ping_request<M: Peer>(&self, _member: &M, _timeout: Duration) -> PingResult {
        PingResult::Pong
    }

    fn announce_failure<M: Peer>(&self, _member: &M) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn name(&self) -> String;
}
