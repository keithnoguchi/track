// SPDX-License-Identifier: GPL-2.0
pub struct Tracker {
    workers: Vec<super::work::Worker>,
}

impl Tracker {
    pub fn new(config: &super::config::Config) -> Tracker {
        let mut workers = vec![];
        for track in &config.tracks {
            let w = super::work::Worker::new(&config, track);
            workers.push(w);
        }
        Tracker { workers }
    }
    pub fn run(&mut self) {
        for w in &mut self.workers {
            w.run();
        }
        loop {}
    }
}
