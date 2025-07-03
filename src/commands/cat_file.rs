use clap::Args;

#[derive(Args)]
pub struct CatFileArgs {}

pub fn run(_args: &CatFileArgs) {
    println!("git cat-file");
}
