use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws};

use super::comment::parse_prompt_option;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct DepTristate {
    pub prompt: String,
    pub symbol: String,
    pub value: String,
    pub other: String,
}

pub fn parse_dep_tristate(input: &str) -> IResult<&str, DepTristate> {
    map(
        tuple((
            ws(tag("dep_tristate")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            ws(parse_constant_symbol),
            ws(parse_constant_symbol),
        )),
        |(_, p, e, i, o)| DepTristate {
            prompt: p.to_string(),
            symbol: e.to_string(),
            value: i.to_string(),
            other: o.to_string(),
        },
    )(input)
}
