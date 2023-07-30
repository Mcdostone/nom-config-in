use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending},
    combinator::{eof, map},
    sequence::terminated,
    IResult,
};
use serde::Serialize;

use crate::{util::ws, ConfigInInput};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Echo {
    pub params: String,
}

pub fn parse_echo(input: ConfigInInput) -> IResult<ConfigInInput, Echo> {
    let (input, _) = ws(tag("echo"))(input)?;
    map(
        ws(terminated(not_line_ending, alt((line_ending, eof)))),
        |cmd: ConfigInInput| Echo {
            params: cmd.to_string(),
        },
    )(input)
}
