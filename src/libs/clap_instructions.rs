pub mod commands;

use clap::{Parser};

use self::commands::Commands;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    commands: Commands,
}