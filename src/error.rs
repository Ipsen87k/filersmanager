use std::fmt::{self,Display};


#[derive(Debug,Clone)]
pub enum Error {
    AsyncTokioIoError(tokio::io::ErrorKind),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::AsyncTokioIoError(e)=>write!(f,"{}",e),
        }
    }
}