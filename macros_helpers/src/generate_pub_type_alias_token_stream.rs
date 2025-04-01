pub fn generate_pub_type_alias_token_stream(alias_type_name_token_stream: &dyn quote::ToTokens, alias_actual_type_name_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {pub type #alias_type_name_token_stream = #alias_actual_type_name_token_stream;}
}
