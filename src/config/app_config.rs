use std::fmt::Debug;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub log: LoggingConfig,
    pub db: DbConfig,
    pub monitoring: MonitoringConfig,
    pub web_server: WebServerConfig,
}

#[derive(Deserialize, Debug)]
pub struct LoggingConfig {
    pub config_file: String,
}

#[derive(Deserialize, Debug)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db_name: String,
}

#[derive(Deserialize, Debug)]
pub struct MonitoringConfig {
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct WebServerConfig {
    pub port: u16,
}
