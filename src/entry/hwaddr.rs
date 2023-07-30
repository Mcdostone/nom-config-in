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
    ConfigInInput,
};

use super::comment::parse_prompt_option;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Hwaddr {
    pub prompt: String,
    pub symbol: Symbol,
    pub value: String,
}

pub fn parse_hwaddr(input: ConfigInInput) -> IResult<ConfigInInput, Hwaddr> {
    map(
        tuple((
            ws(tag("hwaddr")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            ws(parse_constant_symbol),
        )),
        |(_, prompt, sym, value)| Hwaddr {
            prompt: prompt.to_string(),
            symbol: sym.to_string(),
            value: value.fragment().to_string(),
        },
    )(input)
}

pub fn parse_hex_value(input: ConfigInInput) -> IResult<ConfigInInput, ConfigInInput> {
    preceded(space0, parse_constant_symbol)(input)
}
