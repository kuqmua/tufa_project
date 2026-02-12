use proc_macro2::TokenStream as Ts2;
use quote::quote;
pub fn gen_pub_type_alias_ts(
    alias_type_name_ts: &dyn quote::ToTokens,
    alias_actual_type_name_ts: &dyn quote::ToTokens,
) -> Ts2 {
    quote! {pub type #alias_type_name_ts = #alias_actual_type_name_ts;}
}
