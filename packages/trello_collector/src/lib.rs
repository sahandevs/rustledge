mod trello_client;

use collector;
use collector::{Bucket, CollectResult, CollectError, FlatData, Value};
use crate::trello_client::TrelloClient;

const CARDS: &str = "CARDS";
const TITLE: &str = "TITLE";
const DESCRIPTION: &str = "DESCRIPTION";
const COMMENTS: &str = "COMMENTS";

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

impl collector::Collector for TrelloCollector {
    fn convert_to_flat_data(&self, bucket: &Bucket) -> Vec<FlatData> {
        let mut result: Vec<collector::FlatData> = vec![];

        let cards = bucket.get_bucket(CARDS).unwrap();
        for (url, details) in cards.values.iter() {
            let details = match details {
                Value::Bucket(b) => b,
                _ => continue
            };
            if let (Some(title), Some(description), Some(comments)) = (
                details.get_string(TITLE),
                details.get_string(DESCRIPTION),
                details.get_string(COMMENTS),
            ) {
                result.push(collector::FlatData {
                    title: title.to_owned(),
                    body: format!("{}\nComments:\n\n{}", description, comments),
                    ref_link: url.to_owned(),
                });
            }
        }
        result
    }

    fn collect(&self) -> Result<CollectResult, CollectError> {
        let result = self.trello_client.get_all_cards_with_comments();
        let mut cards_bucket = collector::Bucket::new();
        for item in result {
            let mut card_bucket = collector::Bucket::new();
            card_bucket.set(TITLE, Value::String(item.title));
            card_bucket.set(DESCRIPTION, Value::String(item.description));
            card_bucket.set(COMMENTS, Value::String(item.comments.join("\n-----\n")));
            cards_bucket.set(&item.short_url, Value::Bucket(card_bucket));
        }
        let mut bucket = collector::Bucket::new();
        bucket.set(CARDS, Value::Bucket(cards_bucket));
        Ok(CollectResult::New(bucket))
    }
}