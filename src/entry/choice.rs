use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::many1,
    sequence::{delimited, pair, tuple},
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws};

use super::comment::parse_prompt_option;

fn parse_choice_option(input: &str) -> IResult<&str, ChoiceOption> {
    map(
        tuple((ws(parse_constant_symbol), ws(parse_constant_symbol))),
        |(l, r)| ChoiceOption {
            left: l.to_string(),
            right: r.to_string(),
        },
    )(input)
}

pub fn parse_choice(input: &str) -> IResult<&str, Choice> {
    let (input, prompt) = tuple((ws(tag("choice")), ws(parse_prompt_option)))(input)?;
    let (input, ok): (&str, (Vec<ChoiceOption>, &str)) = alt((
        pair(
            delimited(ws(tag("\"")), many1(ws(parse_choice_option)), ws(tag("\""))),
            ws(parse_constant_symbol),
        ),
        delimited(
            ws(tag("\"")),
            pair(many1(ws(parse_choice_option)), ws(parse_constant_symbol)),
            ws(tag("\"")),
        ),
    ))(input)?;
    Ok((
        input,
        Choice {
            prompt: prompt.1.to_string(),
            entries: ok.0,
            default: ok.1.to_string(),
        },
    ))
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct Choice {
    pub prompt: String,
    pub entries: Vec<ChoiceOption>,
    pub default: String,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct ChoiceOption {
    pub left: String,
    pub right: String,
}
