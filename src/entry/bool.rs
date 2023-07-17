use nom::{bytes::complete::tag, combinator::{map, opt}, sequence::tuple, IResult};
use serde::Serialize;

use crate::{
    util::ws,
    KconfigInput, symbol::{Symbol, parse_constant_symbol},
};

use super::comment::parse_prompt_option;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Bool {
    pub prompt: String,
    pub symbol: Symbol,
    pub default: Option<String>
}

pub fn parse_bool(input: KconfigInput) -> IResult<KconfigInput, Bool> {
    dbg!(&input);
    map(
        tuple((
            ws(tag("bool")),
            ws(parse_prompt_option),
            ws(parse_constant_symbol),
            ws(opt(map(parse_constant_symbol, |s| s.to_string()))),
        )),
        |(_, prompt, sym, default)| Bool {
            prompt: prompt.to_string(),
            symbol: Symbol::Constant(sym.to_string()),
            default
        },
    )(input)
}
