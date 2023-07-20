use std::{fs, path::PathBuf};

use clap::Parser;
use entry::Entry;

use serde::Serialize;
use walkdir::WalkDir;

use crate::{
    cli::Args,
    config::{parse_config, Config},
};

pub mod cli;
pub mod config;
pub mod entry;
pub mod symbol;
pub mod util;

#[cfg(test)]
mod lib_test;

#[derive(Debug, Serialize, Clone, PartialEq, Default)]
pub struct Kconfig {
    pub file: String,
    pub entries: Vec<Entry>,
}

#[cfg(test)]
pub mod config_test;
#[cfg(test)]
pub mod symbol_test;
#[cfg(test)]
pub mod util_test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut entries: Vec<PathBuf> = vec![];
    let root = PathBuf::from(args.arg).canonicalize().unwrap();
    if root.metadata().unwrap().is_dir() {
        entries.extend(
            WalkDir::new(root)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry: &walkdir::DirEntry| {
                    let valid_extension = true;
                    entry.file_type().is_file()
                        && entry.file_name().to_str().unwrap().contains("config.in")
                        && valid_extension
                })
                .map(|x| x.path().canonicalize())
                .filter_map(|e: Result<PathBuf, std::io::Error>| e.ok()),
        );
    } else {
        entries.push(root);
    }

    let mut all: Vec<Config> = vec![];
    for current_config in entries.iter() {
        let res = fs::read_to_string(current_config);
        if let Err(e) = res {
            eprintln!("Cannot read {}", current_config.display());
            return Err(Box::new(e));
        }
        let input = res.unwrap();
        if args.verbose {
            eprintln!("Parsing {}", current_config.display());
        }
        match parse_config(&input) {
            Ok((remaining, entries)) => {
                all.push(Config {
                    file: current_config.display().to_string(),
                    entries,
                });
                if !remaining.is_empty() {
                    eprintln!("Remaining:\n{}\nin {}", remaining, current_config.display());
                    panic!();
                }
            }
            Err(e) => {
                eprintln!("Cannot parse {}: {}", current_config.display(), e);
                panic!()
            }
        }
    }

    println!("{}", serde_json::to_string_pretty(&all).unwrap());
    Ok(())
}
