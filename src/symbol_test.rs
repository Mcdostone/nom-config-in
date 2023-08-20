use crate::{
    assert_parsing_eq,
    symbol::{parse_constant_symbol_or_variable, parse_symbol},
};

#[test]
fn test_parse_symbol() {
    let input = "\"hello\"";
    assert_parsing_eq!(parse_symbol, input, Ok(("", "\"hello\"".to_string())))
}

#[test]
fn test_parse_constant_symbol_or_variable() {
    let input = "\"hello\"";
    assert_parsing_eq!(
        parse_constant_symbol_or_variable,
        input,
        Ok(("", "\"hello\"".to_string()))
    );

    let input = "\"$USER\"";
    assert_parsing_eq!(
        parse_constant_symbol_or_variable,
        input,
        Ok(("", "\"$USER\"".to_string()))
    );
}
