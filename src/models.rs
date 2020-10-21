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
    pub permalink: String,
    pub num_comments: i32,
    pub upvote_ratio: Option<f32>,
    pub ups: i32,
    pub score: i32,
    pub total_awards_received: i32,
    pub suggested_sort: Option<String>
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
 
