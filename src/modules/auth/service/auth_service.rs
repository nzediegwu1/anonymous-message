use anyhow::{anyhow, Context, Error, Ok};
use argon2::{password_hash::SaltString, Argon2, PasswordHash};

pub async fn signup() {}

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
