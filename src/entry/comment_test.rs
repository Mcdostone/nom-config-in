use crate::{
    assert_parsing_eq,
    entry::comment::{parse_comment, Comment},
};

#[test]
fn test_parse_comment() {
    let input = "comment 'General setup'";
    assert_parsing_eq!(
        parse_comment,
        input,
        Ok((
            "",
            Comment {
                prompt: "General setup".to_string()
            }
        ))
    )
}
