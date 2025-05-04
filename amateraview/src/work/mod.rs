use crate::Message;
use crate::state::State;
use iced::futures::SinkExt;
use iced::{Subscription, stream};
use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Debug, Clone)]
pub enum Job {
    ListenForPlugins { port: u16 },
    StopListeningForPlugins,
}

pub mod plugins;
pub async fn main_job_loop(
    sender: iced::futures::channel::mpsc::Sender<Message>,
    mut receiver: Receiver<(Job, CancellationToken)>,
) {
    info!("Starting main job loop");
    let mut actors = HashMap::new();
    let s = translate_job_messages_in_ui_messages(sender.clone(), &mut actors);
    info!("Started translator");
    while let Some((job, token)) = receiver.recv().await {
        info!("Received job: {:?}", job);
        match job {
            Job::ListenForPlugins { port } => {
                let tcp_listener = tokio::spawn(plugins::tcp_listener(s.clone(), port, token));
                actors.insert("TCP Listener", tcp_listener);
            }
            Job::StopListeningForPlugins => {
                let h = actors.remove("TCP Listener").unwrap();
                h.abort()
            }
        }
    }
}

fn translate_job_messages_in_ui_messages(
    mut sender: iced::futures::channel::mpsc::Sender<Message>,
    actors: &mut HashMap<&str, JoinHandle<()>>,
) -> Sender<Message> {
    let (s, mut r) = tokio::sync::mpsc::channel(100);
    let translator = tokio::spawn(async move {
        while let Some(msg) = r.recv().await {
            _ = sender.send(msg).await;
        }
    });
    actors.insert("Translator", translator);
    s
}

pub fn worker_listener(_: &State) -> Subscription<Message> {
    Subscription::run_with_id(
        "Worker Listener",
        stream::channel(100, |mut output| async move {
            let (sender, receiver) = tokio::sync::mpsc::channel(100);
            _ = output.send(Message::MainWorkLoop(sender)).await;
            tokio::spawn(main_job_loop(output, receiver));
        }),
    )
}
