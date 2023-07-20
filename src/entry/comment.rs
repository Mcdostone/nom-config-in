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

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Comment {
    pub prompt: String,
}

pub fn parse_comment(input: &str) -> IResult<&str, Comment> {
    map(
        tuple((ws(tag("comment")), ws(parse_prompt_option))),
        |(_, prompt)| Comment {
            prompt: prompt.to_string(),
        },
    )(input)
}

pub fn parse_prompt_option(input: &str) -> IResult<&str, &str> {
    map(
        alt((delimited(ws(char('\'')), take_until("'"), char('\'')),)),
        |d: &str| d.trim(),
    )(input)
}
