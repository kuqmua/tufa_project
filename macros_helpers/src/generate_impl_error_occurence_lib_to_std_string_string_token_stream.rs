pub fn generate_impl_error_occurence_lib_to_std_string_string_token_stream(
    impl_generics_token_stream: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    ident_generics_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens
) -> proc_macro2::TokenStream {
    let error_occurence_lib_snake_case = naming::ErrorOccurenceLibSnakeCase;
    let to_std_string_string_upper_camel_case = naming::ToStdStringStringUpperCamelCase;
    let to_std_string_string_snake_case = naming::ToStdStringStringSnakeCase;
    let std_string_string_token_stream = token_patterns::StdStringString;
    let self_snake_case = naming::SelfSnakeCase;
    quote::quote! {
        impl #impl_generics_token_stream #error_occurence_lib_snake_case::#to_std_string_string_upper_camel_case for #ident #ident_generics_token_stream {
            fn #to_std_string_string_snake_case(&#self_snake_case) -> #std_string_string_token_stream {
                #content_token_stream
            }
        }
    }
}