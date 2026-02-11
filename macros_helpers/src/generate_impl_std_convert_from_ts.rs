use quote::{ToTokens, quote};
pub fn generate_impl_std_convert_from_ts(
    from_type_ts: &dyn ToTokens,
    for_type_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> proc_macro2::TokenStream {
    use naming::ValueSnakeCase;
    let value_snake_case = ValueSnakeCase;
    quote! {
        impl From<#from_type_ts> for #for_type_ts {
            fn from(#value_snake_case: #from_type_ts) -> Self {
                #content_ts
            }
        }
    }
}
