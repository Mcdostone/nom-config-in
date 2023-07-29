use nom::{multi::many0, sequence::delimited, IResult};
use serde::Serialize;

use crate::{
    entry::{parse_entry, Entry},
    util::ws_comment,
};

#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct ConfigIn {
    pub file: String,
    pub entries: Vec<Entry>,
}

pub fn parse_config(input: &str) -> IResult<&str, Vec<Entry>> {
    delimited(ws_comment, many0(parse_entry), ws_comment)(input)
}
