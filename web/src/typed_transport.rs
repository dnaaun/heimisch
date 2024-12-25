use std::task::{ready, Poll};

use futures::{Sink, Stream};
use gloo_net::websocket::futures::WebSocket;
use gloo_utils::errors::JsError;
use pin_project::pin_project;
use shared::sync_engine::{self, typed_transport::ConnOrClosedError};
use url::Url;

#[derive(Debug)]
pub enum ConnError {
    GlooJs(JsError),
    GlooWebsocket(gloo_net::websocket::WebSocketError),
    Send(futures::channel::mpsc::SendError),
    TryRecv(futures::channel::mpsc::TryRecvError),
}

#[pin_project]
pub struct MyWebSocket(#[pin] gloo_net::websocket::futures::WebSocket);

impl sync_engine::typed_transport::TypedTransportTrait for MyWebSocket {
    type ConnError = ConnError;

    async fn establish_conn(url: &Url) -> Result<Self, Self::ConnError> {
        Ok(MyWebSocket(
            gloo_net::websocket::futures::WebSocket::open(url.as_str())
                .map_err(|e| ConnError::GlooJs(e))?,
        ))
    }
}

impl Stream for MyWebSocket {
    type Item = Result<Vec<u8>, ConnOrClosedError<ConnError>>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        let r = match ready!(this.0.poll_next(cx)) {
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

impl Sink<Vec<u8>> for MyWebSocket {
    type Error = ConnError;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.0.poll_ready(cx)).map_err(ConnError::GlooWebsocket))
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: Vec<u8>) -> Result<(), Self::Error> {
        let this = self.project();
        this.0
            .start_send(gloo_net::websocket::Message::Bytes(item))
            .map_err(ConnError::GlooWebsocket)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.0.poll_flush(cx)).map_err(ConnError::GlooWebsocket))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.0.poll_close(cx)).map_err(ConnError::GlooWebsocket))
    }
}
