use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
pub fn gen_impl_std_convert_from_ts(
    from_type_ts: &dyn ToTokens,
    for_type_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    use naming::ValueSc;
    let value_sc = ValueSc;
    quote! {
        impl From<#from_type_ts> for #for_type_ts {
            fn from(#value_sc: #from_type_ts) -> Self {
                #content_ts
            }
        }
    }
}
