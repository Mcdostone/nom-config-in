use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws};

use super::bool::parse_bool_value;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct DefBool {
    pub symbol: String,
    pub value: String,
}

pub fn parse_def_bool(input: &str) -> IResult<&str, DefBool> {
    map(
        tuple((
            ws(tag("define_bool")),
            ws(parse_constant_symbol),
            parse_bool_value,
        )),
        |(_, e, i)| DefBool {
            symbol: e.to_string(),
            value: i.to_string(),
        },
    )(input)
}
