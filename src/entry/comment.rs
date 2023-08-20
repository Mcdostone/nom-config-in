use nom::character::complete::char;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};
use serde::Serialize;

use crate::util::ws;
use crate::ConfigInInput;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Comment {
    pub prompt: String,
}

pub fn parse_comment(input: ConfigInInput) -> IResult<ConfigInInput, String> {
    map(
        tuple((ws(tag("comment")), ws(parse_prompt_option))),
        |(_, prompt)| prompt.to_string(),
    )(input)
}

pub fn parse_prompt_option(input: ConfigInInput) -> IResult<ConfigInInput, String> {
    map(
        alt((
            delimited(ws(char('"')), take_until("\""), char('"')),
            delimited(ws(char('\'')), take_until("'"), char('\'')),
        )),
        |d: ConfigInInput| d.fragment().to_string(),
    )(input)
}
