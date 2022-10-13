use actix_web::{web, Responder};
use tufa_common::json_example::JsonExample;

pub async fn json_example() -> impl Responder {
    println!("json example");
    web::Json(JsonExample {
        first: "first_value_json_example".to_string(),
        second: "second_value_json_example".to_string(),
    })
}
