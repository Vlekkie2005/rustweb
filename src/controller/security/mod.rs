use crate::controller::security::identity_controller::fetch_auth;
use crate::controller::security::login_controller::login;
use crate::controller::security::logout_controller::logout;
use crate::controller::security::register_controller::register;

pub mod login_controller;
pub mod register_controller;
pub mod logout_controller;
pub mod identity_controller;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        login,
        register,
        fetch_auth,
        logout
    ]
}