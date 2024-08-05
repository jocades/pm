use derive_more::From;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Custom(String),
    #[from]
    Io(std::io::Error),
    #[from]
    Parse(serde_json::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Error {
        Error::Custom(e.to_string())
    }
}
