use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn gen_impl_default_ts(ident: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        impl Default for #ident {
            fn default() -> Self {
                #ts
            }
        }
    }
}
