use crate::{
    assert_parsing_eq,
    entry::{
        string::{parse_string, StringConfig},
    },
    symbol::Symbol,
};

#[test]
fn test_parse_string() {
    let input = r#"string '  Prefix for cross devel tools' CROSS_COMPILE "ppc-linux-elf-""#;
    assert_parsing_eq!(
        parse_string,
        input,
        Ok((
            "",
            StringConfig {
                prompt: "Prefix for cross devel tools".to_string(),
                symbol: Symbol::Constant("CROSS_COMPILE".to_string()),
                value: Some("ppc-linux-elf-".to_string())
            }
        ))
    )
}
