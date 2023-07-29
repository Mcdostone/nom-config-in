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

use super::DefineType;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DefineInt {
    pub symbol: Symbol,
    pub default: Option<i64>,
}

pub fn parse_define_int(input: &str) -> IResult<&str, DefineType<i64>> {
    map(
        tuple((
            ws(tag("define_int")),
            ws(parse_constant_symbol),
            ws(opt(map(parse_number, |s| s))),
        )),
        |(_, sym, value)| DefineType {
            symbol: sym.to_string(),
            value,
        },
    )(input)
}
