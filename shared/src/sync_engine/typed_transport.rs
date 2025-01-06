use codee::{Decoder, Encoder};
use futures::{Sink, Stream};
use pin_project::pin_project;
use std::{
    fmt::Debug,
    task::{ready, Poll},
};
use url::Url;

use crate::endpoints::defns::api::websocket_updates::{ClientMsg, ServerMsg};

use super::JsonSerdeToBinaryCodec;

pub enum TypedTransportError<Conn> {
    Closed,
    Conn(Conn),
    Encode(<JsonSerdeToBinaryCodec as Encoder<ClientMsg>>::Error),
    Decode(<JsonSerdeToBinaryCodec as Decoder<ServerMsg>>::Error),
}

impl<Conn> Debug for TypedTransportError<Conn>
where
    Conn: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedTransportError::Closed => f.write_str("TypedError::Closed"),
            TypedTransportError::Conn(e) => f.write_fmt(format_args!("TypedError::Conn({e:?})")),
            TypedTransportError::Encode(e) => {
                f.write_fmt(format_args!("TypedError::Encode({e:?})"))
            }
            TypedTransportError::Decode(e) => {
                f.write_fmt(format_args!("TypedError::Decode({e:?})"))
            }
        }
    }
}

pub enum ConnOrClosedError<Conn> {
    Closed,
    Conn(Conn),
}

pub trait TypedTransportTrait:
    Sized
    + Sink<Vec<u8>, Error = Self::ConnError>
    + Stream<Item = Result<Vec<u8>, ConnOrClosedError<Self::ConnError>>>
{
    type ConnError: Debug;

    #[allow(async_fn_in_trait)]
    async fn establish_conn(url: &Url) -> Result<Self, Self::ConnError>;
}

pub async fn establish<T>(url: &Url) -> Result<TypedTransport<T>, TypedTransportError<T::ConnError>>
where
    T: TypedTransportTrait,
{
    let inner = T::establish_conn(url)
        .await
        .map_err(TypedTransportError::Conn)?;

    Ok(TypedTransport { inner })
}

#[pin_project]
pub struct TypedTransport<I> {
    #[pin]
    inner: I,
}

impl<I> Sink<ClientMsg> for TypedTransport<I>
where
    I: Sink<Vec<u8>>,
{
    type Error = TypedTransportError<<I as Sink<Vec<u8>>>::Error>;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.inner.poll_ready(cx)).map_err(TypedTransportError::Conn))
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: ClientMsg) -> Result<(), Self::Error> {
        let this = self.project();
        let encoded =
            JsonSerdeToBinaryCodec::encode(&item).map_err(TypedTransportError::Encode)?;
        this.inner
            .start_send(encoded)
            .map_err(TypedTransportError::Conn)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.inner.poll_flush(cx)).map_err(TypedTransportError::Conn))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        let this = self.project();
        Poll::Ready(ready!(this.inner.poll_close(cx)).map_err(TypedTransportError::Conn))
    }
}

impl<I> Stream for TypedTransport<I>
where
    I: Stream<Item = Result<Vec<u8>, ConnOrClosedError<<I as Sink<Vec<u8>>>::Error>>>
        + Sink<Vec<u8>>,
{
    type Item = Result<ServerMsg, TypedTransportError<<I as Sink<Vec<u8>>>::Error>>;

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
                    ConnOrClosedError::Closed => TypedTransportError::Closed,
                    ConnOrClosedError::Conn(err) => TypedTransportError::Conn(err),
                })))
            }
        };
        let decoded = match JsonSerdeToBinaryCodec::decode(&encoded) {
            Ok(d) => d,
            Err(err) => {
                let err = TypedTransportError::Decode(err);
                return Poll::Ready(Some(Err(err)));
            }
        };
        Poll::Ready(Some(Ok(decoded)))
    }
}
