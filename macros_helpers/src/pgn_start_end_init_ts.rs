use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn pgn_start_end_init_ts(v: &dyn ToTokens) -> Ts2 {
    quote! {
        let start = #v.pgn.start();
        let end = #v.pgn.end();
    }
}
