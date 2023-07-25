use crate::{
    assert_parsing_eq,
    entry::{bool::Bool, def_bool::DefBool, Entry},
    symbol::Symbol, config::parse_config,
};

#[test]
fn test_parse_file() {
    let input = "bool 'Using SRM as bootloader' CONFIG_ALPHA_SRM
    define_bool CONFIG_PCI y";
    assert_parsing_eq!(
        parse_config,
        input,
        Ok((
            "",
            vec!(
                Entry::Bool(Bool {
                    prompt: "Using SRM as bootloader".to_string(),
                    symbol: Symbol::Constant("CONFIG_ALPHA_SRM".to_string()),
                    default: None
                }),
                Entry::DefBool(DefBool {
                    symbol: "CONFIG_PCI".to_string(),
                    values: vec!("y".to_string())
                })
            )
        ))
    )
}
