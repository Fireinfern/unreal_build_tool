pub mod commands;

use clap::{Parser};

use self::commands::Commands;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    pub commands: Commands,
}