mod client;
pub use client::Client;

mod message;
use message::{Message, Response};

mod connection;
use connection::Connection;

pub mod server;

mod cmd;
pub use cmd::Command;

mod error;
pub use error::{Error, Result};

pub const LOCAL_HOST: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 8421;
