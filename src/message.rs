use crate::Command;

use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, From)]
pub enum Message {
    Request(Command),
    Response(Response),
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Message> {
        Ok(serde_json::from_slice(bytes)?)
    }

    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        let mut bytes = serde_json::to_vec(self)?;
        bytes.push(b'\n');
        Ok(bytes)
    }
}

#[derive(Serialize, Deserialize, Debug, From)]
pub struct Response {
    pub ok: bool,
    pub msg: String,
}

impl Response {
    pub fn new<T: Into<String>>(ok: bool, msg: T) -> Response {
        Response {
            ok,
            msg: msg.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::Ping;

    fn accept<T: Into<Message>>(msg: T) -> Message {
        msg.into()
    }

    #[test]
    fn request() {
        let cmd = Command::from(Ping::new(Some("hello")));
        let msg = accept(cmd);

        println!("{msg:?}");
        assert!(matches!(msg, Message::Request(_)));
    }

    #[test]
    fn response() {
        let res = Response::new(true, "pong");
        let msg = accept(res);

        println!("{msg:?}");
        assert!(matches!(msg, Message::Response(_)));
    }

    #[test]
    fn request_from_bytes() -> crate::Result<()> {
        let out_msg: Message = Command::from(Ping::new(Some("hello"))).into();
        let bytes = out_msg.to_bytes()?;

        let in_msg = Message::from_bytes(&bytes)?;
        println!("{in_msg:?}");
        Ok(())
    }

    #[test]
    fn response_from_bytes() -> crate::Result<()> {
        let msg: Message = Response::new(true, "pong").into();
        let bytes = msg.to_bytes()?;

        let in_msg = Message::from_bytes(&bytes)?;
        println!("{in_msg:?}");
        Ok(())
    }

    #[test]
    fn request_to_bytes() -> crate::Result<()> {
        let out_msg: Message = Command::from(Ping::new(Some("hello"))).into();
        let bytes = out_msg.to_bytes()?;

        let in_msg = Message::from_bytes(&bytes)?;
        println!("{in_msg:?}");
        Ok(())
    }

    #[test]
    fn response_to_bytes() -> crate::Result<()> {
        let msg: Message = Response::new(true, "pong").into();
        let bytes = msg.to_bytes()?;

        let in_msg = Message::from_bytes(&bytes)?;
        println!("{in_msg:?}");
        Ok(())
    }
}
