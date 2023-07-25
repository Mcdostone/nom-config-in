use crate::{
    assert_parsing_eq,
    entry::int::{parse_int, Int},
    symbol::Symbol,
};

#[test]
fn test_parse_int() {
    let input = "int ' number of ftape buffers' NR_FTAPE_BUFFERS 3";
    assert_parsing_eq!(
        parse_int,
        input,
        Ok((
            "",
            Int {
                symbol: Symbol::Constant("NR_FTAPE_BUFFERS".to_string()),
                prompt: "number of ftape buffers".to_string(),
                default: Some(3),
                range: Default::default()
            }
        ))
    )
}
