use anyhow::Result;
use clap::Parser;
use core::panic;
use handlebars::Handlebars;
use std::{any::Any, collections::HashMap, fs, ops::Deref, path::Path};

use crate::abs::{
    iface::{IField, ITable, IType, IYamlData},
    imp::YamlData,
};

mod abs;

fn get_templates(args: &Args) -> Result<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();
    let current_path = Path::new(&args.template_path);

    for entry_res in fs::read_dir(current_path)? {
        let entry = entry_res?;
        let path = entry.path();
        let raw_path = path.clone();
        let content = fs::read_to_string(path)?;
        let key = raw_path.to_str().ok_or(anyhow::anyhow!(
            "Не удалось сохранить шаблон в памяти".to_string()
        ))?;

        tracing::info!("{}", content);
        map.insert(key.to_string(), content);
    }

    Ok(map)
}

fn fill_data(args: &Args) -> Result<Vec<YamlData>, Box<dyn std::error::Error>> {
    let current_path = Path::new(&args.config_folder);
    let mut yaml_data: Vec<YamlData> = Vec::new();

    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let content = fs::read_to_string(&path)?;
            let yaml: YamlData = serde_yaml::from_str(&content)?;
            yaml_data.push(yaml);
        }
    }
    Ok(yaml_data)
}

fn v2() -> anyhow::Result<()> {
    let args_result = fill_args()?;
    let r = fill_data(&args_result);
    let Ok(yaml_data) = r else {
        tracing::error!("Ошибка при чтении YAML: {}", r.unwrap_err());
        std::process::exit(1);
    };

    let tpl_res = get_templates(&args_result);

    let Ok(tpls) = tpl_res else {
        return Err(anyhow::anyhow!(tpl_res.unwrap_err()));
    };

    let mut types_map: HashMap<String, Box<dyn IType>> = HashMap::new();

    let mut data_vec: Vec<Box<dyn ITable>> = Vec::new();

    for single_yaml in yaml_data {
        let types = single_yaml.get_types();
        if types.is_some() {
            for (k, v) in types.unwrap() {
                types_map.insert(k, v);
            }
        }
        let tables = single_yaml.get_tables();
        if tables.is_some() {
            for table in tables.unwrap() {
                data_vec.push(table);
            }
        }
    }

    let reg = Handlebars::new();
    let mut res: HashMap<String, String> = HashMap::new();
    let mut filled_data = YamlData::default();
    filled_data.set_tables(data_vec);
    filled_data.set_types(types_map);

    if let Some(tables) = filled_data.get_tables() {
        for table in tables.iter() {
            let mut unwrapped_data: HashMap<String, Box<dyn Any>> = HashMap::new();

            if let Some(schema) = table.get_schema() {
                unwrapped_data.insert("schema".to_string(), Box::new(schema.to_string()));
            }

            let mut fields = table.get_fields();

            if let Some(extends) = table.get_extends() {
                if let Some(tables_vec) = filled_data.get_tables() {
                    if let Some(base_table) = tables_vec.iter().find(|t| t.get_name() == extends) {
                        let base_fields = base_table.get_fields();
                        for f in base_fields {
                            fields.push(f.clone_box());
                        }
                    }
                }
            }

            unwrapped_data.insert("fields".to_string(), Box::new(fields));

            unwrapped_data.insert("name".to_string(), Box::new(table.get_name()));

            unwrapped_data.insert("types".to_string(), Box::new(filled_data.get_types()));
        }
    }

    // for yaml in filled_data.get_tables().iter() {
    //         let tpl_render_res = reg.render_template(&tpl_content, &filled_data);
    //         let Ok(rendered_tpl) = tpl_render_res else {
    //             return Err(anyhow::anyhow!(tpl_render_res.unwrap_err()));
    //         };
    //         let splitted = tpl_name.split("/").last();
    //         if splitted.is_none() {
    //             panic!("File {}", tpl_name);
    //         }
    //         tracing::info!("{}", rendered_tpl);
    //         res.insert(splitted.unwrap().to_string(), rendered_tpl);
    //     }

    for (file_name, content) in res {
        dbg!(&file_name);
        let path = format!(
            "{}/{}.rs",
            args_result.target_path,
            file_name.replace(".rtpl", "")
        );

        let write_res = fs::write(Path::new(&path), content);
        let Ok(_) = write_res else {
            tracing::error!("{}", write_res.unwrap_err());
            break;
        };
    }
    tracing::info!("ok");
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    v2()
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    config_folder: String,
    #[arg(long)]
    target_path: String,
    #[arg(long)]
    template_path: String,
}

fn fill_args() -> Result<Args> {
    let args = Args::parse();
    if args.config_folder.is_empty() {
        return Err(anyhow::anyhow!(
            "Не указана стартовая папка с конфигурацией dao".to_string()
        ));
    }
    if args.target_path.is_empty() {
        return Err(anyhow::anyhow!(
            "Не указана целевая папка для выгрузки dao".to_string()
        ));
    }
    if args.template_path.is_empty() {
        return Err(anyhow::anyhow!(
            "Не указана целевая папка с шаблонами dao".to_string()
        ));
    }

    Ok(args)
}
