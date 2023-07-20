use nom::{branch::alt, combinator::map, multi::many0, sequence::delimited, IResult};
use serde::Serialize;

use crate::util::{ws, ws_comment};

use self::{
    bool::{parse_bool, Bool},
    comment::{parse_comment, Comment},
    int::{parse_int, Int},
    r#if::{parse_if, If},
};

pub mod bool;
pub mod comment;
pub mod expression;
pub mod r#if;
pub mod int;
//pub mod config;
//pub mod function;
//pub mod r#if;
//pub mod main_menu;
//pub mod menu;
//pub mod menuconfig;
//pub mod source;
//pub mod expression;
//pub mod variable;

#[cfg(test)]
mod bool_test;
#[cfg(test)]
mod comment_test;
#[cfg(test)]
pub mod expression_test;
#[cfg(test)]
pub mod if_test;
#[cfg(test)]
pub mod int_test;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Entry {
    Comment(Comment),
    If(If),
    Bool(Bool),
    Int(Int),
}

pub fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((
        map(ws(parse_bool), Entry::Bool),
        map(ws(parse_int), Entry::Int),
        map(ws(parse_if), Entry::If),
        map(ws(parse_comment), Entry::Comment),
    ))(input)
}

pub fn parse_entries(input: &str) -> IResult<&str, Vec<Entry>> {
    delimited(ws_comment, many0(parse_entry), ws_comment)(input)
}
