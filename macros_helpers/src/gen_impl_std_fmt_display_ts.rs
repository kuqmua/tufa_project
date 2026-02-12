use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn gen_impl_std_fmt_display_ts(
    impl_generics_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    ident_generics_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    use naming::SelfSc;
    let self_sc = SelfSc;
    quote! {
        impl #impl_generics_ts std::fmt::Display for #ident_ts #ident_generics_ts {
            fn fmt(&#self_sc, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #content_ts
            }
        }
    }
}
