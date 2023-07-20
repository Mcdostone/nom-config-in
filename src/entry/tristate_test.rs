use crate::{
    assert_parsing_eq,
    entry::tristate::{parse_tristate, Tristate},
};

#[test]
fn test_parse_tristate() {
    let input = "tristate 'hello' PCI y";
    assert_parsing_eq!(
        parse_tristate,
        input,
        Ok((
            "",
            Tristate {
                prompt: "hello".to_string(),
                symbol: "PCI".to_string(),
                value: "y".to_string()
            }
        ))
    )
}
