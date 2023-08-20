use crate::{
    assert_parsing_eq,
    entry::hwaddr::{parse_hwaddr, Hwaddr},
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
                prompt: "Ethernet address".to_string(),
                symbol: "ELTEST_ETHADR".to_string(),
                value: "00408ccd0000".to_string()
            }
        ))
    )
}
