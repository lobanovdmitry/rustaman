use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware;
use actix_web::web;
use log::info;

use crate::config::app_config::{DbConfig, WebServerConfig};
use crate::db;
use crate::util::Clock;
use crate::web_server::health_check;
use crate::web_server::user_endpoint::user_endpoints;
use crate::web_server::web_state;

pub struct WebServer {}

impl WebServer {
    pub async fn start(config: &WebServerConfig, db_config: &DbConfig) -> std::io::Result<()> {
        info!("Starting web server ...");
        let db_pool = db::db_pool::new_pg_pool(db_config);
        let clock = Clock {};
        let web_state = web_state::WebState {
            db_pool: db_pool.clone(),
        };
        let http_server = HttpServer::new(move || {
            App::new()
                // enable logger
                .wrap(middleware::Logger::default())
                .wrap(middleware::NormalizePath::trim())
                .app_data(web::Data::new(web_state.clone()))
                .app_data(web::Data::new(clock))
                .service(
                    web::scope("/api")
                        .service(user_endpoints(db_pool.clone()))
                        .service(health_check::is_alive),
                )
        }).bind(("0.0.0.0", config.port))?.run();

        info!("Http server started!");
        http_server.await
    }
}
