use crate::{
    assert_parsing_eq,
    entry::hwaddr::{parse_hwaddr, Hwaddr},
    symbol::Symbol,
};

#[test]
fn test_parse_hwaddr() {
    let input = "hwaddr 'Ethernet address' ELTEST_ETHADR 00408ccd0000";
    assert_parsing_eq!(
        parse_hwaddr,
        input,
        Ok((
            "",
            Hwaddr {
                prompt: "Physical memory start address".to_string(),
                symbol: Symbol::Constant("CONFIG_MEMORY_START".to_string()),
                value: "0c000000".to_string()
            }
        ))
    )
}
