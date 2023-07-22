use nom::{bytes::complete::tag, combinator::map, multi::many1, sequence::tuple, IResult};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws};

use super::bool::parse_bool_value;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct DefBool {
    pub symbol: String,
    pub values: Vec<String>,
}

pub fn parse_def_bool(input: &str) -> IResult<&str, DefBool> {
    map(
        tuple((
            ws(tag("define_bool")),
            ws(parse_constant_symbol),
            many1(map(parse_bool_value, |d| d.to_string())),
        )),
        |(_, e, i)| DefBool {
            symbol: e.to_string(),
            values: i,
        },
    )(input)
}
