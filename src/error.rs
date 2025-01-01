use api_utils::db;
use axum::response::IntoResponse;
use casbin_rb_adapter::casbin;

use thiserror::Error;

use crate::{utils::i18n::get_i18n, vo};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    E(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Db(#[from] db::Error),
    #[error("{0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("{0}: {1}")]
    Code(i32, String),
    #[error("{0}")]
    Casbin(#[from] casbin::error::Error),
    #[error("{0}")]
    Validate(#[from] validator::ValidationErrors),
    #[error("{0}")]
    Utils(#[from] api_utils::Error),
}

impl Error {
    pub fn e<T>(s: impl Into<String>) -> Result<T> {
        Err(Error::E(s.into()))
    }
}

impl Error {
    pub fn to_msg(&self) -> (i32, String) {
        let msg = match self {
            Error::E(s) => (1, s),
            Error::Validate(s) => (2, &s.to_string()),
            Error::Code(code, s) => (*code, s),
            _ => {
                // tracing::info!("error: {:?}", self);
                (3, &"Failed".to_string())
            },
        };
        (msg.0, get_i18n(msg.1))
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::E(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Internal(s)
    }
}

impl From<(i32, &str)> for Error {
    fn from((code, msg): (i32, &str)) -> Self {
        Error::Code(code, msg.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        vo::Response::<()>::error(self).into_response()
    }
}
