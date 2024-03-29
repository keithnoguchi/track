// SPDX-License-Identifier: GPL-2.0
use super::{work::Worker, Event};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Tracker {
    workers: Vec<Worker>,
    receiver: Arc<Mutex<mpsc::Receiver<Event>>>,
    delay_in_msec: u64,
    sample_count: u64,
    total_count: u64,
}

impl Tracker {
    pub fn new(config: &super::config::Config) -> Tracker {
        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let receiver = Arc::new(Mutex::new(receiver));
        let delay_in_msec = config.delay_in_msec;
        let sample_count = config.sample_count;
        let total_count = config.total_count;
        let mut workers = vec![];
        for track in &config.tracks {
            workers.push(Worker::new(&config, track, Arc::clone(&sender)));
        }
        Tracker {
            workers,
            receiver,
            delay_in_msec,
            sample_count,
            total_count,
        }
    }
    pub fn run(&mut self) {
        let mut delay = 0;
        for w in &mut self.workers {
            w.run(Duration::from_millis(delay));
            delay += self.delay_in_msec;
        }
        self.chart();
    }
    fn chart(&self) {
        let chart = MultiProgress::new();
        let sty = ProgressStyle::default_bar()
            .template("{msg:18} {bar:36.cyan/blue} {pos:>7}/{len:7} [{elapsed_precise}]")
            .progress_chars("##-");
        let mut bars = HashMap::new();
        for w in &self.workers {
            let pb = chart.add(ProgressBar::new(self.sample_count));
            pb.set_style(sty.clone());
            bars.insert(w.track.parse::<Event>().unwrap(), pb);
        }
        let receiver = Arc::clone(&self.receiver);
        let sample_count = self.sample_count;
        let total_count = self.total_count;
        thread::spawn(move || {
            for _ in 0..total_count {
                let e = receiver.lock().unwrap().recv().unwrap();
                let pb = bars.get(&e).unwrap();
                if pb.position() == sample_count {
                    pb.finish();
                    continue;
                }
                pb.set_message(&format!("{} trend:", e));
                pb.inc(1);
            }
        });
        chart.join_and_clear().unwrap();
    }
    /// dump the result to the standard out forver
    fn _dump(&self) {
        loop {
            eprint!("{:?}", self.receiver.lock().unwrap().recv().unwrap());
        }
    }
}
