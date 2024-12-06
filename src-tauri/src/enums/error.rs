use crate::models::error::ErrorResponse;

#[derive(Debug)]
pub enum ErrorType {
    UserAlreadyExists,
    UserNotFound,
    InvalidPassword,
    Error(String),
}

pub fn map_error(err: ErrorType) -> ErrorResponse {
    match err {
        ErrorType::UserNotFound => ErrorResponse {
            code: 1,
            error: err.to_string(),
            details: None,
        },
        ErrorType::InvalidPassword => ErrorResponse {
            code: 2,
            error: err.to_string(),
            details: None,
        },
        ErrorType::Error(e) => ErrorResponse {
            code: 3,
            error: "Error al intentar procesar la solicitud".to_string(),
            details: Some(e),
        },
        ErrorType::UserAlreadyExists => ErrorResponse {
            code: 4,
            error: err.to_string(),
            details: None,
        },
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::UserAlreadyExists => write!(f, "Usuario ya existe"),
            ErrorType::UserNotFound => write!(f, "Usuario no encontrado"),
            ErrorType::InvalidPassword => write!(f, "Contraseña incorrecta"),
            ErrorType::Error(err) => write!(f, "Error: {}", err),
        }
    }
}

impl std::error::Error for ErrorType {}