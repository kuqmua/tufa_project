use axum::{
    Error,
    body::{Body, HttpBody, to_bytes},
    http::StatusCode,
};
use bytes::Bytes;
use error_occurence_lib::{ErrorOccurence, code_occurence, code_occurence::CodeOccurence};
use http_body::SizeHint;
use http_logic::GetAxumHttpStatusCode;
use thiserror::Error;
#[derive(Debug, Error, ErrorOccurence)]
pub enum BodySizeError {
    ReachedMaximumSizeOfBody {
        #[eo_to_err_string]
        axum_error: Error,
        #[eo_to_err_string_serde]
        maximum_size_of_body_limit_in_bytes: usize,
        #[eo_to_err_string]
        size_hint: SizeHint,
        code_occurence: CodeOccurence,
    },
}
impl GetAxumHttpStatusCode for BodySizeError {
    fn get_axum_http_status_code(&self) -> StatusCode {
        match self {
            Self::ReachedMaximumSizeOfBody { .. } => StatusCode::PAYLOAD_TOO_LARGE,
        }
    }
}
pub async fn check_body_size(body: Body, limit: usize) -> Result<Bytes, BodySizeError> {
    let size_hint = HttpBody::size_hint(&body);
    match to_bytes(body, limit).await {
        Ok(value) => Ok(value),
        Err(er) => Err(BodySizeError::ReachedMaximumSizeOfBody {
            axum_error: er,
            maximum_size_of_body_limit_in_bytes: limit,
            size_hint,
            code_occurence: code_occurence!(),
        }),
    }
}
