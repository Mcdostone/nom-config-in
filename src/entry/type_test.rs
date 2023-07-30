use crate::{
    assert_parsing_eq,
    entry::r#type::{parse_bool, parse_hex, parse_int, parse_string, parse_tristate, Type},
};

#[test]
fn test_parse_bool() {
    let input = "bool 'Kernel math emulation' CONFIG_MATH_EMULATION n";
    assert_parsing_eq!(
        parse_bool,
        input,
        Ok((
            "",
            Type {
                symbol: "CONFIG_MATH_EMULATION".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Bool,
                prompt: "Kernel math emulation".to_string(),
                value: Some("n".to_string())
            }
        ))
    )
}

#[test]
fn test_parse_bool_ref() {
    let input = "bool 'Sparc ESP Scsi Driver' CONFIG_SCSI_SUNESP $CONFIG_SCSI";
    assert_parsing_eq!(
        parse_bool,
        input,
        Ok((
            "",
            Type {
                prompt: "Sparc ESP Scsi Driver".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Bool,
                symbol: "CONFIG_SCSI_SUNESP".to_string(),
                value: Some("$CONFIG_SCSI".to_string())
            }
        ))
    )
}

#[test]
fn test_parse_bool_no_value() {
    let input = "bool 'Using SRM as bootloader' CONFIG_ALPHA_SRM";
    assert_parsing_eq!(
        parse_bool,
        input,
        Ok((
            "",
            Type {
                prompt: "Using SRM as bootloader".to_string(),
                symbol: "CONFIG_ALPHA_SRM".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Bool,
                value: None
            }
        ))
    )
}

#[test]
fn test_parse_hex() {
    let input = "hex 'Physical memory start address' CONFIG_MEMORY_START 0c000000";
    assert_parsing_eq!(
        parse_hex,
        input,
        Ok((
            "",
            Type {
                symbol: "CONFIG_MEMORY_START".to_string(),
                prompt: "Physical memory start address".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Hex,
                value: Some("0c000000".to_string())
            }
        ))
    )
}

#[test]
fn test_parse_int() {
    let input = "int ' number of ftape buffers' NR_FTAPE_BUFFERS 3";
    assert_parsing_eq!(
        parse_int,
        input,
        Ok((
            "",
            Type {
                symbol: "NR_FTAPE_BUFFERS".to_string(),
                prompt: " number of ftape buffers".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Int,
                value: Some((3, None)),
            }
        ))
    )
}

#[test]
fn test_parse_string() {
    let input = r#"string '  Prefix for cross devel tools' CROSS_COMPILE "ppc-linux-elf-""#;
    assert_parsing_eq!(
        parse_string,
        input,
        Ok((
            "",
            Type {
                prompt: "  Prefix for cross devel tools".to_string(),
                symbol: "CROSS_COMPILE".to_string(),
                r#type: crate::entry::r#type::TypeEnum::String,
                value: Some("ppc-linux-elf-".to_string())
            }
        ))
    )
}

#[test]
fn test_parse_tristate() {
    let input = "tristate 'hello' PCI y";
    assert_parsing_eq!(
        parse_tristate,
        input,
        Ok((
            "",
            Type {
                prompt: "hello".to_string(),
                symbol: "PCI".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Tristate,
                value: Some("y".to_string())
            }
        ))
    )
}
