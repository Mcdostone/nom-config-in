use crate::{
    assert_parsing_eq,
    entry::choice::{parse_choice, Choice, ChoiceOption},
};

#[test]
fn test_parse_choice_optional() {
    let input = r#"choice 'Alpha system type'                   \
	"Jensen		CONFIG_ALPHA_JENSEN" Jensen"#;
    assert_parsing_eq!(
        parse_choice,
        input,
        Ok((
            "",
            Choice {
                prompt: "Alpha system type".to_string(),
                entries: vec!(ChoiceOption {
                    left: "Jensen".to_string(),
                    right: "CONFIG_ALPHA_JENSEN".to_string()
                }),
                default: "Jensen".to_string()
            }
        ))
    )
}
