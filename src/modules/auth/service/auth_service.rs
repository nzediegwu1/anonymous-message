use anyhow::{anyhow, Context, Error, Ok};
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use axum::{Extension, Json};

use crate::{
    modules::auth::models::{SignupBody, User},
    ApiContext,
};

pub async fn signup(
    ctx: Extension<ApiContext>,
    Json(body): Json<SignupBody>,
) -> Result<Json<User>, Error> {
    let password_hash = hash_password(body.password).await?;

    let user_id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"insert into users (email, password, name) values ($1, $2, $3) returning id"#,
        body.email,
        password_hash,
        body.name
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(User {
        user_id: user_id.to_string(),
        email: body.email,
        token: "".to_string(),
        name: body.name,
    }))
}

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
