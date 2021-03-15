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
    let sample = include_str!("../test/example_write.ics");
    // let sample = include_str!("../fixtures/two_time_events.ics");

    dbg!(icalendar::parse::read_calendar(sample));

    // let parsed = parse::calendar(sample);
    // println!("{:?}", parsed);
}
