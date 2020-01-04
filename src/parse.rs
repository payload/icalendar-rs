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

////////// Parameters
pub mod parameters;
use parameters::*;

////////// Properties

pub mod properties;
use properties::*;

////////// Components
pub mod components;
use components::*;
