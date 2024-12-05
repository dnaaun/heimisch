use std::sync::Arc;

use dashmap::DashMap;
use shared::{endpoints::defns::api::websocket_updates::Webhook, types::user::UserId};

#[derive(Default)]
pub struct WebsocketUpdatesBucket {
    senders: DashMap<UserId, tokio::sync::broadcast::Sender<Webhook>>,
}

pub const CAPACITY: usize = 1000; // :shrug:

impl WebsocketUpdatesBucket {
    fn unsubscribe(self: &Self, id: &UserId) {
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
                let (sender, _) = tokio::sync::broadcast::channel(CAPACITY);
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
}

pub struct Subscription {
    user_id: UserId,
    receiver: tokio::sync::broadcast::Receiver<Webhook>,
    bucket: Arc<WebsocketUpdatesBucket>,
}

impl Subscription {
    pub async fn recv(
        &mut self,
    ) -> std::result::Result<Webhook, tokio::sync::broadcast::error::RecvError> {
        self.receiver.recv().await
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.bucket.unsubscribe(&self.user_id);
    }
}

// TODO: Write tests.
