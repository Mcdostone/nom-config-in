use crate::{
    assert_parsing_eq,
    entry::exec::{parse_exec, Exec},
};

#[test]
fn test_parse_exec() {
    let input = r#"exec echo "CONFIG_SPARCDEVS=y" >> $CONFIG"#;
    assert_parsing_eq!(
        parse_exec,
        input,
        Ok((
            "",
            Exec {
                command: r#"echo "CONFIG_SPARCDEVS=y" >> $CONFIG"#.to_string(),
            }
        ))
    )
}
