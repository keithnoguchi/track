// SPDX-License-Identifier: GPL-2.0
use super::{work::Worker, Event};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

pub struct Tracker {
    workers: Vec<Worker>,
    receiver: mpsc::Receiver<Event>,
    delay_in_msec: u64,
    _chart: MultiProgress,
    _bars: Vec<ProgressBar>,
}

impl Tracker {
    pub fn new(config: &super::config::Config) -> Tracker {
        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let delay_in_msec = config.delay_in_msec;
        let _chart = MultiProgress::new();
        let _sty = ProgressStyle::default_bar()
            .template("{msg} {bar:40.cyan/blue} {pos:>7}/{len:7} [elapsed_precise}]")
            .progress_chars("##-");
        let mut workers = vec![];
        let mut _bars = vec![];
        for track in &config.tracks {
            let pb = _chart.add(ProgressBar::new(1000));
            pb.set_style(_sty.clone());
            _bars.push(pb);
            workers.push(Worker::new(&config, track, Arc::clone(&sender)));
        }
        Tracker {
            _bars,
            _chart,
            workers,
            receiver,
            delay_in_msec,
        }
    }
    pub fn run(&mut self) {
        let mut delay = 0;
        for w in &mut self.workers {
            w.run(Duration::from_millis(delay));
            delay += self.delay_in_msec;
        }
        loop {
            eprint!("{}", self.receiver.recv().unwrap())
        }
    }
}
