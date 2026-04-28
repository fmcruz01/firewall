use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Error {
    message: String,
    err_type: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    ArgParseError,
}

impl Error {
    pub fn err(err_type: ErrorType, message: String) -> Self {
        Error { message, err_type }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}", self.err_type, self.message)
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
