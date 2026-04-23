pub mod check_body_size;
pub mod check_commit;
mod hdr_val;
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
#[cfg(test)]
mod tests {
    use super::GetAxumHttpStatusCode;
    use axum::http::StatusCode;
    struct TestEr;
    impl GetAxumHttpStatusCode for TestEr {
        const AXUM_HTTP_STATUS_CODE: StatusCode = StatusCode::IM_A_TEAPOT;
    }
    #[test]
    fn get_axum_http_status_code_default_method_returns_associated_const() {
        let er = TestEr;
        assert_eq!(er.get_axum_http_status_code(), StatusCode::IM_A_TEAPOT);
    }
}
