use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws};

use super::{parse_entry, Entry};

pub fn parse_main_menu_option(input: &str) -> IResult<&str, MainMenuOption> {
    map(
        pair(ws(tag("mainmenu_option")), ws(parse_constant_symbol)),
        |(_, prompt)| MainMenuOption {
            value: prompt.to_string(),
        },
    )(input)
}

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct MainMenuOption {
    pub value: String,
}

pub fn parse_main_menu(input: &str) -> IResult<&str, MainMenu> {
    map(
        tuple((
            parse_main_menu_option,
            many0(ws(parse_entry)),
            ws(tag("endmenu")),
        )),
        |(d, e, _)| MainMenu {
            menu_option: d,
            entries: e,
        },
    )(input)
}

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct MainMenu {
    pub menu_option: MainMenuOption,
    pub entries: Vec<Entry>,
}
