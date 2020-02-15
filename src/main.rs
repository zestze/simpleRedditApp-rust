extern crate clap;
use clap::{Arg, App};

//TODO: put in separate class...
struct RedditClient {
    username: String,
    password: String,
    client_id: String,
    client_secret: String,
    session_token: String,
    session_token_type: String
}

impl RedditClient {
    fn new(username: &str) -> RedditClient {
        return RedditClient {
            username: username.to_string(),
            password: username.to_string(),
            client_id: username.to_string(),
            client_secret: username.to_string(),
            session_token: username.to_string(),
            session_token_type: username.to_string()
        }
    }
    fn run(&self, filter: &str, should_map: bool) {
        println!("filter is: {}", filter);
        println!("should is: {}", should_map);
    }
}

fn main() {
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

    println!("matches filter is: {}", matches.value_of("filter").unwrap());
    println!("matches map exists: {}", matches.is_present("map"));

    let reddit_client = RedditClient::new("blah");
    reddit_client.run(matches.value_of("filter").unwrap(),
                        matches.is_present("map"));
}
