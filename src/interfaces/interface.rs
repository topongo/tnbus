use std::collections::HashMap;
use crate::errors::{InterfaceError, QueryError};
use crate::interfaces::Api;
use crate::single_map;
use crate::structures::{Area, BasicRoute, Route};
use chrono::{DateTime, Utc};

enum OptionalMultipleAreas {
    Single(Area),
    Multiple()
}

struct ExpiringContent<K, V> {
    expire_on: DateTime<Utc>,
    content: HashMap<K, V>
}

type LazyContent<K, V> = Option<ExpiringContent<K, V>>;

pub struct Interface {
    api: Api,
    areas: LazyContent<u16, Area>,
    routes: LazyContent<String, Route>,
}

impl Interface {    
    pub fn new(secret: String) -> Interface {
        Interface {
            api: Api::new(secret),
            areas: None,
            routes: None,
        }
    }
    
    pub fn new_no_lazy(secret: String) -> Interface {
        let api = Api::new(secret);
        let areas = Self::areas_static(&api).unwrap();
        Interface { api, areas: Some(
            areas
                .iter()
                .map(|a| {
                    (a.id, a.clone())
                })
                .collect()
        ) }
    }

    fn areas_static(api: &Api) -> Result<Vec<Area>, QueryError> {
        let raw = api.get("areas")?;
        serde_json::from_str(&raw).or_else(|e| Err(Api::gen_server_error(raw, e)))
    }

    pub fn areas(&self) -> Result<Vec<Area>, QueryError> {
        Self::areas_static(&self.api)
    }

    pub fn routes(&self, areas: Vec<Area>) -> Result<Vec<Route>, QueryError> {
        let area_ids: String = areas
            .iter()
            .map(|a| {
                a.id.to_string() + ","
            })
            .collect();
        let raw = self.api.get_with_params("routes", single_map!("areas".to_string(), area_ids))?;
        let des: Result<Vec<BasicRoute>, serde_json::Error> = serde_json::from_str(&raw);
        match des {
            Ok(v) => Ok(self.process_basic_routes(v)),
            Err(e) => Err(Api::gen_server_error(raw, e))
        }
    }

    fn process_basic_routes(&self, mut br: Vec<BasicRoute>) -> Vec<Route> {
        let mut routes = Vec::new();
        loop {
            match br.pop() {
                Some(r) => {
                    let area_target = match &self.area {
                        OptionalMultipleAreas::Multiple(m) => m.get(&r.area_id).unwrap().clone(),
                        OptionalMultipleAreas::Single(a) => {
                            assert_eq!(a.id, r.area_id);
                            a.clone()
                        }
                    };
                    routes.push(Route::from_basic(r, area_target));
                },
                None => break
            }
        }
        routes
    }
}