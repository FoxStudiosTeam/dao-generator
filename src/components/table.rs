use serde::{Deserialize, Serialize};
use hashbrown::HashMap;

use crate::prelude::{Field, Type};

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct RawTable {
    pub name: String,
    #[serde(rename="abstract")]
    #[serde(default)]
    pub is_abstract: bool,
    pub extends: Option<String>,
    pub schema: Option<String>,
    pub fields: Vec<Field>,
}

impl RawTable {
    pub fn extend(&mut self, other: RawTable) {
        self.fields.extend(other.fields);
        if self.schema.is_none() {
            self.schema = other.schema;
        }
    }
}


impl RawTable {
    pub fn complete(mut self, types: &HashMap<String, Type>) -> Result<Table, String> {
        let Some(schema) = self.schema else {
            return Err("Schema is required".to_string());
        };
        for field in self.fields.iter_mut() {
            let Some(t) = types.get(&field.tp) else {
                return Err(format!("Unknown type {}", field.tp));
            };
            field.tp = t.rust_type.clone();
        }
        Ok(Table {
            name: self.name,
            schema,
            fields: self.fields,
        })
    }
}

#[derive(Clone, Serialize, Default, Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub schema: String,
    pub fields: Vec<Field>,
}
