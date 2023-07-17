use nom::{branch::alt, combinator::map, multi::many0, sequence::delimited, IResult};
use serde::Serialize;

use crate::{
    util::{ws, ws_comment},
    KconfigInput,
};

use self::{
    comment::{parse_comment, Comment},
    r#if::{parse_if, If}, bool::{parse_bool, Bool}
};

pub mod comment;
pub mod r#if;
pub mod bool;
pub mod expression;
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
pub mod expression_test;
#[cfg(test)]
mod comment_test;
#[cfg(test)]
pub mod if_test;
#[cfg(test)]
mod bool_test;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Entry {
    Comment(Comment),
    If(If),
    Bool(Bool),
}

pub fn parse_entry(input: KconfigInput) -> IResult<KconfigInput, Entry> {
    alt((
        map(ws(parse_bool), Entry::Bool),
        map(ws(parse_if), Entry::If),
        map(ws(parse_comment), Entry::Comment),
    ))(input)
}

pub fn parse_entries(input: KconfigInput) -> IResult<KconfigInput, Vec<Entry>> {
    delimited(ws_comment, many0(parse_entry), ws_comment)(input)
}
