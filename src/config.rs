use crate::constants;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub context: String,
    pub editor: String,
}

fn get_config_file_dir() -> std::path::PathBuf {
    let mut cfg_dir = dirs::config_dir().expect("Failed to get config directory");
    cfg_dir.push(constants::APP_NAME);
    std::fs::create_dir_all(&cfg_dir).expect("Failed to create config directory");
    cfg_dir.push(constants::CONFIG_FILE);
    cfg_dir
}

pub fn save_config(cfg: &Config) {
    let cfg_dir = get_config_file_dir();
    let cfg_str = toml::to_string(&cfg).expect("Failed to serialize config");
    std::fs::write(cfg_dir.as_path(), cfg_str).expect("Failed to write config file");
}

pub fn get_config() -> Config {
    let cfg_dir = get_config_file_dir();
    let cfg_str = std::fs::read_to_string(cfg_dir.as_path());

    if let Ok(cfg_str) = cfg_str {
        toml::from_str(&cfg_str).expect("Failed to deserialize config")
    } else {
        let mut context_dir = dirs::config_dir().expect("Failed to get config directory");
        context_dir.push(constants::APP_NAME);
        context_dir.push("default_context");

        let cfg = Config {
            context: context_dir.to_str().unwrap().to_string(),
            editor: "vim".to_string(),
        };
        save_config(&cfg);
        cfg
    }
}
