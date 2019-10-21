// SPDX-License-Identifier: GPL-2.0
extern crate twitter_stream;

use twitter_stream::rt::{self, Future, Stream};
use twitter_stream::{Token, TwitterStreamBuilder};

fn main() {
    let c = Config::new();
    let future = TwitterStreamBuilder::filter(&c.token)
        .track(Some("twitter"))
        .listen()
        .flatten_stream()
        .for_each(|json| {
            println!("{}", json);
            Ok(())
        })
        .map_err(|e| println!("error: {}", e));

    rt::run(future);
}

struct Config {
    token: Token,
}

impl Config {
    fn new() -> Config {
        use std::env;
        let ckey = env::var("TRACK_CONSUMER_KEY").unwrap();
        let csecret = env::var("TRACK_CONSUMER_SECRET").unwrap();
        let atoken = env::var("TRACK_ACCESS_TOKEN").unwrap();
        let asecret = env::var("TRACK_ACCESS_SECRET").unwrap();
        let token = Token::new(ckey, csecret, atoken, asecret);
        Config { token }
    }
}
