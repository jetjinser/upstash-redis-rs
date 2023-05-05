use reqwest::{header::InvalidHeaderValue, Error as ReqError};
use url::ParseError;
use serde_json::Error as SerdeError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RedisError>;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("build Redis client failed")]
    ClientError(#[from] ReqError),
    #[error("Invalid header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("parse Url error")]
    UrlError(#[from] ParseError),
    #[error("serialize or deserialize error")]
    SerdeError(#[from] SerdeError),
}
