use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult};
use serde::Serialize;

use crate::util::ws;

use super::comment::parse_prompt_option;

pub fn parse_main_menu_option(input: &str) -> IResult<&str, MainMenuOption> {
    map(
        pair(ws(tag("mainmenu_option")), ws(parse_prompt_option)),
        |(_, prompt)| MainMenuOption {
            value: prompt.to_string(),
        },
    )(input)
}

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct MainMenuOption {
    pub value: String,
}
