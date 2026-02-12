use quote::quote;

pub fn generate_pub_type_alias_ts(
    alias_type_name_ts: &dyn quote::ToTokens,
    alias_actual_type_name_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote! {pub type #alias_type_name_ts = #alias_actual_type_name_ts;}
}
