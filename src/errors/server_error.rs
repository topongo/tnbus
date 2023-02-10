use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ServerError {
    Authentication(String),
    Generic(String)
}

impl ServerError {
    pub(crate) fn new(raw: String) -> ServerError {
        if raw.contains("401") {
            ServerError::Authentication(raw.replace("HTTP Status 401 - ", ""))
        } else {
            ServerError::Generic(raw)
        }
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ServerError {

}