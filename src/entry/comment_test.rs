use crate::{assert_parsing_eq, entry::comment::parse_comment};

#[test]
fn test_parse_comment() {
    let input = "comment 'General setup'";
    assert_parsing_eq!(parse_comment, input, Ok(("", "General setup".to_string())))
}
