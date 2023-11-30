mod config;

pub mod api;
pub use config::ServerConfig;

pub type DbPool = sqlx::Pool<sqlx::Postgres>;
