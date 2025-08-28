use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub tp: String,
    #[serde(rename = "isPrimary")]
    #[serde(default)]
    pub is_primary: bool,
}

