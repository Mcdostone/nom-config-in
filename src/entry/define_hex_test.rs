use crate::{
    assert_parsing_eq,
    entry::define_hex::{parse_define_hex, DefineHex},
    symbol::Symbol,
};

#[test]
fn test_parse_hex() {
    let input = "  define_hex CONFIG_MEMORY_START 0c000000
    ";
    assert_parsing_eq!(
        parse_define_hex,
        input,
        Ok((
            "",
            DefineHex {
                symbol: Symbol::Constant("CONFIG_MEMORY_START".to_string()),
                value: Some("0c000000".to_string())
            }
        ))
    )
}
