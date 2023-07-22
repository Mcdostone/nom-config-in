use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0},
    combinator::{eof, map, opt},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use serde::Serialize;

use crate::{
    symbol::{parse_constant_symbol, Symbol},
    util::ws,
};

use super::comment::parse_prompt_option;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct StringConfig {
    pub symbol: Symbol,
    pub prompt: String,
    pub value: Option<String>,
}

pub fn parse_string(input: &str) -> IResult<&str, StringConfig> {
    map(
        tuple((
            ws(tag("string")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            preceded(space0, opt(parse_string_value)),
        )),
        |(_, p, e, v)| StringConfig {
            prompt: p.to_string(),
            symbol: Symbol::Constant(e.to_string()),
            value: v,
        },
    )(input)
}

pub fn parse_string_value(input: &str) -> IResult<&str, String> {
    map(
        alt((
            parse_prompt_option,
            terminated(not_line_ending, alt((line_ending, eof))),
        )),
        |d| d.to_string(),
    )(input)
}
