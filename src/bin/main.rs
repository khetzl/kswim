use kswim::group::Group;
use kswim::peer::Peer;
use kswim::ping::PingResult;
use rand::Rng;
use std::error::Error;
use tokio::time::Duration;

#[derive(Debug, Clone)]
struct Member {
    id: usize,
    name: String,
    pong_rate: usize,
}

impl Member {
    fn new(id: usize, name: &str, pong_rate: usize) -> Member {
        Member {
            id,
            name: name.to_owned(),
            pong_rate,
        }
    }
}

impl Peer for Member {
    fn ping(&self, _timeout: Duration) -> PingResult {
        let mut rng = rand::thread_rng();
        let sr: usize = rng.gen_range(0..100);
        if sr >= self.pong_rate {
            PingResult::Pang
        } else {
            PingResult::Pong
        }
    }
    fn ping_request<M>(&self, member: &M, timeout: Duration) -> PingResult
    where
        M: Peer,
    {
        member.ping(timeout)
    }

    fn announce_failure<M: Peer>(&self, member: &M) -> Result<(), Box<dyn Error>> {
        println!("we reported {} as faulty to {}", member.name(), self.name());
        Ok(())
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

#[tokio::main]
pub async fn main() {
    let g: Group<usize, Member> = Group::new(Duration::from_secs(1), Duration::from_millis(500), 3);
    g.add(1, Member::new(1, "one", 90)).await;
    g.add(2, Member::new(2, "two", 100)).await;
    g.add(3, Member::new(3, "three", 100)).await;
    g.add(4, Member::new(4, "four", 30)).await;
    g.add(5, Member::new(5, "five", 0)).await;
    g.add(6, Member::new(6, "six", 0)).await;

    println!("added a few");

    //let (i, m) = g.pick_random().unwrap();
    //println!("Hello, world! k:{}, v:{:?}", i, m);

    //let v = g.pick_k_random(3);
    //println!("k random: {:?}", v);

    //let check = g.perform_periodic_check();
    //println!("check check:{:?}", check);
    //let scheduler = g.start_scheduler();

    // Cheap and dodgy trick to limit the demo to a certain time
    tokio::time::sleep(Duration::from_secs(100)).await;
}
