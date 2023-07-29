use entry::Entry;

use nom::{
    combinator::{eof, map},
    multi::many0,
    sequence::delimited,
    IResult,
};
use serde::Serialize;
use util::ws;

use crate::{entry::parse_entry, util::ws_comment};

pub mod config;
pub mod entry;
pub mod symbol;
pub mod util;

#[cfg(test)]
mod lib_test;

#[cfg(test)]
pub mod config_test;
#[cfg(test)]
pub mod symbol_test;
#[cfg(test)]
pub mod util_test;

#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct Config {
    pub file: String,
    pub entries: Vec<Entry>,
}

pub fn parse_config_in(input: &str) -> IResult<&str, Config> {
    let (input, result) = map(delimited(ws_comment, many0(parse_entry), ws_comment), |d| {
        Config {
            file: "".to_string(),
            entries: d,
        }
    })(input)?;
    let (input, _) = ws(eof)(input)?;
    // TODO
    Ok((input, result))
}
