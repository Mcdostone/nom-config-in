use crate::{
    assert_parsing_eq,
    entry::define_string::{parse_define_string, DefineString},
    symbol::Symbol,
};

#[test]
fn test_parse_string() {
    let input = r#"	define_string CONFIG_COMPILE_OPTIONS "-g""#;
    assert_parsing_eq!(
        parse_define_string,
        input,
        Ok((
            "",
            DefineString {
                symbol: Symbol::Constant("CONFIG_COMPILE_OPTIONS".to_string()),
                value: "-g".to_string()
            }
        ))
    )
}
