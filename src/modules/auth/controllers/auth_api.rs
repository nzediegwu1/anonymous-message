use crate::{
    core::extractors::ValidatedBody,
    modules::auth::{
        models::{AuthUser, Message, SignupBody, UserResponse},
        service::{find_all, signup},
        validation_errors::SignupValidationError,
    },
    ApiContext,
};
use axum::{
    body::Body,
    response::{Json, Response},
    Extension,
};
use std::result::Result::Ok;

pub async fn handle_signup(
    ctx: Extension<ApiContext>,
    ValidatedBody(body, _): ValidatedBody<SignupBody, SignupValidationError>,
) -> Result<Json<AuthUser>, Response<Body>> {
    // Process the validated request body
    let user = signup(ctx, Json(body)).await?;
    return Ok(user);
}

pub async fn find_users(
    ctx: Extension<ApiContext>,
) -> Result<Json<Vec<UserResponse>>, Response<Body>> {
    let users = find_all(ctx).await?;
    Ok(users)
}

pub async fn hello_world() -> Json<Message> {
    let message = Message {
        message: "Welcome to Auth API".to_string(),
    };
    Json(message.new())
}
