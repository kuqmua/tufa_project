use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    println!("health_check");
    HttpResponse::Ok().finish()
}
