mod commands;
mod config;
mod context;
mod object;
mod utils;

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
}
