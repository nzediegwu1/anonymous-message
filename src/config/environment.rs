#[derive(Default, Debug)]
pub struct Config {
    pub database_url: String,

    pub rust_log: String,

    pub jwt_secret: String,
}

impl Config {
    pub fn parse() -> Self {
        return Config {
            database_url: std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable"),
            rust_log: std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
            jwt_secret: std::env::var("JWT_SECRET").expect("Missing JWT_SECRET env variable"),
        };
    }
}
