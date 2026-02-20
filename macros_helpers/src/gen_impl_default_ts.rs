use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn gen_impl_default_ts(ident_ts: &dyn ToTokens, content_ts: &dyn ToTokens) -> Ts2 {
    quote! {
        impl Default for #ident_ts {
            fn default() -> Self {
                #content_ts
            }
        }
    }
}
