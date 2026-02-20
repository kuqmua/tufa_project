use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn gen_if_write_is_err_ts(params_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        if {
            use std::fmt::Write as _;
            write!(#params_ts)
        }.is_err() {
            #ts
        }
    }
}
pub fn gen_if_write_is_err_curly_braces_ts(params_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    gen_if_write_is_err_ts(params_ts, ts)
}
