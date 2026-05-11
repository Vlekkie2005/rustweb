use crate::controller::base_route::{index, protected_route};

pub mod base_route;
pub mod security;
pub mod websockets;
pub(crate) mod cors_handler;
pub mod channel;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        index,
        protected_route,
    ]
}
