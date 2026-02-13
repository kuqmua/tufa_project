use naming::{ErrorOccurenceLibSc, SelfSc, ToStdStringStringSc, ToStdStringStringUcc};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use token_patterns::StdStringString;
pub fn gen_impl_error_occurence_lib_to_std_string_string_ts(
    impl_generics_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    ident_generics_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl #impl_generics_ts #ErrorOccurenceLibSc::#ToStdStringStringUcc for #ident_ts #ident_generics_ts {
            fn #ToStdStringStringSc(&#SelfSc) -> #StdStringString {
                #content_ts
            }
        }
    }
}
