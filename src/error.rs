use multipart::client::lazy::LazyIoError as MultipartLazyIoError;
use openssl::error::Error as OpensslError;
use openssl::error::ErrorStack as OpensslErrorStack;
use openssl::ssl::Error as SslError;
use serde_json::Error as SerdeJsonError;
use serde_urlencoded::ser::Error as SerdeUrlEncodeSerError;
use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use std::result::Result;
use std::string::FromUtf8Error;
use std::time::SystemTimeError;
use struct_map::StructMapError;
use ureq::Error as UreqError;

#[derive(Debug)]
pub struct AlipayError(String);

impl AlipayError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        AlipayError(message.into())
    }
}

impl fmt::Display for AlipayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "alipay error: {}", self.0)
    }
}

impl Error for AlipayError {}

impl From<IOError> for AlipayError {
    fn from(error: IOError) -> Self {
        AlipayError::new(error.to_string())
    }
}

impl From<OpensslError> for AlipayError {
    fn from(error: OpensslError) -> Self {
        AlipayError::new(error.to_string())
    }
}
impl From<OpensslErrorStack> for AlipayError {
    fn from(error: OpensslErrorStack) -> Self {
        AlipayError::new(error.to_string())
    }
}
impl From<SslError> for AlipayError {
    fn from(error: SslError) -> Self {
        AlipayError::new(error.to_string())
    }
}
impl From<UreqError> for AlipayError {
    fn from(error: UreqError) -> Self {
        AlipayError::new(error.to_string())
    }
}
impl From<SerdeJsonError> for AlipayError {
    fn from(error: SerdeJsonError) -> Self {
        AlipayError::new(error.to_string())
    }
}
impl From<SerdeUrlEncodeSerError> for AlipayError {
    fn from(error: SerdeUrlEncodeSerError) -> Self {
        AlipayError::new(error.to_string())
    }
}
impl From<FromUtf8Error> for AlipayError {
    fn from(error: FromUtf8Error) -> Self {
        AlipayError::new(error.to_string())
    }
}
impl From<MultipartLazyIoError<'_>> for AlipayError {
    fn from(error: MultipartLazyIoError<'_>) -> Self {
        AlipayError::new(error.to_string())
    }
}
impl From<StructMapError> for AlipayError {
    fn from(error: StructMapError) -> Self {
        AlipayError::new(error.to_string())
    }
}
impl From<SystemTimeError> for AlipayError {
    fn from(error: SystemTimeError) -> Self {
        AlipayError::new(error.to_string())
    }
}

pub type AlipayResult<T> = Result<T, AlipayError>;
