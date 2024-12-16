use futures::{pin_mut, StreamExt};
use serde::{de::DeserializeOwned, Serialize};

use crate::endpoints::defns::api::websocket_updates::WEBSOCKET_UPDATES_ENDPOINT;

use super::{error::SyncErrorSrc, SyncEngine, SyncResult, WSClient};

/// Why? Because `JsonSerdeCodec` from `codee` encodes to / decodes from str, and I want to be able
/// to be able to interpret web socket messages that are in "binary frames" (or whatever the
/// correct terminology) to also be decoded as JSON.
pub struct JsonSerdeToBinaryCodec;

impl<T: DeserializeOwned> codee::Decoder<T> for JsonSerdeToBinaryCodec {
    type Error = serde_json::Error;

    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        serde_json::from_slice(val)
    }
}

impl<T: Serialize> codee::Encoder<T> for JsonSerdeToBinaryCodec {
    type Error = serde_json::Error;

    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        serde_json::to_vec(val)
    }
}

impl<W> SyncEngine<W>
where
    W: WSClient,
{
    pub async fn recv_websocket_updates(&self) -> SyncResult<(), W::Error> {
        loop {
            let (_, recver) = W::establish(WEBSOCKET_UPDATES_ENDPOINT)
                .await
                .map_err(SyncErrorSrc::WebSocket)?;
            pin_mut!(recver);
            loop {
                let fut = recver.next();
                pin_mut!(fut);
                match fut.await {
                    Some(value) => match value {
                        Ok(server_msg) => {
                            tracing::info!("Received server msg: {server_msg:?}")
                        }
                        Err(err) => {
                            tracing::error!("{:?}", err)
                        }
                    },
                    None => break,
                }
            }
        }
    }
}
