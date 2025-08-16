pub fn generate_std_default_default_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl std::default::Default for #ident_token_stream {
            fn default() -> Self {
                #content_token_stream
            }
        }
    }
}
