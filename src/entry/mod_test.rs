use crate::{
    assert_parsing_eq,
    entry::{
        define_type::DefineBool,
        parse_entries,
        r#type::{Type, TypeEnum},
        Entry,
    },
};

#[test]
fn test_parse_entries() {
    let input = "#a comment
    bool 'Using SRM as bootloader' CONFIG_ALPHA_SRM
    define_bool CONFIG_PCI y";
    assert_parsing_eq!(
        parse_entries,
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
