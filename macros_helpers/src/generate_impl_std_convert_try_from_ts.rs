use quote::{ToTokens, quote};
pub fn generate_impl_std_convert_try_from_ts(
    from_type_ts: &dyn ToTokens,
    for_type_ts: &dyn ToTokens,
    error_type_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> proc_macro2::TokenStream {
    use naming::ValueSnakeCase;
    let value_snake_case = ValueSnakeCase;
    quote! {
        impl TryFrom<#from_type_ts> for #for_type_ts {
            type Error = #error_type_ts;
            fn try_from(#value_snake_case: #from_type_ts) -> Result<Self, Self::Error> {
                #content_ts
            }
        }
    }
}
