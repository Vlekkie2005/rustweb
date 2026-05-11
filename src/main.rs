#[macro_use] extern crate rocket;
mod controller;
mod services;
mod config;
mod security;
pub mod models;
pub mod repositories;

use config::AppConfig;
use controller::cors_handler::CORS;
use crate::services::gateway::gateway_service::GatewayService;

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
async fn rocket() -> _ {
    let config = AppConfig::load_envs();
    let pgpool = config.load_pgpool().await;
    let gateway = GatewayService::new(pgpool.clone(), &config.redis_url).await;

    let routes = [
        routes![all_options],
        controller::routes(),
        controller::security::routes(),
        controller::channel::routes(),
        controller::websockets::routes(),
    ]
    .concat();

    rocket::build()
        .attach(CORS)
        .manage(services::security::tokens::JwtService::new(config))
        .manage(repositories::Repositories::init(pgpool))
        .manage(gateway)
        .mount("/", routes)
}