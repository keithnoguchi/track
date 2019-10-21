// SPDX-License-Identifier: GPL-2.0
extern crate track;

fn main() {
    let c = track::config::Config::new();
    let tracker = track::Tracker::new(c);
    tracker.run();
}
