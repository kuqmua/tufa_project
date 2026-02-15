use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn gen_impl_try_from_ts(
    from_type_ts: &dyn ToTokens,
    for_type_ts: &dyn ToTokens,
    error_type_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    use naming::ValueSc;
    let value_sc = ValueSc;
    quote! {
        impl TryFrom<#from_type_ts> for #for_type_ts {
            type Error = #error_type_ts;
            fn try_from(#value_sc: #from_type_ts) -> Result<Self, Self::Error> {
                #content_ts
            }
        }
    }
}
