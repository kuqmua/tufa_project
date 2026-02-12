use quote::quote;

pub fn generate_impl_error_occurence_lib_to_std_string_string_ts(
    impl_generics_ts: &dyn quote::ToTokens,
    ident_ts: &dyn quote::ToTokens,
    ident_generics_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    use naming::{
        ErrorOccurenceLibSnakeCase, SelfSnakeCase, ToStdStringStringSnakeCase,
        ToStdStringStringUpperCamelCase,
    };
    let error_occurence_lib_snake_case = ErrorOccurenceLibSnakeCase;
    let to_std_string_string_upper_camel_case = ToStdStringStringUpperCamelCase;
    let to_std_string_string_snake_case = ToStdStringStringSnakeCase;
    let std_string_string_ts = token_patterns::StdStringString;
    let self_snake_case = SelfSnakeCase;
    quote! {
        impl #impl_generics_ts #error_occurence_lib_snake_case::#to_std_string_string_upper_camel_case for #ident_ts #ident_generics_ts {
            fn #to_std_string_string_snake_case(&#self_snake_case) -> #std_string_string_ts {
                #content_ts
            }
        }
    }
}
