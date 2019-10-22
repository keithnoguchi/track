// SPDX-License-Identifier: GPL-2.0
use super::work::Worker;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time;

pub enum Track {
    Twitter,
    Facebook,
    Google,
    Other,
}

pub struct Tracker {
    workers: Vec<super::work::Worker>,
    receiver: mpsc::Receiver<Track>,
    delay_in_sec: u64,
}

impl Tracker {
    pub fn new(config: &super::config::Config) -> Tracker {
        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let delay_in_sec = config.delay_in_sec;
        let mut workers = vec![];
        for track in &config.tracks {
            workers.push(Worker::new(&config, track, Arc::clone(&sender)));
        }
        Tracker {
            workers,
            receiver,
            delay_in_sec,
        }
    }
    pub fn run(&mut self) {
        let mut delay = 0;
        for w in &mut self.workers {
            (*w).run(time::Duration::from_secs(delay));
            delay += self.delay_in_sec;
        }
        loop {
            self.receiver.recv().unwrap();
            eprint!(".");
        }
    }
}
