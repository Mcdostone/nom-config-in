use crate::{assert_parsing_eq, entry::define::parse_define, entry::define::Define};

// 2.4.2/arch/s390/config.in
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
