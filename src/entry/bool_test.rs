use crate::{
    assert_parsing_eq,
    entry::bool::{parse_bool, Bool},
    symbol::Symbol,
};

#[test]
fn test_parse_bool() {
    let input = "bool 'Kernel math emulation' CONFIG_MATH_EMULATION n";
    assert_parsing_eq!(
        parse_bool,
        input,
        Ok((
            "",
            Bool {
                symbol: Symbol::Constant("CONFIG_MATH_EMULATION".to_string()),
                prompt: "Kernel math emulation".to_string(),
                default: Some("n".to_string())
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
            Bool {
                prompt: "Sparc ESP Scsi Driver".to_string(),
                symbol: Symbol::Constant("CONFIG_SCSI_SUNESP".to_string()),
                default: Some("$CONFIG_SCSI".to_string())
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
            Bool {
                prompt: "Using SRM as bootloader".to_string(),
                symbol: Symbol::Constant("CONFIG_ALPHA_SRM".to_string()),
                default: None
            }
        ))
    )
}
