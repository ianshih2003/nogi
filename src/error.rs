use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ErrorWrapper {
    InvalidFen,
}

impl fmt::Display for ErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorWrapper::InvalidFen => write!(f, "Invalid FEN"),
        }
    }
}

impl Error for ErrorWrapper {}
