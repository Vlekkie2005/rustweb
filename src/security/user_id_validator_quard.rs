use std::ops::Deref;
use rocket::request::FromParam;
use uuid::Uuid;
use crate::security::user_id_validator_quard::Error::Invalid;

pub struct UserId(pub Uuid);

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl<'a> FromParam<'a> for UserId {
    type Error = Error;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match Uuid::parse_str(param) {
            Ok(uuid) => Ok(UserId(uuid)),
            Err(_) => Err(Invalid),
        }
    }
}

impl From<UserId> for Uuid {
    fn from(id: UserId) -> Self {
        id.0
    }
}