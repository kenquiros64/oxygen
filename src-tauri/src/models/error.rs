use serde::Serialize;
use crate::enums::error::ErrorType;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub code: ErrorType,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: ErrorType, message: &str) -> Self {
        ErrorResponse {
            code,
            message: message.to_string(),
        }
    }
}

impl Clone for ErrorResponse {
    fn clone(&self) -> ErrorResponse {
        ErrorResponse {
            code: self.code.clone(),
            message: self.message.clone(),
        }
    }
}
