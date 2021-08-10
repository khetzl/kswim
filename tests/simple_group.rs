#[cfg(test)]
//use crate::test::Member;
use kswim::group::Group;
use kswim::peer::Peer;
use kswim::ping::PingResult;
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

#[tokio::test]
async fn simple_group() {
    let g: Group<usize, Member> = Group::new(Duration::from_secs(1), Duration::from_millis(300), 3);

    g.add(1, Member::new(1, "one", 90)).await;
    g.add(2, Member::new(2, "two", 100)).await;
    g.add(3, Member::new(3, "three", 100)).await;
    g.add(4, Member::new(4, "four", 30)).await;
    g.add(5, Member::new(5, "five", 0)).await;
    g.add(6, Member::new(6, "six", 0)).await;
    g.add(7, Member::new(7, "seven", 0)).await;

    assert_eq!(2 + 2, 4);
}
