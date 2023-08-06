use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending},
    combinator::{map, eof},
    sequence::{pair, terminated},
    IResult,
};

use crate::{util::wsi, ConfigInInput};

pub fn parse_command(input: ConfigInInput) -> IResult<ConfigInInput, String> {
    let end = alt((line_ending, eof));
    map(
        pair(
            alt((tag::<&str, ConfigInInput, _>("echo"), tag("$MAKE"))),
            wsi(terminated(not_line_ending, end)),
        ),
        |(cmd, params)| format!("{} {}", cmd.fragment(), params.fragment()),
    )(input)
}
