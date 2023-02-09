use std::collections::HashMap;
use std::time::Duration;
use reqwest::blocking::{Client, ClientBuilder, Response};
use serde_json;
use crate::errors::{QueryError, ApiError};
use crate::structures::Area;


pub(crate) struct Api {
    secret: String,
    client: Client
}

impl Api {
    const BASE_URI: &'static str = "https://app-tpl.tndigit.it/gtlservice";

    pub(crate) fn new(secret: String) -> Self {
        Api {
            secret,
            client: ClientBuilder::new().timeout(Duration::new(5, 0)).build().unwrap()
        }
    }

    fn get(&self, url: &str) -> Result<Response, QueryError> {
        Ok(
            self.client.get(format!("{}/{}", Self::BASE_URI, url))
                .header("Authorization", format!("Basic {}", self.secret))
                .send()?
        )
    }

    fn post(&self, url: &str, body: HashMap<String, String>) -> Result<Response, QueryError> {
        Ok(
            self.client.post(format!("{}/{}", Self::BASE_URI, url))
                .header("Authorization", format!("Basic {}", self.secret))
                .json(&body)
                .send()?
        )
    }

    pub fn areas(&self) -> Result<Vec<Area>, QueryError> {
        let res = self.get("areadaws")?;
        let raw = res.text()?;
        serde_json::from_str(&raw).or_else(|_| Err(Self::gen_server_error(raw)))
    }

    fn gen_server_error(raw: String) -> QueryError {
        match serde_json::from_str::<ApiError>(&raw) {
            Ok(se) => QueryError::ServerError(se),
            Err(e) => QueryError::Deserialize(e)
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;
    use super::Api;

    fn read_secret() -> String {
        let mut file = File::open("auth").unwrap_or_else(|_| { panic!("Cannot open auth file") });
        let mut secret = String::new();
        file.read_to_string(&mut secret).expect("Cannot read auth file");

        secret
    }

    #[test]
    fn test_plain() {
        let interface = Api::new(read_secret());
        println!("GETing to /test ...");
        match interface.get("/test") {
            Ok(r) => {
                let response_text = r.text().unwrap();
                println!("Response:\n\n====================\n{}\n====================", response_text)
            },
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_areas() {
        let interface = Api::new(read_secret());
        println!("Calling interface.areas()...\n");
        println!("{:?}", interface.areas());
    }
}

