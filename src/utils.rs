use url::Url;
use crate::models;

pub async fn get_saved_posts(endpoint: &str, 
                             client: &reqwest::Client, 
                             auth_info: &models::AuthResponse,
                             last_received: Option<&String>) ->
    Result<models::ResponseData, Box<dyn std::error::Error>> {

    let mut api_url = Url::parse("https://oauth.reddit.com")?;
    api_url.set_path(&endpoint);
    if let Some(last) = &last_received {
        api_url.query_pairs_mut()
            .append_pair("after", last);
    }

    let auth_value = format!("{} {}", auth_info.token_type, auth_info.access_token);
    let builder = client.get(api_url.as_str())
        .header("Authorization", auth_value)
        .header("User-Agent", models::USER_AGENT);
    let resp = builder.send()
        .await?;
    if resp.status().is_server_error() {
        panic!("encountered invalid status: {}", resp.status())
        //resp.error_for_status()? TODO: get this to work somehow...

    } else {
        let api_response = resp.json::<models::ApiResponse>()
            .await?;
        Ok(api_response.data)

    //TODO: check rate limiting...
    //println!("{}", resp.text().await?);
    }

}

pub fn parse_response(saved_posts: &models::ResponseData, 
                      filter: &Option<String>) -> Option<String> {

    let reddit_url = "https://www.reddit.com";
    for child in saved_posts.children.iter() {
        let data = &child.data;
        match filter {
            Some(f) => {
                if *f == data.subreddit.to_lowercase() {
                    println!("{}{}", reddit_url, data.permalink);
                }
            },
            _ => println!("{}{}", reddit_url, data.permalink),
        };
    }
    saved_posts.after.clone()
}
