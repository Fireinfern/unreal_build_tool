pub mod build;
pub mod configurate;

use clap::{Subcommand};

use self::{configurate::Configurate, build::Build};

#[derive(Subcommand, Debug)]
pub enum Commands{
    // Configuration command
    Config(Configurate),
    Build(Build)
}