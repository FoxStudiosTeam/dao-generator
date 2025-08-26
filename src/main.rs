use anyhow::Result;
use clap::Parser;
use std::{fs, path::Path};

use dao_generator::prelude::*;
 
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let schema = YamlSchema::from_dir(args.config_folder)?;
    let res = schema.render_all(args.templates_path)?;
    for (template, rendered) in res {
        fs::create_dir(Path::new(&format!("{}/{}", args.target_path, template))).ok();
        for (name, rendered) in rendered {
            fs::write(Path::new(&format!("{}/{}/{}.rs", args.target_path, template, name)), rendered)?;
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
    templates_path: String,
}
