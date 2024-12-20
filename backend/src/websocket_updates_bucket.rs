use shared::utils::LogErr;
use std::sync::Arc;

use dashmap::DashMap;
use shared::{endpoints::defns::api::websocket_updates::ServerMsg, types::user::UserId};

#[derive(Default)]
pub struct WebsocketUpdatesBucket {
    senders: DashMap<UserId, tokio::sync::broadcast::Sender<ServerMsg>>,
}

pub const DEFAULT_CAPACITY: usize = 1000; // :shrug:

impl WebsocketUpdatesBucket {
    fn unsubscribe(&self, id: &UserId) {
        let sender = match self.senders.get(id) {
            Some(sender) => sender,
            None => {
                tracing::warn!("user id {} not found in WebsocketUpdatesBucket", id);
                return;
            }
        };

        if sender.receiver_count() == 0 {
            drop(sender);
            self.senders.remove(id);
        }
    }

    pub fn subscribe(self: Arc<Self>, id: UserId) -> Subscription {
        let receiver = self
            .senders
            .entry(id)
            .or_insert_with(|| {
                let (sender, _) = tokio::sync::broadcast::channel(DEFAULT_CAPACITY);
                sender
            })
            .value()
            .subscribe();

        Subscription {
            receiver,
            bucket: self.clone(),
            user_id: id,
        }
    }

    pub fn broadcast(&self, id: &UserId, webhook: ServerMsg) {
        let sender = match self.senders.get(id) {
            Some(sender) => sender,
            None => return,
        };

        // We don't let websocket errors affect HTTP return status codes, we just log them.
        let _ = sender.send(webhook).log_err();
    }
}

pub struct Subscription {
    user_id: UserId,
    receiver: tokio::sync::broadcast::Receiver<ServerMsg>,
    bucket: Arc<WebsocketUpdatesBucket>,
}

impl Subscription {
    pub async fn recv(
        &mut self,
    ) -> std::result::Result<ServerMsg, tokio::sync::broadcast::error::RecvError> {
        self.receiver.recv().await
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.bucket.unsubscribe(&self.user_id);
    }
}

// TODO: Write tests.
