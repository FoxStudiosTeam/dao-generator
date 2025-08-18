use std::{any::Any, collections::HashMap};

pub trait ITable : AsAny{
    fn get_schema(&self) -> &Option<String>;
    fn get_fields(&self) -> Vec<Box<dyn IField>>;
    fn get_extends(&self) -> &Option<String>;
    fn get_name(&self) -> &str;
    fn get_types(&self) -> Option<HashMap<String,Box<dyn IType>>>;
}

pub trait IField : AsAny {
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &str;
    fn get_is_primary(&self) -> &Option<bool>;
    fn clone_box(&self) -> Box<dyn IField>;
}

pub trait IType : AsAny {
    fn get_rust_type(&self) -> &str;    
}

pub trait IYamlData {
    fn get_tables(&self) -> Option<Vec<Box<dyn ITable>>>;
    fn get_types(&self) -> Option<HashMap<String, Box<dyn IType>>>;
    fn set_tables(&mut self, tables: Vec<Box<dyn ITable>>);
    fn set_types(&mut self, types: HashMap<String, Box<dyn IType>>);
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}
