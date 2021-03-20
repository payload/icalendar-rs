#![allow(missing_docs, dead_code, unused_variables, unused_imports)]

use std::{convert::Into, default::Default, rc::Rc, sync::Mutex};
use std::str::from_utf8;

use nom::{
    branch::alt,
    bytes::complete::{escaped, escaped_transform, tag, take_till, take_while},
    character::complete::{
        alpha1 as alpha, alphanumeric1 as alphanumeric, char, line_ending, multispace0, one_of,
        space0,
    },
    character::{is_alphabetic, is_alphanumeric},
    combinator::{cut, map, map_res, opt},
    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
    multi::{many0, many_till},
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated, terminatedc},
    Err, IResult,
};

mod utils;
//mod lines;

////////// Parameters
mod parameters;

////////// Properties
pub mod properties;
use properties::*;

////////// Components
pub mod components;
use components::*;
use utils::alpha_or_dash;

use self::utils::{ical_lines, IcalLineReader, IcalTokenReader};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ComponentType<'a> {
    Calendar,
    Event,
    Todo,
    Venue,
    Other(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum IcalToken<'a> {
    Begin(ComponentType<'a>),
    End(ComponentType<'a>),
    Property(self::properties::Property<'a>),
    Garbage(&'a str),
}

fn read_ical_token(input: &str) -> IResult<&str, (&str, IcalToken)> {
    map(
        terminated(
            alt((
                self::components::read_begin,
                self::components::read_end,
                self::properties::read_property,
                map(alpha_or_dash, IcalToken::Garbage),
            )),
            opt(tag("\r")),
        ),
        |token| (input, token),
    )(input)
}

/*

ical_inner_stuff() {
    seq(literal("BEGIN:CAL\n"), cal_body, literal("END:CAL")) -> struct Cal { desc: "", ... }
}

cal_body() {
    seq(alt(desc, date, tag, ical_inner_stuff, not_ical)) -> Vec<CalInnerStuff>
}

not_ical() {
    panic!()
}

*/

pub fn read_calendar_lines<'a>(
    input: &'a str,
) -> impl Iterator<Item = IResult<&'a str, (&'a str, IcalToken)>> {
    ical_lines(&input).map(read_ical_token)
}

#[derive(Debug)]
pub struct Component<'a> {
    r#type: ComponentType<'a>,
    properties: Vec<Property<'a>>,
    components: Vec<Component<'a>>,
}

#[derive(Debug, Default)]
pub struct Calendar<'a> {
    components: Vec<Component<'a>>,
}

// #[derive(Debug)]
// struct Context<'a> {
//     current_component: Option<Component<'a>>,
//     // line_source: IcalLineReader<'a>,
//     line_source: Rc<IcalTokenReader<'a>>,
// }

// impl<'a> Context<'a> {
//     fn fork(&self) -> Self {
//         Context {
//             current_component: None,
//             line_source: self.line_source.clone(),
//         }
//     }
// }


pub fn read_calendar<'a>(input: &'a str) -> Option<Vec<Component<'a>>> {
    // let mut context = Context {
    //     current_component: None,
    //     line_source: Rc::new(Mutex::new(IcalLineReader::new(input).map(read_ical_token))),
    // };

    let reader = IcalLineReader::new(input).map(read_ical_token);
    for bla in reader {
        println!("{:?}", bla);
    }

    Some(vec![])
    // read_components(&mut context)
    // let IcalLineReader::new(input).map(read_ical_token).reduce();
}
