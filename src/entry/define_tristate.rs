use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws};

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct DefineTristate {
    pub symbol: String,
    pub value: Option<String>,
}

pub fn parse_define_tristate(input: &str) -> IResult<&str, DefineTristate> {
    map(
        tuple((
            ws(tag("define_tristate")),
            ws(parse_constant_symbol),
            opt(preceded(
                space0,
                map(parse_constant_symbol, |d| d.to_string()),
            )),
        )),
        |(_, p, value)| DefineTristate {
            symbol: p.to_string(),
            value,
        },
    )(input)
}
