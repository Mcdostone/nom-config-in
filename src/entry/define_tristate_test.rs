use crate::{
    assert_parsing_eq,
    entry::define_tristate::{parse_define_tristate, DefineTristate},
};

#[test]
fn test_parse_dep_tristate() {
    let input = "  define_tristate CONFIG_MATHEMU y";
    assert_parsing_eq!(
        parse_define_tristate,
        input,
        Ok((
            "",
            DefineTristate {
                symbol: "CONFIG_MATHEMU".to_string(),
                value: Some("y".to_string())
            }
        ))
    )
}
