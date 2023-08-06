use crate::{assert_parsing_eq, entry::command::parse_command};

// 2.1.28/drivers/sound/Config.in
#[test]
fn test_parse_command() {
    let input = "$MAKE -C drivers/sound config || exit 1";
    assert_parsing_eq!(
        parse_command,
        input,
        Ok(("", "$MAKE -C drivers/sound config || exit 1".to_string(),))
    )
}
