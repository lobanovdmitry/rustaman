use std::process::{self};

use log::info;

use rustaman::config::app_config_loader::load_app_config;
use rustaman::monitoring::monitoring_server as monitoring;
use rustaman::web_server::web_server::WebServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_config = match load_app_config() {
        Ok(config) => {
            println!("Loaded app_config: {:?}.", config);
            config
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };

    // enable logger
    println!(
        "Loading logging config from file {}...",
        app_config.log.config_file
    );
    log4rs::init_file(app_config.log.config_file, Default::default()).unwrap();
    info!("Application has started!!!");

    // start monitoring
    monitoring::MonitoringServer::new(app_config.monitoring.port).start();

    // start web server
    WebServer::start(&app_config.web_server, &app_config.db).await
}
