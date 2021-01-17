mod trello_client;

use collector;
use collector::{Bucket, CollectResult, CollectError, FlatData};
use crate::trello_client::TrelloClient;

const CARDS: &str = "CARDS";
const TITLE: &str = "TITLE";
const DESCRIPTION: &str = "DESCRIPTION";
const COMMENTS: &str = "COMMENTS";
const URL: &str = "URL";

pub struct TrelloCollector {
    trello_client: TrelloClient,
}

impl TrelloCollector {
    pub fn new(token: &str, key: &str) -> TrelloCollector {
        let trello_client = TrelloClient::new(
            key,
            token,
        );
        TrelloCollector {
            trello_client
        }
    }
}

/*
  Bucket ->
    Cards ->
        Title
        Description
        Url
        List<Comments>

 */

impl collector::Collector for TrelloCollector {
    fn convert_to_flat_data(&self, bucket: &Bucket) -> Vec<FlatData> {
        unimplemented!()
    }

    fn collect(&self) -> Result<CollectResult, CollectError> {
        let _ = self.trello_client.get_all_cards_with_comments();
        let mut cards_bucket = collector::Bucket::new();

        let mut bucket = collector::Bucket::new();
        bucket.set(CARDS, collector::Value::Bucket(cards_bucket));
        Ok(CollectResult::New(bucket))
    }
}