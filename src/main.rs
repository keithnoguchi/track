// SPDX-License-Identifier: GPL-2.0
extern crate track;
extern crate twitter_stream;

use twitter_stream::rt::{self, Future, Stream};
use twitter_stream::TwitterStreamBuilder;

fn main() {
    let c = track::Config::new();
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
