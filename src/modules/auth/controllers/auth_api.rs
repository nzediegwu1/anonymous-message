use crate::{
    core::{extractors::ValidatedBody, models::ApiError},
    modules::auth::{
        models::{Message, SignupBody, User},
        service::signup,
        validation_errors::SignupValidationError,
    },
    ApiContext,
};
use axum::response::IntoResponse;
use axum::{
    body::Body,
    response::{Json, Response},
    Extension,
};

pub async fn handle_signup(
    ctx: Extension<ApiContext>,
    ValidatedBody(body, _): ValidatedBody<SignupBody, SignupValidationError>,
) -> Result<Json<User>, Response<Body>> {
    // Process the validated request body
    let user = signup(ctx, Json(body)).await.map_err(|err| {
        ApiError::BadRequest {
            errors: vec![err.to_string()],
        }
        .into_response()
    })?;
    return Ok(user);
}

pub async fn hello_world() -> Json<Message> {
    let message = Message {
        message: "Welcome to Auth API".to_string(),
    };
    Json(message.new())
}
