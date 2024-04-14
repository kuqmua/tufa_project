#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
)]
pub enum CheckBodySizeErrorNamed {
    ReachedMaximumSizeOfBody {
        #[eo_display]
        axum_error: axum::Error,
        #[eo_display_with_serialize_deserialize]
        maximum_size_of_body_limit_in_bytes: std::primitive::usize,
        #[eo_display_foreign_type]
        size_hint: http_body::SizeHint,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl axum::response::IntoResponse for CheckBodySizeErrorNamed {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            Self::ReachedMaximumSizeOfBody {
                axum_error: _,
                maximum_size_of_body_limit_in_bytes: _,
                size_hint: _,
                code_occurence: _,
            } => axum::http::StatusCode::PAYLOAD_TOO_LARGE,
        };
        let mut res = axum::Json(self.into_serialize_deserialize_version()).into_response(); 
        *res.status_mut() = status_code;
        res
    }
}

pub async fn check_body_size(
    body: axum::body::Body,
    limit: std::primitive::usize,
) -> Result<bytes::Bytes, CheckBodySizeErrorNamed> {
    let size_hint = axum::body::HttpBody::size_hint(&body);
    match axum::body::to_bytes(body, limit).await {
        Ok(value) => Ok(value),
        Err(e) => Err(CheckBodySizeErrorNamed::ReachedMaximumSizeOfBody {
            axum_error: e,
            maximum_size_of_body_limit_in_bytes: limit,
            size_hint: size_hint,
            code_occurence: error_occurence_lib::code_occurence!(),
        })
    }
}


