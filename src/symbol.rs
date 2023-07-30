use nom::{
    branch::alt,
    bytes::complete::take_until,
    character::complete::char,
    character::complete::{alphanumeric1, one_of},
    combinator::{map, recognize},
    multi::many1,
    sequence::delimited,
    IResult,
};

use crate::ConfigInInput;

use super::util::ws;

/// There are two types of symbols: constant and non-constant symbols. Non-constant symbols are the most
/// common ones and are defined with the 'config' statement. Non-constant symbols consist entirely of al-
/// phanumeric characters or underscores. Constant symbols are only part of expressions. Constant symbols
/// are always surrounded by single or double quotes. Within the quote, any other character is allowed and
/// the quotes can be escaped using ''.
pub type Symbol = String;

pub fn parse_symbol(input: ConfigInInput) -> IResult<ConfigInInput, Symbol> {
    alt((
        map(parse_constant_symbol, |c: ConfigInInput| c.to_string()),
        map(
            delimited(ws(char('"')), take_until("\""), char('"')),
            |c: ConfigInInput| format!("\"{}\"", c),
        ),
        map(
            delimited(ws(char('\'')), take_until("'"), char('\'')),
            |c: ConfigInInput| format!("'{}'", c),
        ),
    ))(input)
}

pub fn parse_constant_symbol(input: ConfigInInput) -> IResult<ConfigInInput, ConfigInInput> {
    map(
        recognize(many1(alt((alphanumeric1, recognize(one_of("(),._-/$+")))))),
        |c: ConfigInInput| c,
    )(input)
}
