use base64::DecodeError as Base64DecodeError;
use openssl::error::Error as OpensslError;
use openssl::error::ErrorStack as OpensslErrorStack;
use openssl::ssl::Error as SslError;
use reqwest::Error as ReqwestError;
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
    #[error("Base64DecodeError: {0}")]
    Base64DecodeError(String),
    #[error("SslError: {0}")]
    SslError(String),
    #[error("ReqwestError: {0}")]
    ReqwestError(String),
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
impl From<Base64DecodeError> for AlipayError {
    fn from(error: Base64DecodeError) -> Self {
        AlipayError::Base64DecodeError(error.to_string())
    }
}
impl From<SslError> for AlipayError {
    fn from(error: SslError) -> Self {
        AlipayError::SslError(error.to_string())
    }
}
impl From<ReqwestError> for AlipayError {
    fn from(error: ReqwestError) -> Self {
        AlipayError::ReqwestError(error.to_string())
    }
}

pub type AlipayResult<T> = Result<T, AlipayError>;
