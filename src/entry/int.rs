use nom::{
    bytes::complete::tag,
    character::complete::space1,
    combinator::{map, opt},
    sequence::{pair, preceded, tuple},
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<(i64, i64)>,
}

pub fn parse_int(input: &str) -> IResult<&str, Int> {
    map(
        tuple((
            ws(tag("int")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            ws(opt(map(parse_number, |s| s))),
            opt(preceded(space1, parse_range)),
        )),
        |(_, prompt, sym, default, range)| Int {
            prompt: prompt.to_string(),
            symbol: Symbol::Constant(sym.to_string()),
            range,
            default,
        },
    )(input)
}

pub fn parse_range(input: &str) -> IResult<&str, (i64, i64)> {
    pair(map(parse_number, |s| s), ws(map(parse_number, |s| s)))(input)
}
