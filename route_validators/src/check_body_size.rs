#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CheckBodySizeErrorNamed {
    ReachedMaximumSizeOfBody {
        #[eo_to_std_string_string]
        axum_error: axum::Error,
        #[eo_to_std_string_string_serialize_deserialize]
        maximum_size_of_body_limit_in_bytes: std::primitive::usize,
        #[eo_to_std_string_string]
        size_hint: http_body::SizeHint,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl http_logic::GetAxumHttpStatusCode for CheckBodySizeErrorNamed {
    fn get_axum_http_status_code(&self) -> axum::http::StatusCode {
        match self {
            Self::ReachedMaximumSizeOfBody { .. } => axum::http::StatusCode::PAYLOAD_TOO_LARGE,
        }
    }
}

pub async fn check_body_size(
    body: axum::body::Body,
    limit: std::primitive::usize,
) -> Result<bytes::Bytes, CheckBodySizeErrorNamed> {
    let size_hint = axum::body::HttpBody::size_hint(&body);
    //todo maybe move to router with idenpotent key log or something
    match size_hint.exact() {
        Some(value) => {
            println!(
                "HttpBody::size_hint {value} byte or {} kilobyte or {} megabyte",
                value / 1024,
                value / 1024 / 1024
            );
        }
        None => {
            println!("HttpBody::size_hint is None")
        }
    }
    match axum::body::to_bytes(body, limit).await {
        Ok(value) => Ok(value),
        Err(error) => Err(CheckBodySizeErrorNamed::ReachedMaximumSizeOfBody {
            axum_error: error,
            maximum_size_of_body_limit_in_bytes: limit,
            size_hint,
            code_occurence: error_occurence_lib::code_occurence!(),
        }),
    }
}
