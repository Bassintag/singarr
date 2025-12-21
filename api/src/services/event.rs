use anyhow::{Ok, Result};
use tokio::sync::broadcast;

use crate::models::event::Event;

#[derive(Clone)]
pub struct EventService {
    sender: broadcast::Sender<Event>,
}

impl EventService {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel::<Event>(32);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    pub fn send(&self, event: Event) -> Result<()> {
        self.sender.send(event)?;
        Ok(())
    }
}
