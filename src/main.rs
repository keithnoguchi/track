// SPDX-License-Identifier: GPL-2.0
extern crate twitter_stream;

use std::env;
use twitter_stream::rt::{self, Future, Stream};
use twitter_stream::{Token, TwitterStreamBuilder};

fn main() {
    let consumer_key = env::var("TRACK_CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("TRACK_CONSUMER_SECRET").unwrap();
    let access_token = env::var("TRACK_ACCESS_TOKEN").unwrap();
    let access_secret = env::var("TRACK_ACCESS_SECRET").unwrap();
    let token = Token::new(consumer_key, consumer_secret, access_token, access_secret);
    let future = TwitterStreamBuilder::filter(&token)
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
