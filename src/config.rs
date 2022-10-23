#[path = "./constants.rs"] mod constants;

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    pub base_path: String,
    pub file_exts: Vec<String>,
}

pub fn read_config() -> UserConfig {
    let config_file = fs::read_to_string(constants::CONFIG_FILE_NAME).expect("Unable to read config file at: ./{{CONFIG_FILE_NAME}}");
    let config: UserConfig = serde_json::from_str(&config_file).expect("Unable to parse config file");
    config
}
