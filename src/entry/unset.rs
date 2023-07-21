use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::map,
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Unset {
    pub configs: Vec<String>,
}

pub fn parse_unset(input: &str) -> IResult<&str, Unset> {
    map(
        tuple((
            ws(tag("unset")),
            many1(map(preceded(space0, parse_constant_symbol), |s| {
                s.to_string()
            })),
        )),
        |(_, l)| Unset { configs: l },
    )(input)
}
