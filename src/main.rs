use anyhow::Result;
use clap::Parser;
use handlebars::Handlebars;
use std::{collections::HashMap, fs, path::Path};

use crate::abs::imp::{Field, Table, Type, YamlData};

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

        map.insert(key.to_string(), content);
    }

    Ok(map)
}

fn fill_data(args: &Args) -> Result<Vec<YamlData>, Box<dyn std::error::Error>> {
    let current_path = Path::new(&args.config_folder);
    let mut result: Vec<YamlData> = Vec::new();
    let mut types: HashMap<String, Type> = HashMap::new();
    let mut tables: Vec<Table> = Vec::new();

    let mut yaml_data: Vec<YamlData> = Vec::new();

    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let content = fs::read_to_string(&path)?;
            let yaml: YamlData = serde_yaml::from_str(&content)?;
            if let Some(tbs) = yaml.tables.clone() {
                tables.append(&mut tbs.clone())
            }

            if let Some(tps) = yaml.types.clone() {
                types.extend(tps);
            }

            yaml_data.push(yaml);
        }
    }

    for tb in tables.iter() {
        let mut res = YamlData::default();
        let mut tb = columns_dfs(tb.clone(), &tb.fields, &tables, &types, &args.target_lang);
        tb.types = Some(types.clone());
        res.tables = Some(vec![tb]);
        res.types = Some(types.clone());
        result.push(res);
    }

    Ok(result)
}

fn columns_dfs(
    mut root: Table,
    fields: &Vec<Field>,
    tables: &Vec<Table>,
    types: &HashMap<String, Type>,
    target_lang: &String,
) -> Table {
    let mut parent_table = Table::default();
    if let Some(extends) = root.clone().extends {
        for table in tables.iter() {
            tracing::info!("Пробуем сравнить {}, с {}", table.name, extends);
            if table.name.eq(&extends) {
                parent_table =
                    columns_dfs(table.clone(), &table.fields, tables, &types, target_lang);
                break;
            }
        }
    }

    let mut total_fields: Vec<Field> = Vec::new();
    total_fields.append(&mut fields.clone());
    total_fields.append(&mut parent_table.fields.clone());
    root.fields = Vec::new();

    for field in total_fields {
        let mut field_to_save = field.clone();
        for (tk, tv) in types.iter() {
            if field.tp.eq(tk) {
                field_to_save.tp = tv.rust_type.clone()
            }
        }
        root.fields.push(field_to_save);
    }
    return root;
}

fn generate() -> anyhow::Result<()> {
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

    let reg = Handlebars::new();
    let mut res: HashMap<String, String> = HashMap::new();

    for yml in yaml_data.iter() {
        if let Some(tbs) = yml.tables.clone() {
            for table in tbs.iter() {
                if let Some(_) = &table.extends {
                    for (k, v) in &tpls {
                        let raw_val = serde_json::to_value(table);
                        if let Ok(val) = raw_val {
                            tracing::info!("Info {}", val);
                            if let Ok(tpl) = reg.render_template(v, &val) {
                                tracing::info!("Successfully rendered template {}", k);
                                if let Some(splitted) = table.name.split("/").last() {
                                    res.insert(splitted.to_string(), tpl);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

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
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    generate()
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    target_lang: String,
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
    if args.target_lang.is_empty() {
        return Err(anyhow::anyhow!(
            "Не указан целевой язык для описания dto".to_string()
        ));
    }

    Ok(args)
}
