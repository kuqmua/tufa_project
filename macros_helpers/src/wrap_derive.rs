use proc_macro2::TokenStream as Ts2;
use quote::quote;
#[must_use]
pub fn wrap_derive(v: &[&Ts2]) -> Ts2 {
    quote! {#[derive(#(#v),*)]}
}
