use std::path::PathBuf;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{cut, map, recognize},
    error::{Error, ErrorKind, ParseError},
    multi::many1,
    IResult,
};

use crate::{
    config_in::ConfigIn,
    parse_config_in,
    util::{ws, wsi},
    ConfigInFile, ConfigInInput,
};

/// Entry that reads the specified configuration file. This file is always parsed.
pub type Source = ConfigIn;

pub fn parse_source(input: ConfigInInput) -> IResult<ConfigInInput, Source> {
    let (input, _) = ws(tag("source"))(input)?;
    let (input, file) = wsi(parse_filepath)(input)?;
    let source_kconfig_file = ConfigInFile::new(input.clone().extra.root_dir, PathBuf::from(file));
    let source_content = source_kconfig_file
        .read_to_string()
        .map_err(|_| nom::Err::Error(Error::from_error_kind(input.clone(), ErrorKind::Fail)))?;

    let binding = source_content.clone();
    #[allow(clippy::let_and_return)]
    let x = match cut(parse_config_in)(ConfigInInput::new_extra(
        &binding,
        source_kconfig_file.clone(),
    )) {
        Ok((_, kconfig)) => Ok((input, kconfig)),
        Err(_e) => Err(nom::Err::Error(nom::error::Error::new(
            ConfigInInput::new_extra("", source_kconfig_file),
            ErrorKind::Fail,
        ))),
    };
    x
}
pub fn parse_filepath(input: ConfigInInput) -> IResult<ConfigInInput, &str> {
    map(
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("_-/."))))))),
        |c: ConfigInInput| c.trim(),
    )(input)
}
