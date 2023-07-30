use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{
    symbol::parse_constant_symbol,
    util::{ws, wsi},
    ConfigInInput,
};

use super::{expression::parse_number, hex::parse_hex_value, string::parse_string_value};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DefineType<T> {
    pub symbol: String,
    pub value: Option<T>,
}

pub type DefineTristate = DefineType<String>;
pub type DefineString = DefineType<String>;
pub type DefineTBool = DefineType<String>;
pub type DefineHex = DefineType<String>;

pub fn parse_define_string(input: ConfigInInput) -> IResult<ConfigInInput, DefineType<String>> {
    map(
        tuple((
            ws(tag("define_string")),
            ws(parse_constant_symbol),
            preceded(space0, parse_string_value),
        )),
        |(_, sym, value)| DefineType {
            symbol: sym.to_string(),
            value: Some(value),
        },
    )(input)
}

pub fn parse_define_hex(input: ConfigInInput) -> IResult<ConfigInInput, DefineType<String>> {
    map(
        tuple((
            ws(tag("define_hex")),
            ws(parse_constant_symbol),
            opt(map(parse_hex_value, |d| d.to_string())),
        )),
        |(_, sym, value)| DefineType {
            symbol: sym.to_string(),
            value,
        },
    )(input)
}

pub fn parse_define_int(input: ConfigInInput) -> IResult<ConfigInInput, DefineType<i64>> {
    ws(map(
        tuple((
            tag("define_int"),
            wsi(parse_constant_symbol),
            wsi(opt(map(parse_number, |s| s))),
        )),
        |(_, sym, value)| DefineType {
            symbol: sym.to_string(),
            value,
        },
    ))(input)
}

pub fn parse_define_tristate(input: ConfigInInput) -> IResult<ConfigInInput, DefineTristate> {
    map(
        tuple((
            ws(tag("define_tristate")),
            ws(parse_constant_symbol),
            opt(preceded(
                space0,
                map(parse_constant_symbol, |d| d.to_string()),
            )),
        )),
        |(_, p, value)| DefineTristate {
            symbol: p.to_string(),
            value,
        },
    )(input)
}
