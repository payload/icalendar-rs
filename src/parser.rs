use libical::vcalendar::IcalVCalendar;
use libical::IcalVEvent;

use crate::Component;
use crate::Event;
use crate::Calendar;

impl From<IcalVCalendar> for Calendar {

    fn from(ical: IcalVCalendar) -> Calendar {
        let mut cal = Calendar::new();
        ical.events_iter()
            .map(|event| Event::from(event))
            .for_each(|ev| {
                cal.push(ev);
            });
        cal
    }
}

impl From<IcalVEvent> for Event {

    fn from(event: IcalVEvent) -> Event {
        let mut event = Event::new();

        if let Some(dtend) = event.get_dtend().map(|t| t.as_string()) {
            event.add_property("DTEND", &dtend);
        }

        if let Some(dtstart) = event.get_dtstart().map(|t| t.as_string()) {
            event.add_property("DTSTART", &dtstart);
        }

        if let Some(summary) = event.get_summary() {
            event.add_property("SUMMARY", &summary);
        }

        if let Some(description) = event.get_description() {
            event.add_property("DESCRIPTION", &description);
        }

        if let Some(location) = event.get_location() {
            event.add_property("LOCATION", &location);
        }

        if let Some(priority) = event.get_priority() {
        }

        let recur_datetimes = event.get_recur_datetimes().into_iter().map(|t| t.as_string());
        let uid             = dbg!(ical.get_uid());

        event.uid(&uid);

        event
    }
}

pub fn parse(s: &str) -> std::io::Result<Calendar> {
    IcalVCalendar::from_str(s, None).map(Calendar::from)
}

