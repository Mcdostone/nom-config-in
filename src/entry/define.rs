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
pub struct Define {
    pub configs: Vec<String>,
}

pub fn dd(input: &str) -> IResult<&str, String> {
    preceded(space0, map(parse_constant_symbol, |d| d.to_string()))(input)
}

pub fn parse_define(input: &str) -> IResult<&str, Define> {
    map(tuple((ws(tag("define")), many1(dd))), |(_, value)| Define {
        configs: value,
    })(input)
}
