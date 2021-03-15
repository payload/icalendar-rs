use super::{IcalToken, parameters::{read_parameters, Parameter}, utils, utils::alpha_or_dash};
use nom::{IResult, bytes::complete::tag, character::complete::{line_ending, multispace0}, combinator::{map, opt}, sequence::{preceded, separated_pair, tuple}};
#[cfg(test)]
use pretty_assertions::assert_eq;

/// Zero-copy version of `properties::Property`
#[derive(PartialEq, Debug, Clone)]
pub struct Property<'a> {
    pub key: &'a str,
    pub val: &'a str,
    pub params: Vec<Parameter<'a>>,
}

#[test]
#[rustfmt::skip]
fn test_property() {
    assert_eq!(property("KEY:VALUE\n"), Ok(("", Property{key: "KEY", val: "VALUE", params: vec![]} )));

    assert_eq!(
        property("KEY;foo=bar:VALUE\n"),
        Ok(("", Property{key: "KEY", val: "VALUE", params: vec![
            Parameter{key:"foo", val: "bar"}
            ]})));
    assert_eq!(
        property("KEY;foo=bar:VALUE space separated\n"),
        Ok(("", Property{key: "KEY", val: "VALUE space separated", params: vec![
            Parameter{key:"foo", val: "bar"}
            ]})));
    // TODO: newlines followed by spaces must be ignored
    assert_eq!(
        property("KEY;foo=bar:VALUE\n newline separated\n"),
        Ok(("", Property{key: "KEY", val: "VALUE\n newline separated", params: vec![
            Parameter{key:"foo", val: "bar"}
            ]})));
}

#[test]
#[rustfmt::skip]
fn parse_property_with_breaks() {

    let sample_0 = "DESCRIPTION:Hey, I'm gonna have a party\n BYOB: Bring your own beer.\n Hendri\n k\n";

    let expectation = Property {
        key: "DESCRIPTION",
        val: "Hey, I'm gonna have a party\n BYOB: Bring your own beer.\n Hendri\n k",
        params: vec![]
    };

    assert_eq!(property(sample_0), Ok(("", expectation)));
}

fn property(input: &str) -> IResult<&str, Property> {
    map(
        tuple((
            separated_pair(
                tuple((
                    preceded(multispace0, alpha_or_dash), // key
                    read_parameters,                      // params
                )),
                tag(":"),         // separator
                utils::ical_line, // val
            ),
            opt(line_ending),
        )),
        |(((key, params), val), _)| Property { key, val, params },
    )(input)
}

pub fn read_property(input: &str) -> IResult<&str, IcalToken> {
    map(property, super::IcalToken::Property)(input)
}

#[test]
fn test_read_property() {
    assert_eq!(
        read_property("KEY;foo=bar:VALUE\n"),
        Ok((
            "",
            super::IcalToken::Property(Property {
                key: "KEY",
                val: "VALUE",
                params: vec![Parameter {
                    key: "foo",
                    val: "bar"
                }]
            })
        ))
    );
}
