use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Other(String),
    Io(std::io::Error),
    Parse(String),
}

impl std::error::Error for Error {}

// unsafe impl Send for Error {}
// unsafe impl Sync for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => e.fmt(f),
            Error::Other(e) => e.fmt(f),
            Error::Parse(e) => e.fmt(f),
        }
    }
}

impl From<String> for Error {
    fn from(e: String) -> Error {
        Error::Other(e)
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Error {
        Error::Other(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        match e.classify() {
            serde_json::error::Category::Io => Error::Io(e.into()),
            serde_json::error::Category::Eof => "stream ended early".into(),
            serde_json::error::Category::Syntax => {
                Error::Parse(format!("protocol error; invalid JSON {e}"))
            }
            serde_json::error::Category::Data => {
                Error::Parse(format!("protocol error; {e}"))
            }
        }
    }
}
