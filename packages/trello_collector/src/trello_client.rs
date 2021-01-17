use serde::de::DeserializeOwned;
use serde::Deserialize;

pub struct TrelloClient {
    key: String,
    token: String,
    host: String,
}

#[derive(Debug)]
pub struct CardsWithComments {
    pub title: String,
    pub description: String,
    pub short_url: String,
    pub comments: Vec<String>,
}

#[derive(Deserialize)]
pub struct CardBadges {
    pub comments: i32,
}

#[derive(Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    #[serde(rename = "desc")]
    pub description: String,
    #[serde(rename = "shortUrl")]
    pub short_url: String,
    pub badges: CardBadges
}

#[derive(Deserialize)]
pub struct CommentData {
    pub text: String
}

#[derive(Deserialize)]
pub struct CardAction {
    pub data: CommentData
}

#[derive(Deserialize)]
pub struct Board {
    pub id: String,
    pub name: String,
}

impl TrelloClient {
    pub fn new(key: &str, token: &str) -> TrelloClient {
        TrelloClient {
            host: "https://api.trello.com/1".to_owned(),
            key: key.to_owned(),
            token: token.to_owned(),
        }
    }

    fn get<T: DeserializeOwned>(&self, path: &str) -> T {
        let resp = reqwest::blocking::get(&format!(
            "{}{}key={}&token={}",
            self.host,
            path,
            self.key,
            self.token
        ))
            .unwrap()
            .json::<T>()
            .unwrap();
        resp
    }

    pub fn get_all_cards_with_comments(&self) -> Vec<CardsWithComments> {
        let mut result: Vec<CardsWithComments> = vec![];
        let boards = self.get::<Vec<Board>>("/members/me/boards/?");
        for board in &boards {
            let cards = self.get::<Vec<Card>>(&format!("/boards/{}/cards/?", board.id));
            for card in &cards {
                let mut result_card = CardsWithComments {
                    title: card.name.to_string(),
                    short_url: card.short_url.to_string(),
                    description: card.description.to_string(),
                    comments: vec![],
                };
                if card.badges.comments > 0 {
                    let actions = self.get::<Vec<CardAction>>(&format!("/cards/{}/actions?filter=commentCard&", card.id));
                    for action in actions {
                        result_card.comments.push(action.data.text.clone());
                    }
                }
                result.push(result_card);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn get_client() -> Result<TrelloClient, ()> {
        let key = env::var("TRELLO_KEY");
        let token = env::var("TRELLO_TOKEN");
        if !key.is_ok() || !token.is_ok() { return Err(()); };
        Ok(TrelloClient::new(
            &key.unwrap(),
            &token.unwrap(),
        ))
    }

    #[test]
    fn get_all_cards_with_comments_works_with_not_errors() {
        let client = match get_client() {
            Ok(v) => v,
            _ => {
                println!("Please fill TRELLO_KEY and TRELLO_TOKEN in env::var to run this test");
                return;
            }
        };

        let result = client.get_all_cards_with_comments();
        println!("{:?}", result);
    }
}