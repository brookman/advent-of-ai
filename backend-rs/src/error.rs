use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum CrudError {
    #[error("an entity with ID `{0}` does not exist")]
    UnknownId(Uuid),

    #[error("an entity with ID `{0}` already exists")]
    DuplicateId(Uuid),

    #[error("serialization error")]
    Serializiation,

    #[error("io error")]
    IO,

    #[error("unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
#[error("DTO is invalid: {0}")]
pub struct DtoValidationError(pub String);

impl IntoResponse for CrudError {
    fn into_response(self) -> Response {
        match self {
            CrudError::UnknownId(id) => (StatusCode::BAD_REQUEST, format!("ID {} not found", id)),
            CrudError::DuplicateId(id) => {
                (StatusCode::BAD_REQUEST, format!("ID {} already exists", id))
            }
            CrudError::Serializiation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Serialization issue"),
            ),
            CrudError::IO => (StatusCode::INTERNAL_SERVER_ERROR, format!("IO issue")),
            CrudError::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, format!("Unknown error")),
        }
        .into_response()
    }
}

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
            AppError::CrudError(c) => c.into_response(),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
        }
    }
}

pub enum AppError {
    InternalServerError(anyhow::Error),
    ValidationError(DtoValidationError),
    CrudError(CrudError),
    Unauthorized,
}

impl From<anyhow::Error> for AppError {
    fn from(inner: anyhow::Error) -> Self {
        AppError::InternalServerError(inner)
    }
}

impl From<CrudError> for AppError {
    fn from(inner: CrudError) -> Self {
        AppError::CrudError(inner)
    }
}

impl From<DtoValidationError> for AppError {
    fn from(inner: DtoValidationError) -> Self {
        AppError::ValidationError(inner)
    }
}
