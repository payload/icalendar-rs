use libical::vcalendar::IcalVCalendar;
use libical::IcalVEvent;

use crate::Component;
use crate::Event;
use crate::Calendar;

use std::{
    io
};

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

    fn from(ical: IcalVEvent) -> Event {
        let mut event = Event::new();

        if let Some(dtend) = ical.get_dtend().map(|t| t.as_string()) {
            event.add_property("DTEND", &dtend);
        }

        if let Some(dtstart) = ical.get_dtstart().map(|t| t.as_string()) {
            event.add_property("DTSTART", &dtstart);
        }

        if let Some(summary) = ical.get_summary() {
            event.add_property("SUMMARY", &summary);
        }

        if let Some(description) = ical.get_description() {
            event.add_property("DESCRIPTION", &description);
        }

        if let Some(location) = ical.get_location() {
            event.add_property("LOCATION", &location);
        }

        let recur_datetimes = ical.get_recur_datetimes().into_iter().map(|t| t.as_string());
        let uid             = ical.get_uid();

        event.uid(&uid);

        event
    }
}

pub fn parse(s: &str) -> io::Result<Calendar> {
    IcalVCalendar::from_str(s, None).map(Calendar::from)
}

