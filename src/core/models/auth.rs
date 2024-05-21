use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: i64,
}

pub struct JwtUser {
    pub email: String,
    pub id: String,
}
