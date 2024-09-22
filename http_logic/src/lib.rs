pub trait GetAxumHttpStatusCode {
    fn get_axum_http_status_code(&self) -> axum::http::StatusCode;
}
