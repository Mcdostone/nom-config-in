use crate::{
    assert_parsing_eq,
    config_in::parse_config,
    entry::{
        define_type::DefineBool,
        r#type::{Type, TypeEnum},
        Entry,
    },
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
                Entry::Bool(Type {
                    prompt: "Using SRM as bootloader".to_string(),
                    symbol: "CONFIG_ALPHA_SRM".to_string(),
                    r#type: TypeEnum::Bool,
                    value: None
                }),
                Entry::DefineBool(DefineBool {
                    symbol: "CONFIG_PCI".to_string(),
                    r#type: TypeEnum::Bool,
                    value: "y".to_string()
                })
            )
        ))
    )
}
