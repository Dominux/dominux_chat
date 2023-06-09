use axum::http::StatusCode;
use sea_orm::error::DbErr;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("environment variable `{0}` is not set")]
    EnvConfigLoadingError(String),
    #[error("environment variable `{0}` cannot be parsed")]
    EnvVarParsingError(String),
    #[error("cannot establish connection with db")]
    DBConnectionError,
    #[error("not found: `{0}`")]
    NotFound(String),
    #[error("cannot decode token")]
    TokenDecodingError,
    #[error("wrong credentials")]
    AuthenticationError,
    #[error("unknown error")]
    Unknown,
}

impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        match e {
            DbErr::ConnectionAcquire => Self::DBConnectionError,
            DbErr::RecordNotFound(s) => Self::NotFound(s),
            _ => {
                println!("[DB Error] {e}");
                Self::Unknown
            }
        }
    }
}

impl From<AppError> for (StatusCode, String) {
    fn from(e: AppError) -> Self {
        match &e {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, e.to_string()),
            AppError::TokenDecodingError => (StatusCode::UNAUTHORIZED, e.to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_owned(),
            ),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
