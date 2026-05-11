use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        // please remember to fix this and change the origin to 
        let origin_value = _request.headers().get_one("origin");

        if let Some(origin) = origin_value {
            response.set_header(Header::new("Access-Control-Allow-Origin", origin));
        } else {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        }

        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Authorization"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}