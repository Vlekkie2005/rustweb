use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;
use crate::services::security::tokens::JwtService;

pub struct AuthenticatedUser(pub String);

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
    ServiceMissing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jwt_svc = match req.rocket().state::<JwtService>() {
            Some(svc) => svc,
            None => return Outcome::Error((Status::InternalServerError, TokenError::ServiceMissing)),
        };

        let jar = req.cookies()
            .get_private("jwt")
            .map(|cookie| cookie.value().to_string())
            .ok_or((Status::Unauthorized, TokenError::Missing))
            .and_then(|token| {
                jwt_svc.verify(&token)
                    .map(|data| data.claims.sub) // Success: extract the Claims
                    .map_err(|_| (Status::Unauthorized, TokenError::Invalid))
            });

        match jar {
            Ok(sub) => Outcome::Success(AuthenticatedUser(sub)),
            Err(e) => Outcome::Error(e),
        }
    }
}