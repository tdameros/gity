pub mod cat_file;
pub mod init;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Init(init::InitArgs),
    CatFile(cat_file::CatFileArgs),
}
