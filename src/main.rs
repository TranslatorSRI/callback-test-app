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

#[rocket::main]
async fn main() {
    let launch_result = rocket::build().mount("/", routes![callback]).launch().await;
    match launch_result {
        Ok(_) => info!("Rocket shut down gracefully."),
        Err(err) => warn!("Rocket had an error: {}", err),
    };
}
