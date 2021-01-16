use std::env;
use std::fs;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;
use bucket;
use git_collector::{create_bucket_from_head, Repository};
use serde::Serialize;

fn create_tantivy_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT | STORED);
    schema_builder.build()
}

fn setup_index(schema: &Schema) -> tantivy::Index {
    let path = "./test_artifacts/test_repo";
    fs::remove_dir_all(path).unwrap_or_default();
    fs::create_dir_all(path).unwrap();
    let index = Index::create_in_dir(Path::new(path), schema.clone()).unwrap();
    index
}

fn fill_test_data(schema: &Schema, index: &tantivy::Index) {
    let mut index_writer = index.writer(50_000_000).unwrap();
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let git_path = env::current_dir().unwrap();
    let repo = Repository::open(git_path.as_path()).unwrap();
    let bucket = create_bucket_from_head(&repo).unwrap();

    let files = bucket.get_bucket("FILES").unwrap();
    for (file_name, content) in files.values.iter() {
        let content = match content {
            bucket::Value::String(val) => val,
            _ => continue,
        };
        let mut doc = Document::default();
        doc.add_text(title, file_name);
        doc.add_text(body, content);
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

    QueryParser::for_index(&index, vec![title, body])
}

pub struct IndexServer {
    _index: tantivy::Index,
    schema: Schema,
    reader: tantivy::IndexReader,
    query_parser: QueryParser,
}

pub fn create_index_server() -> IndexServer {
    println!("Setting up the index server");
    let schema = create_tantivy_schema();
    let index = setup_index(&schema);
    fill_test_data(&schema, &index);
    let reader = create_reader(&index);
    let query_parser = create_query_parser(&schema, &index);
    println!("Finished setting up the index server");
    IndexServer {
        _index: index,
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

    let mut result = Vec::new();
    for (_, doc_address) in top_docs {
        let doc = searcher.doc(doc_address).unwrap();
        let title_value = doc.get_first(title).unwrap().text().unwrap();
        let body_value = doc.get_first(body).unwrap().text().unwrap();
        result.push(SearchResult {
            ref_link: "".to_string(),
            title: title_value.to_string(),
            description: body_value.to_string(),
        })
    }
    result
}