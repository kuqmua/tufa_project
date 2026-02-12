use quote::quote;

pub fn gen_std_default_default_ts(
    ident_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote! {
        impl Default for #ident_ts {
            fn default() -> Self {
                #content_ts
            }
        }
    }
}
