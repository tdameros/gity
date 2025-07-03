pub mod user;

use crate::config::user::User;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};

pub const GITY_DIR_NAME: &str = ".gity";
pub const OBJECTS_DIR_NAME: &str = "objects";
pub static OBJECTS_PATH: Lazy<PathBuf> =
    Lazy::new(|| Path::new(GITY_DIR_NAME).join(OBJECTS_DIR_NAME));

#[allow(dead_code)]
pub struct Config {
    pub user: User,
}
