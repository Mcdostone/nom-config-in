use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult};
use serde::Serialize;

use crate::util::ws;

use super::comment::parse_prompt_option;

pub fn parse_main_menu_name(input: &str) -> IResult<&str, MainMenuName> {
    map(
        pair(ws(tag("mainmenu_name")), ws(parse_prompt_option)),
        |(_, prompt)| MainMenuName {
            prompt: prompt.to_string(),
        },
    )(input)
}

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct MainMenuName {
    pub prompt: String,
}
