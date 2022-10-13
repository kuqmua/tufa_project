use actix_web::{web, HttpResponse, Responder};
use tufa_common::json_example::JsonExample;

pub async fn json_example_post(json: web::Json<JsonExample>) -> impl Responder {
    println!("json example {:#?}", json);
    HttpResponse::Ok().finish()
}
