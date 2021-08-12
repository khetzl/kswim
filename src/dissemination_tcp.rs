use crate::group::Group;
use crate::peer::Peer;
use crate::ping::PingResult;
use std::hash::Hash;

use std::net::SocketAddr;

use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
//use tokio::prelude::*;
use tokio::task::JoinHandle;
use tokio::time::Duration;

#[derive(Debug)]
enum ConnectionHandlerMsg {
    Send { msg: String },
}

//enum ConnectionHandlerResult {
//    Ok,
//}

#[derive(Debug, Clone)]
pub struct Member {
    id: String,
    address: SocketAddr,
    send_channel: mpsc::Sender<ConnectionHandlerMsg>,
}

impl Member {
    pub async fn new(id: String, address: SocketAddr) -> Member {
        let (send_channel, mut rx) = mpsc::channel(32);
        let (ready_tx, ready_rx) = oneshot::channel();
        tokio::spawn(async move { connection_handler(rx, ready_tx, address).await });
        match ready_rx.await {
            Err(err) => println!("Sender dropped: {:?}", err),
            Ok(()) => (),
        }
        Member {
            id,
            address,
            send_channel,
        }
    }

    pub fn get_address(&self) -> &SocketAddr {
        &self.address
    }
}

impl Peer for Member {
    fn ping(&mut self, _timeout: Duration) -> PingResult {
        if let Err(_) = self.send_channel.try_send(ConnectionHandlerMsg::Send {
            msg: String::from("ping"),
        }) {
            println!("the receiver dropped");
        }
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
        println!("waiting for incoming connections...");
        let (socket, _) = listener.accept().await.expect("Listener failure");
        tokio::spawn(async move {
            handle_socket(socket).await;
        });
    }
}

async fn handle_socket(mut stream: TcpStream) {
    println!("socket stuff");
    let mut line = String::new();
    let mut stream = BufReader::new(stream);
    loop {
        println!("waiting to read a line...");
        stream.read_line(&mut line).await.unwrap();
        //let mut buf = [0; 1024];
        //stream.read(&mut buf).await;

        println!("msg:{:?}", line);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn connection_handler(
    mut rx: mpsc::Receiver<ConnectionHandlerMsg>,
    ready_tx: oneshot::Sender<()>,
    address: SocketAddr,
) {
    let mut i = 0;
    let mut stream: TcpStream;
    loop {
        println!("#{} handshake attempt to peer", i);
        match TcpStream::connect(&address).await {
            Err(_err) => (), // Retry
            Ok(accepted_connection) => {
                stream = accepted_connection;
                break;
            }
        }
        i += 1;
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    println!("Connection successful!");

    ready_tx.send(()).expect("ERROR: while sending ready sign");
    loop {
        match rx.recv().await {
            Some(ConnectionHandlerMsg::Send { msg }) => {
                println!("Attempt to send something: {}", msg);
                if let Err(err) = stream.write(msg.as_bytes()).await {
                    eprintln!("failed to write to socket; err = {:?}", err);
                }
                //stream.flush().await;
                //stream.write
            }
            _ => println!("ERROR: connection handler failure"),
        }
    }
}
