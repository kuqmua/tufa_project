use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn pagination_start_end_init_ts(v: &dyn ToTokens) -> Ts2 {
    quote! {
        let start = #v.pagination.start();
        let end = #v.pagination.end();
    }
}
