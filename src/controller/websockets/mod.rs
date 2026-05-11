use crate::controller::websockets::gateway_controller::gateway;

pub mod gateway_controller;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        gateway
    ]
}