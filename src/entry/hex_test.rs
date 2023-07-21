use crate::{
    assert_parsing_eq,
    entry::{bool::{parse_bool, Bool}, hex::{parse_hex, Hex}},
    symbol::Symbol,
};

#[test]
fn test_parse_hex() {
    let input = "hex 'Physical memory start address' CONFIG_MEMORY_START 0c000000";
    assert_parsing_eq!(
        parse_hex,
        input,
        Ok((
            "",
            Hex {
                symbol: Symbol::Constant("CONFIG_MEMORY_START".to_string()),
                prompt: "Physical memory start address".to_string(),
                value: Some("0c000000".to_string())
            }
        ))
    )
}
