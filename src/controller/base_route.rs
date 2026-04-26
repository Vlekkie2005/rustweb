use crate::security::user_auth_guard::AuthenticatedUser;


#[get("/")]
pub fn index() -> String {
    "hello world".to_string()
}

// removes the error of user not being used in the auth guard also good login test.
#[rocket::get("/protected")]
pub fn protected_route(user: AuthenticatedUser) -> String {
    format!("Your internal ID is: {}", user.0)
}