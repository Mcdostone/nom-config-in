use crate::{
    assert_parsing_eq,
    entry::unset::{parse_unset, Unset},
};

#[test]
fn test_parse_unset() {
    let input = "unset CONFIG_PCI CONFIG_ALPHA_LCA CONFIG_ALPHA_APECS";
    assert_parsing_eq!(
        parse_unset,
        input,
        Ok((
            "",
            Unset {
                configs: vec!(
                    "CONFIG_PCI".to_string(),
                    "CONFIG_ALPHA_LCA".to_string(),
                    "CONFIG_ALPHA_APECS".to_string()
                )
            }
        ))
    )
}
