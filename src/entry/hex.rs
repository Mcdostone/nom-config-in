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
pub struct Hex {
    pub prompt: String,
    pub symbol: Symbol,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

pub fn parse_hex(input: &str) -> IResult<&str, Hex> {
    map(
        tuple((
            ws(tag("hex")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            opt(map(parse_hex_value, |d: &str| d.to_string())),
        )),
        |(_, prompt, sym, value)| Hex {
            prompt: prompt.to_string(),
            symbol: Symbol::Constant(sym.to_string()),
            value,
        },
    )(input)
}

pub fn parse_hex_value(input: &str) -> IResult<&str, &str> {
    preceded(space0, parse_constant_symbol)(input)
}
