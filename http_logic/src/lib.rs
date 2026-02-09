use axum::http::StatusCode;
pub trait GetAxumHttpStatusCode {
    fn get_axum_http_status_code(&self) -> StatusCode;
}
