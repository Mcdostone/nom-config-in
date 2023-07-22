use crate::{assert_parsing_eq, entry::define::parse_define, entry::define::Define};

#[test]
fn test_parse_define() {
    let input = " define CONFIG_KCORE ELF";
    assert_parsing_eq!(
        parse_define,
        input,
        Ok((
            "",
            Define {
                configs: vec!("CONFIG_KCORE".to_string(), "ELF".to_string())
            }
        ))
    )
}
