use crate::{
    assert_parsing_eq,
    entry::{
        r#type::Type,
        string::{parse_string},
    },
};

#[test]
fn test_parse_string() {
    let input = r#"string '  Prefix for cross devel tools' CROSS_COMPILE "ppc-linux-elf-""#;
    assert_parsing_eq!(
        parse_string,
        input,
        Ok((
            "",
            Type {
                prompt: "Prefix for cross devel tools".to_string(),
                symbol: "CROSS_COMPILE".to_string(),
                value: Some("ppc-linux-elf-".to_string())
            }
        ))
    )
}
