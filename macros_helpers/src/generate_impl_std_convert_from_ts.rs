pub fn generate_impl_std_convert_from_ts(
    from_type_ts: &dyn quote::ToTokens,
    for_type_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote! {
        impl From<#from_type_ts> for #for_type_ts {
            fn from(#value_snake_case: #from_type_ts) -> Self {
                #content_ts
            }
        }
    }
}
