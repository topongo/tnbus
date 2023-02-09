use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum AreaType {
    E,
    U
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Area {
    #[serde(alias = "areaId")]
    id: u8,
    #[serde(alias = "areaDesc")]
    description: String,
    #[serde(alias = "type")]
    ty: AreaType
}

