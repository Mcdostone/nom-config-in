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

use super::comment::parse_prompt_option;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct DepBool {
    pub prompt: String,
    pub symbol: String,
    pub select: Vec<String>,
}

pub fn parse_dep_bool(input: &str) -> IResult<&str, DepBool> {
    map(
        tuple((
            ws(tag("dep_bool")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            ws(preceded(
                space0,
                many1(map(parse_constant_symbol, |d| d.to_string())),
            )),
        )),
        |(_, p, e, i)| DepBool {
            prompt: p.to_string(),
            symbol: e.to_string(),
            select: i,
        },
    )(input)
}
