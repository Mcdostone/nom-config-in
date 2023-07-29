use crate::{
    assert_parsing_eq,
    symbol::{parse_symbol},
};

#[test]
fn test_parse_symbol() {
    let input = "\"hello\"";
    assert_parsing_eq!(parse_symbol, input, Ok(("", "\"hello\"".to_string())))
}
