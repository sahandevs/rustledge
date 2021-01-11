#![feature(proc_macro_hygiene, decl_macro)]

use bucket;
use git_collector::{create_bucket_from_head, Repository};
use regex::RegexBuilder;
use rocket::{post, routes};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::env;

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
    // for testing purposes we use current repo to create a bucket
    let git_path = env::current_dir().unwrap();
    let repo = Repository::open(git_path.as_path()).unwrap();
    let bucket = create_bucket_from_head(&repo).unwrap();

    let query = RegexBuilder::new(&data.query)
        .case_insensitive(true)
        .build()
        .unwrap();

    let files = bucket.get_bucket("FILES").unwrap();
    let mut result = Vec::new();

    for (file_name, content) in files.values.iter() {
        let content = match content {
            bucket::Value::String(val) => val,
            _ => continue,
        };
        if query.is_match(&content) {
            result.push(SearchResult {
                description: content.clone(),
                title: file_name.clone(),
                ref_link: "http://google.com/".to_string(),
            });
        }
    }
    Json(result)
}

pub fn main() {
    rocket::ignite().mount("/", routes![search]).launch();
}

#[cfg(test)]
mod tests {}
