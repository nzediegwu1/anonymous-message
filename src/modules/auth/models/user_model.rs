use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize)]
pub struct AuthUser {
    pub user_id: String,
    pub email: String,
    pub token: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SignupBody {
    #[validate(email, length(max = 80))]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(length(max = 80, min = 2))]
    pub name: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub profile_link: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct UserName {
    pub name: String,
}
