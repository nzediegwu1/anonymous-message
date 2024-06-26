use crate::{
    core::{
        models::{ApiError, JwtUser},
        utils::auth_token,
    },
    modules::auth::models::{AuthUser, LoginBody, LoginUser, SignupBody, UserName, UserResponse},
    ApiContext,
};
use anyhow::{anyhow, Context, Error};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordVerifier};
use axum::{body::Body, http::Response, response::IntoResponse};
use axum::{Extension, Json};
use uuid::Uuid;

async fn hash_password(password: String) -> Result<String, Error> {
    Ok(
        tokio::task::spawn_blocking(move || -> Result<String, Error> {
            let salt = SaltString::generate(rand::thread_rng());
            Ok(PasswordHash::generate(Argon2::default(), password, &salt)
                .map_err(|e| anyhow!("Failed to generate password hash {e}"))?
                .to_string())
        })
        .await
        .context("Panic in generating password hash")??,
    )
}

pub async fn signup(
    ctx: Extension<ApiContext>,
    Json(body): Json<SignupBody>,
) -> Result<Json<AuthUser>, Response<Body>> {
    // check if the user exists in db, if exists, throw already exists error
    // otherwise save the user, generate token and return auth-user with token
    match sqlx::query_scalar!(r#"select email from "users" where email = $1"#, body.email)
        .fetch_one(&ctx.db)
        .await
    {
        Ok(_) => Err(ApiError::Conflict("Email already exists".to_string()).into_response()),
        Err(sqlx::Error::RowNotFound) => {
            let password_hash = hash_password(body.password)
                .await
                .map_err(|error| ApiError::InternalServer(error.to_string()).into_response())?;

            let user_id = sqlx::query_scalar!(
                // language=PostgreSQL
                r#"insert into users (email, password, name) values ($1, $2, $3) returning id"#,
                body.email,
                password_hash,
                body.name
            )
            .fetch_one(&ctx.db)
            .await
            .map_err(|err| ApiError::Database(err).into_response())?;

            let token = auth_token(JwtUser {
                email: body.email.clone(),
                id: user_id.to_string(),
            })
            .map_err(|err| ApiError::InternalServer(err.to_string()).into_response())?;

            Ok(Json(AuthUser {
                user_id: user_id.to_string(),
                email: body.email,
                token,
                name: body.name,
            }))
        }
        Err(e) => Err(ApiError::Database(e).into_response()),
    }
}

pub async fn find_all(
    ctx: Extension<ApiContext>,
) -> Result<Json<Vec<UserResponse>>, Response<Body>> {
    let result = sqlx::query_as!(
        UserResponse,
        r#"select id::text as user_id, email, name, created_at, profile_link from "users""#
    )
    .fetch_all(&ctx.db)
    .await;

    match result {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(ApiError::Database(e).into_response()),
    }
}

pub async fn find_by_id(
    ctx: Extension<ApiContext>,
    user_id: Uuid,
) -> Result<Json<UserName>, Response<Body>> {
    let result = sqlx::query_scalar(r#"select name from "users" where id=$1"#)
        .bind(user_id)
        .fetch_optional(&ctx.db)
        .await;
    match result {
        Ok(result_content) => match result_content {
            Some(name) => Ok(Json(UserName { name })),
            None => Err(ApiError::NotFound("User not found".to_string()).into_response()),
        },
        Err(e) => Err(ApiError::InternalServer(e.to_string()).into_response()),
    }
}

pub async fn login(
    ctx: Extension<ApiContext>,
    Json(body): Json<LoginBody>,
) -> Result<Json<AuthUser>, Response<Body>> {
    let exists = sqlx::query_as!(
        LoginUser,
        r#"select id::text as user_id, password, name from "users" where email=$1"#,
        body.email
    )
    .fetch_one(&ctx.db)
    .await;
    match exists {
        Ok(found) => {
            // check if the found.password matches
            let password_str = found.password.unwrap_or_default();
            let parsed_password_hash = PasswordHash::new(&password_str)
                .map_err(|error| ApiError::InternalServer(error.to_string()).into_response())?;

            Argon2::default()
                .verify_password(body.password.as_bytes(), &parsed_password_hash)
                .map_err(|_| {
                    ApiError::Unauthorized("Incorrect email or password".to_string())
                        .into_response()
                })?;

            let token = auth_token(JwtUser {
                email: body.email.clone(),
                id: found.user_id.clone().unwrap_or_default(),
            })
            .map_err(|err| ApiError::InternalServer(err.to_string()).into_response())?;

            Ok(Json(AuthUser {
                user_id: found.user_id.unwrap_or_default(),
                email: body.email,
                token,
                name: found.name.unwrap_or_default(),
            }))
        }
        Err(e) => Err(ApiError::Database(e).into_response()),
    }
}
