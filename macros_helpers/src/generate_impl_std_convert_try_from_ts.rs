pub fn generate_impl_std_convert_try_from_ts(
    from_type_ts: &dyn quote::ToTokens,
    for_type_ts: &dyn quote::ToTokens,
    error_type_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming::ValueSnakeCase;
    quote::quote! {
        impl TryFrom<#from_type_ts> for #for_type_ts {
            type Error = #error_type_ts;
            fn try_from(#value_snake_case: #from_type_ts) -> Result<Self, Self::Error> {
                #content_ts
            }
        }
    }
}
