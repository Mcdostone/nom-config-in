use crate::{
    assert_parsing_eq,
    entry::dep_tristate::{parse_dep_tristate, DepTristate},
};

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
                other: "$CONFIG_SCSI".to_string()
            }
        ))
    )
}
