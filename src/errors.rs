use thiserror::Error;
use async_graphql::{ErrorExtensions, FieldError};

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Could not find resource")]
    NotFoundError(String),

    #[error("ServerError")]
    ServerError(String),

    #[error("No Extensions")]
    ErrorWithoutExtensions,
}

impl ErrorExtensions for QueryError {
    // lets define our base extensions
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            QueryError::NotFoundError(resource) => e.set("code", format!("{} not found", resource)),
            QueryError::ServerError(reason) => e.set("reason", reason.to_string()),
            QueryError::ErrorWithoutExtensions => {}
        })
    }
}