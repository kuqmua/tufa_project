use proc_macro2::TokenStream as Ts2;
use quote::quote;
#[must_use]
pub fn wrap_derive(values: &[&Ts2]) -> Ts2 {
    quote! {#[derive(#(#values),*)]}
}
