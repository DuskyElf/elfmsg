#[macro_use] extern crate rocket;
use rocket::serde::json;

#[get("/")]
fn index() -> json::Value {
    json::json!({
        "response": "Hello World"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
