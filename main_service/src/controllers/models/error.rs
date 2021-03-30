use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use sqlx::error::Error as sqlx_error;
use std::num::ParseIntError;

#[derive(Debug, Display)]
pub enum RepositoryError {
    SQLXError(sqlx_error),
    #[display(fmt = "Customer id must be a positive integer.")]
    ParseParameterError(ParseIntError),
    AccountNotFound,
    HTTPClientError,
    UnexpectedError,
}

impl From<sqlx_error> for RepositoryError {
    fn from(err: sqlx_error) -> RepositoryError {
        RepositoryError::SQLXError(err)
    }
}

impl From<ParseIntError> for RepositoryError {
    fn from(err: ParseIntError) -> RepositoryError {
        RepositoryError::ParseParameterError(err)
    }
}

impl From<reqwest::Error> for RepositoryError {
    fn from(err: reqwest::Error) -> RepositoryError {
        RepositoryError::HTTPClientError
    }
}

impl ResponseError for RepositoryError {
    fn error_response(&self) -> HttpResponse {
        let detail = &self.to_string();
        HttpResponse::BadRequest().body(detail)
    }
}
