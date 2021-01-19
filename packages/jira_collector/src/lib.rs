mod jira_client;

use crate::jira_client::JiraClient;
use collector;
use collector::{Bucket, CollectError, CollectResult, FlatData};

// const TITLE: &str = "TITLE";

pub struct JiraCollector {
    jira_client: JiraClient,
}

impl JiraCollector {
    pub fn new(host: &str, username: &str, password: &str) -> JiraCollector {
        let jira_client = JiraClient::new(host, username, password);
        JiraCollector { jira_client }
    }
}

impl collector::Collector for JiraCollector {
    fn convert_to_flat_data(&self, bucket: &Bucket) -> Vec<FlatData> {
        let _ = bucket;
        unimplemented!()
    }

    fn collect(&self) -> Result<CollectResult, CollectError> {
        let _ = &self.jira_client;
        unimplemented!()
    }
}
