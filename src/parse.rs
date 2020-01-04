#![allow(missing_docs, dead_code, unused_variables, unused_imports)]

#[cfg(test)]
use pretty_assertions::assert_eq;

use std::convert::Into;
use std::str::from_utf8;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while},
    character::complete::{
        alpha1 as alpha, alphanumeric1 as alphanumeric, char, line_ending, multispace0, one_of,
        space0,
    },
    character::{is_alphabetic, is_alphanumeric},
    combinator::{cut, map, map_res, opt},
    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
    multi::{many0, many_till},
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated},
    Err, IResult,
};

use crate::properties;

////////// Parameters

/// Zero-copy version of `properties::Parameter`
#[derive(PartialEq, Debug, Clone)]
struct Parameter<'a> {
    key: &'a str,
    val: &'a str,
}

impl<'a> Into<properties::Parameter> for Parameter<'a> {
    fn into(self) -> properties::Parameter {
        properties::Parameter::new(self.key, self.val)
    }
}

#[test]
#[rustfmt::skip]
fn parse_parameter() {
    let dbg = |x| {println!("{:?}", x); x};
    assert_eq!(
        dbg(parameter(b";KEY=VALUE")),
        Ok((&[][..], Parameter{key: "KEY", val: "VALUE"})));
    assert_eq!(
        dbg(parameter(b"; KEY=VALUE")),
        Ok((&[][..], Parameter{key: "KEY", val: "VALUE"})));
    assert!(dbg(parameter(b";KEY")).is_err());
    assert!(dbg(parameter(b";KEY=")).is_err());
}

fn parameter<'a>(i: &'a [u8]) -> IResult<&'a [u8], Parameter> {
    let (i, _) = tag(";")(i)?;
    let (i, _) = space0(i)?;
    let (i, key) = map_res(alpha, from_utf8)(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, val) = map_res(alphanumeric, from_utf8)(i)?;
    Ok((i, Parameter { key, val }))
}

// parameter list
#[test]
#[rustfmt::skip]
fn parse_parameter_list() {
    assert_eq!(
        parameter_list(b";KEY=VALUE"),
        Ok( (&[][..], vec![Parameter{key: "KEY", val: "VALUE"}])));

    assert_eq!(
        parameter_list(b";KEY=VALUE;DATE=TODAY"),
        Ok( (&[][..], vec![
             Parameter{key: "KEY", val: "VALUE"},
             Parameter{key: "DATE", val:"TODAY"}
        ])));

    assert_eq!(
        parameter_list(b";KEY=VALUE;DATE=20170218"),
        Ok( (&[][..], vec![
             Parameter{key: "KEY", val: "VALUE"},
             Parameter{key: "DATE", val:"20170218"}
        ])));
}

fn parameter_list<'a>(i: &'a [u8]) -> IResult<&'a [u8], Vec<Parameter>> {
    many0(parameter)(i)
}

////////// Properties

/// Zero-copy version of `properties::Property`
#[derive(PartialEq, Debug, Clone)]
struct Property<'a> {
    key: &'a str,
    val: &'a str,
    params: Vec<Parameter<'a>>,
}

#[test]
#[rustfmt::skip]
fn parse_property() {
    assert_eq!( property(b"KEY:VALUE\n"), Ok((&[][..], Property{key: "KEY", val: "VALUE", params: vec![]} )));

    assert_eq!(
        property(b"KEY;foo=bar:VALUE\n"),
        Ok((&[][..], Property{key: "KEY", val: "VALUE", params: vec![
            Parameter{key:"foo", val: "bar"}
            ]})));
}

#[test]
#[ignore]
#[rustfmt::skip]
fn parse_property_with_breaks() {

    let sample_0 = b"DESCRIPTION:Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendri\n k";

    let expectation = Property {
        key: "DESCRIPTION",
        val: "Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik",
        params: vec![]
    };

    assert_eq!(property(sample_0), Ok((&[][..], expectation)));
}

fn property<'a>(i: &'a [u8]) -> IResult<&'a [u8], Property> {
    let (i, _) = multispace0(i)?;
    let (i, key) = map_res(alpha, from_utf8)(i)?;
    let (i, params) = parameter_list(i)?;
    let (i, _) = tag(":")(i)?;

    let (i, val) = map_res(alphanumeric, from_utf8)(i)?;

    let (i, _) = line_ending(i)?;
    Ok((i, Property { key, val, params }))
}

#[test]
#[rustfmt::skip]
fn parse_property_list() {

    assert_eq!(
        property_list(b"KEY;foo=bar:VALUE\n  KEY;foo=bar; DATE=20170218:VALUE\n"),
        Ok((&[][..], vec![
             Property{key: "KEY", val: "VALUE", params: vec![ Parameter{key:"foo", val: "bar"} ]},
             Property{key: "KEY", val: "VALUE", params: vec![
                 Parameter{key:"foo", val: "bar"},
                 Parameter{key:"DATE", val: "20170218"},
             ]}
        ]))
        );
    assert_eq!(
        property_list(b"KEY;foo=bar:VALUE\nKEY;foo=bar;DATE=20170218:VALUE\n"),
        Ok((&[][..], vec![
             Property{key: "KEY", val: "VALUE", params: vec![ Parameter{key:"foo", val: "bar"} ]},
             Property{key: "KEY", val: "VALUE", params: vec![
                 Parameter{key:"foo", val: "bar"},
                 Parameter{key:"DATE", val: "20170218"},
             ]}
        ]))
        );
    assert_eq!(
        property_list(b""),
        Ok((&[][..], vec![ ])));
}

fn property_list<'a>(i: &'a [u8]) -> IResult<&'a [u8], Vec<Property>> {
    many0(property)(i)
}

////////// Components

#[derive(PartialEq, Debug, Clone)]
pub struct Component<'a> {
    name: &'a str,
    properties: Vec<Property<'a>>,
}

#[test]
#[ignore]
#[rustfmt::skip]
fn parse_empty_component() {
    assert_eq!(component(b"BEGIN:VEVENT\nEND:VEVENT\n"), Ok((&[][..], Component{name: "VEVENT", properties: vec![]})));

    assert_eq!(
        component(b"BEGIN:VEVENT\n\nEND:VEVENT\n"),
        Ok((&[][..],
             Component{name: "VEVENT", properties: vec![]}
             )));
    assert_eq!(
        component(b"BEGIN:VEVENT\nEND:VEVENT\n"),
        Ok((&[][..],
             Component{name: "VEVENT", properties: vec![]}
             )));
}

#[test]
#[rustfmt::skip]
fn parse_component() {
    let sample_0 = b"BEGIN:VEVENT\nKEY;foo=bar:VALUE\nKEY;foo=bar;DATE=20170218:VALUE\nEND:VEVENT\n";
    let sample_1 = b"BEGIN:VEVENT
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

    //assert_eq!(
    //    component(sample_1),
    //    Done(&[][..], expectation.clone()));

    assert_eq!(
        component(sample_1).unwrap().1,
        expectation.clone());
}

pub fn calendar(raw: &str) -> Vec<Component> {
    let parsed = components(raw.as_bytes());
    println!("{:?}", parsed);
    if let Ok((_, components)) = parsed {
        components
    } else {
        vec![]
    }
}

fn component<'a>(i: &'a [u8]) -> IResult<&'a [u8], Component> {
    let (i, _) = tag("BEGIN:")(i)?;
    let (i, name) = map_res(alpha, from_utf8)(i)?;
    let (i, (properties, _)) = many_till(property, tag("END:"))(i)?;
    let (i, _) = tag(name)(i)?;

    Ok((i, Component { name, properties }))
}

fn components<'a>(i: &'a [u8]) -> IResult<&'a [u8], Vec<Component>> {
    many0(component)(i)
}
