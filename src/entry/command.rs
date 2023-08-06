use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending},
    combinator::{eof, map, not},
    sequence::{terminated, pair},
    IResult,
};
use serde::Serialize;

use crate::{util::{ws, wsi}, ConfigInInput, config_in::ConfigIn};


pub fn parse_command(input: ConfigInInput) -> IResult<ConfigInInput, String> {
    map(pair(
        alt((
            tag::<&str, ConfigInInput, _>("echo"),
            tag("$MAKE"),
        )),
        wsi(terminated(not_line_ending, line_ending))
    ), |(cmd, params)| format!("{} {}", cmd.fragment(), params.fragment()))(input)
}
