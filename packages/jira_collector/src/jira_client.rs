use goji::{Credentials, Jira, SearchOptions};

pub struct JiraClient {
    username: String,
    password: String,
    host: String,
}

#[derive(Debug)]
pub struct IssueWithComments {
    pub title: String,
    pub description: String,
    pub comments: String,
}

impl JiraClient {
    pub fn new(host: &str, username: &str, password: &str) -> JiraClient {
        JiraClient {
            host: host.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    fn get_client(&self) -> Jira {
        Jira::new(
            &self.host,
            Credentials::Basic(self.username.clone(), self.password.clone()),
        ).unwrap()
    }

    pub fn get_all_issues_with_comments(&self) -> Vec<IssueWithComments> {
        let client = self.get_client();
        println!("{:?}", client.boards().list(&Default::default()));
        let search_options = SearchOptions::builder().max_results(std::u64::MAX).build();

        for search_item in client.search().iter("", &search_options).unwrap() {
            println!("{:?}", search_item);
        }
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn get_client() -> JiraClient {
        JiraClient::new(
            &env::var("JIRA_HOST").unwrap(),
            &env::var("JIRA_USERNAME").unwrap(),
            &env::var("JIRA_PASSWORD").unwrap(),
        )
    }

    #[test]
    fn get_all_issues_with_comments_works_with_no_errors() {
        let client = get_client();
        let result = client.get_all_issues_with_comments();
        println!("{:?}", result);
    }
}