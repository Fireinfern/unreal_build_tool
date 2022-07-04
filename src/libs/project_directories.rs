use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub unreal_build_path: String,
}

pub fn get_project_directory() -> Result<ProjectDirs, String> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "fireinfern", "BuildTool") {
        return Ok(proj_dirs);
    }
    return Err("Couldn't find the project dirs".to_string());
}