use collector;
use collector::{Bucket, CollectResult, CollectError, FlatData};
use trello;

const CARDS: &str = "CARDS";
const TITLE: &str = "TITLE";
const DESCRIPTION: &str = "DESCRIPTION";
const COMMENTS: &str = "COMMENTS";
const URL: &str = "URL";

pub struct TrelloCollector {
    trello_client: trello::Client,
}

impl TrelloCollector {
    pub fn new(token: &str, key: &str) -> TrelloCollector {
        let trello_client = trello::Client::new(
            "https://api.trello.com",
            token,
            key,
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
        let _ = match trello::Board::get_all(&self.trello_client) {
            Ok(value) => value,
            Err(e) => return Err(CollectError::General)
        };
        let mut cards_bucket = collector::Bucket::new();

        let mut bucket = collector::Bucket::new();
        bucket.set(CARDS, collector::Value::Bucket(cards_bucket));
        Ok(CollectResult::New(bucket))
    }
}