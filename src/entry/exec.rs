use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending},
    combinator::{eof, map},
    sequence::terminated,
    IResult,
};
use serde::Serialize;

use crate::util::ws;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Exec {
    pub command: String,
}

pub fn parse_exec(input: &str) -> IResult<&str, Exec> {
    let (input, _) = ws(tag("exec"))(input)?;
    map(
        ws(terminated(not_line_ending, alt((line_ending, eof)))),
        |cmd: &str| Exec {
            command: cmd.to_string(),
        },
    )(input)
}
