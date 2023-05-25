#[macro_use] extern crate rocket;

use rocket::State;
use dotenv::dotenv;
use rocket::serde::json;

use core_server::*;

#[get("/")]
fn index() -> json::Value {
    json::json!({
        "response": "Hello World"
    })
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool: DbPool = sqlx::postgres::PgPool::connect(&db_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();

    rocket::build()
        .manage(db_pool)
        .mount("/", routes![index])
}
