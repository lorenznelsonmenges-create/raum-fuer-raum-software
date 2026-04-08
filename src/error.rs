use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    Sqlx(sqlx::Error),
    Internal(String),
    PdfError(String),
    NotFound,
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Sqlx(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Internal(s) => (StatusCode::INTERNAL_SERVER_ERROR, s),
            AppError::PdfError(s) => (StatusCode::INTERNAL_SERVER_ERROR, s),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Ressource nicht gefunden".to_string()),
            AppError::BadRequest(s) => (StatusCode::BAD_REQUEST, s),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Sqlx(err)
    }
}
