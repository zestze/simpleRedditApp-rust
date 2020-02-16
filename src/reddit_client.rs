use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::utils::*;

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

    pub async fn run(&self, filter: String, should_map: bool) -> 
        Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let auth_response = self.authorize(&client).await?;

        let endpoint = format!("/user/{}/saved", self.username);
        let mut saved_posts = get_saved_posts(&endpoint, &client, &auth_response, None)
            .await?;

        let mut seen_posts = HashSet::new();
        let mut last_seen_post = parse_response(&saved_posts, &filter);

        let mut subreddit_map = HashMap::new();

        while last_seen_post.is_some() && 
            !seen_posts.contains(&last_seen_post) {
            seen_posts.insert(last_seen_post.clone());

            saved_posts = get_saved_posts(&endpoint, &client, &auth_response, 
                                          last_seen_post.as_ref())
                .await?;

            last_seen_post = parse_response(&saved_posts, &filter);
            update_map(&mut subreddit_map, &saved_posts);
        }

        if should_map {
            print_map(subreddit_map);
        }
        Ok(())
    }
}

