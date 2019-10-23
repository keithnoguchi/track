// SPDX-License-Identifier: GPL-2.0
use std::env;

pub struct Config {
    pub consumer_key: String,
    pub consumer_sec: String,
    pub access_tkn: String,
    pub access_sec: String,
    pub delay_in_msec: u64,
    pub sample_count: u64,
    pub total_count: u64,
    pub default_tracks: Vec<&'static str>,
    pub tracks: Vec<String>,
}

impl Config {
    pub fn new(argv: Vec<String>) -> Config {
        let consumer_key = env::var("TRACK_CONSUMER_KEY").unwrap_or("dummy".to_string());
        let consumer_sec = env::var("TRACK_CONSUMER_SECRET").unwrap_or("dummy".to_string());
        let access_tkn = env::var("TRACK_ACCESS_TOKEN").unwrap_or("dummy".to_string());
        let access_sec = env::var("TRACK_ACCESS_SECRET").unwrap_or("dummy".to_string());
        let delay_in_msec = 10;
        let sample_count = 1000;
        let default_tracks = vec![
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
        let mut tracks = Vec::new();
        if argv.len() > 2 {
            tracks.push(argv[1].clone());
            tracks.push(argv[2].clone());
        } else {
            tracks.push("facebook".to_string());
            tracks.push("google".to_string());
        }
        let total_count = tracks.len() as u64 * sample_count;
        Config {
            consumer_key,
            consumer_sec,
            access_tkn,
            access_sec,
            delay_in_msec,
            sample_count,
            total_count,
            default_tracks,
            tracks,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default_tracks() {
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
        let config = super::Config::new(vec![]);
        assert_eq!(want, config.default_tracks);
    }
    #[test]
    fn delay_in_msec() {
        let config = super::Config::new(vec![]);
        assert_eq!(10, config.delay_in_msec);
    }
    #[test]
    fn sample_count() {
        let config = super::Config::new(vec![]);
        assert_eq!(1000, config.sample_count);
    }
    #[test]
    fn total_count() {
        let config = super::Config::new(vec![]);
        assert_eq!(2000, config.total_count);
    }
}
