use std::fs;
use serde::Deserialize;
use std::collections::HashSet;
use std::collections::HashMap;

const USER_AGENT: &str = "simpleRedditClient/0.1 by ZestyZeke";

#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String
}

pub struct RedditClient {
    username: String,
    password: String,
    client_id: String,
    client_secret: String
}

//TODO: should put all of these in a sperate file...
#[derive(Deserialize, Debug)]
struct ChildData {
    subreddit: String,
    permalink: String
}

#[derive(Deserialize, Debug)]
struct Child {
    data: ChildData 
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    children: Vec<Child>,
    after: Option<String>
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: ResponseData 
}

// {
//   'children': [
//        'data': {
//          'subreddit': <val>,
//          'permalink': <val>
//        }
//   ]
// }

 
async fn get_saved_posts(endpoint: &str, 
                         client: &reqwest::Client, 
                         auth_info: &AuthResponse,
                         last_received: Option<&String>) ->
    Result<ResponseData, Box<dyn std::error::Error>> {

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
        .json::<ApiResponse>()
        .await?;
    //TODO: do something with the resp status?
    //let resp_status = resp.status();

    //TODO: check rate limiting...
    //println!("{}", resp.text().await?);
    Ok(resp.data)
}

fn parse_response(saved_posts: &ResponseData, filter: &String) -> Option<String> {

    let reddit_url = "https://www.reddit.com";
    for child in saved_posts.children.iter() {
        let data = &child.data;
        if *filter == data.subreddit.to_lowercase() {
            println!("{}{}", reddit_url, data.permalink);
        }
    }
    saved_posts.after.clone()
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

fn print_map(subreddit_map: HashMap<String, u32>) {
    let mut list: Vec<(&String, &u32)> = subreddit_map.iter().collect();
    list.sort_by(|a, b| b.1.cmp(a.1));

    for (name, count) in list {
        println!("{:<30}: {}", name, count);
    }
}

fn update_map(subreddit_map: &mut HashMap<String, u32>, saved_posts: &ResponseData) {
    for child in saved_posts.children.iter() {
        let subreddit_name = child.data.subreddit.to_string();
        subreddit_map.entry(subreddit_name)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
}
