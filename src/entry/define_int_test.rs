use crate::{
    assert_parsing_eq,
    entry::{
        define_int::{parse_define_int},
        DefineType,
    },
};

#[test]
fn test_parse_int() {
    let input = "define_int CONFIG_MAX_MEMSIZE 2048";
    assert_parsing_eq!(
        parse_define_int,
        input,
        Ok((
            "",
            DefineType {
                symbol: "CONFIG_MAX_MEMSIZE".to_string(),
                value: Some(2048)
            }
        ))
    )
}
