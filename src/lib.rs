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
        for track in &self.config.tracks {
            Worker::new(&self.config, track).run();
        }
    }
}

pub struct Worker<'a> {
    config: &'a config::Config,
    track: &'a str,
}

impl<'a> Worker<'a> {
    pub fn new(config: &'a config::Config, track: &'a str) -> Worker<'a> {
        Worker { config, track }
    }
    pub fn run(&self) {
        use twitter_stream::rt::{self, Future, Stream};
        use twitter_stream::TwitterStreamBuilder;
        let future = TwitterStreamBuilder::filter(&self.config.token)
            .track(Some(self.track))
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
