// SPDX-License-Identifier: GPL-2.0
use super::work::Worker;
use std::fmt;
use std::str;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time;

#[derive(Clone, Copy)]
pub enum Track {
    Twitter,
    Facebook,
    Google,
    Travel,
    Art,
    Music,
    Photography,
    Love,
    Fashion,
    Food,
    Other,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Track::Twitter => write!(f, "T"),
            Track::Facebook => write!(f, "F"),
            Track::Google => write!(f, "G"),
            Track::Travel => write!(f, "Tr"),
            Track::Art => write!(f, "Ar"),
            Track::Music => write!(f, "Mu"),
            Track::Photography => write!(f, "Ph"),
            Track::Love => write!(f, "Lo"),
            Track::Fashion => write!(f, "Fa"),
            Track::Food => write!(f, "Fo"),
            _ => write!(f, "."),
        }
    }
}

impl str::FromStr for Track {
    type Err = ();
    fn from_str(s: &str) -> Result<Track, ()> {
        match s {
            "twitter" => Ok(Track::Twitter),
            "facebook" => Ok(Track::Facebook),
            "google" => Ok(Track::Google),
            "travel" => Ok(Track::Travel),
            "art" => Ok(Track::Art),
            "music" => Ok(Track::Music),
            "photography" => Ok(Track::Photography),
            "love" => Ok(Track::Love),
            "fashion" => Ok(Track::Fashion),
            "food" => Ok(Track::Food),
            _ => Ok(Track::Other),
        }
    }
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
            w.run(time::Duration::from_secs(delay));
            delay += self.delay_in_sec;
        }
        loop {
            eprint!("{}", self.receiver.recv().unwrap())
        }
    }
}
