extern crate chrono;
extern crate icalendar;
use chrono::*;
use icalendar::*;

fn main() {
    let sample = include_str!("../test/example_write.ics");
    let parsed = parse::calendar(sample);
    println!("{:?}", parsed);
}
