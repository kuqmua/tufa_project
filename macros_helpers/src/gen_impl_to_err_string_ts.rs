use naming::{ErOccurenceLibSc, SelfSc, ToErrStringSc, ToErrStringUcc};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use token_patterns::StringTs;
pub fn gen_impl_to_err_string_ts(
    impl_generics_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    ident_generics_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl #impl_generics_ts #ErOccurenceLibSc::#ToErrStringUcc for #ident_ts #ident_generics_ts {
            fn #ToErrStringSc(&#SelfSc) -> #StringTs {
                #content_ts
            }
        }
    }
}
