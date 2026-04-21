pub mod check_body_size;
pub mod check_commit;
#[cfg(test)]
pub(crate) mod test_hlp;
//todo request per second middleware
use axum::http::StatusCode;
use naming as _;
pub trait GetAxumHttpStatusCode {
    const AXUM_HTTP_STATUS_CODE: StatusCode;
    fn get_axum_http_status_code(&self) -> StatusCode {
        Self::AXUM_HTTP_STATUS_CODE
    }
}
