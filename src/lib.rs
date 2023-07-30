use std::{fs, io, path::PathBuf};

use config_in::ConfigIn;

use nom::{
    combinator::{eof, map},
    multi::many0,
    sequence::delimited,
    IResult,
};

use nom_locate::LocatedSpan;
use util::ws;

use crate::{entry::parse_entry, util::ws_comment};

pub mod config_in;
pub mod entry;
pub mod symbol;
pub mod util;

#[cfg(test)]
mod lib_test;

#[cfg(test)]
pub mod config_test;
#[cfg(test)]
pub mod symbol_test;
#[cfg(test)]
pub mod util_test;

pub type ConfigInInput<'a> = LocatedSpan<&'a str, ConfigInFile>;

/// Represents a Kconfig file.
/// - [root_dir] is the absolute path of the kernel root directory.
/// - [file] is the path the the Kconfig you want to parse
#[derive(Debug, Clone, Default)]
pub struct ConfigInFile {
    pub root_dir: PathBuf,
    pub file: PathBuf,
}

impl ConfigInFile {
    pub fn new(root_dir: PathBuf, file: PathBuf) -> Self {
        Self { root_dir, file }
    }

    pub fn full_path(&self) -> PathBuf {
        self.root_dir.join(&self.file)
    }

    pub fn read_to_string(&self) -> io::Result<String> {
        fs::read_to_string(self.full_path())
    }
}

pub fn parse_config_in(input: ConfigInInput) -> IResult<ConfigInInput, ConfigIn> {
    let file = &input.extra.file.clone();
    let (input, result) = map(delimited(ws_comment, many0(parse_entry), ws_comment), |d| {
        ConfigIn {
            file: file.display().to_string(),
            entries: d,
        }
    })(input)?;
    let (input, _) = ws(eof)(input)?;
    // TODO
    Ok((input, result))
}
