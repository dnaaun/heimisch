use futures::{Sink, Stream};
use std::{
    fmt::Debug,
    pin::Pin,
    task::{Context, Poll},
};
use url::Url;

use crate::endpoints::defns::api::websocket_updates::{ClientMsg, ServerMsg};

use super::transport::TransportTrait;

/// Mock implementation of `TypedTransportTrait`.
#[derive(Debug)]
pub struct MockTransport {
    // This marker ensures we follow the TypedTransportTrait.
    marker: std::marker::PhantomData<()>,
}

/// Define type for connection error
#[derive(Debug)]
pub struct MockConnError;

/// Implement TypedTransportTrait for MockTypedTransport
impl TransportTrait for MockTransport {
    type TransportError = MockConnError;

    async fn establish(_url: &Url) -> Result<Self, Self::TransportError> {
        Ok(MockTransport {
            marker: std::marker::PhantomData,
        })
    }
}

/// Implement Sink for MockTypedTransport
impl Sink<ClientMsg> for MockTransport {
    type Error = MockConnError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Always ready to accept new messages
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, _item: ClientMsg) -> Result<(), Self::Error> {
        // Do nothing with the incoming message
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Always ready as we do nothing
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // No resources to release, so always ready
        Poll::Ready(Ok(()))
    }
}

/// Implement Stream for MockTypedTransport
impl Stream for MockTransport {
    type Item = Result<ServerMsg, MockConnError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Never provides any items
        Poll::Ready(None)
    }
}
