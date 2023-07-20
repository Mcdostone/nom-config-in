use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::util::ws;

use super::{
    expression::{parse_expression, Expression},
    parse_entry, Entry,
};

pub fn parse_if(input: &str) -> IResult<&str, If> {
    map(
        tuple((
            ws(parse_if_condition),
            many0(parse_entry),
            opt(parse_else),
            ws(tag("fi")),
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
    pub else_block: Option<Vec<Entry>>,
}

pub fn parse_if_condition(input: &str) -> IResult<&str, Expression> {
    map(
        tuple((
            ws(tag("if")),
            ws(delimited(tag("["), ws(parse_expression), ws(tag("]")))),
            ws(tag(";")),
            ws(tag("then")),
        )),
        |(_, e, _, _)| e,
    )(input)
}

pub fn parse_else(input: &str) -> IResult<&str, Vec<Entry>> {
    preceded(ws(tag("else")), many0(parse_entry))(input)
}
