// SPDX-License-Identifier: GPL-2.0
extern crate twitter_stream;

pub mod config;
pub mod work;

pub struct Tracker {
    config: config::Config,
}

impl Tracker {
    pub fn new(config: config::Config) -> Tracker {
        Tracker { config }
    }
    pub fn run(&self) {
        for track in &self.config.tracks {
            work::Worker::new(&self.config, track).run();
        }
    }
}
