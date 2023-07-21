use crate::{
    assert_parsing_eq,
    entry::def_bool::{parse_def_bool, DefBool},
};

#[test]
fn test_parse_def_bool() {
    let input = "define_bool PCI y";
    assert_parsing_eq!(
        parse_def_bool,
        input,
        Ok((
            "",
            DefBool {
                symbol: "PCI".to_string(),
                values: vec!("y".to_string())
            }
        ))
    )
}