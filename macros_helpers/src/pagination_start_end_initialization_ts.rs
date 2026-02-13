use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn pagination_start_end_initialization_ts(value_ts: &dyn ToTokens) -> Ts2 {
    quote! {
        let start = #value_ts.pagination.start();
        let end = #value_ts.pagination.end();
    }
}
