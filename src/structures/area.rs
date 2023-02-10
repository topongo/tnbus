use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AreaType {
    E,
    U
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Area {
    #[serde(alias = "areaId")]
    pub id: u16,
    #[serde(alias = "areaDesc")]
    pub label: String,
    #[serde(alias = "type")]
    pub ty: AreaType
}