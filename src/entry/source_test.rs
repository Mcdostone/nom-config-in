use std::path::PathBuf;

use crate::{
    config_in::ConfigIn,
    entry::source::{parse_source, Source},
    ConfigInFile, ConfigInInput,
};

#[test]
fn test_parse_source() {
    assert_parsing_source_eq(
        "source empty",
        Ok((
            "",
            Source {
                file: "empty".to_string(),
                entries: vec![],
            },
        )),
    )
}

#[test]
fn test_parse_source_fail_file_not_exist() {
    let res = parse_source(ConfigInInput::new_extra(
        "source a/random/file",
        ConfigInFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            ..Default::default()
        },
    ));
    assert!(res.is_err())
}

#[test]
fn test_parse_source_fail_to_parse() {
    let res = parse_source(ConfigInInput::new_extra(
        "source Cargo.toml",
        ConfigInFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            ..Default::default()
        },
    ));
    assert!(res.is_err())
}

fn assert_parsing_source_eq(
    input: &str,
    expected: Result<(&str, ConfigIn), nom::Err<nom::error::Error<ConfigInInput>>>,
) {
    let res = parse_source(ConfigInInput::new_extra(
        input,
        ConfigInFile {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"),
            ..Default::default()
        },
    ))
    .map(|r| (r.0.fragment().to_owned(), r.1));
    assert_eq!(res, expected)
}
