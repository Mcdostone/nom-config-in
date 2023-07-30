use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use crate::{symbol::parse_constant_symbol, util::ws, ConfigInInput};

use super::{comment::parse_prompt_option, r#type::Type};

pub fn parse_bool(input: ConfigInInput) -> IResult<ConfigInInput, Type<String>> {
    map(
        tuple((
            ws(tag("bool")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            opt(map(parse_bool_value, |d| d.to_string())),
        )),
        |(_, prompt, sym, value)| Type {
            prompt: prompt.to_string(),
            symbol: sym.to_string(),
            value,
        },
    )(input)
}

pub fn parse_bool_value(input: ConfigInInput) -> IResult<ConfigInInput, ConfigInInput> {
    map(
        alt((
            ws(alt((tag("y"), tag("n")))),
            ws(alt((tag("\"y\""), tag("\"n\"")))),
            preceded(space0, parse_constant_symbol),
        )),
        |d| d,
    )(input)
}
