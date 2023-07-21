use crate::{
    assert_parsing_eq,
    entry::env_variable::{parse_env_variable, EnvVariable},
};

#[test]
fn test_parse_env_variable() {
    let input = "  MODULES=y";
    assert_parsing_eq!(
        parse_env_variable,
        input,
        Ok((
            "",
            EnvVariable {
                name: "MODULES".to_string(),
                value: "y".to_string()
            }
        ))
    )
}
