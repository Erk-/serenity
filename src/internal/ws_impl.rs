use flate2::read::ZlibDecoder;
use gateway::GatewayError;
use internal::prelude::*;
use serde_json;
use websocket::{
    Message,
    protocol::WebSocket,
    client::AutoStream,
};

pub trait ReceiverExt {
    fn recv_json(&mut self) -> Result<Option<Value>>;
}

pub trait SenderExt {
    fn send_json(&mut self, value: &Value) -> Result<()>;
}

impl ReceiverExt for WebSocket<AutoStream> {
    fn recv_json(&mut self) -> Result<Option<Value>> {
        Ok(match self.read_message()? {
            Message::Binary(bytes) => {
                serde_json::from_reader(ZlibDecoder::new(&bytes[..])).map(Some)?
            },
            //Message::Close(data) => return Err(Error::Gateway(GatewayError::Closed(data))),
            Message::Text(payload) => {
                serde_json::from_str(&payload).map(Some)?
            },
            Message::Ping(x) => {
                self.write_message(Message::Pong(x))
                    .map_err(Error::from)?;

                None
            },
            Message::Pong(_) => None,
        })
    }
}

impl SenderExt for WebSocket<AutoStream> {
    fn send_json(&mut self, value: &Value) -> Result<()> {
        serde_json::to_string(value)
            .map(Message::Text)
            .map_err(Error::from)
            .and_then(|m| self.write_message(m).map_err(Error::from))
    }
}
