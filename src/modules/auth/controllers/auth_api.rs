use crate::{
    core::extractors::ValidatedBody,
    modules::auth::{
        models::{Message, SignupBody, User},
        validation_errors::SignupValidationError,
    },
};
use axum::{
    body::Body,
    response::{Json, Response},
};

pub async fn handle_signup(
    ValidatedBody(body, _): ValidatedBody<SignupBody, SignupValidationError>,
) -> Result<Json<User>, Response<Body>> {
    // Process the validated request body
    Ok(Json(User {
        email: body.email,
        user_id: String::from("random_id"),
        token: String::from("random_token")
    }))
}

pub async fn hello_world() -> Json<Message> {
    let message = Message {
        message: "Welcome to Auth API".to_string(),
    };
    Json(message.new())
}
