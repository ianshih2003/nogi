use core::fmt;
use std::{error::Error, num::ParseIntError};

#[derive(Debug, PartialEq)]
pub enum ErrorWrapper {
    InvalidFen,
    InvalidCoordinates,
    InvalidNumber,
}

impl fmt::Display for ErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorWrapper::InvalidFen => write!(f, "Invalid FEN"),
            ErrorWrapper::InvalidCoordinates => write!(f, "Invalid Coordinates"),
            ErrorWrapper::InvalidNumber => write!(f, "Invalid Number"),
        }
    }
}

impl Error for ErrorWrapper {}

impl From<ParseIntError> for ErrorWrapper {
    fn from(_: ParseIntError) -> Self {
        ErrorWrapper::InvalidNumber
    }
}
