#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{post, routes};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
}

#[derive(Serialize)]
struct SearchResult {
    title: String,
    description: String,
    ref_link: String,
}

#[post("/search", data = "<data>")]
fn search(data: Json<SearchRequest>) -> Json<Vec<SearchResult>> {
    let mut result = Vec::new();
    result.push(SearchResult {
        description: "desc".to_string(),
        title: "title".to_string(),
        ref_link: "http://google.com/".to_string(),
    });
    Json(result)
}

pub fn main() {
    rocket::ignite().mount("/", routes![search]).launch();
}

#[cfg(test)]
mod tests {}