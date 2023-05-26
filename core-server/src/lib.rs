#[macro_use] extern crate sqlx;
#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;

use rocket::serde::json;

use sqlx::types::Uuid;

pub mod auth;
pub type DbPool = sqlx::Pool<sqlx::Postgres>;

#[derive(Responder, Debug)]
pub enum ApiResponse {
    #[response(status = 200, content_type = "json")]
    Ok(json::Value),
    #[response(status = 400, content_type = "json")]
    CErr(json::Value),
    #[response(status = 500, content_type = "json")]
    SErr(json::Value)
}

pub struct User {
    id: Uuid,
    username: String,
    password: String,
    email: String,
    verified_email: bool,
}
