use crate::{
    assert_parsing_eq,
    entry::{
        main_menu_option::{parse_main_menu_option, MainMenuOption},
        r#type::{Type, TypeEnum},
        Entry,
    },
};

#[test]
fn test_parse_main_menu_endmenu() {
    let input = r#"mainmenu_option next_comment
    comment 'Kernel hacking'
    
    bool 'Kernel profiling support' CONFIG_PROFILE
    endmenu"#;
    assert_parsing_eq!(
        parse_main_menu_option,
        input,
        Ok((
            "",
            MainMenuOption {
                comment: "Kernel hacking".to_string(),
                entries: vec!(Entry::Bool(Type {
                    r#type: TypeEnum::Bool,
                    prompt: "Kernel profiling support".to_string(),
                    symbol: "CONFIG_PROFILE".to_string(),
                    value: vec!()
                }))
            }
        ))
    )
}
