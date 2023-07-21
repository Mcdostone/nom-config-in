use nom::{
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
            preceded(space0, opt(map(parse_prompt_option, |s| s.to_string()))),
        )),
        |(_, p, e, v)| StringConfig {
            prompt: p.to_string(),
            symbol: Symbol::Constant(e.to_string()),
            value: v,
        },
    )(input)
}
