use crate::{
    assert_parsing_eq,
    entry::echo::{parse_echo, Echo},
};

#[test]
fn test_parse_exec() {
    let input = "echo touch .makesound";
    assert_parsing_eq!(
        parse_echo,
        input,
        Ok((
            "",
            Echo {
                params: "touch .makesound".to_string(),
            }
        ))
    )
}
