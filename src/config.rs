// SPDX-License-Identifier: GPL-2.0
use std::env;

pub struct Config {
    pub consumer_key: String,
    pub consumer_sec: String,
    pub access_tkn: String,
    pub access_sec: String,
    pub delay_in_msec: u64,
    pub tracks: Vec<&'static str>,
}

impl Config {
    pub fn new() -> Config {
        let consumer_key = env::var("TRACK_CONSUMER_KEY").unwrap_or("dummy".to_string());
        let consumer_sec = env::var("TRACK_CONSUMER_SECRET").unwrap_or("dummy".to_string());
        let access_tkn = env::var("TRACK_ACCESS_TOKEN").unwrap_or("dummy".to_string());
        let access_sec = env::var("TRACK_ACCESS_SECRET").unwrap_or("dummy".to_string());
        let delay_in_msec = 10;
        let tracks = vec![
            "twitter",
            "facebook",
            //"google",
            //"travel",
            //"art",
            //"music",
            //"photography",
            //"love",
            //"fashion",
            //"food",
        ];
        Config {
            consumer_key,
            consumer_sec,
            access_tkn,
            access_sec,
            delay_in_msec,
            tracks,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default_track_entries() {
        let want = vec![
            "twitter",
            "facebook",
            //"google",
            //"travel",
            //"art",
            //"music",
            //"photography",
            //"love",
            //"fashion",
            //"food",
        ];
        let config = super::Config::new();
        assert_eq!(want, config.tracks);
    }
    #[test]
    fn delay_in_msec() {
        let config = super::Config::new();
        assert_eq!(10, config.delay_in_msec);
    }
}
