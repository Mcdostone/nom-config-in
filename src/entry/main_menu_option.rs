use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::many0,
    sequence::{pair, preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{
    util::{ws, wsi},
    ConfigInInput,
};

use super::{comment::parse_comment, parse_entry, Entry};

pub fn parse_main_menu_option(input: ConfigInInput) -> IResult<ConfigInInput, MainMenuOption> {
    map(
        tuple((
            preceded(
                pair(ws(tag("mainmenu_option")), wsi(tag("next_comment"))),
                ws(parse_comment),
            ),
            many0(ws(parse_entry)),
            ws(tag("endmenu")),
        )),
        |(d, e, _)| MainMenuOption {
            comment: d,
            entries: e,
        },
    )(input)
}

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct MainMenuOption {
    pub comment: String,
    pub entries: Vec<Entry>,
}
