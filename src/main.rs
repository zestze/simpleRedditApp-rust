use clap::{Arg, App};
mod reddit_client;
mod utils;
mod config;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("simple_reddit_app")
        .version("0.1")
        .author("zest")
        .about("Reddit app to filter through saved posts by subreddit")
        .arg(Arg::with_name("filter")
             .short("f")
             .long("filter")
             .help("the subreddit to filter on")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("map")
             .short("m")
             .long("map")
             .help("if given, indicates to print a category map of saved posts"))
        .get_matches();

    let client = reddit_client::RedditClient::new();
    let filter = matches.value_of("filter")
        .unwrap()
        .to_string()
        .to_lowercase();
    client.run(filter, matches.is_present("map"))
        .await
}
