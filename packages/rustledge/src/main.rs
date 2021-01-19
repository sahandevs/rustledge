#![feature(proc_macro_hygiene, decl_macro)]

mod index_server;
mod config;

use rocket::State;
use rocket::{post, get, routes};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use crate::index_server::{SearchResult, search_top_docs, IndexServer, create_index_server, recreate_index_server_db};
use crate::config::{Config, read_config};
use std::env;
use rocket::config::Environment;
use std::path;
use std::fs;
use rocket::response::content;

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
}

#[post("/search", data = "<data>")]
fn search(data: Json<SearchRequest>, index_server: State<Box<IndexServer>>) -> Json<Vec<SearchResult>> {
    let result = search_top_docs(&data.query, &index_server);
    Json(result)
}

#[derive(Deserialize)]
struct RecreateRequest {
    secret: String,
}

#[derive(Serialize)]
struct ResultMessage {
    is_ok: bool,
    message: String,
}

#[post("/recreate", data = "<data>")]
fn recreate(data: Json<RecreateRequest>, index_server: State<Box<IndexServer>>, config: State<Box<Config>>) -> Json<ResultMessage> {
    if data.secret != config.api.internal_commands_secret {
        return Json(
            ResultMessage {
                is_ok: false,
                message: "Provided secret is incorrect".to_string(),
            }
        );
    }
    let result = recreate_index_server_db(&index_server, &config);
    Json(
        match result {
            Ok(_) => ResultMessage {
                is_ok: true,
                message: "Index db recreated successfully".to_string(),
            },
            Err(_) => ResultMessage {
                is_ok: false,
                message: "An error occurred while recreating index db".to_string(),
            },
        }
    )
}

#[get("/")]
fn ui() -> content::Html<String> {
    let index = fs::read_to_string("./web/index.html").unwrap();
    content::Html(index)
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
    run_with_config(config);
}

fn run_with_config(config: Config) {
    let index_server = create_index_server(&config);

    let rocket_config = rocket::Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(config.api.port)
        .finalize()
        .unwrap();

    rocket::custom(rocket_config)
        .manage(Box::new(index_server))
        .manage(Box::new(config))
        .mount("/", routes![search, recreate, ui])
        .launch();
}


#[cfg(test)]
mod tests {}
