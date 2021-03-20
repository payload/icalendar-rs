#![allow(missing_docs, dead_code, unused_variables, unused_imports)]

use std::convert::Into;
use std::rc::Rc;
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
    sequence::{delimited, preceded, separated_pair, terminated},
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

use self::utils::{ical_lines, IcalLineReader};

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
}

fn read_ical_token(input: &str) -> IResult<&str, (&str, IcalToken)> {
    map(
        alt((
            self::components::read_begin,
            self::components::read_end,
            self::properties::read_property,
        )),
        |token| (input, token),
    )(input)
}

pub fn read_calendar_lines<'a>(
    input: &'a str,
) -> impl Iterator<Item = IResult<&'a str, (&'a str, IcalToken)>> {
    ical_lines(&input).map(read_ical_token)
}

fn read_components<'a>(ctx: &mut Context<'a>) -> Vec<Component<'a>> {
    if let Some(token) = ctx.line_source.next() {
        println!("{:?}", token);
        todo!()
    } else {
        panic!("It's over!")
    }
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

#[derive(Debug)]
struct Context<'a> {
    current_cal: Option<Calendar<'a>>,
    current_component: Option<Component<'a>>,
    higher_component: Option<Component<'a>>,
    calendars: Vec<Calendar<'a>>,
    line_source: IcalLineReader<'a>,
}

pub fn read_calendar<'a>(input: &'a str) -> Vec<Component<'a>> {
    let mut context = Context {
        current_cal: None,
        current_component: None,
        higher_component: None,
        calendars: Default::default(),
        line_source: IcalLineReader::new(input),
    };
    read_components(&mut context)
}
