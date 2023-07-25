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

use super::string::parse_string_value;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DefineString {
    pub symbol: Symbol,
    pub value: String,
}

pub fn parse_define_string(input: &str) -> IResult<&str, DefineString> {
    map(
        tuple((
            ws(tag("define_string")),
            ws(parse_constant_symbol),
            preceded(space0, parse_string_value),
        )),
        |(_, sym, value)| DefineString {
            symbol: Symbol::Constant(sym.to_string()),
            value,
        },
    )(input)
}
