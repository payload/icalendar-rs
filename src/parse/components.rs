use super::*;
use crate::parse::parameters::Parameter;
use nom::{combinator::optc, sequence::tuple};
#[cfg(test)]
use pretty_assertions::assert_eq;

#[derive(PartialEq, Debug, Clone)]
pub struct Component<'a> {
    pub name: &'a str,
    pub properties: Vec<Property<'a>>,
    pub components: Vec<Component<'a>>,
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
    map(
        map_res(preceded(tag("BEGIN:"), alpha), read_component_type),
        |(_, t)| IcalToken::Begin(t),
    )(input)
}

/// END:...
pub fn read_end(input: &str) -> IResult<&str, IcalToken> {
    map(
        map_res(preceded(tag("END:"), alpha), read_component_type),
        |(_, t)| IcalToken::End(t),
    )(input)
}

#[test]
#[rustfmt::skip]
fn parse_empty_component1() {
    assert_eq!(
        component("BEGIN:VEVENT\nEND:VEVENT\n"),
        Ok(("", Component{name: "VEVENT", properties: vec![], components: vec![] }))
    );

}

#[test]
#[rustfmt::skip]
#[ignore]
fn parse_empty_component2() {
    assert_eq!(
        component("BEGIN:VEVENT\n\nEND:VEVENT\n"),
        Ok(("", Component{name: "VEVENT", properties: vec![], components: vec![]})),
        "empty component with empty line");
}

#[test]
#[rustfmt::skip]
fn parse_component() {
    let sample_0 = "BEGIN:VEVENT\nKEY;foo=bar:VALUE\nKEY;foo=bar;DATE=20170218:VALUE\nEND:VEVENT\n";
    let sample_1 = "BEGIN:VEVENT
KEY;foo=bar:VALUE
KEY;foo=bar;DATE=20170218:VALUE
END:VEVENT
";

    //assert_eq!(from_utf8(sample_0), from_utf8(sample_1));

    let expectation = Component{name: "VEVENT", properties: vec![
            Property{key: "KEY", val: "VALUE", params: vec![
                Parameter{key:"foo", val: "bar"},
            ]},
            Property{key: "KEY", val: "VALUE", params: vec![
                Parameter{key:"foo", val: "bar"},
                Parameter{key:"DATE", val: "20170218"},
            ]},
            ], components: vec![]};

    println!("expectation: {:#?}", expectation);
    println!("vs reality : {:#?}", component(sample_1));

    assert_eq!(
        component(sample_1).unwrap().1,
        expectation.clone());
}

pub fn calendar(raw: &str) -> Vec<Component> {
    let parsed = components(raw);
    println!("{:?}", parsed);
    if let Ok((_, components)) = parsed {
        components
    } else {
        vec![]
    }
}

enum ComponentBody<'a> {
    Property(Property<'a>),
    Component(Component<'a>),
}

pub fn component<'a>(i: &str) -> IResult<&str, Component> {
    let (i, _) = preceded(opt(tag("\n")), tag("BEGIN:"))(i)?;
    let (i, name) = alpha(i)?;

    let ctx = format!("inside {}", name);
    let property_ctx = ctx.clone();
    let (i, (body_elements, _)) = many_till(
        alt((
            map(
                context("component", preceded(opt(tag("\n")), component)),
                |x| ComponentBody::Component(x),
            ),
            map(
                context("property", preceded(opt(tag("\n")), property)),
                ComponentBody::Property,
            ),
        )),
        context(
            "preceded",
            preceded(opt(tag("\n")), preceded(tag("END:"), tag(name))),
        ),
    )(i)?;
    let (i, _) = opt(tag("\n"))(i)?;

    let mut properties = Vec::new();
    let mut components = Vec::new();
    for el in body_elements {
        match el {
            ComponentBody::Component(c) => components.push(c),
            ComponentBody::Property(p) => properties.push(p),
        }
    }

    // Ok((i, Component { name, properties, components }))
    Ok((
        i,
        Component {
            name,
            properties,
            components,
        },
    ))
}

#[test]
fn test_component() {
    assert_eq!(
        component("BEGIN:FOO\nEND:FOO").unwrap(),
        ("", Component {
            name: "FOO",
            properties: vec![],
            components: vec![]
        })
    );
    assert_eq!(
        component("BEGIN:FOO\nFOO-PROP:spam\nEND:FOO").unwrap(),
        ("", Component {
            name: "FOO",
            properties: vec![Property {key :"FOO-PROP", val: "spam", params: vec![]}],
            components: vec![]
        })
    );
    assert_eq!(
        component("BEGIN:FOO\nFOO-PROP:spam\nBEGIN:BAR\nBAR-PROP:spam\nEND:BAR\nEND:FOO").unwrap(),
        (
            "",
            Component {
                name: "FOO",
                properties: vec![Property {
                    key: "FOO-PROP",
                    val: "spam",
                    params: vec![]
                }],
                components: vec![Component {
                    name: "BAR",
                    properties: vec![Property {
                        key: "BAR-PROP",
                        val: "spam",
                        params: vec![]
                    }],
                    components: vec![]
                }]
            }
        )
    );
}

pub fn components<'a>(i: &str) -> IResult<&str, Vec<Component>> {
    many0(component)(i)
}
