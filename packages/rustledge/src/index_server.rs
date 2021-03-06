use std::fs;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;
use git_collector::{GitCollector};
use trello_collector::{TrelloCollector};
use serde::Serialize;
use crate::config::Config;
use collector;
use collector::Collector;

fn create_tantivy_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT | STORED);
    schema_builder.add_text_field("ref_link", TEXT | STORED);
    schema_builder.build()
}

fn setup_index(schema: &Schema, config: &Config) -> tantivy::Index {
    let path = Path::new(&config.index_server.db_path);
    fs::create_dir_all(path).unwrap_or_default();

    let index = Index::open_in_dir(path);
    if let Ok(index) = index { return index; }
    let index = Index::create_in_dir(path, schema.clone()).unwrap();
    index
}

fn fill_data(schema: &Schema, index: &tantivy::Index, config: &Config) {
    let mut new_index_records: Vec<collector::FlatData> = vec![];

    println!("Indexing started");
    for repo in &config.git_repos {
        println!("Indexing git repo: {}", repo);
        let git_path = Path::new(repo);
        let collector = GitCollector::new(git_path);
        let bucket = collector.collect().unwrap();
        match &bucket {
            collector::CollectResult::New(data) => {
                let mut new = collector.convert_to_flat_data(data);
                new_index_records.append(&mut new);
            }
            _ => unimplemented!(),
        }
    }

    println!("Indexing trello cards");
    let trello_collector = TrelloCollector::new(&config.trello.token, &config.trello.key);
    let bucket = trello_collector.collect().unwrap();
    match &bucket {
        collector::CollectResult::New(data) => {
            let mut new = trello_collector.convert_to_flat_data(data);
            new_index_records.append(&mut new);
        }
        _ => unimplemented!(),
    }
    println!("Indexing done!");
    let mut index_writer = index.writer(50_000_000).unwrap();
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let ref_link = schema.get_field("ref_link").unwrap();

    for record in &new_index_records {
        let mut doc = Document::default();
        doc.add_text(title, &record.title);
        doc.add_text(body, &record.body);
        doc.add_text(ref_link, &record.ref_link);
        index_writer.add_document(doc);
    }

    index_writer.commit().unwrap();
}

fn create_reader(index: &tantivy::Index) -> tantivy::IndexReader {
    index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .unwrap()
}

fn create_query_parser(schema: &Schema, index: &tantivy::Index) -> QueryParser {
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let ref_link = schema.get_field("ref_link").unwrap();

    QueryParser::for_index(&index, vec![title, body, ref_link])
}

pub struct IndexServer {
    index: tantivy::Index,
    schema: Schema,
    reader: tantivy::IndexReader,
    query_parser: QueryParser,
}

pub fn recreate_index_server_db(index_server: &IndexServer, config: &Config) -> Result<(), ()> {
    println!("Recreating index server");
    // we execute following code in a block expression
    // so we ensure index_write is freed before we call the fill_data
    // tantivy doesn't allow more than one index_write at a same time
    // and because we are using an index_writer inside fill_data, we have to free this one first.
    {
        let index_writer = index_server.index.writer(50_000_000).unwrap();
        if !index_writer.delete_all_documents().is_ok() {
            return Err(());
        }
    }
    fill_data(&index_server.schema, &index_server.index, config);
    println!("Recreating index server done!");
    Ok(())
}

pub fn create_index_server(config: &Config) -> IndexServer {
    println!("Setting up the index server");
    let schema = create_tantivy_schema();
    let index = setup_index(&schema, config);
    let reader = create_reader(&index);
    let query_parser = create_query_parser(&schema, &index);
    println!("Finished setting up the index server");
    IndexServer {
        index,
        schema,
        reader,
        query_parser,
    }
}

#[derive(Serialize)]
pub struct SearchResult {
    title: String,
    description: String,
    ref_link: String,
}

pub fn search_top_docs(query: &str, index_server: &IndexServer) -> Vec<SearchResult> {
    let searcher = index_server.reader.searcher();

    let query = index_server.query_parser.parse_query(query).unwrap();
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();

    let title = index_server.schema.get_field("title").unwrap();
    let body = index_server.schema.get_field("body").unwrap();
    let ref_link = index_server.schema.get_field("ref_link").unwrap();

    let mut result = Vec::new();
    for (_, doc_address) in top_docs {
        let doc = searcher.doc(doc_address).unwrap();
        let title_value = doc.get_first(title).unwrap().text().unwrap();
        let body_value = doc.get_first(body).unwrap().text().unwrap();
        let ref_link_value = doc.get_first(ref_link).unwrap().text().unwrap();
        result.push(SearchResult {
            ref_link: ref_link_value.to_string(),
            title: title_value.to_string(),
            description: body_value.to_string(),
        })
    }
    result
}