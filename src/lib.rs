// SPDX-License-Identifier: GPL-2.0
extern crate twitter_stream;

pub mod config;
pub mod work;

pub struct Tracker<'a> {
    config: config::Config,
    workers: Vec<work::Worker<'a>>,
}

impl<'a> Tracker<'a> {
    pub fn new(config: config::Config) -> Tracker<'a> {
        let workers = vec![];
        Tracker { config, workers }
    }
    pub fn run(&'a mut self) {
        for track in &self.config.tracks {
            let w = work::Worker::new(&self.config, track);
            self.workers.push(w);
        }
        // XXX Single worker only now.
        self.workers[0].run();
    }
}
