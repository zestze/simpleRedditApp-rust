use std::fs;
use serde::Deserialize;
use std::collections::HashSet;

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

const USER_AGENT: &str = "simpleRedditClient/0.1 by ZestyZeke";

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

    async fn authorize(&self, client: &reqwest::Client) -> 
        Result<AuthResponse, Box<dyn std::error::Error>> {

        let auth_url = "https://www.reddit.com/api/v1/access_token";

        let params = [("grant_type", "password"), ("username", &self.username), 
            ("password", &self.password)];

        let builder = client.post(auth_url)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header("User-Agent", USER_AGENT)
            .form(&params);
        let resp = builder.send()
            .await?;
        //TODO: do something with the resp status?
        //let resp_status = resp.status();

        //TODO: is there a more elegant way of doing this?
        let auth_response = resp.json::<AuthResponse>().await?;
        Ok(auth_response)
    }

    async fn get_saved_posts(&self, endpoint: &str, 
                             client: &reqwest::Client, 
                             auth_info: AuthResponse,
                             last_received: Option<String>) ->
        Result<serde_json::Value, Box<dyn std::error::Error>> {

        let api_name = "https://oauth.reddit.com";
        let mut api_url = format!("{}{}", api_name, endpoint).to_string();
        if let Some(last) = &last_received {
            api_url = format!("{}?after={}", api_url, last).to_string();
        }

        let auth_value = format!("{} {}", auth_info.token_type, auth_info.access_token);
        let builder = client.get(&api_url)
            .header("Authorization", auth_value)
            .header("User-Agent", USER_AGENT);
        let resp = builder.send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        //TODO: do something with the resp status?
        //let resp_status = resp.status();

        //TODO: check rate limiting...
        //println!("{}", resp.text().await?);

        Ok(resp["data"].clone())
    }

    pub async fn run(&self, filter: &str, should_map: bool) -> 
        Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let auth_response = self.authorize(&client).await?;

        let endpoint = format!("/user/{}/saved", self.username);
        let saved_posts = self.get_saved_posts(&endpoint, &client, auth_response, None)
            .await?;

        let mut seen_posts = HashSet::new();

        Ok(())
    }
}
