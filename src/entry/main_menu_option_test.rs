use crate::{
    assert_parsing_eq,
    entry::main_menu_option::{parse_main_menu_option, MainMenuOption},
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
