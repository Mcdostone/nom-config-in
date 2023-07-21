use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{
    symbol::{parse_constant_symbol, Symbol},
    util::ws,
};

use super::comment::parse_prompt_option;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Bool {
    pub prompt: String,
    pub symbol: Symbol,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

pub fn parse_bool(input: &str) -> IResult<&str, Bool> {
    map(
        tuple((
            ws(tag("bool")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            opt(map(parse_bool_value, |d: &str| d.to_string())),
        )),
        |(_, prompt, sym, default)| Bool {
            prompt: prompt.to_string(),
            symbol: Symbol::Constant(sym.to_string()),
            default,
        },
    )(input)
}

pub fn parse_bool_value(input: &str) -> IResult<&str, &str> {
    alt((
        ws(alt((tag("y"), tag("n")))),
        ws(alt((tag("\"y\""), tag("\"n\"")))),
        preceded(space0, parse_constant_symbol),
    ))(input)
}
