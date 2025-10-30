use std::{env, path::PathBuf};
pub mod store;
pub mod jwt;

pub fn auth_version() -> String {
    env::var("AUTH_VERSION").unwrap_or("1".to_string())
}

pub fn storage_path() -> PathBuf {
    let path = env::var("PASS_FILE").unwrap_or(String::from("./pass"));
    
    PathBuf::from(
        path.as_str()
    )
}