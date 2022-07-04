mod libs;

use clap::Parser;
use libs::clap_instructions::{Args};
use crate::libs::project_directories::{get_configuration, Config, save_configuration};

fn main() {
    let args = Args::parse();

    // dbg!(args);

    match &args.commands {
        libs::clap_instructions::commands::Commands::Config(_config) => {
            save_configuration(_config.engine_build_path.to_string(), _config.build_target_path.to_string());
        },
        libs::clap_instructions::commands::Commands::Build(_build) => {
            let config = get_configuration();
            dbg!(config);
        },
    }
}
