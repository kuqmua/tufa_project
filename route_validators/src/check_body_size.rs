use axum::{
    body::{Body, HttpBody, to_bytes},
    http::StatusCode,
};
use error_occurence_lib::code_occurence;
use error_occurence_lib::code_occurence::CodeOccurence;
use http_body::SizeHint;
use http_logic::GetAxumHttpStatusCode;

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum ErrorNamed {
    ReachedMaximumSizeOfBody {
        #[eo_to_err_string]
        axum_error: axum::Error,
        #[eo_to_err_string_serialize_deserialize]
        maximum_size_of_body_limit_in_bytes: usize,
        #[eo_to_err_string]
        size_hint: SizeHint,
        code_occurence: CodeOccurence,
    },
}
impl GetAxumHttpStatusCode for ErrorNamed {
    fn get_axum_http_status_code(&self) -> StatusCode {
        match self {
            Self::ReachedMaximumSizeOfBody { .. } => StatusCode::PAYLOAD_TOO_LARGE,
        }
    }
}
pub async fn check_body_size(body: Body, limit: usize) -> Result<bytes::Bytes, ErrorNamed> {
    let size_hint = HttpBody::size_hint(&body);
    match to_bytes(body, limit).await {
        Ok(value) => Ok(value),
        Err(error) => Err(ErrorNamed::ReachedMaximumSizeOfBody {
            axum_error: error,
            maximum_size_of_body_limit_in_bytes: limit,
            size_hint,
            code_occurence: code_occurence!(),
        }),
    }
}
