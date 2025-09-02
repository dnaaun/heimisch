use std::{
    ops::DerefMut,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use futures::{channel::mpsc, Sink, Stream};
use parking_lot::Mutex;

use crate::endpoints::defns::api::websocket_updates::{ClientMsg, ServerMsg};

use super::TransportTrait;

pub struct MockTransportHandler {
    pub sender: mpsc::Sender<ServerMsg>,
    pub recver: mpsc::Receiver<ClientMsg>,
}

#[derive(Clone)]
pub struct MockTransport {
    recver: Arc<Mutex<mpsc::Receiver<ServerMsg>>>,
    sender: mpsc::Sender<ClientMsg>,
}

impl TransportTrait for MockTransport {
    type TransportError = mpsc::SendError;
}

impl MockTransport {
    pub fn new() -> (Self, MockTransportHandler) {
        let (server_msg_sender, server_msg_receiver) = mpsc::channel(100);
        let (client_msg_sender, client_msg_receiver) = mpsc::channel(100);

        (
            Self {
                recver: Arc::new(Mutex::new(server_msg_receiver)),
                sender: client_msg_sender,
            },
            MockTransportHandler {
                sender: server_msg_sender,
                recver: client_msg_receiver,
            },
        )
    }
}

impl Stream for MockTransport {
    type Item = Result<ServerMsg, mpsc::SendError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut mut_ref = self.recver.lock();
        let mut_ref = mut_ref.deref_mut();
        let pinned = std::pin::pin!(mut_ref);
        pinned.poll_next(cx).map(|opt| opt.map(Ok))
    }
}

impl Sink<ClientMsg> for MockTransport {
    type Error = mpsc::SendError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.sender).poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: ClientMsg) -> Result<(), Self::Error> {
        Pin::new(&mut self.sender).start_send(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.sender).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.sender).poll_close(cx)
    }
}
