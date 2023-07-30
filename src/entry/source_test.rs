use crate::{
    assert_parsing_eq,
    entry::source::{parse_source, Source},
};

#[test]
fn test_parse_source() {
    let input = "source /path/config.in";
    assert_parsing_eq!(
        parse_source,
        input,
        Ok((
            "",
            Source {
                file: "/path/config.in".to_string(),
                entries: vec!()
            }
        ))
    )
}
