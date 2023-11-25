mod config;

pub use config::ServerConfig;

pub type DbPool = sqlx::Pool<sqlx::Postgres>;
