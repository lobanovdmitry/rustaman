use std::fs;

use crate::config::app_config::AppConfig;

const DEFAULT_CONFIG_NAME: &str = "app.toml";

pub fn load_app_config() -> Result<AppConfig, String> {
    let filename = DEFAULT_CONFIG_NAME;
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => return Err(format!("Could not read config file`{}`: {}.", filename, e)),
    };
    let app_config = match toml::from_str(&contents) {
        Ok(app_config) => app_config,
        Err(e) => {
            return Err(format!(
                "Could not load config from config file `{}`: {}.",
                filename, e
            ));
        }
    };
    Ok(app_config)
}
