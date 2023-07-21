use crate::{
    assert_parsing_eq,
    entry::{
        bool::Bool,
        comment::Comment,
        main_menu_option::{parse_main_menu, parse_main_menu_option, MainMenu, MainMenuOption},
        Entry,
    },
    symbol::Symbol,
};

#[test]
fn test_parse_main_menu() {
    let input = "mainmenu_option next_coment";
    assert_parsing_eq!(
        parse_main_menu_option,
        input,
        Ok((
            "",
            MainMenuOption {
                value: "next_coment".to_string(),
            }
        ))
    )
}

#[test]
fn test_parse_main_menu_endmenu() {
    let input = r#"mainmenu_option next_comment
    comment 'Kernel hacking'
    
    bool 'Kernel profiling support' CONFIG_PROFILE
    endmenu"#;
    assert_parsing_eq!(
        parse_main_menu,
        input,
        Ok((
            "",
            MainMenu {
                menu_option: MainMenuOption {
                    value: "next_comment".to_string(),
                },
                entries: vec!(
                    Entry::Comment(Comment {
                        prompt: "Kernel hacking".to_string()
                    }),
                    Entry::Bool(Bool {
                        prompt: "Kernel profiling support".to_string(),
                        symbol: Symbol::Constant("CONFIG_PROFILE".to_string()),
                        default: None
                    })
                )
            }
        ))
    )
}
