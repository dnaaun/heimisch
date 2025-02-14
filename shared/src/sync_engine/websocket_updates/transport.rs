use crate::{
    endpoints::defns::api::websocket_updates::{ClientMsg, ServerMsg},
    sync_engine::websocket_updates::binary_transport::{
        BinaryTransportError, BinaryTransportTrait, ConnOrClosedError, JsonSerdeToBinaryCodec,
    },
};
use codee::{Decoder, Encoder};
use futures::{Sink, Stream};
use pin_project::pin_project;
use std::fmt::Debug;
use std::task::{ready, Poll};
use url::Url;

#[derive(Clone)]
#[pin_project]
pub struct Transport<I> {
    #[pin]
    inner: I,
}

impl<I> Sink<ClientMsg> for Transport<I>
where
    I: Sink<Vec<u8>>,
{
    type Error = BinaryTransportError<<I as Sink<Vec<u8>>>::Error>;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.inner.poll_ready(cx)).map_err(BinaryTransportError::Conn))
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: ClientMsg) -> Result<(), Self::Error> {
        let this = self.project();
        let encoded =
            JsonSerdeToBinaryCodec::encode(&item).map_err(BinaryTransportError::Encode)?;
        this.inner
            .start_send(encoded)
            .map_err(BinaryTransportError::Conn)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.inner.poll_flush(cx)).map_err(BinaryTransportError::Conn))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.inner.poll_close(cx)).map_err(BinaryTransportError::Conn))
    }
}

impl<I> Stream for Transport<I>
where
    I: Stream<Item = Result<Vec<u8>, ConnOrClosedError<<I as Sink<Vec<u8>>>::Error>>>
        + Sink<Vec<u8>>,
{
    type Item = Result<ServerMsg, BinaryTransportError<<I as Sink<Vec<u8>>>::Error>>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let encoded = ready!(this.inner.poll_next(cx));
        let encoded = match encoded {
            Some(e) => e,
            None => return Poll::Ready(None),
        };

        let encoded = match encoded {
            Ok(e) => e,
            Err(err) => {
                return Poll::Ready(Some(Err(match err {
                    ConnOrClosedError::Closed => BinaryTransportError::Closed,
                    ConnOrClosedError::Conn(err) => BinaryTransportError::Conn(err),
                })))
            }
        };
        let decoded = match JsonSerdeToBinaryCodec::decode(&encoded) {
            Ok(d) => d,
            Err(err) => {
                let err = BinaryTransportError::Decode(err);
                return Poll::Ready(Some(Err(err)));
            }
        };
        Poll::Ready(Some(Ok(decoded)))
    }
}

pub trait TransportTrait:

    Sized + Sink<ClientMsg> + Stream<Item = Result<ServerMsg, Self::TransportError>>
    + 'static
{
    type TransportError: Debug;
}

impl<T: BinaryTransportTrait> Transport<T> {
    pub async fn new(
        url: Url,
    ) -> Result<Self, <Transport<T> as TransportTrait>::TransportError> {
        let inner = T::establish_conn(url)
            .await
            .map_err(BinaryTransportError::Conn)?;

        Ok(Transport { inner })
    }
}

impl<T> TransportTrait for Transport<T>
where
    T: BinaryTransportTrait,
{
    type TransportError = BinaryTransportError<<T as Sink<Vec<u8>>>::Error>;
}

#[cfg(test)]
pub mod tests;
