use std::{env::args, fs::read_to_string};

fn _read_line_by_line(sample: &str) {
    for line in icalendar::parse::read_calendar_lines(sample) {
        match line {
            Ok((_, line)) => {
                println!("✅ {:?}", line)
            } // {dbg!(line);},
            Err(error) => {
                println!("🤬 {:?}", error)
            }
        }
    }
}

fn main() {
    if let Some(sample) = args().nth(1).map(read_to_string) {
        dbg!(icalendar::parse::read_calendar(&sample.unwrap()));
    }

    // let parsed = parse::calendar(sample);
    // println!("{:?}", parsed);
}
