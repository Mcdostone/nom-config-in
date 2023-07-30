use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::tuple, IResult};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws, ConfigInInput};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EnvVariable {
    pub name: String,
    pub value: String,
}

pub fn parse_env_variable(input: ConfigInInput) -> IResult<ConfigInInput, EnvVariable> {
    map(
        tuple((
            ws(parse_constant_symbol),
            ws(tag("=")),
            ws(parse_constant_symbol),
        )),
        |(name, _, value)| EnvVariable {
            name: name.to_string(),
            value: value.to_string(),
        },
    )(input)
}

pub fn parse_bool_value(input: ConfigInInput) -> IResult<ConfigInInput, ConfigInInput> {
    ws(alt((tag("y"), tag("n"))))(input)
}
