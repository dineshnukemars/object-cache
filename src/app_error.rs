use std::fmt::{Display, Formatter};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned()
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "** Bang at\n     {}", &self.message)
    }
}

pub trait MapError<T> {
    fn map_to_app_error(self, message: &str) -> Result<T, AppError>;
}

impl<T, E: Display> MapError<T> for Result<T, E> {
    fn map_to_app_error(self, message: &str) -> Result<T, AppError> {
        match self {
            Ok(data) => {
                Ok(data)
            }
            Err(e) => {
                Err(AppError::new(&format!("{}\n      {}", message, e)))
            }
        }
    }
}