use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn gen_pub_type_al_ts(
    al_type_name_ts: &dyn ToTokens,
    al_actual_type_name_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {pub type #al_type_name_ts = #al_actual_type_name_ts;}
}
