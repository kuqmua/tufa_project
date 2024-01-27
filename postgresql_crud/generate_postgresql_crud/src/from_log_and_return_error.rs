pub fn from_log_and_return_error(
    try_ident_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    error_log_call_token_stream: &proc_macro2::TokenStream,
    try_ident_response_variants_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        let error = #try_ident_upper_camel_case_token_stream::from(e);
        #error_log_call_token_stream
        return #try_ident_response_variants_token_stream::from(error)
    }
}
