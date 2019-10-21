// SPDX-License-Identifier: GPL-2.0
extern crate twitter_stream;

pub struct Config {
    pub token: twitter_stream::Token,
}

impl Config {
    pub fn new() -> Config {
        use std::env;
        use twitter_stream::Token;
        let ckey = env::var("TRACK_CONSUMER_KEY").unwrap();
        let csecret = env::var("TRACK_CONSUMER_SECRET").unwrap();
        let atoken = env::var("TRACK_ACCESS_TOKEN").unwrap();
        let asecret = env::var("TRACK_ACCESS_SECRET").unwrap();
        let token = Token::new(ckey, csecret, atoken, asecret);
        Config { token }
    }
}

pub struct Tracker {
    config: Config,
}

impl Tracker {
    pub fn new(config: Config) -> Tracker {
        Tracker { config }
    }
    pub fn run(&self) {
        use twitter_stream::rt::{self, Future, Stream};
        use twitter_stream::TwitterStreamBuilder;
        let future = TwitterStreamBuilder::filter(&self.config.token)
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
}
