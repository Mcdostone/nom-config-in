use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{map, opt},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws, ConfigInInput};

use super::comment::parse_prompt_option;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DefType {
    pub prompt: String,
    pub symbol: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DefTypeWithValue<T> {
    pub prompt: String,
    pub symbol: String,
    pub value: T,
    pub dependencies: Vec<String>,
}

pub type DepBool = DefType;

pub fn parse_dep_bool(input: ConfigInInput) -> IResult<ConfigInInput, DepBool> {
    map(
        tuple((
            ws(tag("dep_bool")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            parse_dependencies,
        )),
        |(_, p, e, dependencies)| DepBool {
            prompt: p.to_string(),
            symbol: e.to_string(),
            dependencies,
        },
    )(input)
}

fn parse_dependencies(input: ConfigInInput) -> IResult<ConfigInInput, Vec<String>> {
    many1(preceded(
        space1,
        map(parse_constant_symbol, |d| d.to_string()),
    ))(input)
}

pub type DepTristate = DefTypeWithValue<Option<String>>;
pub fn parse_dep_tristate(input: ConfigInInput) -> IResult<ConfigInInput, DepTristate> {
    map(
        tuple((
            ws(tag("dep_tristate")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            opt(ws(parse_tristate_value)),
            parse_dependencies,
        )),
        |(_, p, e, i, dependencies)| DepTristate {
            prompt: p.to_string(),
            symbol: e.to_string(),
            value: i,
            dependencies,
        },
    )(input)
}

pub fn parse_tristate_value(input: ConfigInInput) -> IResult<ConfigInInput, String> {
    ws(map(
        alt((tag("y"), tag("n"), tag("m"))),
        |d: ConfigInInput| d.fragment().to_string(),
    ))(input)
}
