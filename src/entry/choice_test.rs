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

#[test]
fn test_parse_choice_variant() {
    let input = r#"choice 'Processor type' \
	"6xx/7xx		CONFIG_6xx \
	 630/Power3(64-Bit) 	CONFIG_PPC64 \
	 860/821		CONFIG_8xx" 6xx/7xx"#;
    assert_parsing_eq!(
        parse_choice,
        input,
        Ok((
            "",
            Choice {
                prompt: "Processor type".to_string(),
                entries: vec!(
                    ChoiceOption {
                    left: "6xx/7xx".to_string(),
                    right: "CONFIG_6xx".to_string()
                    },
                    ChoiceOption {
                        left: "630/Power3(64-Bit)".to_string(),
                        right: "CONFIG_PPC64".to_string()
                        },
                        ChoiceOption {
                            left: "860/821".to_string(),
                            right: "CONFIG_8xx".to_string()
                            },
                ),
                default: "6xx/7xx".to_string()
            }
        ))
    )
}


