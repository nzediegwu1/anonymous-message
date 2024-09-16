use crate::{
    core::{
        extractors::{Authorized, CustomPath, ValidatedBody},
        models::Claims,
    },
    modules::auth::{
        models::{AuthUser, LoginBody, Message, SignupBody, UserName, UserResponse},
        service::{find_all, find_by_id, login, signup},
        validation_errors::{LoginValidationError, SignupValidationError},
    },
    ApiContext,
};
use axum::{
    body::Body,
    response::{Json, Response},
    Extension,
};
use std::result::Result::Ok;
use uuid::Uuid;

pub async fn handle_signup(
    ctx: Extension<ApiContext>,
    ValidatedBody(body, _): ValidatedBody<SignupBody, SignupValidationError>,
) -> Result<Json<AuthUser>, Response<Body>> {
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
    CustomPath(user_id): CustomPath<Uuid>,
    Authorized(_): Authorized<Claims>,
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

pub async fn handle_login(
    ctx: Extension<ApiContext>,
    ValidatedBody(body, _): ValidatedBody<LoginBody, LoginValidationError>,
) -> Result<Json<AuthUser>, Response<Body>> {
    let user = login(ctx, Json(body)).await?;
    Ok(user)
}
