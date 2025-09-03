use std::{
    ops::DerefMut,
    pin::pin,
    task::{ready, Poll},
};

use futures::{Sink, Stream};
use gloo_utils::errors::JsError;
use shared::sync_engine::websocket_updates::binary_transport::{
    BinaryTransportTrait, ConnOrClosedError,
};
use url::Url;
use utils::JustSend;

#[derive(Debug)]
pub enum ConnError {
    GlooJs(JsError),
    GlooWebsocket(gloo_net::websocket::WebSocketError),
    Send(futures::channel::mpsc::SendError),
    TryRecv(futures::channel::mpsc::TryRecvError),
}

pub struct BinaryTransport(JustSend<gloo_net::websocket::futures::WebSocket>);

impl BinaryTransportTrait for BinaryTransport {
    type ConnError = ConnError;

    async fn establish_conn(url: Url) -> Result<Self, Self::ConnError> {
        Ok(BinaryTransport(JustSend::new(
            gloo_net::websocket::futures::WebSocket::open(url.as_str())
                .map_err(ConnError::GlooJs)?,
        )))
    }
}

impl Stream for BinaryTransport {
    type Item = Result<Vec<u8>, ConnOrClosedError<ConnError>>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = pin!(self.0.deref_mut());
        let r = match ready!(this.poll_next(cx)) {
            Some(r) => r,
            None => return Poll::Ready(None),
        };

        let r = match r {
            Ok(r) => r,
            Err(err) => {
                return Poll::Ready(Some(Err(ConnOrClosedError::Conn(
                    ConnError::GlooWebsocket(err),
                ))))
            }
        };

        let bytes = match r {
            gloo_net::websocket::Message::Text(t) => t.as_bytes().to_vec(),
            gloo_net::websocket::Message::Bytes(vec) => vec,
        };

        Poll::Ready(Some(Ok(bytes)))
    }
}

impl Sink<Vec<u8>> for BinaryTransport {
    type Error = ConnError;

    fn poll_ready(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        let this = pin!(self.0.deref_mut());
        Poll::Ready(ready!(this.poll_ready(cx)).map_err(ConnError::GlooWebsocket))
    }

    fn start_send(mut self: std::pin::Pin<&mut Self>, item: Vec<u8>) -> Result<(), Self::Error> {
        let this = pin!(self.0.deref_mut());
        this.start_send(gloo_net::websocket::Message::Bytes(item))
            .map_err(ConnError::GlooWebsocket)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        let this = pin!(self.0.deref_mut());
        Poll::Ready(ready!(this.poll_flush(cx)).map_err(ConnError::GlooWebsocket))
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        let this = pin!(self.0.deref_mut());
        Poll::Ready(ready!(this.poll_close(cx)).map_err(ConnError::GlooWebsocket))
    }
}
