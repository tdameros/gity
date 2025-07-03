use std::path;
use clap::Args;

use crate::config::{GITY_DIR_NAME, OBJECTS_PATH};
use std::path::Path;

#[derive(Args)]
pub struct InitArgs {}

pub fn run(_args: &InitArgs) {
    let gity_path = Path::new(GITY_DIR_NAME);
    if !gity_path.exists() {
        std::fs::create_dir(gity_path).unwrap();
        std::fs::create_dir(OBJECTS_PATH.clone()).unwrap();
        println!("Initialized empty Gity repository in {}", path::absolute(gity_path).unwrap().display());
    }
}
