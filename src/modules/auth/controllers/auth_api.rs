use crate::{
    core::{
        extractors::{Authorized, ValidatedBody},
        models::Claims,
    },
    modules::auth::{
        models::{AuthUser, Message, SignupBody, UserName, UserResponse},
        service::{find_all, find_by_id, signup},
        validation_errors::SignupValidationError,
    },
    ApiContext,
};
use axum::{
    body::Body,
    extract::Path,
    response::{Json, Response},
    Extension,
};
use std::result::Result::Ok;
use uuid::Uuid;

pub async fn handle_signup(
    ctx: Extension<ApiContext>,
    ValidatedBody(body, _): ValidatedBody<SignupBody, SignupValidationError>,
) -> Result<Json<AuthUser>, Response<Body>> {
    // Process the validated request body
    let user = signup(ctx, Json(body)).await?;
    Ok(user)
}

pub async fn find_users(
    ctx: Extension<ApiContext>,
    Authorized(_): Authorized<Claims>,
) -> Result<Json<Vec<UserResponse>>, Response<Body>> {
    let users = find_all(ctx).await?;
    Ok(users)
}

pub async fn find_user(
    ctx: Extension<ApiContext>,
    //TODO: implement custom Uuid extractor so that 400 error response will be standard
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserName>, Response<Body>> {
    let user = find_by_id(ctx, user_id).await?;
    Ok(user)
}

pub async fn hello_world() -> Json<Message> {
    let message = Message {
        message: "Welcome to Auth API".to_string(),
    };
    Json(message.new())
}
