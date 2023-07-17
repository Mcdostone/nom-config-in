use crate::{
    assert_parsing_eq,
    entry::{bool::{Bool, parse_bool}},
    symbol::Symbol,
};


#[test]
fn test_parse_bool() {
    let input = "bool 'Kernel math emulation' CONFIG_MATH_EMULATION n";
    assert_parsing_eq!(
        parse_bool,
        input,
        Ok((
            "",
            Bool {
                symbol: Symbol::Constant("CONFIG_MATH_EMULATION".to_string()),
                prompt: "Kernel math emulation".to_string(),
                default: Some("n".to_string())
            }
        ))
    )
}
