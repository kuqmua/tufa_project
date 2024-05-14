pub(crate) fn from_log_and_return_error(
    try_ident_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    error_log_call_token_stream: &proc_macro2::TokenStream,
    try_ident_response_variants_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let error_snake_case_token_stream = <naming_constants::Error as naming_constants::Naming>::snake_case_token_stream();
    quote::quote! {
        let #error_snake_case_token_stream = #try_ident_upper_camel_case_token_stream::from(#error_snake_case_token_stream);
        #error_log_call_token_stream
        return #try_ident_response_variants_token_stream::from(#error_snake_case_token_stream)
    }
}
