use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{
    entry::expression::parse_number,
    symbol::{parse_constant_symbol, Symbol},
    util::ws,
};

use super::comment::parse_prompt_option;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Int {
    pub prompt: String,
    pub symbol: Symbol,
    pub default: Option<i64>,
}

pub fn parse_int(input: &str) -> IResult<&str, Int> {
    map(
        tuple((
            ws(tag("int")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            ws(opt(map(parse_number, |s| s))),
        )),
        |(_, prompt, sym, default)| Int {
            prompt: prompt.to_string(),
            symbol: Symbol::Constant(sym.to_string()),
            default,
        },
    )(input)
}
