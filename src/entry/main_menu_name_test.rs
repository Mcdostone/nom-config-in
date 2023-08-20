use crate::{
    assert_parsing_eq,
    entry::main_menu_name::{parse_main_menu_name, MainMenuName},
};

#[test]
fn test_parse_main_menu() {
    let input = "mainmenu_name \"BPF subsystem\"";
    assert_parsing_eq!(
        parse_main_menu_name,
        input,
        Ok((
            "",
            MainMenuName {
                prompt: "BPF subsystem".to_string(),
            }
        ))
    )
}
