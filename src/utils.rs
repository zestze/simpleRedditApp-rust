use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

pub const USER_AGENT: &str = "simpleRedditClient/0.1 by ZestyZeke";

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String
}

#[derive(Deserialize, Debug)]
pub struct ChildData {
    subreddit: String,
    permalink: String
}

#[derive(Deserialize, Debug)]
pub struct Child {
    data: ChildData 
}

#[derive(Deserialize, Debug)]
pub struct ResponseData {
    children: Vec<Child>,
    after: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
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
 
pub async fn get_saved_posts(endpoint: &str, 
                             client: &reqwest::Client, 
                             auth_info: &AuthResponse,
                             last_received: Option<&String>) ->
    Result<ResponseData, Box<dyn std::error::Error>> {

    let mut api_url = Url::parse("https://oauth.reddit.com")?;
    api_url.set_path(&endpoint);
    if let Some(last) = &last_received {
        api_url.query_pairs_mut()
            .append_pair("after", last);
    }

    let auth_value = format!("{} {}", auth_info.token_type, auth_info.access_token);
    let builder = client.get(api_url.as_str())
        .header("Authorization", auth_value)
        .header("User-Agent", USER_AGENT);
    let resp = builder.send()
        .await?;
    if resp.status().is_server_error() {
        panic!("encountered invalid status: {}", resp.status())
        //resp.error_for_status()? TODO: get this to work somehow...

    } else {
        let api_response = resp.json::<ApiResponse>()
            .await?;
        Ok(api_response.data)

    //TODO: check rate limiting...
    //println!("{}", resp.text().await?);
    }

}

pub fn parse_response(saved_posts: &ResponseData, filter: &String) -> Option<String> {

    let reddit_url = "https://www.reddit.com";
    for child in saved_posts.children.iter() {
        let data = &child.data;
        if *filter == data.subreddit.to_lowercase() {
            println!("{}{}", reddit_url, data.permalink);
        }
    }
    saved_posts.after.clone()
}

pub fn print_map(subreddit_map: HashMap<String, u32>) {
    let mut list: Vec<(&String, &u32)> = subreddit_map.iter().collect();
    list.sort_by(|a, b| b.1.cmp(a.1));

    for (name, count) in list {
        println!("{:<30}: {}", name, count);
    }
}

pub fn update_map(subreddit_map: &mut HashMap<String, u32>, saved_posts: &ResponseData) {
    for child in saved_posts.children.iter() {
        let subreddit_name = child.data.subreddit.to_string();
        subreddit_map.entry(subreddit_name)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
}
