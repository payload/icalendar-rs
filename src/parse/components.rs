use super::*;
use nom::sequence::tuple;
#[cfg(test)]
use pretty_assertions::assert_eq;

#[derive(PartialEq, Debug, Clone)]
pub struct Component<'a> {
    pub name: &'a str,
    pub properties: Vec<Property<'a>>,
}

/// VCALENDAR, VEVENT, VVENUE, VTODO
pub fn read_component_type<'a>(input: &'a str) -> IResult<&str, ComponentType<'a>> {
    alt((
        map(tag("VCALENDAR"), |_| ComponentType::Calendar),
        map(tag("VEVENT"), |_| ComponentType::Event),
        map(tag("VVENUE"), |_| ComponentType::Venue),
        map(tag("VTODO"), |_| ComponentType::Todo),
        map(alphanumeric, ComponentType::Other),
    ))(input)
}

/// BEGIN:...
pub fn read_begin(input: &str) -> IResult<&str, IcalToken> {
    map(map_res(preceded(tag("BEGIN:"), alpha), read_component_type), |(_, t)| {
        IcalToken::Begin(t)
    })(input)
}

/// END:...
pub fn read_end(input: &str) -> IResult<&str, IcalToken> {
    map(map_res(preceded(tag("END:"), alpha), read_component_type), |(_, t)| {
        IcalToken::End(t)
    })(input)
}