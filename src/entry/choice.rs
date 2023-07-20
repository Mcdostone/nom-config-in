use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::many1,
    sequence::{delimited, tuple},
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
    let (input, options) =
        delimited(ws(tag("\"")), many1(ws(parse_choice_option)), ws(tag("\"")))(input)?;
    let (input, def) = ws(parse_constant_symbol)(input)?;
    Ok((
        input,
        Choice {
            prompt: prompt.1.to_string(),
            entries: options,
            default: def.to_string(),
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
