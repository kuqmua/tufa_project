use naming::ValueSc;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn gen_impl_try_from_ts(
    from_type_ts: &dyn ToTokens,
    for_type_ts: &dyn ToTokens,
    er_type_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl TryFrom<#from_type_ts> for #for_type_ts {
            type Error = #er_type_ts;
            fn try_from(#ValueSc: #from_type_ts) -> Result<Self, Self::Error> {
                #content_ts
            }
        }
    }
}
