// SPDX-License-Identifier: GPL-2.0
extern crate twitter_stream;

use std::thread;

pub struct Worker {
    token: twitter_stream::Token,
    track: String,
}

impl Worker {
    pub fn new(c: &super::Config, track: &str) -> Worker {
        let token = twitter_stream::Token::new(
            c.consumer_key.clone(),
            c.consumer_sec.clone(),
            c.access_tkn.clone(),
            c.access_sec.clone(),
        );
        let track = track.to_string();
        Worker { token, track }
    }
    pub fn run(&self) {
        use twitter_stream::rt::{self, Future, Stream};
        use twitter_stream::TwitterStreamBuilder;
        let token = self.token.clone();
        let track = self.track.clone();
        thread::spawn(move || {
            let track = &track[..];
            let future = TwitterStreamBuilder::filter(&token)
                .track(Some(track))
                .listen()
                .flatten_stream()
                .for_each(|json| {
                    println!("{}", json);
                    Ok(())
                })
                .map_err(|e| println!("error: {}", e));
            rt::run(future);
        });
    }
}
