pub static STATUS_CODES_CHECKER: &str = "StatusCodesChecker";

pub fn generate_enum_status_codes_checker_upper_camel_case_token_stream(
    ident: &syn::Ident,
    proc_macro_name_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let enum_status_codes_checker_name_stringified = format!("{ident}{STATUS_CODES_CHECKER}");
    enum_status_codes_checker_name_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {enum_status_codes_checker_name_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}

pub fn generate_tvfrr_extraction_logic_snake_case_token_stream(
    ident_snake_case_stringified: &str,
    proc_macro_name_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let tvfrr_extraction_logic_stringified =
        format!("tvfrr_extraction_logic_{ident_snake_case_stringified}");
    tvfrr_extraction_logic_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| {
            panic!(
                "{proc_macro_name_ident_stringified} {tvfrr_extraction_logic_stringified} {}",
                proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE
            )
        })
}

pub fn generate_ident_request_error_upper_camel_case_token_stream(
    ident: &syn::Ident,
    proc_macro_name_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let ident_request_error_stringified = format!("{ident}RequestError");
    ident_request_error_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| {
            panic!(
                "{proc_macro_name_ident_stringified} {ident_request_error_stringified} {}",
                proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE
            )
        })
}

pub fn generate_try_from_response_ident_snake_case_token_stream(
    ident_snake_case_stringified: &str,
    proc_macro_name_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let try_from_response_stringified = format!("try_from_response_{ident_snake_case_stringified}");
    try_from_response_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| {
            panic!(
                "{proc_macro_name_ident_stringified} {try_from_response_stringified} {}",
                proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE
            )
        })
}
