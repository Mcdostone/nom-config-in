use nom::{bytes::complete::tag, combinator::map, sequence::pair, IResult};
use serde::Serialize;

use crate::util::ws;

use super::comment::parse_prompt_option;

pub fn parse_main_menu(input: &str) -> IResult<&str, MainMenu> {
    map(
        pair(ws(tag("mainmenu_name")), ws(parse_prompt_option)),
        |(_, prompt)| MainMenu {
            prompt: prompt.to_string(),
        },
    )(input)
}

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct MainMenu {
    pub prompt: String,
}
