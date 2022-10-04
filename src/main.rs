#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use std::collections::HashMap;

#[post("/receive_callback_query", format = "json", data = "<query>")]
fn callback(query: Json<HashMap<String, Value>>) -> Value {
    match query.get("callback") {
        Some(callback_value) => {
            let message = format!("Successfully received message from {}", callback_value.to_string());
            println!("got message from {}", message);
            info!("got message from {}", message);
            json!({ "msg": message })
        }
        None => {
            warn!("failed");
            json!({ "msg": "Failed" })
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![callback])
}
