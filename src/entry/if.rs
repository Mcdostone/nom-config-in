use nom::{bytes::complete::tag, combinator::{map, opt}, multi::many0, sequence::{tuple, preceded, delimited}, IResult};
use serde::Serialize;

use crate::{
    util::ws,
    KconfigInput,
};

use super::{parse_entry, Entry, expression::{Expression, parse_expression}};

pub fn parse_if(input: KconfigInput) -> IResult<KconfigInput, If> {
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
            else_block: e 
            },
    )(input)
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct If {
    pub condition: Expression,
    pub if_block: Vec<Entry>,
    pub else_block: Option<Vec<Entry>>,
}


pub fn parse_if_condition(input: KconfigInput) -> IResult<KconfigInput, Expression> {
    map(
        tuple((
            ws(tag("if")), 
            ws(delimited(tag("["), ws(parse_expression), ws(tag("]")))),
            ws(tag(";")),
            ws(tag("then"))
        )), |(_, e, _, _),| e)(input)
}

pub fn parse_else(input: KconfigInput) -> IResult<KconfigInput, Vec<Entry>> {
    preceded(ws(tag("else")), many0(parse_entry))(input)
}
