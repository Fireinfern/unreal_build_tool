use clap::{Args, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Target {
    Win,
    LinuxServer,
    Linux
}

#[derive(Args, Debug)]
pub struct Build{
    #[clap(long, value_parser)]
    pub project_path: String,

    #[clap(arg_enum, value_parser, default_value_t=Target::Win)]
    pub target: Target,
}