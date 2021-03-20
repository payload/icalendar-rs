#![allow(missing_docs, dead_code, unused_variables, unused_imports)]

use std::{convert::Into, default::default, rc::Rc, sync::Mutex};
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
}

fn read_ical_token(input: &str) -> IResult<&str, (&str, IcalToken)> {
    map(
        terminated(
            alt((
                self::components::read_begin,
                self::components::read_end,
                self::properties::read_property,
            )),
            opt(tag("\r")),
        ),
        |token| (input, token),
    )(input)
}

pub fn read_calendar_lines<'a>(
    input: &'a str,
) -> impl Iterator<Item = IResult<&'a str, (&'a str, IcalToken)>> {
    ical_lines(&input).map(read_ical_token)
}

fn read_components<'a>(ctx: &mut Context<'a>) -> Option<Vec<Component<'a>>> {
    match (ctx.current_component.as_ref(), ctx.line_source.next()) {
        (None, None) => None,

        // 
        (None, Some(Ok((_, (_, IcalToken::Begin(r#type)))))) => {
            let new_component = Component {
                r#type,
                properties: Default::default(),
                components: Default::default(),
            };
            // open new component
            None
        },
        (Some(current_component), Some(Ok((_, (_, IcalToken::End(name)))))) =>{
            // closing current component
            None
        },

        (None, Some(_)) => None,
        (Some(_), None) => None,
        (Some(_), Some(_)) => None,
    }
    // if let Some(parsed) = ctx.line_source.next() {
    //     if let Ok((_left_over, (_input, token))) = parsed  {
    //     }
    // } else {
    //     panic!("It's over!")
    // }
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
    current_component: Option<Component<'a>>,
    // line_source: IcalLineReader<'a>,
    line_source: Rc<Mutex<IcalTokenReader<'a>>>,
}

impl<'a> Context<'a> {
    fn fork(&self) -> Self {
        Context {
            current_component: None,
            line_source: self.line_source.clone(),
        }
    }
}


pub fn read_calendar<'a>(input: &'a str) -> Option<Vec<Component<'a>>> {
    let mut context = Context {
        current_component: None,
        line_source: Rc::new(Mutex::new(IcalLineReader::new(input).map(read_ical_token))),
    };
    read_components(&mut context)
    let IcalLineReader::new(input).map(read_ical_token).reduce();
}
