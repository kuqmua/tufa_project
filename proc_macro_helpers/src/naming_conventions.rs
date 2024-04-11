//////////////////////////////////////////////////////////////

pub fn serialize_deserialize_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Serialize as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Deserialize as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn serialize_deserialize_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Serialize as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Deserialize as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn with_serialize_deserialize_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        <naming_constants::With as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Serialize as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Deserialize as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
// pub fn with_serialize_deserialize_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
//     let value = with_serialize_deserialize_upper_camel_case_stringified();
//     value.parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
// }
pub fn with_serialize_deserialize_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}_{}",
        <naming_constants::With as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Serialize as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Deserialize as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn error_occurence_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Occurence as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn error_occurence_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Error as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Occurence as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn syn_type_path_stringified() -> std::string::String {
    format!(
        "syn::Type::{}", 
        <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn supports_only_supported_container_stringified() -> std::string::String {
    format!(
        "{} {}",
        naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        naming_constants::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT
    )
}
pub fn code_occurence_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Code as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Occurence as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn code_occurence_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = code_occurence_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn code_occurence_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Code as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Occurence as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn code_occurence_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = code_occurence_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn is_none_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Is as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::None as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn is_none_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Is as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::None as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn error_named_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn try_from_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn try_from_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::From as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn try_from_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = try_from_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn from_str_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Str as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn from_str_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = from_str_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn from_str_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::From as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Str as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn from_str_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = from_str_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn column_not_found_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        <naming_constants::Column as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Not as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Found as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn request_error_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Request as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn row_not_found_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        <naming_constants::Row as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Not as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Found as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn type_not_found_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        <naming_constants::Type as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Not as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Found as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn column_index_out_of_bounds_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}{}{}",
        <naming_constants::Column as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Index as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Out as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Of as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Bounds as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn column_decode_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Column as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Decode as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn pool_timed_out_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        <naming_constants::Pool as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Timed as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Out as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn pool_closed_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Pool as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Closed as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn worker_crashed_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Worker as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Crashed as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn json_data_error_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        <naming_constants::Json as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Data as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn json_syntax_error_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        <naming_constants::Json as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Syntax as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn missing_json_content_type_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}{}",
        <naming_constants::Missing as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Json as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Content as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Type as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn bytes_rejection_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Bytes as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Rejection as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn unexpected_case_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Unexpected as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Case as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn expected_type_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Expected as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Type as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn expected_type_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = expected_type_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn expected_type_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Expected as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Type as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn expected_type_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = expected_type_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn unexpected_status_code_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        <naming_constants::Unexpected as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Status as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Code as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn unexpected_status_code_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = unexpected_status_code_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn failed_to_get_response_text_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}{}{}",
        <naming_constants::Failed as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::To as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Get as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Response as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Text as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn failed_to_get_response_text_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = failed_to_get_response_text_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn deserialize_response_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Deserialize as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Response as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn deserialize_response_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = deserialize_response_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn status_code_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Status as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Code as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn status_code_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = status_code_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn response_text_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Response as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Text as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn response_text_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = response_text_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn response_text_result_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}_{}",
        <naming_constants::Response as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Text as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Result as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn response_text_result_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = response_text_result_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn std_option_option_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        <naming_constants::Std as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Option as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Option as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
// pub fn std_option_option_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
//     let value = std_option_option_upper_camel_case_stringified();
//     value.parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
// }
pub fn wrapper_vec_column_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}",
        <naming_constants::Wrapper as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Column as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn wrapper_vec_column_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = wrapper_vec_column_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn wrapper_vec_column_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}_{}",
        <naming_constants::Wrapper as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Vec as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Column as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn wrapper_vec_column_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = wrapper_vec_column_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn primary_key_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Primary as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Key as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn primary_key_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = primary_key_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn order_by_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Order as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::By as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn order_by_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = order_by_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn order_by_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Order as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::By as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn order_by_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = order_by_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn primary_keys_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}",
        <naming_constants::Primary as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Keys as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn primary_keys_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = primary_keys_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn not_unique_primary_keys_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}{}{}",
        <naming_constants::Not as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Unique as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Primary as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Keys as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn not_unique_primary_keys_upper_camel_case_token_stream() -> proc_macro2::TokenStream {
    let value = not_unique_primary_keys_upper_camel_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn not_unique_primary_keys_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}_{}_{}",
        <naming_constants::Not as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Unique as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Primary as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Keys as naming_constants::Naming>::snake_case_stringified(),
    )
}
pub fn not_unique_primary_keys_snake_case_token_stream() -> proc_macro2::TokenStream {
    let value = not_unique_primary_keys_snake_case_stringified();
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn response_variants_upper_camel_case_stringified() -> std::string::String {
    format!(
        "{}{}",
        <naming_constants::Response as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Variants as naming_constants::Naming>::upper_camel_case_stringified(),
    )
}
pub fn tvfrr_extraction_logic_snake_case_stringified() -> std::string::String {
    format!(
        "{}_{}_{}",
        <naming_constants::Tvfrr as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Extraction as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::Logic as naming_constants::Naming>::snake_case_stringified(),
    )
}
/////////////////////////////////////////////////////////////////////

pub trait SelfParametersUpperCamelCaseTokenStream {
    fn self_parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfParametersUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_parameters_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}",
            self.to_upper_camel_case_stringified(),
            <naming_constants::Parameters as naming_constants::Naming>::snake_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadUpperCamelCaseTokenStream {
    fn self_payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}",
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::snake_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}{}",
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified()
        )
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self.self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseStringified {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified(),
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseTokenStream {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseTokenStream for T
where
    T: SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self.self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseStringified {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}",
            self.to_snake_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            with_serialize_deserialize_snake_case_stringified(),
            <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::From as naming_constants::Naming>::snake_case_stringified(),
            self.to_snake_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseTokenStream {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseTokenStream for T
where
    T: SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self.self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseStringified {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified(),
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            error_named_upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseTokenStream {
    fn self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseTokenStream for T
where
    T: SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseStringified,
{
    fn self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self.self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> {
        let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();
        handle.push_value(
            syn::PathSegment {
                ident: proc_macro2::Ident::new(
                    &format!(
                        "{}{}{}",
                        self.self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(),
                        <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
                        <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
                    ),
                    proc_macro2::Span::call_site()
                ),
                arguments: syn::PathArguments::None,
            }
        );
        handle
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}",
            self.to_snake_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::From as naming_constants::Naming>::snake_case_stringified(),
            self.to_snake_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::snake_case_stringified(),
            with_serialize_deserialize_snake_case_stringified()
        )
    }
}

pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream {
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified,
{
    fn self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self
            .self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfSnakeCaseStringified {
    fn try_self_snake_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn try_self_snake_case_stringified(&self) -> std::string::String {
        format!(
            "{}_{}",
            <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            self.to_snake_case_stringified()
        )
    }
}

pub trait TrySelfSnakeCaseTokenStream {
    fn try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfSnakeCaseTokenStream for T
where
    T: TrySelfSnakeCaseStringified,
{
    fn try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_snake_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfResponseVariantsUpperCamelCaseStringified {
    fn try_self_response_variants_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfResponseVariantsUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_response_variants_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            response_variants_upper_camel_case_stringified()
        )
    }
}

pub trait TrySelfResponseVariantsUpperCamelCaseTokenStream {
    fn try_self_response_variants_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfResponseVariantsUpperCamelCaseTokenStream for T
where
    T: TrySelfResponseVariantsUpperCamelCaseStringified,
{
    fn try_self_response_variants_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_response_variants_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfUpperCamelCaseStringified {
    fn try_self_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfUpperCamelCaseTokenStream {
    fn try_self_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfUpperCamelCaseTokenStream for T
where
    T: TrySelfUpperCamelCaseStringified,
{
    fn try_self_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified {
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Element as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::With as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Serialize as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Deserialize as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: SelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value =
            self.self_payload_element_with_serialize_deserialize_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementUpperCamelCaseStringified {
    fn self_payload_element_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> SelfPayloadElementUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}",
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Element as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementUpperCamelCaseTokenStream {
    fn self_payload_element_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementUpperCamelCaseTokenStream for T
where
    T: SelfPayloadElementUpperCamelCaseStringified,
{
    fn self_payload_element_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.self_payload_element_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified
    for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Element as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Element as naming_constants::Naming>::upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream
    for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self.self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseStringified {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_stringified(
        &self,
    ) -> std::string::String;
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCasePunctuated
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCasePunctuated
    for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_punctuated(
        &self,
    ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> {
        let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();
        handle.push_value(
            syn::PathSegment {
                ident: proc_macro2::Ident::new(
                    &format!(
                        "{}{}{}",
                        self.self_payload_element_try_from_self_payload_element_with_serialize_deserialize_upper_camel_case_stringified(),
                        <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
                        <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
                    ),
                    proc_macro2::Span::call_site()
                ),
                arguments: syn::PathArguments::None,
            }
        );
        handle
    }
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseStringified
    for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}_{}_{}_{}_{}_{}_{}_{}_{}",
            self.to_snake_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Element as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::From as naming_constants::Naming>::snake_case_stringified(),
            self.to_snake_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Element as naming_constants::Naming>::snake_case_stringified(),
            with_serialize_deserialize_snake_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseTokenStream {
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseTokenStream
    for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeSnakeCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self
            .self_payload_element_try_from_self_payload_element_with_serialize_deserialize_snake_sase_stringified(
            );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseStringified
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseStringified
    for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}",
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Element as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Element as naming_constants::Naming>::upper_camel_case_stringified(),
            with_serialize_deserialize_upper_camel_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T>
    SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream
    for T
where
    T: SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseStringified,
{
    fn self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self.self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfErrorNamedUpperCamelCaseTokenStream {
    fn try_self_error_named_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfErrorNamedUpperCamelCaseTokenStream for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_error_named_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!(
            "{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfRequestErrorUpperCamelCaseStringified {
    fn try_self_request_error_upper_camel_case_stringified(&self) -> std::string::String;
}

impl<T> TrySelfRequestErrorUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_request_error_upper_camel_case_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Request as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfRequestErrorUpperCamelCaseTokenStream {
    fn try_self_request_error_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfRequestErrorUpperCamelCaseTokenStream for T
where
    T: TrySelfRequestErrorUpperCamelCaseStringified,
{
    fn try_self_request_error_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_request_error_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseStringified {
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseStringified for T
where
    T: SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}",
            self.self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream {
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream for T
where
    T: PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseStringified,
{
    fn payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self.payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait PayloadWithSerializeDeserializeTryFromPayloadErrorNamedUpperCamelCaseStringified {
    fn payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> PayloadWithSerializeDeserializeTryFromPayloadErrorNamedUpperCamelCaseStringified for T
where
    T: SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseStringified,
{
    fn payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}",
            self.self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_stringified(),
            <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait PayloadWithSerializeDeserializeTryFromPayloadErrorNamedUpperCamelCaseTokenStream {
    fn payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> PayloadWithSerializeDeserializeTryFromPayloadErrorNamedUpperCamelCaseTokenStream for T
where
    T: PayloadWithSerializeDeserializeTryFromPayloadErrorNamedUpperCamelCaseStringified,
{
    fn payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value = self.payload_with_serialize_deserialize_try_from_payload_error_named_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfWithSerializeDeserializeUpperCamelCaseStringified {
    fn try_self_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String;
}

impl<T> TrySelfWithSerializeDeserializeUpperCamelCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_with_serialize_deserialize_upper_camel_case_stringified(
        &self,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::With as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Serialize as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Deserialize as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfWithSerializeDeserializeUpperCamelCaseTokenStream {
    fn try_self_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfWithSerializeDeserializeUpperCamelCaseTokenStream for T
where
    T: TrySelfWithSerializeDeserializeUpperCamelCaseStringified,
{
    fn try_self_with_serialize_deserialize_upper_camel_case_token_stream(
        &self,
    ) -> proc_macro2::TokenStream {
        let value =
            self.try_self_with_serialize_deserialize_upper_camel_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfResponseVariantsStatusCodeStringified {
    fn try_self_response_variants_status_code_stringified(
        &self,
        status_code: &crate::status_code::StatusCode,
    ) -> std::string::String;
}

impl<T> TrySelfResponseVariantsStatusCodeStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_response_variants_status_code_stringified(
        &self,
        status_code: &crate::status_code::StatusCode,
    ) -> std::string::String {
        format!(
            "{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::Response as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Variants as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(status_code),
        )
    }
}

pub trait TrySelfResponseVariantsStatusCodeTokenStream {
    fn try_self_response_variants_status_code_token_stream(
        &self,
        status_code: &crate::status_code::StatusCode,
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfResponseVariantsStatusCodeTokenStream for T
where
    T: TrySelfResponseVariantsStatusCodeStringified,
{
    fn try_self_response_variants_status_code_token_stream(
        &self,
        status_code: &crate::status_code::StatusCode,
    ) -> proc_macro2::TokenStream {
        let value =
            self.try_self_response_variants_status_code_stringified(status_code);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TrySelfWithSerializeDeserializeStringified {
    fn try_self_with_serialize_deserialize_stringified(&self) -> std::string::String;
}

impl<T> TrySelfWithSerializeDeserializeStringified for T
where
    T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
{
    fn try_self_with_serialize_deserialize_stringified(&self) -> std::string::String {
        format!(
            "{}{}{}{}{}",
            <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            self.to_upper_camel_case_stringified(),
            <naming_constants::With as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Serialize as naming_constants::Naming>::upper_camel_case_stringified(),
            <naming_constants::Deserialize as naming_constants::Naming>::upper_camel_case_stringified(),
        )
    }
}

pub trait TrySelfWithSerializeDeserializeTokenStream {
    fn try_self_with_serialize_deserialize_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TrySelfWithSerializeDeserializeTokenStream for T
where
    T: TrySelfWithSerializeDeserializeStringified,
{
    fn try_self_with_serialize_deserialize_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.try_self_with_serialize_deserialize_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub trait TvfrrExtractionLogicTrySelfSnakeCaseStringified {
    fn tvfrr_extraction_logic_try_self_snake_case_stringified(&self) -> std::string::String;
}

impl<T> TvfrrExtractionLogicTrySelfSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn tvfrr_extraction_logic_try_self_snake_case_stringified(&self) -> std::string::String {
        format!(
            "{}_{}_{}",
            tvfrr_extraction_logic_snake_case_stringified(),
            <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            self.to_snake_case_stringified()
        )
    }
}

pub trait TvfrrExtractionLogicTrySelfSnakeCaseTokenStream {
    fn tvfrr_extraction_logic_try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<T> TvfrrExtractionLogicTrySelfSnakeCaseTokenStream for T
where
    T: TvfrrExtractionLogicTrySelfSnakeCaseStringified,
{
    fn tvfrr_extraction_logic_try_self_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.tvfrr_extraction_logic_try_self_snake_case_stringified();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

//
pub trait TrySelfSnakeCasePrintlnStringified {
    fn try_self_snake_case_println_stringified(
        &self,
        test_operation_print_in_info: &crate::TestOperationPrintlnInfo,
    ) -> std::string::String;
}

impl<T> TrySelfSnakeCasePrintlnStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn try_self_snake_case_println_stringified(
        &self,
        test_operation_print_in_info: &crate::TestOperationPrintlnInfo,
    ) -> std::string::String {
        let slashes = "-------";
        format!(
            "\"{}{}{} {}{}\"",
            slashes,
            <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            self.to_snake_case_stringified(),
            proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(test_operation_print_in_info),
            slashes,
        )
    }
}

pub trait TrySelfSnakeCasePrintlnTokenStream {
    fn try_self_snake_case_println_token_stream(
        &self,
        test_operation_print_in_info: &crate::TestOperationPrintlnInfo,
    ) -> proc_macro2::TokenStream;
}

impl<T> TrySelfSnakeCasePrintlnTokenStream for T
where
    T: TrySelfSnakeCasePrintlnStringified,
{
    fn try_self_snake_case_println_token_stream(
        &self,
        test_operation_print_in_info: &crate::TestOperationPrintlnInfo,
    ) -> proc_macro2::TokenStream {
        let value =
            self.try_self_snake_case_println_stringified(test_operation_print_in_info);
        let value_token_stream = value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
        quote::quote! {println!(#value_token_stream);}
    }
}

pub trait WrapIntoStartEndPrintlnSelfTokenStream {
    fn wrap_into_start_end_println_self_token_stream(
        &self,
        test_content_token_stream: &proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream;
}

impl<T> WrapIntoStartEndPrintlnSelfTokenStream for T
where
    T: TrySelfSnakeCasePrintlnTokenStream,
{
    fn wrap_into_start_end_println_self_token_stream(
        &self,
        test_content_token_stream: &proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let start_println_token_stream =
            self.try_self_snake_case_println_token_stream(&crate::TestOperationPrintlnInfo::Start);
        let end_println_token_stream =
            self.try_self_snake_case_println_token_stream(&crate::TestOperationPrintlnInfo::End);
        quote::quote! {
            #start_println_token_stream
            #test_content_token_stream
            #end_println_token_stream
        }
    }
}

pub trait SwaggerUrlPathSelfQuotesStringified {
    fn swagger_url_path_self_quotes_stringified(
        &self,
        table_name_stringified: &str,
    ) -> std::string::String;
}

impl<T> SwaggerUrlPathSelfQuotesStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn swagger_url_path_self_quotes_stringified(
        &self,
        table_name_stringified: &str,
    ) -> std::string::String {
        proc_macro_common::generate_quotes::generate_quotes_stringified(&format!(
            "/{}/{}",
            table_name_stringified,
            self.to_snake_case_stringified(),
        ))
    }
}

pub trait SwaggerUrlPathSelfQuotesTokenStream {
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        table_name_stringified: &str,
    ) -> proc_macro2::TokenStream;
}

impl<T> SwaggerUrlPathSelfQuotesTokenStream for T
where
    T: SwaggerUrlPathSelfQuotesStringified,
{
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        table_name_stringified: &str,
    ) -> proc_macro2::TokenStream {
        let value =
            self.swagger_url_path_self_quotes_stringified(table_name_stringified);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

//
pub trait UrlHandleSelfSnakeCaseStringified {
    fn url_handle_self_snake_case_stringified(
        &self,
        table_name_stringified: &str,
    ) -> std::string::String;
}

impl<T> UrlHandleSelfSnakeCaseStringified for T
where
    T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
{
    fn url_handle_self_snake_case_stringified(
        &self,
        table_name_stringified: &str,
    ) -> std::string::String {
        format!(
            "\"{{}}/{}/{}\"",
            table_name_stringified,
            self.to_snake_case_stringified()
        )
    }
}

pub trait UrlHandleSelfSnakeCaseTokenStream {
    fn url_handle_self_snake_case_token_stream(
        &self,
        table_name_stringified: &str,
    ) -> proc_macro2::TokenStream;
}

impl<T> UrlHandleSelfSnakeCaseTokenStream for T
where
    T: UrlHandleSelfSnakeCaseStringified,
{
    fn url_handle_self_snake_case_token_stream(
        &self,
        table_name_stringified: &str,
    ) -> proc_macro2::TokenStream {
        let value = self.url_handle_self_snake_case_stringified(table_name_stringified);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}
// fn generate_url_handle_token_stream(
//     table_name_stringified: &str,
//     operation_name_snake_case_stringified: &str,
//     proc_macro_name_upper_camel_case_ident_stringified: &str,
// ) -> proc_macro2::TokenStream {
//     let url_handle_stringified = format!("\"{{}}/{table_name_stringified}/{operation_name_snake_case_stringified}\"");
//     url_handle_stringified.parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {url_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
// }
//
