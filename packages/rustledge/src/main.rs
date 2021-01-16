#![feature(proc_macro_hygiene, decl_macro)]

mod index_server;
mod config;

use rocket::State;
use rocket::{post, routes};
use rocket_contrib::json::Json;
use serde::{Deserialize};
use crate::index_server::{SearchResult, search_top_docs, IndexServer, create_index_server};
use crate::config::{Config, read_config};
use std::env;
use rocket::config::Environment;
use std::path;

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
}


#[post("/search", data = "<data>")]
fn search(data: Json<SearchRequest>, index_server: State<Box<IndexServer>>) -> Json<Vec<SearchResult>> {
    let result = search_top_docs(&data.query, &index_server);
    Json(result)
}

pub fn main() {
    let mut config_path: String = String::from("./config.json");
    for argument in env::args() {
        if argument.starts_with("--config=") {
            config_path = argument.replace("--config=", "");
        }
    }
    if config_path == "" { panic!("providing a config file (--config=) is required ") }

    let config = read_config(path::Path::new(&config_path));
    run_with_config(&config);
}

fn run_with_config(config: &Config) {
    let index_server = create_index_server(config);

    let rocket_config = rocket::Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(config.api.port)
        .finalize()
        .unwrap();

    rocket::custom(rocket_config)
        .manage(Box::new(index_server))
        .mount("/", routes![search])
        .launch();
}


#[cfg(test)]
mod tests {}
