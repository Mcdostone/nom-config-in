use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0},
    combinator::{eof, map, opt},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws, ConfigInInput};

use super::{comment::parse_prompt_option, expression::parse_number};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Type<T> {
    pub prompt: String,
    pub r#type: TypeEnum,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum TypeEnum {
    String,
    Hex,
    Int,
    Tristate,
    Bool,
}

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
            r#type: super::r#type::TypeEnum::Bool,
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

pub fn parse_hex(input: ConfigInInput) -> IResult<ConfigInInput, Type<String>> {
    map(
        tuple((
            ws(tag("hex")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            opt(map(parse_hex_value, |d| d.fragment().to_string())),
        )),
        |(_, prompt, sym, value)| Type {
            prompt: prompt.to_string(),
            r#type: TypeEnum::Hex,
            symbol: sym.to_string(),
            value,
        },
    )(input)
}

pub fn parse_hex_value(input: ConfigInInput) -> IResult<ConfigInInput, ConfigInInput> {
    preceded(space0, parse_constant_symbol)(input)
}

pub type Int = (i64, Option<(i64, i64)>);

pub fn parse_int(input: ConfigInInput) -> IResult<ConfigInInput, Type<Int>> {
    map(
        tuple((
            ws(tag("int")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            parse_int_value,
        )),
        |(_, prompt, sym, value)| Type {
            prompt: prompt.to_string(),
            symbol: sym.to_string(),
            r#type: TypeEnum::Int,
            value: Some(value),
        },
    )(input)
}

pub fn parse_int_value(input: ConfigInInput) -> IResult<ConfigInInput, Int> {
    tuple((
        ws(map(parse_number, |s| s)),
        opt(pair(map(parse_number, |s| s), ws(map(parse_number, |s| s)))),
    ))(input)
}

pub fn parse_tristate(input: ConfigInInput) -> IResult<ConfigInInput, Type<String>> {
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
            r#type: super::r#type::TypeEnum::Tristate,
        },
    )(input)
}

pub fn parse_tristate_value(input: ConfigInInput) -> IResult<ConfigInInput, ConfigInInput> {
    alt((parse_bool_value, tag("m")))(input)
}

pub fn parse_string(input: ConfigInInput) -> IResult<ConfigInInput, Type<String>> {
    map(
        tuple((
            ws(tag("string")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            preceded(space0, opt(parse_string_value)),
        )),
        |(_, p, e, v)| Type {
            prompt: p.to_string(),
            symbol: e.to_string(),
            value: v,
            r#type: super::r#type::TypeEnum::String,
        },
    )(input)
}

pub fn parse_string_value(input: ConfigInInput) -> IResult<ConfigInInput, String> {
    map(
        alt((
            parse_prompt_option,
            map(
                terminated(not_line_ending::<ConfigInInput, _>, alt((line_ending, eof))),
                |d| d.to_string(),
            ),
        )),
        |d| d.to_string(),
    )(input)
}
