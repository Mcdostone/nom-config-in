use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::map,
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
pub struct Hwaddr {
    pub prompt: String,
    pub symbol: Symbol,
    pub value: String,
}

pub fn parse_hwaddr(input: &str) -> IResult<&str, Hwaddr> {
    map(
        tuple((
            ws(tag("hwaddr")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            ws(map(parse_constant_symbol, |d: &str| d.to_string())),
        )),
        |(_, prompt, sym, value)| Hwaddr {
            prompt: prompt.to_string(),
            symbol: Symbol::Constant(sym.to_string()),
            value,
        },
    )(input)
}

pub fn parse_hex_value(input: &str) -> IResult<&str, &str> {
    preceded(space0, parse_constant_symbol)(input)
}
