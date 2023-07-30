/*
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
                    value: None
                }),
                Entry::DefBool(DefBool {
                    symbol: "CONFIG_PCI".to_string(),
                    values: vec!("y".to_string())
                })
            )
        ))
    )
}
*/
