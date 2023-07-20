use nom::{branch::alt, combinator::map, multi::many0, sequence::delimited, IResult};
use serde::Serialize;

use crate::util::{ws, ws_comment};

use self::{
    bool::{parse_bool, Bool},
    choice::{parse_choice, Choice},
    comment::{parse_comment, Comment},
    def_bool::{parse_def_bool, DefBool},
    dep_tristate::{parse_dep_tristate, DepTristate},
    echo::{parse_echo, Echo},
    exec::{parse_exec, Exec},
    int::{parse_int, Int},
    main_menu::{parse_main_menu, MainMenu},
    main_menu_option::{parse_main_menu_option, MainMenuOption},
    r#if::{parse_if, If},
    source::{parse_source, Source},
    tristate::{parse_tristate, Tristate},
};

pub mod bool;
pub mod choice;
pub mod comment;
pub mod def_bool;
pub mod dep_tristate;
pub mod echo;
pub mod exec;
pub mod expression;
pub mod r#if;
pub mod int;
pub mod main_menu;
pub mod main_menu_option;
pub mod source;
pub mod tristate;

#[cfg(test)]
mod bool_test;
#[cfg(test)]
mod choice_test;
#[cfg(test)]
mod comment_test;
#[cfg(test)]
mod def_bool_test;
#[cfg(test)]
pub mod dep_tristate_test;
#[cfg(test)]
mod echo_test;
#[cfg(test)]
pub mod exec_test;
#[cfg(test)]
pub mod expression_test;
#[cfg(test)]
pub mod if_test;
#[cfg(test)]
pub mod int_test;
#[cfg(test)]
mod main_menu_test;

#[cfg(test)]
pub mod main_menu_option_test;
#[cfg(test)]
pub mod source_test;
#[cfg(test)]
pub mod tristate_test;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Entry {
    Comment(Comment),
    If(If),
    Bool(Bool),
    Exec(Exec),
    Int(Int),
    Echo(Echo),
    DepTristate(DepTristate),
    MainMenu(MainMenu),
    MainMenuOption(MainMenuOption),
    Choice(Choice),
    Source(Source),
    Tristate(Tristate),
    DefBool(DefBool),
}

pub fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((
        map(ws(parse_bool), Entry::Bool),
        map(ws(parse_int), Entry::Int),
        map(ws(parse_exec), Entry::Exec),
        map(ws(parse_def_bool), Entry::DefBool),
        map(ws(parse_echo), Entry::Echo),
        map(ws(parse_if), Entry::If),
        map(ws(parse_comment), Entry::Comment),
        map(ws(parse_tristate), Entry::Tristate),
        map(ws(parse_main_menu), Entry::MainMenu),
        map(ws(parse_main_menu_option), Entry::MainMenuOption),
        map(ws(parse_choice), Entry::Choice),
        map(ws(parse_dep_tristate), Entry::DepTristate),
        map(ws(parse_source), Entry::Source),
    ))(input)
}

pub fn parse_entries(input: &str) -> IResult<&str, Vec<Entry>> {
    delimited(ws_comment, many0(parse_entry), ws_comment)(input)
}
