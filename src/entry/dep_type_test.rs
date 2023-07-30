/*/
#[test]
fn test_parse_dep_bool() {
    let input = "dep_bool '  ACPI support' CONFIG_ACPI $CONFIG_PM";
    assert_parsing_eq!(
        parse_dep_bool,
        input,
        Ok((
            "",
            DepBool {
                prompt: "  ACPI support".to_string(),
                symbol: "CONFIG_ACPI".to_string(),
                depends_on: vec!("$CONFIG_PM".to_string())
            }
        ))
    )
}



#[test]
fn test_parse_dep_tristate() {
    let input = "dep_tristate 'SCSI disk support' CONFIG_BLK_DEV_SD y $CONFIG_SCSI";
    assert_parsing_eq!(
        parse_dep_tristate,
        input,
        Ok((
            "",
            DepTristate {
                prompt: "SCSI disk support".to_string(),
                symbol: "CONFIG_BLK_DEV_SD".to_string(),
                value: "y".to_string(),
                depends_on: Some("$CONFIG_SCSI".to_string())
            }
        ))
    )
}

#[test]
fn test_parse_dep_tristate_variant() {
    let input = "dep_tristate 'SCSI disk support' CONFIG_BLK_DEV_SD $CONFIG_SCSI";
    assert_parsing_eq!(
        parse_dep_tristate,
        input,
        Ok((
            "",
            DepTristate {
                prompt: "SCSI disk support".to_string(),
                symbol: "CONFIG_BLK_DEV_SD".to_string(),
                value: "$CONFIG_SCSI".to_string(),
                depends_on: None,
            }
        ))
    )
}
*/
