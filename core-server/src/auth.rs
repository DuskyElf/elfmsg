use bcrypt;
use rocket::serde::json;
use rocket::{Route, State};
use tokio::task::spawn_blocking;

use crate::*;

pub fn routes() -> Vec<Route> {
    routes![login, sign_up, is_username_free, is_email_free]
}

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

impl LoginData {
    fn is_correct(&self) -> bool {
        self.username.len() <= 16 && self.password.len() <= 32
    }
}

#[post("/login", format = "json", data = "<login_data>")]
async fn login(db_pool: &State<DbPool>, login_data: json::Json<LoginData>) -> ApiResponse {
    let login_data = login_data.into_inner();
    if !login_data.is_correct() {
        return ApiResponse::CErr(json::json!({
            "status": "error",
            "error": "InvalidPassword"
        }));
    }

    let Ok(user) = query_as!{
        User,
        "SELECT * FROM users WHERE lower(username)=lower($1)",
        login_data.username
    } .fetch_optional(db_pool.inner())
    .await else {
        return ApiResponse::SErr(json::json!({
            "status": "error",
            "error": "ServerError"
        }))
    };

    let Some(user) = user else {
        return ApiResponse::CErr(json::json!({
            "status": "error",
            "error": "InvalidUsername"
        }));
    };

    let pass_check = bcrypt::verify(login_data.password, &user.password).unwrap();
    if !pass_check {
        return ApiResponse::CErr(json::json!({
            "status": "error",
            "error": "InvalidPassword"
        }));
    }
    
    ApiResponse::Ok(json::json!({
        "status": "success"
    }))
}

#[derive(Deserialize)]
struct SignUpData {
    username: String,
    email: String,
    password: String,
}

impl SignUpData {
    fn check_correctness(&self) -> bool {
        self.username.len() <= 16
        && self.password.len() <= 32
        && self.email.len() <= 320
    }
}

#[post("/sign-up", format = "json", data = "<sign_up_data>")]
async fn sign_up(db_pool: &State<DbPool>, sign_up_data: json::Json<SignUpData>) -> ApiResponse {
    let sign_up_data = sign_up_data.into_inner();
    if !sign_up_data.check_correctness() {
        return ApiResponse::CErr(json::json!({
            "status": "error",
            "error": "InvalidPassword"
        }));
    }
    let hashed_password = spawn_blocking(||
        bcrypt::hash(sign_up_data.password, bcrypt::DEFAULT_COST).unwrap()
    ).await.unwrap();

    let Ok(user) = query_as!{
        User,
        "INSERT INTO users (username, password, email) VALUES (lower($1), $2, lower($3)) RETURNING *",
        sign_up_data.username, hashed_password, sign_up_data.email
    }.fetch_one(db_pool.inner())
        .await else {
        return ApiResponse::SErr(json::json!({
            "status": "error",
            "error": "ServerError"
        }))
    };

    ApiResponse::Ok(json::json!({
        "status": "success",
        "user_id": user.id.to_string()
    }))
}

#[post("/is-username-free", format = "json", data = "<user_name>")]
async fn is_username_free(db_pool: &State<DbPool>, user_name: json::Json<String>) -> ApiResponse {
    let user_name = user_name.into_inner();

    let Ok(db_res) = query_as!{
        User,
        "SELECT * FROM users WHERE lower(username)=lower($1)",
        user_name
    } .fetch_optional(db_pool.inner())
    .await else {
        return ApiResponse::SErr(json::json!({
            "status": "error",
            "error": "ServerError"
        }))
    };

    let response = db_res.is_none();

    ApiResponse::Ok(json::json!({
        "status": "success",
        "response": response
    }))
}

#[post("/is-email-free", format = "json", data = "<email>")]
async fn is_email_free(db_pool: &State<DbPool>, email: json::Json<String>) -> ApiResponse {
    let email = email.into_inner();

    let Ok(db_res) = query_as!{
        User,
        "SELECT * FROM users WHERE lower(email)=lower($1)",
        email
    } .fetch_optional(db_pool.inner())
    .await else {
        return ApiResponse::SErr(json::json!({
            "status": "error",
            "error": "ServerError"
        }))
    };

    let response = db_res.is_none();

    ApiResponse::Ok(json::json!({
        "status": "success",
        "response": response
    }))
}
