use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use serde_json::value::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BasicNews {
    pub id_feed: Option<u16>,
    #[serde(alias = "agencyId")]
    pub agency_id: String,
    #[serde(alias = "serviceType")]
    pub service_type: String,
    #[serde(alias = "startDate")]
    pub start_date: DateTime<Utc>,
    #[serde(alias = "endDate")]
    pub end_date: DateTime<Utc>,
    pub header: String,
    pub details: String,
    #[serde(alias = "endDate")]
    pub stop_id: String,
    pub url: String,

    #[serde(flatten)]
    extras: HashMap<String, Value>
}

#[derive(Serialize, Debug)]
pub struct News {
    pub id_feed: Option<u16>,
    pub agency_id: u16,
    pub service_type: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub header: String,
    pub details: String,
    pub stop_id: String,
    pub url: String,
    //pub route_ids: Vec<u16>,
}

impl News {
    pub(crate) fn from_basic(bn: BasicNews) -> News {
        News {
            id_feed: bn.id_feed,
            agency_id: bn.agency_id.parse::<u16>().unwrap(),
            service_type: bn.service_type,
            start_date: bn.start_date,
            end_date: bn.end_date,
            header: bn.header,
            details: bn.details,
            stop_id: bn.stop_id,
            url: bn.url,
            //route_ids: bn.route_ids,
        }
    }
}

