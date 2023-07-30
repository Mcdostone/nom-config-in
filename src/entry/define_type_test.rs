use crate::{
    assert_parsing_eq,
    entry::{
        define_type::{
            parse_define_hex, parse_define_int, parse_define_string, parse_define_tristate,
            DefineTristate,
        },
        DefineType,
    },
};

#[test]
fn test_parse_define_int() {
    let input = "define_int CONFIG_MAX_MEMSIZE 2048";
    assert_parsing_eq!(
        parse_define_int,
        input,
        Ok((
            "",
            DefineType {
                symbol: "CONFIG_MAX_MEMSIZE".to_string(),
                value: Some(2048)
            }
        ))
    )
}

#[test]
fn test_parse_define_string() {
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

#[test]
fn test_parse_define_tristate() {
    let input = "  define_tristate CONFIG_MATHEMU y";
    assert_parsing_eq!(
        parse_define_tristate,
        input,
        Ok((
            "",
            DefineTristate {
                symbol: "CONFIG_MATHEMU".to_string(),
                value: Some("y".to_string())
            }
        ))
    )
}

#[test]
fn test_parse_define_hex() {
    let input = "  define_hex CONFIG_MEMORY_START 0c000000";
    assert_parsing_eq!(
        parse_define_hex,
        input,
        Ok((
            "",
            DefineType {
                symbol: "CONFIG_MEMORY_START".to_string(),
                value: Some("0c000000".to_string())
            }
        ))
    )
}