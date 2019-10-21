// SPDX-License-Identifier: GPL-2.0
pub struct Config {
    pub token: twitter_stream::Token,
    pub tracks: Vec<&'static str>,
}

impl Config {
    pub fn new() -> Config {
        use std::env;
        use twitter_stream::Token;
        let ckey = env::var("TRACK_CONSUMER_KEY").unwrap_or("dummy".to_string());
        let csecret = env::var("TRACK_CONSUMER_SECRET").unwrap_or("dummy".to_string());
        let atoken = env::var("TRACK_ACCESS_TOKEN").unwrap_or("dummy".to_string());
        let asecret = env::var("TRACK_ACCESS_SECRET").unwrap_or("dummy".to_string());
        let token = Token::new(ckey, csecret, atoken, asecret);
        let tracks = vec![
            "twitter",
            "facebook",
            "google",
            "travel",
            "art",
            "music",
            "photography",
            "love",
            "fashion",
            "food",
        ];
        Config { token, tracks }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default_track_entries() {
        let want = vec![
            "twitter",
            "facebook",
            "google",
            "travel",
            "art",
            "music",
            "photography",
            "love",
            "fashion",
            "food",
        ];
        let config = super::Config::new();
        assert_eq!(want, config.tracks);
    }
}
