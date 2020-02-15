use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String
}

//TODO: put in separate class...
pub struct RedditClient {
    username: String,
    password: String,
    client_id: String,
    client_secret: String
}

impl RedditClient {
    pub fn new() -> RedditClient {
        let load_file = |f| -> String {
            fs::read_to_string(f)
                .expect("wasn't able to read file!")
                .trim_end()
                .to_string()
        };

        return RedditClient {
            username: load_file("creds/user.name"),
            password: load_file("creds/user.pass"),
            client_id: load_file("creds/client.id"),
            client_secret: load_file("creds/client.secret")
        }
    }

    async fn authorize(&self) -> Result<AuthResponse, Box<dyn std::error::Error>> {

        let client = reqwest::Client::new();
        let auth_url = "https://www.reddit.com/api/v1/access_token"; //TODO: make const...
        let user_agent = "simpleRedditClient/0.1 by ZestyZeke";

        let params = [("grant_type", "password"), ("username", &self.username), 
            ("password", &self.password)];

        let builder = client.post(auth_url)
            .basic_auth(self.client_id.clone(), Some(self.client_secret.clone()))
            .header("User-Agent", user_agent)
            .form(&params);
        let resp = builder.send()
            .await?;
        let resp_status = resp.status();

        //TODO: is there a more elegant way of doing this?
        let auth_response = resp.json::<AuthResponse>().await?;
        Ok(auth_response)
    }

    pub async fn run(&self, filter: &str, should_map: bool) -> 
        Result<(), Box<dyn std::error::Error>> {
        let auth_response = self.authorize().await?;

        Ok(())
    }
}
