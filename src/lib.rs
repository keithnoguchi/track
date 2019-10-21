// SPDX-License-Identifier: GPL-2.0
extern crate twitter_stream;

pub mod config;

pub struct Tracker {
    config: config::Config,
}

impl Tracker {
    pub fn new(config: config::Config) -> Tracker {
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
