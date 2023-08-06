use crate::{
    assert_parsing_eq,
    entry::dep_type::{parse_dep_bool, parse_dep_tristate, DepBool, DepTristate},
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
                prompt: "  ACPI support".to_string(),
                symbol: "CONFIG_ACPI".to_string(),
                dependencies: vec!("$CONFIG_PM".to_string())
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
                value: Some("y".to_string()),
                dependencies: vec!("$CONFIG_SCSI".to_string())
            }
        ))
    )
}

// 1.3.54/drivers/scsi/Config.in
#[test]
fn test_parse_dep_tristate_value_or_dep() {
    let input = "dep_tristate 'Adaptec AHA152X support' CONFIG_SCSI_AHA152X $CONFIG_SCSI";
    assert_parsing_eq!(
        parse_dep_tristate,
        input,
        Ok((
            "",
            DepTristate {
                prompt: "Adaptec AHA152X support".to_string(),
                symbol: "CONFIG_SCSI_AHA152X".to_string(),
                value: None,
                dependencies: vec!("$CONFIG_SCSI".to_string())
            }
        ))
    )
}


// 1.3.54/drivers/scsi/Config.in
#[test]
fn test_parse_dep_tristate_no_value_no_dep() {
    let input = "dep_tristate 'BusLogic SCSI support' CONFIG_SCSI_BUSLOGIC";
    assert_parsing_eq!(
        parse_dep_tristate,
        input,
        Ok((
            "",
            DepTristate {
                prompt: "BusLogic SCSI support".to_string(),
                symbol: "CONFIG_SCSI_BUSLOGIC".to_string(),
                value: None,
                dependencies: vec!()
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
                value: None,
                dependencies: vec!("$CONFIG_SCSI".to_string()),
            }
        ))
    )
}



// 2.1.79/drivers/fc4/Config.in
#[test]
fn test_parse_dep_tristate_variant_2() {
    let input = r#"dep_tristate 'SparcSTORAGE Array 100 and 200 series' CONFIG_SCSI_PLUTO "$CONFIG_SCSI""#;
    assert_parsing_eq!(
        parse_dep_tristate,
        input,
        Ok((
            "",
            DepTristate {
                prompt: "SparcSTORAGE Array 100 and 200 series".to_string(),
                symbol: "CONFIG_SCSI_PLUTO".to_string(),
                value: None,
                dependencies: vec!("\"$CONFIG_SCSI\"".to_string()),
            }
        ))
    )
}



// 2.2.19/drivers/isdn/Config.in
#[test]
fn test_parse_mbool_no_value() {
    let input = "   dep_mbool    '    CAPI2.0 filesystem support' CONFIG_ISDN_CAPI_CAPIFS_BOOL $CONFIG_ISDN_CAPI_CAPI20";
    assert_parsing_eq!(
        parse_dep_mbool,
        input,
        Ok((
            "",
            Type {
                prompt: "    CAPI2.0 filesystem support".to_string(),
                symbol: "CONFIG_ISDN_CAPI_CAPIFS_BOOL".to_string(),
                r#type: crate::entry::r#type::TypeEnum::bool,
                value: Some("CONFIG_ISDN_CAPI_CAPI20".to_string())
            }
        ))
    )
}