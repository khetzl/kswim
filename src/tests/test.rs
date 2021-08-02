#[cfg(test)]
//use super::*;
use crate::group::Group;
//use crate::peer::Peer;
//use crate::ping::PingResult;
use crate::tests::test_member::Member;

//use std::error::Error;
use tokio::time::Duration;

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
