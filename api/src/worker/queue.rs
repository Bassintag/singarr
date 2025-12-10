use anyhow::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::worker::job::Job;

#[derive(Clone)]
pub struct Queue {
    sender: UnboundedSender<Job>,
}

impl Queue {
    pub fn with_receiver() -> (Self, UnboundedReceiver<Job>) {
        let (sender, reciever) = mpsc::unbounded_channel();
        (Self { sender }, reciever)
    }

    pub fn enqueue(&self, job: Job) -> Result<()> {
        println!("ENQUEUE {:?}", job);
        self.sender.send(job)?;

        Ok(())
    }
}
