use clap::Args;

#[derive(Args, Debug)]
#[clap(about="This subcommand helps sets the paths needed to start building with the command line")]
pub struct Configurate{

    #[clap(value_parser, long)]
    pub engine_build_path: String,

    #[clap(value_parser, long)]
    pub build_target_path: String
}