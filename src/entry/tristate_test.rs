use crate::{
    assert_parsing_eq,
    entry::{r#type::Type, tristate::parse_tristate},
};

#[test]
fn test_parse_tristate() {
    let input = "tristate 'hello' PCI y";
    assert_parsing_eq!(
        parse_tristate,
        input,
        Ok((
            "",
            Type {
                prompt: "hello".to_string(),
                symbol: "PCI".to_string(),
                value: Some("y".to_string())
            }
        ))
    )
}
