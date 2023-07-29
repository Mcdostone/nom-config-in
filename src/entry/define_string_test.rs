use crate::{
    assert_parsing_eq,
    entry::{define_string::parse_define_string, DefineType},
};

#[test]
fn test_parse_string() {
    let input = r#"	define_string CONFIG_COMPILE_OPTIONS "-g""#;
    assert_parsing_eq!(
        parse_define_string,
        input,
        Ok((
            "",
            DefineType {
                symbol: "CONFIG_COMPILE_OPTIONS".to_string(),
                value: Some("-g".to_string())
            }
        ))
    )
}
