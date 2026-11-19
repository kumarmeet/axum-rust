use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("post {0} not found")]
    NotFound(u32),
    #[error("title must be not empty")]
    EmptyTitle,
    #[error(transparent)]
    BadJson(#[from] JsonRejection),
}

// without implement this trait, axum will not access any custom type
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::EmptyTitle => StatusCode::BAD_REQUEST,
            ApiError::BadJson(rejection) => rejection.status(),
        };

        let message = match &self {
            ApiError::BadJson(rejection) => rejection.body_text(),
            other => other.to_string(),
        };

        (status, Json(json!({"error": message}))).into_response()
    }
}
