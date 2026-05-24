use tokio::sync::broadcast;

#[derive(Clone)]
pub struct BroadcastManager {
    pub sender:
        broadcast::Sender<String>,
}

impl BroadcastManager {
    pub fn new() -> Self {
        let (sender, _) =
            broadcast::channel(100);

        Self { sender }
    }

    pub fn broadcast(
        &self,
        message: String,
    ) {
        let _ =
            self.sender.send(message);
    }

    pub fn subscribe(
        &self,
    ) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }
}