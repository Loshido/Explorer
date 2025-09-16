use std::{env, time::{Duration, Instant, UNIX_EPOCH}};
use super::auth_version;
use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };
use serde::{Deserialize, Serialize};

fn encode_secret() -> EncodingKey {
    let secret = env::var("JWT_SECRET").unwrap_or("lizard".to_string());
    EncodingKey::from_secret(secret.as_bytes())
}
fn decode_secret() -> DecodingKey {
    let secret = env::var("JWT_SECRET").unwrap_or("lizard".to_string());
    DecodingKey::from_secret(secret.as_bytes())
}

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
    version: String
}

pub fn sign() -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::default();

    let key = encode_secret();

    // 1 semaine
    let exp = UNIX_EPOCH.elapsed().unwrap() + Duration::from_secs(60 * 60 * 24 * 7);
    let exp = exp.as_millis();
    let claims = Claims {
        iss: "nogata".to_string(),
        sub: "explorer".to_string(),
        iat: Instant::now().elapsed().as_millis() as i64,
        exp: exp as i64,
        version: auth_version()
    };

    encode(&header, &claims, &key)
}

pub fn verify(token: &str) -> bool {
    let key = decode_secret();
    let token = decode::<Claims>(&token, &key, &Validation::default());

    if token.is_err() {
        return false;
    }

    let token = token.unwrap();
    let version = auth_version();

    token.claims.version.eq(&version)
}