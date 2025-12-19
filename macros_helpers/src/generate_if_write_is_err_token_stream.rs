pub fn generate_if_write_is_err_token_stream(parameters_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {{
        use std::fmt::Write as _;
        if write!(#parameters_token_stream).is_err() {
            #content_token_stream
        }
    }}
}
pub fn generate_if_write_is_err_curly_braces_token_stream(parameters_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let token_stream = generate_if_write_is_err_token_stream(parameters_token_stream, content_token_stream);
    quote::quote! {{#token_stream}}
}
