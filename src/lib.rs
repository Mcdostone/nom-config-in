use nom_locate::LocatedSpan;
use regex::Regex;
use std::path::PathBuf;
use std::{fs, io};

//pub mod attribute;
pub mod entry;
pub mod symbol;
pub mod util;

pub use self::entry::Entry;

pub type KconfigInput<'a> = LocatedSpan<&'a str, KconfigFile>;

#[derive(Debug, Clone, Default)]
pub struct KconfigFile {
    pub root_dir: PathBuf,
    pub file: PathBuf,
    pub fail_on_missing_source: bool,
}

impl KconfigFile {
    pub fn new(root_dir: PathBuf, file: PathBuf, fail_on_missing_source: bool) -> Self {
        Self {
            root_dir,
            file,
            fail_on_missing_source,
        }
    }

    pub fn full_path(&self) -> PathBuf {
        self.root_dir.join(&self.file)
    }

    pub fn read_to_string(&self) -> io::Result<String> {
        let input = fs::read_to_string(self.full_path())?;
        let re = Regex::new("\\\\\n").unwrap();
        let input = re.replace_all(&input, "");
        let re = Regex::new("\t").unwrap();
        let input = re.replace_all(&input, "    ");
        Ok(input.to_string())
    }
}

#[cfg(test)]
pub mod lib_test;
#[cfg(test)]
pub mod symbol_test;
#[cfg(test)]
pub mod util_test;