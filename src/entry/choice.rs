use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{map, recognize},
    multi::many1,
    sequence::{delimited, pair, tuple},
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws, ConfigInInput};

use super::comment::parse_prompt_option;

fn parse_choice_option_label(input: ConfigInInput) -> IResult<ConfigInInput, String> {
    map(
        recognize(many1(alt((alphanumeric1, recognize(one_of(",._()-/$+")))))),
        |c: ConfigInInput| c.to_string(),
    )(input)
}

fn parse_choice_option(input: ConfigInInput) -> IResult<ConfigInInput, ChoiceOption> {
    map(
        tuple((ws(parse_choice_option_label), ws(parse_constant_symbol))),
        |(l, r)| ChoiceOption {
            left: l,
            right: r.to_string(),
        },
    )(input)
}

pub fn parse_choice(input: ConfigInInput) -> IResult<ConfigInInput, Choice> {
    let (input, prompt) = tuple((ws(tag("choice")), ws(parse_prompt_option)))(input)?;
    let (input, ok) = alt((
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
