use crate::peer::Peer;
use crate::registry::Registry;

//use std::error::Error;
use std::error::Error;
use std::hash::Hash;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::Duration;

type HandlerResult = Result<(), Box<dyn Error + Send>>;

#[derive(Debug)]
enum HandlerMsg<I, M>
where
    I: Hash + Eq + std::fmt::Debug,
    M: Peer + std::fmt::Debug,
{
    Add {
        reply_channel: oneshot::Sender<HandlerResult>,
        id: I,
        member: M,
    },
    Remove {
        id: I,
    },
    PeriodicCheck,
}

pub struct Group<I, M>
where
    I: Clone + Send + Hash + Eq + std::fmt::Debug,
    M: Clone + Send + Peer + std::fmt::Debug,
{
    handler: JoinHandle<()>,
    handler_sender: mpsc::Sender<HandlerMsg<I, M>>,
    scheduler: JoinHandle<()>,
}

impl<I, M> Group<I, M>
where
    I: 'static + Clone + Send + Hash + Eq + std::fmt::Debug,
    M: 'static + Clone + Send + Peer + std::fmt::Debug,
{
    pub fn new(protocol_period: Duration, ping_timeout: Duration, k: usize) -> Group<I, M> {
        assert!(protocol_period > ping_timeout);
        // FIXME: channel size is hardcoded
        let (handler_sender, rx) = mpsc::channel(32);

        let handler = tokio::spawn(async move {
            handler(rx, ping_timeout, k).await;
        });

        let scheduler_tx = handler_sender.clone();
        let scheduler = tokio::spawn(async move {
            loop {
                // FIXME: perhaps a more sophisticated scheduler is needed here.
                scheduler_tx.send(HandlerMsg::PeriodicCheck).await.unwrap();
                tokio::time::sleep(protocol_period).await;
            }
        });

        Group {
            handler,
            handler_sender,
            scheduler,
        }
    }

    pub async fn add(&self, id: I, member: M) {
        let (reply_sender, mut rx) = oneshot::channel::<HandlerResult>();
        self.handler_sender
            .send(HandlerMsg::Add {
                reply_channel: reply_sender,
                id: id,
                member: member,
            })
            .await
            .unwrap();
        match rx.await {
            Ok(v) => println!("got = {:?}", v),
            Err(_) => println!("the sender dropped"),
        }
    }

    pub async fn remove(&self, id: I) {
        self.handler_sender
            .send(HandlerMsg::Remove { id: id })
            .await
            .unwrap();
    }

    // FIXME: not sure of these, or touching the handles will ever be needed,
    //        perhaps when we need to wait for them to finish (forever)
    pub fn get_handler(&self) -> &JoinHandle<()> {
        &self.handler
    }

    pub fn get_scheduler(&self) -> &JoinHandle<()> {
        &self.scheduler
    }
}

async fn handler<I, M>(mut rx: mpsc::Receiver<HandlerMsg<I, M>>, ping_timeout: Duration, k: usize)
where
    I: 'static + Clone + Send + Hash + Eq + std::fmt::Debug,
    M: 'static + Clone + Send + Peer + std::fmt::Debug,
{
    let mut registry: Registry<I, M> = Registry::new(ping_timeout, k);
    loop {
        if let Some(msg) = rx.recv().await {
            match msg {
                HandlerMsg::PeriodicCheck => registry.perform_periodic_check(),
                HandlerMsg::Add {
                    reply_channel,
                    id,
                    member,
                } => {
                    registry.add(id, member);
                    let reply = reply_channel.send(Ok(()));
                    println!("Responded and result:{:?}", reply);
                }
                HandlerMsg::Remove { id } => registry.remove(&id),
            }
        } else {
            println!("stopped");
            break;
        }
    }
}
