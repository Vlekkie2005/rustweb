#[macro_use] extern crate rocket;

mod controller;
mod services;
mod config;
mod security;
pub mod models;
pub mod repositories;

use config::AppConfig;
use crate::repositories::UserRepository;

#[launch]
async fn rocket() -> _ {
    let config = AppConfig::load_envs();
    let pool = config.load_pgpool().await;

    let jwt_service = services::security::tokens::JwtService::new(config);

    let repo = UserRepository::new(pool);

    rocket::build()
        .manage(jwt_service)
        .manage(repo)
        .mount("/", routes![
            controller::base_route::index,
            controller::base_route::protected_route,
            controller::security::register_controller::register,
            controller::security::login_controller::login,
            controller::security::logout_controller::logout,
            controller::security::identity_controller::fetch_auth
        ])
}