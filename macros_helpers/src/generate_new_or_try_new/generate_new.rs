pub fn generate_new_token_stream(parameters_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {
        fn new(#parameters_token_stream) -> Self {
            #content_token_stream
        }
    }
}
pub fn generate_const_new_token_stream(parameters_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    super::const_space_content_token_stream(&generate_new_token_stream(parameters_token_stream, content_token_stream))
}
pub fn generate_pub_new_token_stream(parameters_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    super::pub_space_content_token_stream(&generate_new_token_stream(parameters_token_stream, content_token_stream))
}
pub fn generate_pub_const_new_token_stream(parameters_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    super::pub_space_content_token_stream(&generate_const_new_token_stream(parameters_token_stream, content_token_stream))
}
pub fn generate_impl_new_for_ident_token_stream(ident_token_stream: &dyn quote::ToTokens, parameters_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    super::impl_ident_content_token_stream(ident_token_stream, &generate_new_token_stream(parameters_token_stream, content_token_stream))
}
pub fn generate_impl_const_new_for_ident_token_stream(ident_token_stream: &dyn quote::ToTokens, parameters_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    super::impl_ident_content_token_stream(ident_token_stream, &generate_new_token_stream(parameters_token_stream, content_token_stream))
}
pub fn generate_impl_pub_new_for_ident_token_stream(ident_token_stream: &dyn quote::ToTokens, parameters_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    super::impl_ident_content_token_stream(ident_token_stream, &generate_pub_new_token_stream(parameters_token_stream, content_token_stream))
}
pub fn generate_impl_pub_const_new_for_ident_token_stream(ident_token_stream: &dyn quote::ToTokens, parameters_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    super::impl_ident_content_token_stream(ident_token_stream, &generate_pub_const_new_token_stream(parameters_token_stream, content_token_stream))
}
