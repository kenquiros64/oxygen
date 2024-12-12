use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ErrorType {
    UserAlreadyExists,
    UserNotFound,
    InvalidPassword,
    NoInternetConnection,
    UnreachableInternet,
    Error,
}

impl Clone for ErrorType {
    fn clone(&self) -> ErrorType {
        match self {
            ErrorType::UserAlreadyExists => ErrorType::UserAlreadyExists,
            ErrorType::UserNotFound => ErrorType::UserNotFound,
            ErrorType::InvalidPassword => ErrorType::InvalidPassword,
            ErrorType::NoInternetConnection => ErrorType::NoInternetConnection,
            ErrorType::UnreachableInternet => ErrorType::UnreachableInternet,
            ErrorType::Error => ErrorType::Error,
        }
    }
}