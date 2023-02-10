use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::errors::ServerError;
use super::ApiError;

#[derive(Debug)]
pub enum QueryError {
    Reqwest(reqwest::Error),
    ApiError(ApiError),
    ServerError(ServerError),
    Deserialize(serde_json::Error, String),
    Generic(String)
}

impl Display for QueryError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for QueryError {}

impl From<reqwest::Error> for QueryError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<ServerError> for QueryError {
    fn from(e: ServerError) -> Self {
        Self::ServerError(e)
    }
}

