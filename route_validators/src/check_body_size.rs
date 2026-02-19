use axum::{
    Error,
    body::{Body, HttpBody, to_bytes},
    http::StatusCode,
};
use bytes::Bytes;
use location_lib::{Location, code_occurence, code_occurence::CodeOccurence};
use http_body::SizeHint;
use http_logic::GetAxumHttpStatusCode;
use thiserror::Error;
#[derive(Debug, Error, Location)]
pub enum BodySizeEr {
    ReachedMaximumSizeOfBody {
        #[eo_to_err_string]
        er: Error,
        #[eo_to_err_string_serde]
        maximum_size_of_body_limit_in_bytes: usize,
        #[eo_to_err_string]
        size_hint: SizeHint,
        code_occurence: CodeOccurence,
    },
}
impl GetAxumHttpStatusCode for BodySizeEr {
    fn get_axum_http_status_code(&self) -> StatusCode {
        match self {
            Self::ReachedMaximumSizeOfBody { .. } => StatusCode::PAYLOAD_TOO_LARGE,
        }
    }
}
pub async fn check_body_size(body: Body, limit: usize) -> Result<Bytes, BodySizeEr> {
    let size_hint = HttpBody::size_hint(&body);
    match to_bytes(body, limit).await {
        Ok(value) => Ok(value),
        Err(er) => Err(BodySizeEr::ReachedMaximumSizeOfBody {
            er,
            maximum_size_of_body_limit_in_bytes: limit,
            size_hint,
            code_occurence: code_occurence!(),
        }),
    }
}
