use serde::Deserialize;

pub const USER_AGENT: &str = "simpleRedditClient/0.1 by ZestyZeke";

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String
}

#[derive(Deserialize, Debug)]
pub struct ChildData {
    pub subreddit: String,
    pub permalink: String
}

#[derive(Deserialize, Debug)]
pub struct Child {
    pub data: ChildData 
}

#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub children: Vec<Child>,
    pub after: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub data: ResponseData 
}

// {
//   'children': [
//        'data': {
//          'subreddit': <val>,
//          'permalink': <val>
//        }
//   ]
// }
 
