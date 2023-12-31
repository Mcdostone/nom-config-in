use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{eof, map, opt, value},
    multi::many0,
    sequence::tuple,
    IResult,
};
use serde::Serialize;

use crate::{
    symbol::{parse_constant_symbol, parse_symbol},
    util::{ws, wsi},
    ConfigInInput,
};

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
pub type DepMbool = DefType;

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

pub fn parse_dep_mbool(input: ConfigInInput) -> IResult<ConfigInInput, DepMbool> {
    map(
        tuple((
            ws(tag("dep_mbool")),
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
    many0(map(wsi(parse_symbol), |d| d.to_string()))(input)
}

pub type DepTristate = DefTypeWithValue<Option<String>>;
pub fn parse_dep_tristate(input: ConfigInInput) -> IResult<ConfigInInput, DepTristate> {
    map(
        tuple((
            ws(tag("dep_tristate")),
            wsi(parse_prompt_option),
            wsi(parse_constant_symbol),
            opt(wsi(parse_tristate_value)),
            alt((
                wsi(parse_dependencies),
                value(vec![], wsi(line_ending)),
                value(vec![], wsi(eof)),
            )),
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
