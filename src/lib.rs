mod client;
pub use client::Client;

mod connection;
pub use connection::Connection;

pub mod server;

mod cmd;
pub use cmd::Command;

mod state;
use state::State;

mod error;
pub use error::{Error, Result};

pub const LOCAL_HOST: &'static str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 8421;
