use crate::group::Group;
use crate::peer::Peer;
use crate::ping::PingResult;
use std::hash::Hash;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;
use tokio::time::Duration;

#[derive(Debug)]
pub struct Member {
    id: String,
    address: SocketAddr,
    connection: TcpStream,
}

impl Member {
    pub async fn new(id: String, address: SocketAddr) -> Member {
        let connection: TcpStream = TcpStream::connect(&address).await.unwrap();
        Member {
            id,
            address,
            connection,
        }
    }

    pub fn get_address(&self) -> &SocketAddr {
        &self.address
    }
}

impl Peer for Member {
    fn ping(&mut self, _timeout: Duration) -> PingResult {
        let msg = b"ping\n";
        async {
            self.connection.write(msg).await;
        };
        PingResult::Pong
    }
    fn ping_request<M: Peer>(&mut self, _target: &mut M, _timeout: Duration) -> PingResult {
        PingResult::Pong
    }

    fn name(&self) -> String {
        self.id.clone()
    }
}

pub struct DisseminationComponent<I, M>
where
    I: 'static + Clone + Send + Hash + Eq + std::fmt::Debug,
    M: 'static + Clone + Send + Peer + std::fmt::Debug,
{
    group: Group<I, M>,
    listener_handle: JoinHandle<()>,
    //local: M,
}

impl<I, M> DisseminationComponent<I, M>
where
    I: 'static + Clone + Send + Hash + Eq + std::fmt::Debug,
    M: 'static + Clone + Send + Peer + std::fmt::Debug,
{
    pub fn new(
        listener_address: SocketAddr,
        protocol_period: Duration,
        timeout_period: Duration,
        sample_size: usize,
    ) -> DisseminationComponent<I, M> {
        let group: Group<I, M> = Group::new(protocol_period, timeout_period, sample_size);
        let listener_handle = tokio::spawn(async move { listener_worker(listener_address).await });

        DisseminationComponent {
            group,
            listener_handle,
        }
    }

    pub fn group(&self) -> &Group<I, M> {
        &self.group
    }

    pub fn listener_handle(&self) -> &JoinHandle<()> {
        &self.listener_handle
    }
}

async fn listener_worker(addr: SocketAddr) {
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failure to bind listener");
    loop {
        let (socket, _) = listener.accept().await.expect("Listener failure");
        tokio::spawn(async move {
            handle_socket(socket).await;
        });
    }
}

async fn handle_socket(mut _stream: TcpStream) {
    println!("socket stuff");
    /*let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        println!("msg:{:?}", buffer);

    */
}
