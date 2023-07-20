use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws};

use super::comment::parse_prompt_option;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct Tristate {
    pub prompt: String,
    pub symbol: String,
    pub value: String,
}

pub fn parse_tristate(input: &str) -> IResult<&str, Tristate> {
    map(
        tuple((
            ws(tag("tristate")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            ws(parse_constant_symbol),
        )),
        |(_, p, e, i)| Tristate {
            prompt: p.to_string(),
            symbol: e.to_string(),
            value: i.to_string(),
        },
    )(input)
}
