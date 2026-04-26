use crate::AppConfig;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rcgen::{KeyPair, RsaKeySize};
use rocket::http::Cookie;
use rocket::time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub struct JwtService {
    config: AppConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(config: AppConfig) -> Self {
        let key_pair = KeyPair::generate_rsa_for(&rcgen::PKCS_RSA_SHA256, RsaKeySize::_2048)
            .expect("Failed to generate RSA key");

        let priv_pem = key_pair.serialize_pem();
        let pub_pem = key_pair.public_key_pem();
        
        let encoding_key = EncodingKey::from_rsa_pem(priv_pem.as_bytes())
            .expect("Failed to create encoding key");
        let decoding_key = DecodingKey::from_rsa_pem(pub_pem.as_bytes())
            .expect("Failed to create decoding key");

        Self { config, encoding_key, decoding_key }
    }

    pub fn create(&self, user_id: Uuid) -> Cookie<'static> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;

        let exp_timestamp = (now + Duration::seconds(self.config.jwt_exp as i64)).timestamp();

        let claims = Claims {
            sub: user_id.to_string().to_owned(),
            iat,
            exp: exp_timestamp as usize,
        };

        let header = Header::new(Algorithm::RS256);

        let token = encode(&header, &claims, &self.encoding_key)
            .expect("Failed to create token");

        let cookie_exp = OffsetDateTime::from_unix_timestamp(exp_timestamp)
            .expect("Invalid timestamp");

        Cookie::build(("jwt", token.to_string()))
            .path("/")
            .http_only(true)
            .secure(true)
            .expires(cookie_exp)
            .build()
    }

    pub fn verify(&self, token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
        let validation = Validation::new(Algorithm::RS256);

        decode::<Claims>(token, &self.decoding_key, &validation)
    }
}