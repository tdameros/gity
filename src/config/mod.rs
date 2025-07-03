use crate::config::user::User;

pub mod user;

#[allow(dead_code)]
pub struct Config {
    pub user: User,
}
