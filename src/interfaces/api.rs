use std::collections::HashMap;
use std::time::Duration;
use reqwest::blocking::{Client, ClientBuilder, Response};
use serde_json;
use crate::errors::{QueryError, ApiError, InterfaceError, ServerError};
use crate::single_map;
use crate::structures::{Area, BasicRoute, Route};


pub struct Api {
    secret: String,
    client: Client
}

enum MethodType {
    Get,
    Post
}

struct Method {
    ty: MethodType,
    body: Option<HashMap<String, String>>
}

impl Api {
    const BASE_URI: &'static str = "https://app-tpl.tndigit.it/gtlservice";

    pub fn new(secret: String) -> Self {
        Api {
            secret,
            client: ClientBuilder::new().timeout(Duration::new(5, 0)).build().unwrap()
        }
    }

    fn request(&self, method: Method, url: &str) -> Result<String, QueryError> {
        let full_uri = format!("{}/{}", Self::BASE_URI, url);
        let request = match method.ty {
            MethodType::Get => self.client.get(full_uri),
            MethodType::Post => self.client.post(full_uri)
        }.header("Authorization", format!("Basic {}", self.secret));
        let response = if let Some(body) = method.body {
            request.query(&body).send()?
        } else {
            request.send()?
        };

        let raw = response.text()?;

        if raw.starts_with("HTTP Status ") {
            Err(QueryError::ServerError(ServerError::new(raw)))
        } else {
            Ok(raw)
        }
    }

    pub(crate) fn get(&self, url: &str) -> Result<String, QueryError> {
        self.get_with_params(url, HashMap::new())
    }

    pub(crate) fn get_with_params(&self, url: &str, params: HashMap<String, String>) -> Result<String, QueryError> {
        self.request(Method { ty: MethodType::Get, body: Some(params) }, url)
    }

    pub(crate) fn post(&self, url: &str, body: HashMap<String, String>) -> Result<String, QueryError> {
        self.request(Method { ty: MethodType::Post, body: Some(body) }, url)
    }

    pub(crate) fn gen_server_error(raw: String, original: serde_json::Error) -> QueryError {
        match serde_json::from_str::<ApiError>(&raw) {
            Ok(se) => QueryError::ApiError(se),
            Err(_) => QueryError::Deserialize(original, raw)
        }
    }
}


