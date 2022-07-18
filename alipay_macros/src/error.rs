use std::fmt;

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Error(content.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "structmap error: {}", self.0)
    }
}

impl std::error::Error for Error {}
