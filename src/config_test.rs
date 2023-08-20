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
                    value: vec!()
                }),
                Entry::DefineBool(DefineBool {
                    symbol: "CONFIG_PCI".to_string(),
                    r#type: TypeEnum::Bool,
                    value: vec!("y".to_string())
                })
            )
        ))
    )
}

#[test]
fn test_parse_command() {
    let input = "$MAKE -C drivers/sound config || exit 1";
    assert_parsing_eq!(
        parse_config,
        input,
        Ok((
            "",
            vec!(Entry::Command(
                "$MAKE -C drivers/sound config || exit 1".to_string()
            ),)
        ))
    )
}
