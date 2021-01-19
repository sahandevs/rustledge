pub struct JiraClient {
    username: String,
    password: String,
    host: String,
}

impl JiraClient {

    pub fn new(host: &str, username: &str, password: &str) -> JiraClient {
        JiraClient {
            host: host.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

}
