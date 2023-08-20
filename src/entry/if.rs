use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{util::ws, ConfigInInput};

use super::{
    expression::{parse_expression, Expression},
    parse_entry, Entry,
};

pub fn parse_if(input: ConfigInInput) -> IResult<ConfigInInput, If> {
    map(
        tuple((
            ws(parse_if_condition),
            many0(parse_entry),
            opt(parse_else),
            ws(opt(tag("fi"))),
        )),
        |(condition, entries, e, _)| If {
            condition,
            if_block: entries,
            else_block: e,
        },
    )(input)
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct If {
    pub condition: Expression,
    pub if_block: Vec<Entry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub else_block: Option<Vec<Entry>>,
}

pub fn parse_if_condition(input: ConfigInInput) -> IResult<ConfigInInput, Expression> {
    map(
        tuple((
            ws(tag("if")),
            ws(ws(parse_expression)),
            opt(ws(tag(";"))),
            opt(ws(tag("then"))),
        )),
        |(_, e, _, _)| e,
    )(input)
}

pub fn parse_else(input: ConfigInInput) -> IResult<ConfigInInput, Vec<Entry>> {
    preceded(ws(tag("else")), many0(parse_entry))(input)
}
