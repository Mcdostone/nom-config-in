use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult};
use serde::Serialize;

use crate::{symbol::parse_constant_symbol, util::ws};

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
