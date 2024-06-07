use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("DTO is invalid: {0}")]
pub struct DtoValidationError(pub String);

impl IntoResponse for DtoValidationError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InternalServerError(i) => {
                (StatusCode::INTERNAL_SERVER_ERROR, i.to_string()).into_response()
            }
            AppError::ValidationError(v) => v.into_response(),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
        }
    }
}

pub enum AppError {
    InternalServerError(anyhow::Error),
    ValidationError(DtoValidationError),
    Unauthorized,
}

impl From<anyhow::Error> for AppError {
    fn from(inner: anyhow::Error) -> Self {
        AppError::InternalServerError(inner)
    }
}

impl From<DtoValidationError> for AppError {
    fn from(inner: DtoValidationError) -> Self {
        AppError::ValidationError(inner)
    }
}
