use clap::{Arg, App};
mod reddit_client;
mod utils;
mod config;
mod models;
mod mapper;
mod subparsers;

fn to_lower(word: Option<&str>) -> Option<String> {
    match word {
        Some(w) => {
            Some(w.to_string()
                 .to_lowercase())
        },
        _ => None,
    }
}

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
             .takes_value(true))
        .arg(Arg::with_name("map")
             .short("m")
             .long("map")
             .help("if given, indicates to print a category map of saved posts"))
        .get_matches();

    let filter = to_lower(matches.value_of("filter"));
    let mut client = reddit_client::RedditClient::new(matches.is_present("map"));
    //TODO: maybe pass subparsers to run function? so don't have to make client mut'able?
    client.run(filter).await
}
