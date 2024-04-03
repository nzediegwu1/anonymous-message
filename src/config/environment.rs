#[derive(Default, Debug)]
pub struct Config {
    pub database_url: String,

    pub rust_log: String,
}
impl Config {
    pub fn parse() -> Self {
        return Config {
            database_url: std::env::var("DATABASE_URL").expect("Set DATABASE_URL env variable"),
            rust_log: std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        };
    }
}
