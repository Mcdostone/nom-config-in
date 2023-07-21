use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{map, recognize},
    multi::many1,
    sequence::preceded,
    IResult,
};

use serde::Serialize;

use crate::util::ws;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Source {
    pub file: String,
}

pub fn parse_source(input: &str) -> IResult<&str, Source> {
    map(preceded(ws(tag("source")), ws(parse_path)), |s: &str| {
        Source {
            file: s.to_string(),
        }
    })(input)
}

pub fn parse_path(input: &str) -> IResult<&str, &str> {
    map(
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("_/."))))))),
        |c: &str| c.trim(),
    )(input)
}
