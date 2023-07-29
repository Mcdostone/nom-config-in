use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use super::{bool::parse_bool_value, comment::parse_prompt_option, r#type::Type};
use crate::{symbol::parse_constant_symbol, util::ws};

pub fn parse_tristate(input: &str) -> IResult<&str, Type<String>> {
    map(
        tuple((
            ws(tag("tristate")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            preceded(space0, opt(map(parse_tristate_value, |d| d.to_string()))),
        )),
        |(_, p, e, i)| Type {
            prompt: p.to_string(),
            symbol: e.to_string(),
            value: i,
        },
    )(input)
}

pub fn parse_tristate_value(input: &str) -> IResult<&str, &str> {
    alt((parse_bool_value, tag("m")))(input)
}
