use nom::{bytes::complete::{tag, take_until}, combinator::map, sequence::{tuple, delimited}, IResult, branch::alt};
use serde::Serialize;
use nom::character::complete::char;

use crate::{
    util::ws,
    KconfigInput,
};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Comment {
    pub prompt: String
}

pub fn parse_comment(input: KconfigInput) -> IResult<KconfigInput, Comment> {
    map(
        tuple((
            ws(tag("comment")),
            ws(parse_prompt_option),
        )),
        |(_, prompt)| Comment {
            prompt: prompt.to_string()
        },
    )(input)
}


pub fn parse_prompt_option(input: KconfigInput) -> IResult<KconfigInput, &str> {
    map(
        alt((
            delimited(ws(char('\'')), take_until("'"), char('\'')),
        )),
        |d: KconfigInput| d.fragment().to_owned().trim(),
    )(input)
}
