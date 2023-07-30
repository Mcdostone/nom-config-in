use std::path::PathBuf;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::{cut, map, recognize},
    error::ErrorKind,
    multi::many1,
    IResult,
};

use crate::{config_in::ConfigIn, parse_config_in, util::ws, ConfigInFile, ConfigInInput};

/// Entry that reads the specified configuration file. This file is always parsed.
pub type Source = ConfigIn;

pub fn parse_source(input: ConfigInInput) -> IResult<ConfigInInput, Source> {
    let (input, _) = ws(tag("source"))(input)?;
    let (input, file) = parse_path(input)?;

    let source_config_in_file =
        ConfigInFile::new(input.clone().extra.root_dir, PathBuf::from(file));
    if let Ok(ff) = source_config_in_file.read_to_string() {
        return match cut(parse_config_in)(ConfigInInput::new_extra(
            ff.as_str(),
            source_config_in_file.clone(),
        )) {
            Ok((_, kconfig)) => Ok((input, kconfig)),
            Err(_err) => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    ConfigInInput::new_extra("", source_config_in_file),
                    ErrorKind::Fail,
                )))
            }
        };
    }
    Err(nom::Err::Error(nom::error::Error::new(
        ConfigInInput::new_extra("", source_config_in_file),
        ErrorKind::Fail,
    )))
}

pub fn parse_path(input: ConfigInInput) -> IResult<ConfigInInput, &str> {
    map(
        recognize(ws(many1(alt((alphanumeric1, recognize(one_of("_-/."))))))),
        |c: ConfigInInput| c.trim(),
    )(input)
}
