// Imports
// Internal crates
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::Value;
use serde_json::json;

// External crates
use std::fmt::{Display, Formatter, Result};

// Custom error enum
pub enum ServerError {
    SetupError(String),
}

// Implementations
impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ServerError::SetupError(msg) => write!(f, "{}", msg),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status_code, msg) = match self {
            ServerError::SetupError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let response_body: Json<Value> = Json(json!({
            "message": msg
        }));

        (status_code, response_body).into_response()
    }
}
