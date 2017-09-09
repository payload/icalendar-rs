#![allow(missing_docs, dead_code, unused_variables, unused_imports)]

use std::str::from_utf8;
use std::convert::Into;

use nom::*;
use nom::IResult::*;

use crate::properties;

////////// Parameters

/// Zero-copy version of `properties::Parameter`
#[derive(PartialEq, Debug, Clone)]
struct Parameter<'a> {
    key: &'a str,
    val: &'a str
}

impl<'a> Into<properties::Parameter> for Parameter<'a> {
    fn into(self) -> properties::Parameter {
        properties::Parameter::new(self.key, self.val)
    }
}


#[test]
fn parse_parameter() {
    assert_eq!(
        parameter(b";KEY=VALUE"),
        Done(&[][..], Parameter{key: "KEY", val: "VALUE"})
        );
    assert_eq!(
        parameter(b"; KEY=VALUE"),
        Done(&[][..], Parameter{key: "KEY", val: "VALUE"})
        );
    assert!( parameter(b";KEY").is_incomplete());
    assert!( parameter(b";KEY=").is_incomplete());
}

named!(parameter(&[u8]) -> Parameter,
       do_parse!(
           tag!(";") >>
           opt!(space) >>
           key: map_res!(alpha, from_utf8) >>
           tag!("=") >>
           val: map_res!(alphanumeric, from_utf8) >>
           (Parameter{key: key, val: val})
           )
      );

// parameter list

#[test]
fn parse_parameter_list() {
    assert_eq!(
        parameter_list(b";KEY=VALUE"),
        Done(&[][..], vec![Parameter{key: "KEY", val: "VALUE"}])
        );

    assert_eq!(
        parameter_list(b";KEY=VALUE;DATE=TODAY"),
        Done(&[][..], vec![
             Parameter{key: "KEY", val: "VALUE"},
             Parameter{key: "DATE", val:"TODAY"}
        ])
        );

    assert_eq!(
        parameter_list(b";KEY=VALUE;DATE=20170218"),
        Done(&[][..], vec![
             Parameter{key: "KEY", val: "VALUE"},
             Parameter{key: "DATE", val:"20170218"}
        ])
        );
}

named!(parameter_list(&[u8]) -> Vec<Parameter>,
        many0!(parameter)
);



////////// Properies

/// Zero-copy version of `properties::Property`
#[derive(PartialEq, Debug, Clone)]
struct Property<'a> {
    key: &'a str,
    val: &'a str,
    params: Vec<Parameter<'a>>
}

#[test]
fn parse_propery() {
    assert_eq!( property(b"KEY:VALUE\n"), Done(&[][..], Property{key: "KEY", val: "VALUE", params: vec![]} ));

    assert_eq!(
        property(b"KEY;foo=bar:VALUE\n"),
        Done(&[][..], Property{key: "KEY", val: "VALUE", params: vec![
            Parameter{key:"foo", val: "bar"}
            ]})
        );
}

#[test]
#[ignore]
fn parse_property_with_breaks() {

    let sample_0 = b"DESCRIPTION:Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendri\n k";

    let expectation = Property {
        key: "DESCRIPTION",
        val: "Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik",
        params: vec![]
    };

    assert_eq!(property(sample_0), Done(&[][..], expectation));
}

named!(property(&[u8]) -> Property,
    do_parse!(
        opt!(multispace) >>
        key: map_res!(alpha, from_utf8) >>
        params: parameter_list >>
        tag!(":") >>
        val: map_res!(alphanumeric, from_utf8) >>
        line_ending >>
        (Property{key: key, val: val, params: params})
    )
);

#[test]
fn parse_propery_list() {

    assert_eq!(
        property_list(b"KEY;foo=bar:VALUE\n  KEY;foo=bar; DATE=20170218:VALUE\n"),
        Done(&[][..], vec![
             Property{key: "KEY", val: "VALUE", params: vec![ Parameter{key:"foo", val: "bar"} ]},
             Property{key: "KEY", val: "VALUE", params: vec![
                 Parameter{key:"foo", val: "bar"},
                 Parameter{key:"DATE", val: "20170218"},
             ]}
        ])
        );
    assert_eq!(
        property_list(b"KEY;foo=bar:VALUE\nKEY;foo=bar;DATE=20170218:VALUE\n"),
        Done(&[][..], vec![
             Property{key: "KEY", val: "VALUE", params: vec![ Parameter{key:"foo", val: "bar"} ]},
             Property{key: "KEY", val: "VALUE", params: vec![
                 Parameter{key:"foo", val: "bar"},
                 Parameter{key:"DATE", val: "20170218"},
             ]}
        ])
        );
    assert_eq!(
        property_list(b""),
        Done(&[][..], vec![ ]));
}

named!(property_list(&[u8]) -> Vec<Property>,
        many0!(property)
);

////////// Components

#[derive(PartialEq, Debug, Clone)]
pub struct Component<'a> {
    name: &'a str,
    properties: Vec<Property<'a>>
}

#[test]
#[ignore]
fn parse_empty_component() {
    assert_eq!(component(b"BEGIN:VEVENT\nEND:VEVENT\n"), Done(&[][..], Component{name: "VEVENT", properties: vec![]}));

    assert_eq!(
        component(b"BEGIN:VEVENT\n\nEND:VEVENT\n"),
        Done(&[][..],
             Component{name: "VEVENT", properties: vec![]}
             ));
    assert_eq!(
        component(b"BEGIN:VEVENT\nEND:VEVENT\n"),
        Done(&[][..],
             Component{name: "VEVENT", properties: vec![]}
             ));
}

#[test]
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

named!{components(&[u8]) -> Vec<Component>,
    many0!(component)
}

pub fn calendar(raw: &str) -> Vec<Component> {
    let parsed = components(raw.as_bytes());
    println!("{:?}", parsed);
    if let Done(_, components) = parsed {
        components
    } else { vec![] }
}

named!(component(&[u8]) -> Component,
    // dbg!(
    do_parse!(
        tag!("BEGIN:") >> name: map_res!(alpha, from_utf8) >>
        properties: many_till!(

            do_parse!(p:property >> (p)),
            tag!("END:")

        ) >>
        tag!(name) >>

        (Component{name: name, properties: properties.0})
        )
        //)
    );

