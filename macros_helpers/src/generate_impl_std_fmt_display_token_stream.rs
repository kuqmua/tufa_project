pub fn generate_impl_std_fmt_display_token_stream(impl_generics_token_stream: &dyn quote::ToTokens, ident_token_stream: &dyn quote::ToTokens, ident_generics_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    let self_snake_case = naming::SelfSnakeCase;
    quote::quote! {
        impl #impl_generics_token_stream std::fmt::Display for #ident_token_stream #ident_generics_token_stream {
            fn fmt(&#self_snake_case, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #content_token_stream
            }
        }
    }
}
