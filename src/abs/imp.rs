use std::{any::Any, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::abs::iface::{AsAny, IField, ITable, IType, IYamlData};

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
struct Type {
    #[serde(rename = "rustType")]
    rust_type: String,
}

impl IType for Type {
    fn get_rust_type(&self) -> &str {
        &self.rust_type
    }    
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
struct Field {
    name: String,
    #[serde(rename = "type")]
    tp: String,
    is_primary: Option<bool>,
}

impl IField for Field {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> &str {
        &self.tp
    }

    fn get_is_primary(&self) -> &Option<bool> {
        &self.is_primary
    }
    fn clone_box(&self) -> Box<dyn IField> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn IField> {
    fn clone(&self) -> Box<dyn IField> {
        self.clone_box()
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
struct Table {
    name: String,
    extends: Option<String>,
    schema: Option<String>,
    fields: Vec<Field>,
    types: Option<HashMap<String, Type>>,
}

impl ITable for Table {
    fn get_schema(&self) -> &Option<String> {
        &self.schema
    }

    fn get_fields(&self) -> Vec<Box<dyn IField>> {
        self.fields
            .iter()
            .map(|f| Box::new(f.clone()) as Box<dyn IField>)
            .collect()
    }

    fn get_extends(&self) -> &Option<String> {
        &self.extends
    }

    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_types(&self) -> Option<HashMap<String, Box<dyn IType>>> {
        self.types.as_ref().map(|map| {
            map.iter()
                .map(|(k, v)| {
                    let boxed: Box<dyn IType> = Box::new(v.clone());
                    (k.clone(), boxed)
                })
                .collect()
        })
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct YamlData {
    tables: Option<Vec<Table>>,
    types: Option<HashMap<String, Type>>,
}

impl IYamlData for YamlData {
    
    fn get_tables(&self) -> Option<Vec<Box<dyn ITable>>> {
        self.tables.as_ref().map(|vec| {
            vec.iter()
                .cloned()
                .map(|t| Box::new(t) as Box<dyn ITable>)
                .collect()
        })
    }

    fn get_types(&self) -> Option<HashMap<String, Box<dyn IType>>> {
        self.types.as_ref().map(|map| {
            map.iter()
                .map(|(k, v)| {
                    let boxed: Box<dyn IType> = Box::new(v.clone());
                    (k.clone(), boxed)
                })
                .collect()
        })
    }
    
    fn set_tables(&mut self, tables: Vec<Box<dyn ITable>>) {
        let concrete: Vec<Table> = tables
            .into_iter()
            .filter_map(|t| t.as_any().downcast_ref::<Table>().cloned())
            .collect();

        self.tables = Some(concrete);
    }

    fn set_types(&mut self, types: HashMap<String, Box<dyn IType>>) {
        let concrete: HashMap<String, Type> = types
            .into_iter()
            .filter_map(|(k, v)| {
                v.as_any()
                    .downcast_ref::<Type>()
                    .cloned()
                    .map(|typed| (k, typed))
            })
            .collect();

        self.types = Some(concrete);
    }
}
