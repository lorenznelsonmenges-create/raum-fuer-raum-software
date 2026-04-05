use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Datenbankfehler: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Nicht gefunden")]
    NotFound,
    
    #[error("Ungültige Anfrage: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                // Intern loggen wir den echten Fehler
                eprintln!("Interner Datenbankfehler: {:?}", e);
                // Dem Client senden wir eine neutrale Meldung
                (StatusCode::INTERNAL_SERVER_ERROR, "Ein interner Datenbankfehler ist aufgetreten.".to_string())
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Die angeforderte Ressource wurde nicht gefunden.".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
