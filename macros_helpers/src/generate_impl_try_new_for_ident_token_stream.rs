pub fn generate_impl_try_new_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl #ident_token_stream {
            pub fn try_new(#parameters_token_stream) -> Result<Self, #err_type_token_stream> {
                #content_token_stream
            }
        }
    }
}
pub fn generate_pub_try_new_token_stream(
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens
) -> proc_macro2::TokenStream {
    quote::quote! {
        pub fn try_new(#parameters_token_stream) -> Result<Self, #err_type_token_stream> {
            #content_token_stream
        }
    }
}