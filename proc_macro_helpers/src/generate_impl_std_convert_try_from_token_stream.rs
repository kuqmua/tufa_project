pub fn generate_impl_std_convert_try_from_token_stream(
    from_type_token_stream: &proc_macro2::TokenStream,
    for_type_token_stream: &proc_macro2::TokenStream,
    error_type_token_stream: &proc_macro2::TokenStream,
    content_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming_conventions::ValueSnakeCase;
    quote::quote!{
        impl std::convert::TryFrom<#from_type_token_stream> for #for_type_token_stream {
            type Error = #error_type_token_stream;
            fn try_from(#value_snake_case: #from_type_token_stream) -> Result<Self, Self::Error> {
                #content_token_stream
            }
        }
    }
}