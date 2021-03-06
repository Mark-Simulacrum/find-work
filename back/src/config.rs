use std::fs::File;

use serde_json;

const CONFIG_PATH: &'static str = "../data/config.json";

/// Configuration for the server.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub repository: String,
    pub username: String,
    pub token: String,
    pub addr: String,
    pub static_path: String,
    pub index_path: String,
    pub dev_mode: bool,
}

/// Reads a config from CONFIG_PATH.
pub fn read_config() -> ::Result<Config> {
    let file = File::open(CONFIG_PATH)?;
    let config: Config = serde_json::from_reader(file)?;
    Ok(config)
}
