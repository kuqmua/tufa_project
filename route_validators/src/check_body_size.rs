use axum::{
    body::{Body, HttpBody, to_bytes},
    http::StatusCode,
};
use error_occurence_lib::code_occurence::CodeOccurence;

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum ErrorNamed {
    ReachedMaximumSizeOfBody {
        #[eo_to_std_string_string]
        axum_error: axum::Error,
        #[eo_to_std_string_string_serialize_deserialize]
        maximum_size_of_body_limit_in_bytes: usize,
        #[eo_to_std_string_string]
        size_hint: http_body::SizeHint,
        code_occurence: CodeOccurence,
    },
}
impl http_logic::GetAxumHttpStatusCode for ErrorNamed {
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
            code_occurence: error_occurence_lib::code_occurence!(),
        }),
    }
}
