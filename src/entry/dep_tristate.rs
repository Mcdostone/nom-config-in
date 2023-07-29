use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws};

use super::comment::parse_prompt_option;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct DepTristate {
    pub prompt: String,
    pub symbol: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<String>,
}

pub fn parse_dep_tristate(input: &str) -> IResult<&str, DepTristate> {
    map(
        tuple((
            ws(tag("dep_tristate")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            ws(parse_constant_symbol),
            opt(preceded(
                space0,
                map(parse_constant_symbol, |d| d.to_string()),
            )),
        )),
        |(_, p, e, i, depends_on)| DepTristate {
            prompt: p.to_string(),
            symbol: e.to_string(),
            value: i.to_string(),
            depends_on,
        },
    )(input)
}
