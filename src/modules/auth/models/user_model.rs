use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize)]
pub struct User {
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
