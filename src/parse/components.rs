
use super::*;
#[cfg(test)]
use pretty_assertions::assert_eq;

#[derive(PartialEq, Debug, Clone)]
pub struct Component<'a> {
    pub name: &'a str,
    pub properties: Vec<Property<'a>>,
}

#[test]
#[rustfmt::skip]
#[ignore]
fn parse_empty_component1() {
    assert_eq!(
        component("BEGIN:VEVENT\nEND:VEVENT\n"),
        Ok(("", Component{name: "VEVENT", properties: vec![]}))
    );

}

#[test]
#[rustfmt::skip]
#[ignore]
fn parse_empty_component2() {
    assert_eq!(
        component("BEGIN:VEVENT\n\nEND:VEVENT\n"),
        Ok(("", Component{name: "VEVENT", properties: vec![]})),
        "empty component with empty line");
}

#[test]
#[rustfmt::skip]
#[ignore]
fn parse_empty_component3() {
    assert_eq!(
        component("BEGIN:VEVENT\nEND:VEVENT\n"),
        Ok(("", Component{name: "VEVENT", properties: vec![]})),
        "empty component");
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
            ]};

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

pub fn component(i: &str) -> IResult<&str, Component> {
    let (i, _) = tag("BEGIN:")(i)?;
    let (i, name) = alpha(i)?;
    let (i, (properties, _)) = many_till(property, tag("END:"))(i)?;
    let (i, _) = tag(name)(i)?;

    Ok((i, Component { name, properties }))
}

pub fn components(i: &str) -> IResult<&str, Vec<Component>> {
    many0(component)(i)
}
