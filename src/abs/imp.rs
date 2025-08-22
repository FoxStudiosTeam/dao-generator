use std::{collections::HashMap};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Type {
    #[serde(rename = "rustType")]
    pub rust_type: String,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub tp: String,
    pub is_primary: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Table {
    pub name: String,
    pub extends: Option<String>,
    pub schema: Option<String>,
    pub fields: Vec<Field>,
    pub types: Option<HashMap<String, Type>>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct YamlData {
    pub tables: Option<Vec<Table>>,
    pub types: Option<HashMap<String, Type>>,
}