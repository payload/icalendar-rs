use super::*;
#[cfg(test)] use pretty_assertions::assert_eq;

/// Zero-copy version of `properties::Parameter`
#[derive(PartialEq, Debug, Clone)]
pub struct Parameter<'a> {
    pub key: &'a str,
    pub val: &'a str,
}

impl<'a> Into<crate::properties::Parameter> for Parameter<'a> {
    fn into(self) -> crate::properties::Parameter {
        crate::properties::Parameter::new(self.key, self.val)
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
    assert_eq!(
        dbg(parameter(b"; KEY=VAL UE")),
        Ok((&[][..], Parameter{key: "KEY", val: "VAL UE"})));
    assert_eq!(
        dbg(parameter(b"; KEY=")),
        Ok((&[][..], Parameter{key: "KEY", val: ""})));
}

#[test]
#[rustfmt::skip]
fn parse_parameter_error() {
    let dbg = |x| {println!("{:?}", x); x};
    assert!(dbg(parameter(b";KEY")).is_err());
}

pub fn parameter<'a>(i: &'a [u8]) -> IResult<&'a [u8], Parameter> {
    let (i, _) = tag(";")(i)?;
    let (i, _) = space0(i)?;
    let (i, key) = map_res(alpha, from_utf8)(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, val) = map_res(utils::alphanumeric_or_space, from_utf8)(i)?;
    Ok((i, Parameter { key, val }))
}

// parameter list
#[test]
#[rustfmt::skip]
pub fn parse_parameter_list() {
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

pub fn parameter_list<'a>(i: &'a [u8]) -> IResult<&'a [u8], Vec<Parameter>> {
    many0(parameter)(i)
}

