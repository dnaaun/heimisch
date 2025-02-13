use codee::{Decoder, Encoder};
use futures::{Sink, Stream};
use url::Url;
use std:: fmt::Debug;

use crate::endpoints::defns::api::websocket_updates::{ClientMsg, ServerMsg};

/// Why? Because `JsonSerdeCodec` from `codee` encodes to / decodes from str, and I want to be able
/// to be able to interpret web socket messages that are in "binary frames" (or whatever the
/// correct terminology) to also be decoded as JSON.
pub struct JsonSerdeToBinaryCodec;

impl<T: serde::de::DeserializeOwned> codee::Decoder<T> for JsonSerdeToBinaryCodec {
    type Error = serde_json::Error;

    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        serde_json::from_slice(val)
    }
}

impl<T: serde::Serialize> codee::Encoder<T> for JsonSerdeToBinaryCodec {
    type Error = serde_json::Error;

    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        serde_json::to_vec(val)
    }
}
pub enum BinaryTransportError<Conn> {
    Closed,
    Conn(Conn),
    Encode(<JsonSerdeToBinaryCodec as Encoder<ClientMsg>>::Error),
    Decode(<JsonSerdeToBinaryCodec as Decoder<ServerMsg>>::Error),
}

impl<Conn> std::fmt::Debug for BinaryTransportError<Conn>
where
    Conn: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryTransportError::Closed => f.write_str("TypedError::Closed"),
            BinaryTransportError::Conn(e) => f.write_fmt(format_args!("TypedError::Conn({e:?})")),
            BinaryTransportError::Encode(e) => {
                f.write_fmt(format_args!("TypedError::Encode({e:?})"))
            }
            BinaryTransportError::Decode(e) => {
                f.write_fmt(format_args!("TypedError::Decode({e:?})"))
            }
        }
    }
}

pub enum ConnOrClosedError<Conn> {
    Closed,
    Conn(Conn),
}

pub trait BinaryTransportTrait:
    Sized
    + Sink<Vec<u8>, Error = Self::ConnError>
    + Stream<Item = Result<Vec<u8>, ConnOrClosedError<Self::ConnError>>>
{
    type ConnError: Debug;

    #[allow(async_fn_in_trait)]
    async fn establish_conn(url: Url) -> Result<Self, Self::ConnError>;
}
