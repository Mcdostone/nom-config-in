use nom::{branch::alt, combinator::map, multi::many0, sequence::delimited, IResult};
use serde::Serialize;

use crate::util::{ws, ws_comment};

use self::{
    bool::{parse_bool, Bool},
    choice::{parse_choice, Choice},
    comment::{parse_comment, Comment},
    def_bool::{parse_def_bool, DefBool},
    define_int::{parse_define_int, DefineInt},
    dep_tristate::{parse_dep_tristate, DepTristate},
    echo::{parse_echo, Echo},
    env_variable::{parse_env_variable, EnvVariable},
    exec::{parse_exec, Exec},
    int::{parse_int, Int},
    main_menu_name::{parse_main_menu_name, MainMenuName},
    main_menu_option::{parse_main_menu, parse_main_menu_option, MainMenu, MainMenuOption},
    r#if::{parse_if, If},
    source::{parse_source, Source},
    string::{parse_string, StringConfig},
    tristate::{parse_tristate, Tristate},
    unset::{parse_unset, Unset},
};

pub mod bool;
pub mod choice;
pub mod comment;
pub mod def_bool;
pub mod define_int;
pub mod dep_tristate;
pub mod echo;
pub mod env_variable;
pub mod exec;
pub mod expression;
pub mod r#if;
pub mod int;
pub mod main_menu_name;
pub mod main_menu_option;
pub mod source;
pub mod string;
pub mod tristate;
pub mod unset;

#[cfg(test)]
mod bool_test;
#[cfg(test)]
mod choice_test;
#[cfg(test)]
mod comment_test;
#[cfg(test)]
mod def_bool_test;
#[cfg(test)]
mod define_int_test;
#[cfg(test)]
pub mod dep_tristate_test;
#[cfg(test)]
mod echo_test;
#[cfg(test)]
mod env_variable_test;
#[cfg(test)]
pub mod exec_test;
#[cfg(test)]
pub mod expression_test;
#[cfg(test)]
pub mod if_test;
#[cfg(test)]
pub mod int_test;
#[cfg(test)]
mod main_menu_name_test;
#[cfg(test)]
mod string_test;
#[cfg(test)]
mod unset_test;

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
    EnvVariable(EnvVariable),
    Unset(Unset),
    DepTristate(DepTristate),
    MainMenuName(MainMenuName),
    DefineInt(DefineInt),
    String(StringConfig),
    MainMenuOption(MainMenuOption),
    Choice(Choice),
    Source(Source),
    Tristate(Tristate),
    DefBool(DefBool),
    MainMenu(MainMenu),
}

pub fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((
        map(ws(parse_bool), Entry::Bool),
        map(ws(parse_int), Entry::Int),
        map(ws(parse_exec), Entry::Exec),
        map(ws(parse_def_bool), Entry::DefBool),
        map(ws(parse_define_int), Entry::DefineInt),
        map(ws(parse_echo), Entry::Echo),
        map(ws(parse_string), Entry::String),
        map(ws(parse_if), Entry::If),
        map(ws(parse_unset), Entry::Unset),
        map(ws(parse_comment), Entry::Comment),
        map(ws(parse_tristate), Entry::Tristate),
        map(ws(parse_main_menu_name), Entry::MainMenuName),
        map(ws(parse_main_menu), Entry::MainMenu),
        map(ws(parse_main_menu_option), Entry::MainMenuOption),
        map(ws(parse_choice), Entry::Choice),
        map(ws(parse_env_variable), Entry::EnvVariable),
        map(ws(parse_dep_tristate), Entry::DepTristate),
        map(ws(parse_source), Entry::Source),
    ))(input)
}

pub fn parse_entries(input: &str) -> IResult<&str, Vec<Entry>> {
    delimited(ws_comment, many0(parse_entry), ws_comment)(input)
}
