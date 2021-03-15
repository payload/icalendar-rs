#![allow(missing_docs, dead_code, unused_variables, unused_imports)]

use std::convert::Into;
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

use self::utils::ical_lines;

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

pub fn read_calendar_lines(input: &str) -> impl Iterator<Item = IResult<&str, (&str, IcalToken)>> {
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

#[derive(Debug, Default)]
struct Context<'a> {
    current_cal: Option<Calendar<'a>>,
    current_component: Option<Component<'a>>,
    higher_component: Option<Component<'a>>,
    calendars: Vec<Calendar<'a>>,
}

pub fn read_calendar<'a>(input: &'a str) -> Vec<Calendar<'a>> {
    read_calendar_lines(input)
        .enumerate()
        .fold(Context::default(), |ctx, (line_index, token)| {
            let (_, (line, token)) = token.unwrap();
            let current_component_type = ctx.current_component.as_ref().map(|c| c.r#type.clone());

            let Context {
                mut current_cal,
                mut current_component,
                mut higher_component,
                mut calendars,
            } = ctx;

            match token {
                IcalToken::Begin(typ) => {
                    match (typ, &mut current_cal, &mut current_component) {
                        (ComponentType::Calendar, None, None) => {
                            // OK fresh start
                            current_cal = Some(Calendar::default());
                        }
                        (ComponentType::Calendar, None, Some(_)) => {
                            // ERR this is weird
                        }
                        (ComponentType::Calendar, Some(_), None) => {
                            // ERR this is weird too
                        }
                        (ComponentType::Calendar, Some(_), Some(_)) => {
                            // ERR this is extra weird
                        }

                        (r#type, None, None) => {
                            // OK lets just create a new component, even without a calendar
                            current_cal = Some(Calendar::default());
                            current_component = Some(Component {
                                r#type,
                                properties: Default::default(),
                                components: Default::default(),
                            });
                        }

                        (r#type, Some(cal), None) => {
                            // OK lets just create a new component
                            current_component = Some(Component {
                                r#type,
                                properties: Default::default(),
                                components: Default::default(),
                            });
                        }
                        (typ, _, Some(_)) => {
                            // OK, this 
                        }
                    }
                }
                IcalToken::End(typ) => {
                    match (
                        typ,
                        &mut current_cal,
                        &mut current_component,
                        current_component_type,
                    ) {
                        (ComponentType::Calendar, None, _, _) => {
                            panic!("unexpected end of calendar");
                        }
                        (ComponentType::Calendar, Some(_cal), _, None) => {
                            // OK close calendar
                            calendars.push(current_cal.take().unwrap())
                        }
                        (ComponentType::Calendar, Some(_), _, Some(_)) => {
                            // ERR, you don't get to start a new component, you haven't ENDed the last one yet, you never finish anything!
                        }

                        (ended_type, _, _, None) => {
                            panic!("unexpected end of {:?}, nothing opened yet", ended_type);
                        }

                        (ended_type, _, _, Some(r#type)) if ended_type != r#type => {
                            panic!(
                                "unexpected end of {:?}, expected {:?} (LINE {})\n{:?}",
                                ended_type,
                                r#type,
                                line_index + 1,
                                line
                            );
                        }

                        (ended_type, None, _, Some(r#type)) if ended_type == r#type => {
                            // ok I guess, we just create a calendar anyway?
                            let new_component = current_component.take().unwrap();
                            current_cal = Some(Calendar {
                                components: vec![new_component],
                            });
                        }

                        (ended_type, Some(_cal), _, Some(r#type)) if ended_type == r#type => {
                            // ok I guess, we just create a calendar anyway?
                            let new_component = current_component.take().unwrap();
                            _cal.components.push(new_component);
                        }

                        (a, b, c, Some(d)) => {
                            unreachable!(
                                "now this is really unexpected {:?}, {:?}, {:?}, {:?}",
                                a, b, c, d
                            )
                        }
                    }
                }
                IcalToken::Property(property) => {
                    // match ctx.current_component {}
                }
            };
            Context {
                calendars,
                current_cal,
                current_component,
            }
        })
        .calendars
}
