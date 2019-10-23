// SPDX-License-Identifier: GPL-2.0
extern crate track;

use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let c = track::Config::new(argv);
    let mut tracker = track::Tracker::new(&c);
    tracker.run();
}
