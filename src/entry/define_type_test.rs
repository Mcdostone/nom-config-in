use crate::{
    assert_parsing_eq,
    entry::{
        define_type::{
            parse_define_bool, parse_define_hex, parse_define_int, parse_define_string,
            parse_define_tristate, DefineTristate,
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
                r#type: crate::entry::r#type::TypeEnum::Int,
                value: 2048
            }
        ))
    )
}

// 2.1.132/arch/i386/config.in
#[test]
fn test_parse_define_bool() {
    let input = "define_bool CONFIG_PCI_BIOS \"y\"";
    assert_parsing_eq!(
        parse_define_bool,
        input,
        Ok((
            "",
            DefineType {
                symbol: "CONFIG_PCI_BIOS".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Bool,
                value: vec!("\"y\"".to_string())
            }
        ))
    )
}

// 2.2.20/arch/ppc/config.in
#[test]
fn test_parse_define_bool_list_dependencies() {
    let input = "define_bool CONFIG_INPUT_KEYBDEV $CONFIG_INPUT_ADBHID $CONFIG_VT";
    assert_parsing_eq!(
        parse_define_bool,
        input,
        Ok((
            "",
            DefineType {
                symbol: "CONFIG_INPUT_KEYBDEV".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Bool,
                value: vec!("$CONFIG_INPUT_ADBHID".to_string(), "$CONFIG_VT".to_string())
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
                r#type: crate::entry::r#type::TypeEnum::String,
                value: "-g".to_string()
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
                r#type: crate::entry::r#type::TypeEnum::Tristate,
                value: "y".to_string()
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
                r#type: crate::entry::r#type::TypeEnum::Hex,
                value: "0c000000".to_string()
            }
        ))
    )
}
