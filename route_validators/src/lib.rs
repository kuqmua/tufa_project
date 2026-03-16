pub mod check_body_size;
pub mod check_commit;
//todo request per second middleware
use axum::http::StatusCode;
pub trait GetAxumHttpStatusCode {
    fn get_axum_http_status_code(&self) -> StatusCode;
}
