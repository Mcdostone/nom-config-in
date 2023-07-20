use nom::{bytes::complete::tag, combinator::eof, sequence::pair};

use crate::{entry::comment::parse_prompt_option, util::ws};

#[test]
fn test_ws() {
    let input = "";
    assert_eq!(ws(eof::<&str, ()>)(input), Ok(("", "")))
}

#[test]
fn test_ws_backslash() {
    let input = r#"     \
        hello  "#;
    let result = ws(tag::<&str, &str, ()>("hello"))(input).unwrap();
    assert_eq!(result, ("  ", "hello"))
}

#[test]
fn test_ws_backslash_space() {
    let input = "hello   world";
    let result = pair(ws(tag::<&str, &str, ()>("hello")), ws(tag("world")))(input).unwrap();
    assert_eq!(result, ("", ("hello", "world")))
}

#[test]
fn test_ws_backslash_other() {
    let input = r#" 'hello world'     \
        hello  "#;
    let result = parse_prompt_option(input);
    assert_eq!(result, Ok(("     \\\n        hello  ", "hello world")))
}
