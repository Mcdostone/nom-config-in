use crate::{
    assert_parsing_eq,
    entry::dep_bool::{parse_dep_bool, DepBool},
};

#[test]
fn test_parse_dep_bool() {
    let input = "dep_bool '  ACPI support' CONFIG_ACPI $CONFIG_PM";
    assert_parsing_eq!(
        parse_dep_bool,
        input,
        Ok((
            "",
            DepBool {
                prompt: "ACPI support".to_string(),
                symbol: "CONFIG_ACPI".to_string(),
                depends_on: vec!("$CONFIG_PM".to_string())
            }
        ))
    )
}
