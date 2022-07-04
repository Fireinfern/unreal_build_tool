use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub engine_build_path: String,
    pub build_target_path: String,
}

fn get_project_directory() -> Result<ProjectDirs, String> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "fireinfern", "BuildTool") {
        return Ok(proj_dirs);
    }
    return Err("Couldn't find the project dirs".to_string());
}

pub fn get_configuration() -> Config {
    let proj_dirs = get_project_directory().expect("No Project Dir found");

    let config_dir = proj_dirs.config_dir();

    let config_file = fs::read_to_string(config_dir.join("config.toml"));

    let config: Config = match config_file {
        Ok(file) => toml::from_str(&file).unwrap(),
        Err(_) => {
            panic!("Config file not found!");
        }
    };

    return config;
}

pub fn save_configuration(engine_build_path: String, build_target_path: String) {
    assert!(
        !Path::new(&engine_build_path).exists(),
        "Engine Build folder path is not valid!"
    );
    assert!(
        !Path::new(&build_target_path).exists(),
        "Build_target_path is not valid"
    );

    let config: Config = Config {
        engine_build_path: engine_build_path,
        build_target_path: build_target_path,
    };

    let dir = get_project_directory().expect("no directory found!");

    let config_dir_path = dir.config_dir();

    let toml_config = toml::to_string(&config).unwrap();

    fs::write(config_dir_path.join("config.toml"), toml_config).expect("Couldn't save file!");

    println!("Saved configurations!");
}
