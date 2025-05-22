pub fn generate_impl_new_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote!{
        impl #ident_token_stream {
            pub fn new(#parameters_token_stream) -> Self {
                #content_token_stream
            }
        }
    }
}
