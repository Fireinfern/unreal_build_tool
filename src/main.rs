mod libs;

use std::fs;
use clap::Parser;
use libs::project_directories::{get_project_directory, Config};
use libs::clap_instructions::{Args};

fn dir_test()
{
    let proj_dirs = get_project_directory().expect("No Project Dir found");

    let config_dir = proj_dirs.config_dir();

    let config_file = fs::read_to_string(config_dir.join("config.toml"));

    let config: Config = match config_file {
        Ok(file) => toml::from_str(&file).unwrap(),
        Err(_) => Config {
            unreal_build_path: "./".to_string(),
        },
    };

    let toml = toml::to_string(&config).unwrap();

    fs::write(config_dir.join("config.toml"), toml).expect("Could not safe configuration");
}

fn main() {
    let args = Args::parse();

    dbg!(args);
}
