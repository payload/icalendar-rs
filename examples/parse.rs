extern crate chrono;
extern crate icalendar;
use chrono::*;
use icalendar::*;

fn main() {
    let cal = include_str!("../invoicer.ics");
    let parsed = parse::calendar(&cal);
    println!("{:?}", parsed);
}
