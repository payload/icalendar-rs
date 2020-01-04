
use super::*;
#[cfg(test)] use pretty_assertions::assert_eq;

/// Zero-copy version of `properties::Property`
#[derive(PartialEq, Debug, Clone)]
pub struct Property<'a> {
    pub key: &'a str,
    pub val: &'a str,
    pub params: Vec<Parameter<'a>>,
}

#[test]
#[rustfmt::skip]
fn parse_property() {
    assert_eq!(property(b"KEY:VALUE\n"), Ok((&[][..], Property{key: "KEY", val: "VALUE", params: vec![]} )));

    assert_eq!(
        property(b"KEY;foo=bar:VALUE\n"),
        Ok((&[][..], Property{key: "KEY", val: "VALUE", params: vec![
            Parameter{key:"foo", val: "bar"}
            ]})));
    assert_eq!(
        property(b"KEY;foo=bar:VALUE space separated\n"),
        Ok((&[][..], Property{key: "KEY", val: "VALUE space separated", params: vec![
            Parameter{key:"foo", val: "bar"}
            ]})));
    // TODO: newlines followed by spaces must be ignored
    assert_eq!(
        property(b"KEY;foo=bar:VALUE\n newline separated\n"),
        Ok((&[][..], Property{key: "KEY", val: "VALUE\n newline separated", params: vec![
            Parameter{key:"foo", val: "bar"}
            ]})));
}

#[test]
#[rustfmt::skip]
fn parse_property_with_breaks() {

    let sample_0 = b"DESCRIPTION:Hey, I'm gonna have a party\n BYOB: Bring your own beer.\n Hendri\n k\n";

    let expectation = Property {
        key: "DESCRIPTION",
        val: "Hey, I'm gonna have a party\n BYOB: Bring your own beer.\n Hendri\n k",
        params: vec![]
    };

    assert_eq!(property(sample_0), Ok((&[][..], expectation)));
}

pub fn property<'a>(i: &'a [u8]) -> IResult<&'a [u8], Property> {
    let (i, _) = multispace0(i)?;
    let (i, key) = map_res(alpha, from_utf8)(i)?;
    let (i, params) = parameter_list(i)?;
    let (i, _) = tag(":")(i)?;

    let (i, val) = map_res(utils::ical_lines, from_utf8)(i)?;
    // let (i, val) = map_res(utils::alphanumeric_or_space, from_utf8)(i)?;

    let (i, _) = line_ending(i)?;
    Ok((i, Property { key, val, params }))
}

/*
#[test]
#[rustfmt::skip]
fn parse_property_list() {

    assert_eq!(
        property_list(b"KEY;foo=bar:VALUE\n  KEYA;foo=bar; DATE=20170218:VALUE\n"),
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

pub fn property_list<'a>(i: &'a [u8]) -> IResult<&'a [u8], Vec<Property>> {
    many0(property)(i)
}
*/
