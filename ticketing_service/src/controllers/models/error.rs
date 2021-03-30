use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use sqlx::error::Error as sqlx_error;
use std::num::ParseIntError;

#[derive(Debug, Display)]
pub enum RepositoryError {
    SQLXError(sqlx_error),
    #[display(fmt = "Customer id must be a positive integer.")]
    ParseParameterError(ParseIntError),
    TicketNotFound,
    QuantityNotFound,
    InsufficientTicketsAvailable,
}

#[derive(Debug, Display)]
pub enum ClientError {
    #[display(fmt = "Please supply a quantity of tickets desired to be reserved.")]
    QuantityNotFound,
}
impl From<ClientError> for RepositoryError {
    fn from(err: ClientError) -> RepositoryError {
        RepositoryError::QuantityNotFound
    }
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

impl ResponseError for RepositoryError {
    fn error_response(&self) -> HttpResponse {
        let detail = &self.to_string();
        HttpResponse::BadRequest().body(detail)
    }
}
