use std::fmt;
#[derive(Debug)]
pub struct DeriveError(String);
impl DeriveError {
    pub fn new<S: Into<String>>(content: S) -> Self {
        DeriveError(content.into())
    }
}

impl fmt::Display for DeriveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type DeriveResult<T> = Result<T, DeriveError>;
