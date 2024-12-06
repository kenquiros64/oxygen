use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub(crate) code: i32,
    pub(crate) error: String,
    pub(crate) details: Option<String>,
}