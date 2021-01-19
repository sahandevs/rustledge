use std::path::Path;
use serde_json;
use std::{fs, env};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Config {
    pub git_repos: Vec<String>,
    pub index_server: IndexServerConfig,
    pub api: ApiConfig,
}

#[derive(Deserialize)]
pub struct IndexServerConfig {
    pub db_path: String,
}

#[derive(Deserialize)]
pub struct ApiConfig {
    pub port: u16
}

pub fn read_config(path: &Path) -> Config {
    let raw_config_str = fs::read_to_string(path).unwrap();
    let mut config: Config = serde_json::from_str(&raw_config_str).unwrap();
    process_config(&mut config);
    config
}

#[inline]
fn _replace_string_with_env(str: &mut String) {
    if str.starts_with("env:") {
        *str = env::var(str.replace("env:", "")).unwrap();
    }
}

/// replace env:VARIABLE values with value from environment variables
fn process_config(config: &mut Config) {
    for repo in &mut config.git_repos {
        _replace_string_with_env(repo);
    }
    _replace_string_with_env(&mut config.index_server.db_path);
}