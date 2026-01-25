use std::io;
use std::io::ErrorKind;

mod owned;

pub use owned::*;

#[derive(Debug)]
pub struct ReadError {
    pub message: String,
}

impl ReadError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub type BBReadResult<T> = Result<T, ReadError>;

impl From<ReadError> for io::Error {
    fn from(_: ReadError) -> Self {
        ErrorKind::UnexpectedEof.into()
    }
}
