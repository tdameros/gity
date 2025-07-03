mod commands;
mod config;
mod object;

use clap::Parser;
use commands::{cat_file, init, Commands};

#[derive(Parser)]
#[command(name = "gity")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => init::run(args),
        Commands::CatFile(args) => cat_file::run(args),
    }
    // let mut tree = Tree::new("".to_string(), vec![]);
    // tree.save(OBJECTS_PATH.as_path()).unwrap();
    // let mut blob = Blob::new("file.txt".to_string(), "".to_string());
    // blob.save(OBJECTS_PATH.as_path()).unwrap();
    // let mut tree = Tree::new("".to_string(), vec![Box::new(blob)]);
    // tree.save(OBJECTS_PATH.as_path()).unwrap();
    // let mut empty_tree = Tree::new("".to_string(), vec![]);
    // empty_tree.save(OBJECTS_PATH.as_path()).unwrap();
}
