use std::collections::HashSet;
use crate::utils;
use crate::config::Config;
use crate::models;
use crate::mapper;

pub struct RedditClient {
    username: String,
    password: String,
    client_id: String,
    client_secret: String,
    subparsers: Vec<Box<dyn mapper::SubParser>>
}

impl RedditClient {
    pub fn new(map: bool) -> Self {
        let config = Config::new("creds/config.json");

        let mut subparsers = Vec::<Box<dyn mapper::SubParser>>::new();
        if map { subparsers.push(Box::new(mapper::Mapper::new())); }
        RedditClient {
            username: config.user.name,
            password: config.user.password,
            client_id: config.client.id,
            client_secret: config.client.secret,
            subparsers: subparsers
        }
    }

    async fn authorize(&self, client: &reqwest::Client) -> 
        Result<models::AuthResponse, Box<dyn std::error::Error>> {

        let auth_url = "https://www.reddit.com/api/v1/access_token";

        let params = [("grant_type", "password"), ("username", &self.username), 
            ("password", &self.password)];

        let builder = client.post(auth_url)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header("User-Agent", models::USER_AGENT)
            .form(&params);
        let resp = builder.send()
            .await?;
        //TODO: do something with the resp status?
        //let resp_status = resp.status();

        //TODO: is there a more elegant way of doing this?
        let auth_response = resp.json::<models::AuthResponse>().await?;
        Ok(auth_response)
    }

    pub async fn run(&mut self, filter: Option<String>) -> 
        Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let auth_response = self.authorize(&client).await?;

        let endpoint = format!("/user/{}/saved", self.username);
        let mut saved_posts = utils::get_saved_posts(&endpoint, &client, &auth_response, None)
            .await?;

        let mut seen_posts = HashSet::new();
        let mut last_seen_post = utils::parse_response(&saved_posts, &filter);

        while last_seen_post.is_some() && 
            !seen_posts.contains(&last_seen_post) {
            seen_posts.insert(last_seen_post.clone());

            saved_posts = utils::get_saved_posts(&endpoint, &client, &auth_response, 
                                          last_seen_post.as_ref())
                .await?;

            last_seen_post = utils::parse_response(&saved_posts, &filter);
            for subparser in self.subparsers.iter_mut() {
                subparser.update(&saved_posts);
            }
        }

        for subparser in self.subparsers.iter_mut() {
            subparser.print();
        }
        Ok(())
    }
}

