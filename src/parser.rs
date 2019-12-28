use libical::vcalendar::IcalVCalendar;
use libical::IcalVEvent;

use crate::Component;
use crate::Event;
use crate::Calendar;

use std::convert::TryFrom;

impl TryFrom<IcalVCalendar> for Calendar {
    type Error = i32;

    fn try_from(ical: IcalVCalendar) -> Result<Calendar, Self::Error> {
        let mut cal = Calendar::new();
        let events = ical.events_iter()
            .map(|event| Event::try_from(event))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .for_each(|ev| {
                cal.push(ev);
            });

        Ok(cal)
    }
}

impl TryFrom<IcalVEvent> for Event {
    type Error = i32;

    fn try_from(ical: IcalVEvent) -> Result<Event, Self::Error> {
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

        Ok(event)
    }
}

pub fn parse(s: &str) -> Result<Calendar, i32> {
    IcalVCalendar::from_str(s, None).map_err(|_| 0).and_then(Calendar::try_from)
}

