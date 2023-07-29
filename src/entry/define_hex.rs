use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};


use crate::{symbol::parse_constant_symbol, util::ws};

use super::{hex::parse_hex_value, DefineType};

pub fn parse_define_hex(input: &str) -> IResult<&str, DefineType<String>> {
    map(
        tuple((
            ws(tag("define_hex")),
            ws(parse_constant_symbol),
            opt(map(parse_hex_value, |d: &str| d.to_string())),
        )),
        |(_, sym, value)| DefineType {
            symbol: sym.to_string(),
            value,
        },
    )(input)
}
