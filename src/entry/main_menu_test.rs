use crate::{
    assert_parsing_eq,
    entry::main_menu::{parse_main_menu, MainMenu},
};

#[test]
fn test_parse_main_menu() {
    let input = "mainmenu_name \"BPF subsystem\"";
    assert_parsing_eq!(
        parse_main_menu,
        input,
        Ok((
            "",
            MainMenu {
                prompt: "BPF subsystem".to_string(),
            }
        ))
    )
}
