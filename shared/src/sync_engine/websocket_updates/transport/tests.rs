use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{channel::mpsc, Sink, Stream};

use crate::endpoints::defns::api::websocket_updates::{ClientMsg, ServerMsg};

use super::TransportTrait;

pub struct MockTransportHandler {
    pub sender: mpsc::Sender<ServerMsg>,
    pub recver: mpsc::Receiver<ClientMsg>,
}

pub struct MockTransport {
    recer: mpsc::Receiver<ServerMsg>,
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
                recer: server_msg_receiver,
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

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.recer)
            .poll_next(cx)
            .map(|opt| opt.map(Ok))
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

