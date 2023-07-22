use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{
    symbol::{parse_constant_symbol, Symbol},
    util::ws,
};

use super::hex::parse_hex_value;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DefineHex {
    pub symbol: Symbol,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

pub fn parse_define_hex(input: &str) -> IResult<&str, DefineHex> {
    map(
        tuple((
            ws(tag("define_hex")),
            ws(parse_constant_symbol),
            opt(map(parse_hex_value, |d: &str| d.to_string())),
        )),
        |(_, sym, value)| DefineHex {
            symbol: Symbol::Constant(sym.to_string()),
            value,
        },
    )(input)
}
