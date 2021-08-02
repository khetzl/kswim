//use kswim::group::Group;
//use kswim::dissemination_tcp::DisseminationComponent;
//use kswim::test::test_member::Member;
//use kswim::peer::Peer;
//use kswim::ping::PingResult;

use std::env;
//use std::error::Error;
use std::net::SocketAddr;
use std::process;
use tokio::time::Duration;

#[tokio::main]
pub async fn main() {
    /*
    let g: Group<usize, Member> = Group::new(Duration::from_secs(1), Duration::from_millis(500), 3);
    g.remove(7).await;
    println!("all removed");
     */

    let mut args = env::args();
    args.next();
    let port = args.next().unwrap_or(String::from("9090"));

    let ip = "127.0.0.1";
    let listener_address = format!("{}:{}", ip, port);
    println!("Configured to listen on: {}", listener_address);

    let _addr = listener_address
        .parse::<SocketAddr>()
        .unwrap_or_else(|err| {
            eprintln!("ERROR: {}", err);
            process::exit(1)
        });

    /*
        let dc: DisseminationComponent<usize, Member> =
            DisseminationComponent::new(addr, Duration::from_secs(1), Duration::from_millis(300), 3);

        let g = dc.group();
    */
    /*
        g.add(1, Member::new(1, "one", 90)).await;
        g.add(2, Member::new(2, "two", 100)).await;
        g.add(3, Member::new(3, "three", 100)).await;
        g.add(4, Member::new(4, "four", 30)).await;
        g.add(5, Member::new(5, "five", 0)).await;
        g.add(6, Member::new(6, "six", 0)).await;
        g.add(7, Member::new(7, "seven", 0)).await;

        println!("added a few");
    */
    // Cheap and dodgy trick to limit the demo to a certain time
    tokio::time::sleep(Duration::from_secs(100)).await;
}
