use openssl::error::Error as OpensslError;
use openssl::error::ErrorStack as OpensslErrorStack;
use std::io::Error as IOError;
use std::result::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlipayError {
    #[error("IoError: {0}")]
    IOError(String),
    #[error("OpensslError: {0}")]
    OpensslError(String),
    #[error("OpensslErrorStack: {0}")]
    OpensslErrorStack(String),
}

impl From<IOError> for AlipayError {
    fn from(error: IOError) -> Self {
        AlipayError::IOError(error.to_string())
    }
}

impl From<OpensslError> for AlipayError {
    fn from(error: OpensslError) -> Self {
        AlipayError::OpensslError(error.to_string())
    }
}
impl From<OpensslErrorStack> for AlipayError {
    fn from(error: OpensslErrorStack) -> Self {
        AlipayError::OpensslErrorStack(error.to_string())
    }
}

pub type AlipayResult<T> = Result<T, AlipayError>;
