use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult};
use serde::Serialize;

use crate::{util::ws, ConfigInInput};

use super::comment::parse_prompt_option;

pub fn parse_main_menu_name(input: ConfigInInput) -> IResult<ConfigInInput, MainMenuName> {
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
