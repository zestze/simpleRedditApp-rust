use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Client {
    pub id: String,
    pub secret: String
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub client: Client,
    pub user: User 
}

impl Config {
    pub fn new(filename: &str) -> Config {
        let s = fs::read_to_string(filename)
            .expect("wasn't able to read file!");
        serde_json::from_str(&s)
            .expect("json was not well formatted")
    }
}
