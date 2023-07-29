use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};


use crate::{
    symbol::{parse_constant_symbol},
    util::ws,
};

use super::{string::parse_string_value, DefineType};

pub fn parse_define_string(input: &str) -> IResult<&str, DefineType<String>> {
    map(
        tuple((
            ws(tag("define_string")),
            ws(parse_constant_symbol),
            preceded(space0, parse_string_value),
        )),
        |(_, sym, value)| DefineType {
            symbol: sym.to_string(),
            value: Some(value),
        },
    )(input)
}
