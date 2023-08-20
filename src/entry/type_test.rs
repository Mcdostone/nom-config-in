use crate::{
    assert_parsing_eq,
    entry::r#type::{
        parse_bool, parse_hex, parse_int, parse_string, parse_tristate, IntValue, Type,
    },
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
                value: vec!("n".to_string())
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
                value: vec!("$CONFIG_SCSI".to_string())
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
                value: vec!()
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
                value: vec!("0c000000".to_string())
            }
        ))
    )
}

// 2.2.0/drivers/sound/lowlevel/Config.in
#[test]
fn test_parse_hex_variant() {
    let input =
        "hex 'I/O base for Audio Excel DSP 16 220, 240' CONFIG_AEDSP16_BASE $CONFIG_SB_BASE 220";
    assert_parsing_eq!(
        parse_hex,
        input,
        Ok((
            "",
            Type {
                symbol: "CONFIG_AEDSP16_BASE".to_string(),
                prompt: "I/O base for Audio Excel DSP 16 220, 240".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Hex,
                value: vec!("$CONFIG_SB_BASE".to_string(), "220".to_string())
            }
        ))
    )
}

// 2.2.0/drivers/sound/lowlevel/Config.in
#[test]
fn test_parse_hex_variant_1() {
    let input = "
    hex 'SC-6600 CDROM Interface I/O Address' CONFIG_SC6600_CDROMBASE 0";
    assert_parsing_eq!(
        parse_hex,
        input,
        Ok((
            "",
            Type {
                symbol: "CONFIG_SC6600_CDROMBASE".to_string(),
                prompt: "SC-6600 CDROM Interface I/O Address".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Hex,
                value: vec!("0".to_string())
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
                value: vec!(IntValue::Number(3))
            }
        ))
    )
}

// 2.1.28/drivers/sound/Config.in
#[test]
fn test_parse_int_env_variable() {
    let input = "int 'Audio Excel DSP 16 IRQ 5, 7, 9, 10, 11' \
    AEDSP16_SBC_IRQ $SBC_IRQ";
    assert_parsing_eq!(
        parse_int,
        input,
        Ok((
            "",
            Type {
                symbol: "AEDSP16_SBC_IRQ".to_string(),
                prompt: "Audio Excel DSP 16 IRQ 5, 7, 9, 10, 11".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Int,
                value: vec!(IntValue::Variable("$SBC_IRQ".to_string())),
            }
        ))
    )
}

// 2.1.33/drivers/scsi/Config.in
#[test]
fn test_parse_int_variant() {
    let input = "  int  '  Pedantic EPP-checking'   CONFIG_SCSI_PPA_HAVE_PEDANTIC 2 0 3";
    assert_parsing_eq!(
        parse_int,
        input,
        Ok((
            "",
            Type {
                symbol: "CONFIG_SCSI_PPA_HAVE_PEDANTIC".to_string(),
                prompt: "  Pedantic EPP-checking".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Int,
                value: vec!(
                    IntValue::Number(2),
                    IntValue::Number(0),
                    IntValue::Number(3)
                ),
            }
        ))
    )
}

// 2.4.22/arch/cris/drivers/Config.in
#[test]
fn test_parse_int_variant_2() {
    let input =
        "int '  Extern clock frequency (baudrate=clk/8) (Hz)' CONFIG_ETRAX_EXTERN_PB6CLK_FREQ";
    assert_parsing_eq!(
        parse_int,
        input,
        Ok((
            "",
            Type {
                symbol: "CONFIG_ETRAX_EXTERN_PB6CLK_FREQ".to_string(),
                prompt: "  Extern clock frequency (baudrate=clk/8) (Hz)".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Int,
                value: vec!(),
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
                value: "ppc-linux-elf-".to_string()
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

// 2.1.132/arch/alpha/Config.in
#[test]
fn test_parse_tristate_no_value() {
    let input = "tristate 'Kernel support for a.out (ECOFF) binaries' CONFIG_BINFMT_AOUT";
    assert_parsing_eq!(
        parse_tristate,
        input,
        Ok((
            "",
            Type {
                prompt: "Kernel support for a.out (ECOFF) binaries".to_string(),
                symbol: "CONFIG_BINFMT_AOUT".to_string(),
                r#type: crate::entry::r#type::TypeEnum::Tristate,
                value: None
            }
        ))
    )
}
