// SPDX-License-Identifier: GPL-2.0
extern crate track;

fn main() {
    use track::track::Tracker;
    let c = track::config::Config::new();
    let mut tracker = Tracker::new(&c);
    tracker.run();
}
