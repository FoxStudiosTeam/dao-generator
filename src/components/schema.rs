use std::{any, fs, path::Path};

use serde::{Deserialize, Serialize};
use hashbrown::HashMap;
use anyhow::Result;
use tracing::{error, warn};

use crate::{components::table, prelude::{DefaultHelpers, ErrOr, RawTable, RenderScheme, Table, Type}};

#[derive(Clone, Deserialize, Default, Debug)]
struct RawYamlSchema {
    pub tables: Vec<RawTable>,
    #[serde(default)]
    pub types: HashMap<String, Type>,
}

impl RawYamlSchema {
    fn flatten(self) -> Result<YamlSchema> {
        let tables: HashMap<String, RawTable> = self.tables.into_iter().map(|t| (t.name.clone(), t)).collect();
        let mut flatten_tables: HashMap<String, Table> = Default::default();

        let mut abstract_tables : HashMap<String, RawTable> = Default::default();
        // Complete abstract tables
        for (table_name, table) in tables.iter() {
            if !table.is_abstract {continue;};

            let mut table = table.clone();
            let mut extends = table.extends.clone(); 
            // Flatten abstract table hierarchy
            while let Some(parent) = &extends {
                let Some(parent) = tables.get(parent) else {
                    error!("Table {} extends unknown table {}", table_name, parent);
                    continue;
                };
                extends = parent.extends.clone();
                table.extend(parent.clone());
            }
            abstract_tables.insert(table_name.clone(), table);
        }

        // Complete non-abstract tables
        for (table_name, table) in tables.iter() {
            if table.is_abstract {continue;};

            let mut table = table.clone();
            if let Some(extends) = &table.extends {
                let Some(parent) = tables.get(extends) else {
                    error!("Table {} extends unknown table {}", table_name, extends);
                    continue;
                };
                table.extend(parent.clone());
            }
            flatten_tables.insert(table_name.clone(), table.complete(&self.types).map_err(|e: String| anyhow::anyhow!(e))?);
        }
        Ok(YamlSchema { tables: flatten_tables, types: self.types })
    }

    fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut result = Self::default();
        for entry in fs::read_dir(&path)? {
            let e: Result<()> = (||{
                let entry = entry?.path();
                if entry.is_file() {
                    let content = Self::from_file(entry)?;
                    result.extend(content);
                }
                Ok(())
            })(); 
            if let Err(e) = e {
                warn!("Failed to read schema file, skipping: {}", e);
            }
        }
        Ok(result)
    }
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())?;
        Ok(serde_yaml::from_str(&content)?)
    }
    fn extend(&mut self, schema: Self) {
        self.tables.extend(schema.tables);
        self.types.extend(schema.types);
    }

}

#[derive(Clone, Serialize, Default, Debug)]
pub struct YamlSchema {
    pub tables: HashMap<String, Table>,
    pub types: HashMap<String, Type>,
}


impl YamlSchema {
    pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(RawYamlSchema::from_dir(path)?.flatten()?)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(RawYamlSchema::from_file(path)?.flatten()?)
    }

    pub fn extend(&mut self, schema: Self) {
        for (k, v) in self.tables.iter() {
            let Some(t) = schema.tables.get(k) else {continue;};
            if !t.eq(v) {
                warn!("Overlapping table defined in multiple files. Will be overwritten: {}: {} overrides {}", k, t.name, v.name);
            }
        }
        self.tables.extend(schema.tables);

        for (k, v) in self.types.iter() {
            let Some(t) = schema.types.get(k) else {continue;};
            if !t.eq(v) {
                warn!("Overlapping type defined in multiple files. Will be overwritten: {}: {:?} overrides {:?}", k, t, v);
            }
        }
        self.types.extend(schema.types);
    }
}





impl RenderScheme for YamlSchema {
    fn render_all<P: AsRef<Path>>(&self, templates: P) -> anyhow::Result<HashMap<String, Vec<(String, String)>>> {
        // todo: make global registry
        let mut reg = handlebars::Handlebars::new();
        reg.register_default_helpers();
        let dir = fs::read_dir(templates)?;
        let mut templates = vec![];
        for e in dir {
            let e = e?;
            let path = e.path();
            if path.is_dir() {continue;}
            let e: std::result::Result<String, anyhow::Error> = (||{
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| anyhow::anyhow!("Can't get file stem"))?
                    .to_string();
                reg.register_template_file(&name, path)?;
                Ok(name)
            })();
            let Ok(name) = e else {
                warn!("Failed to register template: {}", e.unwrap_err());
                continue;
            };
            templates.push(name);
        }

        let mut result: HashMap<String, Vec<(String, String)>> = Default::default();
        for template in templates {
            let mut data = vec![];
            for (name, table) in self.tables.iter() {
                let rendered = reg.render(&template, &table)?;
                data.push((name.clone(), rendered));
            }
            result.insert(template, data);
        }
        Ok(result)
    }
}