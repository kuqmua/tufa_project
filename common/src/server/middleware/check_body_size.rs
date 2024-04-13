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

pub async fn check_body_size(body: axum::body::Body) -> Result<bytes::Bytes, CheckBodySizeErrorNamed> {
    let size_hint = axum::body::HttpBody::size_hint(&body);
    match axum::body::to_bytes(
        body, 
        constants::MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES//1 megabyte//todo move it to config or something?
    ).await {
        Ok(value) => Ok(value),
        Err(e) => Err(CheckBodySizeErrorNamed::ReachedMaximumSizeOfBody {
            axum_error: e,
            maximum_size_of_body_limit_in_bytes: constants::MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES,
            size_hint: size_hint,
            code_occurence: error_occurence_lib::code_occurence!(),
        })
    }
}


