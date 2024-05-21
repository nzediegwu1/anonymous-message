use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};

use crate::{
    config::Config,
    core::models::{Claims, JwtUser},
};

pub fn auth_token(user: JwtUser) -> Result<String, Error> {
    let config = Config::parse();
    encode(
        &Header::default(),
        &Claims {
            sub: user.id,
            email: user.email,
            exp: (Utc::now() + Duration::days(1)).timestamp(),
        },
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let config = Config::parse();
    let result = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(result.claims)
}
