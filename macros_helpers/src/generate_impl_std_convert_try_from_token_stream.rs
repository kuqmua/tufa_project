pub fn generate_impl_std_convert_try_from_token_stream(from_type_token_stream: &dyn quote::ToTokens, for_type_token_stream: &dyn quote::ToTokens, error_type_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote! {
        impl std::convert::TryFrom<#from_type_token_stream> for #for_type_token_stream {
            type Error = #error_type_token_stream;
            fn try_from(#value_snake_case: #from_type_token_stream) -> Result<Self, Self::Error> {
                #content_token_stream
            }
        }
    }
}
