// SPDX-License-Identifier: GPL-2.0
pub struct Config {
    pub token: twitter_stream::Token,
    pub tracks: Vec<&'static str>,
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
        let tracks = vec!["twitter"];
        Config { token, tracks }
    }
}
