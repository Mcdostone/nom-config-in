use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::char,
    character::complete::{alphanumeric1, line_ending, multispace1, not_line_ending, one_of},
    combinator::{eof, map, opt, recognize, verify},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::expression::{parse_if_expression_attribute, Expression};

pub fn parse_prompt(input: KconfigInput) -> IResult<KconfigInput, Prompt> {
    map(
        tuple((
            ws(tag("prompt")),
            parse_prompt_option,
            opt(parse_if_expression_attribute),
        )),
        |(_, p, i)| Prompt {
            prompt: p.to_string(),
            r#if: i,
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

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Prompt {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}
