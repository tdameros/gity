mod config;
mod object;

use clap::Parser;
use object::{blob::Blob, tree::Tree, Object};
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    command: String,
}

const GITY_DIR_NAME: &str = ".gity";
const OBJECTS_DIR_NAME: &str = "objects";
static OBJECTS_PATH: Lazy<PathBuf> = Lazy::new(|| Path::new(GITY_DIR_NAME).join(OBJECTS_DIR_NAME));

fn main() {
    let args = Args::parse();

    let mut tree = Tree::new("".to_string(), vec![]);
    tree.save(OBJECTS_PATH.as_path()).unwrap();
    let mut blob = Blob::new("file.txt".to_string(), "".to_string());
    blob.save(OBJECTS_PATH.as_path()).unwrap();
    let mut tree = Tree::new("".to_string(), vec![Box::new(blob)]);
    tree.save(OBJECTS_PATH.as_path()).unwrap();
    let mut empty_tree = Tree::new("".to_string(), vec![]);
    empty_tree.save(OBJECTS_PATH.as_path()).unwrap();

    match args.command.as_str() {
        "init" => {
            init();
        }
        "add" => {
            println!("Adding");
        }
        _ => {
            eprintln!("Invalid command");
        }
    }
}

fn init() {
    let gity_path = Path::new(".gity/");
    if !gity_path.exists() {
        std::fs::create_dir(gity_path).unwrap();
        std::fs::create_dir(gity_path.join("objects")).unwrap();
    }
}
