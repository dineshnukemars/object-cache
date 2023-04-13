use std::fmt::{Display, Formatter};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CacheError {
    message: String,
}

impl CacheError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned()
        }
    }
}

impl Display for CacheError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "** Bang at\n     {}", &self.message)
    }
}

pub trait MapError<T> {
    fn map_to_cache_error(self, message: &str) -> Result<T, CacheError>;
}

impl<T, E: Display> MapError<T> for Result<T, E> {
    fn map_to_cache_error(self, message: &str) -> Result<T, CacheError> {
        match self {
            Ok(data) => {
                Ok(data)
            }
            Err(e) => {
                Err(CacheError::new(&format!("{}\n      {}", message, e)))
            }
        }
    }
}