use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn gen_if_write_is_err_ts(prms_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        if {
            use std::fmt::Write as _;
            write!(#prms_ts)
        }.is_err() {
            #ts
        }
    }
}
