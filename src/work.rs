// SPDX-License-Identifier: GPL-2.0
extern crate twitter_stream;

use super::Track;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use twitter_stream::rt::{self, Future, Stream};
use twitter_stream::TwitterStreamBuilder;

pub struct Worker {
    token: twitter_stream::Token,
    track: String,
    sender: Arc<Mutex<mpsc::Sender<Track>>>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(c: &super::Config, track: &str, sender: Arc<Mutex<mpsc::Sender<Track>>>) -> Worker {
        let token = twitter_stream::Token::new(
            c.consumer_key.clone(),
            c.consumer_sec.clone(),
            c.access_tkn.clone(),
            c.access_sec.clone(),
        );
        let track = track.to_string();
        Worker {
            token,
            track,
            sender,
            thread: None,
        }
    }
    pub fn run(&mut self, delay: Duration) {
        let msg = self.track.parse::<super::Track>().unwrap();
        let token = self.token.clone();
        let track = self.track.clone();
        let sender = Arc::clone(&self.sender);
        let thread = thread::spawn(move || {
            thread::sleep(delay);
            let track = &track[..];
            let future = TwitterStreamBuilder::filter(&token)
                .track(Some(track))
                .listen()
                .flatten_stream()
                .for_each(move |_json| {
                    //println!("{}", _json);
                    sender.lock().unwrap().send(msg).unwrap();
                    Ok(())
                })
                .map_err(|e| println!("error: {}", e));
            rt::run(future);
        });
        self.thread = Some(thread);
    }
}
