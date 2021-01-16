#![feature(proc_macro_hygiene, decl_macro)]

mod index_server;

use rocket::State;
use rocket::{post, routes};
use rocket_contrib::json::Json;
use serde::{Deserialize};
use crate::index_server::{SearchResult, search_top_docs, IndexServer, create_index_server};

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
    let index_server = create_index_server();
    rocket::ignite()
        .manage(Box::new(index_server))
        .mount("/", routes![search]).launch();
}


#[cfg(test)]
mod tests {}
