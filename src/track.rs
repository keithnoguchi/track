// SPDX-License-Identifier: GPL-2.0
pub struct Tracker<'a> {
    workers: Vec<super::work::Worker<'a>>,
}

impl<'a> Tracker<'a> {
    pub fn new(config: &'a super::config::Config) -> Tracker<'a> {
        let mut workers = vec![];
        for track in &config.tracks {
            let w = super::work::Worker::new(config, track);
            workers.push(w);
        }
        Tracker { workers }
    }
    pub fn run(&'a mut self) {
        // XXX Single worker only for now.
        self.workers[0].run();
    }
}
