use std::path::Path;
use serde_json;
use std::fs;
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
    let config: Config = serde_json::from_str(&raw_config_str).unwrap();
    config
}