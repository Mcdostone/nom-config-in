use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub arg: String,
    #[arg(long)]
    pub verbose: bool,
}
