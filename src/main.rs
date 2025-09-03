use anyhow::Result;
use clap::Parser;
use std::{fs, path::{Path, PathBuf}};

use schema_reader::prelude::*;
 
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let schema = Schema::from_dir(args.config_folder)?;

    let mut reg = handlebars::Handlebars::new();
    reg.register_schema_reader_helpers();

    if args.template_path.is_dir() {
        let dir = fs::read_dir(args.template_path)?;
        for entry in dir {
            let entry = entry?;
            if !entry.path().is_file() {continue;} //todo: recursive support
            let stem = entry.path().file_stem().unwrap().to_str().unwrap().to_owned();
            fs::create_dir(Path::new(&format!("{}/{}", args.target_path, stem))).ok();
            let template = entry.path();
            reg.register_template_file(&stem, &template)?;
            let res = schema.render_tables(&reg, &stem)?;
            for (table_name, rendered) in res {
                fs::write(Path::new(&format!("{}/{}/{}.rs", args.target_path, stem, table_name)), rendered)?;
            }
        }
    } else {
        let template = args.template_path;
        let stem = template.file_stem().unwrap().to_str().unwrap().to_owned();
        reg.register_template_file(&stem, &template)?;
        let res = schema.render_tables(&reg, &stem)?;
        for (table_name, rendered) in res {
            fs::write(Path::new(&format!("{}/{}.rs", args.target_path, table_name)), rendered)?;
        }
    }
    Ok(())
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
    template_path: PathBuf,
}
