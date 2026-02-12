use quote::quote;

pub fn gen_impl_error_occurence_lib_to_std_string_string_ts(
    impl_generics_ts: &dyn quote::ToTokens,
    ident_ts: &dyn quote::ToTokens,
    ident_generics_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    use naming::{ErrorOccurenceLibSc, SelfSc, ToStdStringStringSc, ToStdStringStringUcc};
    let error_occurence_lib_sc = ErrorOccurenceLibSc;
    let to_std_string_string_ucc = ToStdStringStringUcc;
    let to_std_string_string_sc = ToStdStringStringSc;
    let std_string_string_ts = token_patterns::StdStringString;
    let self_sc = SelfSc;
    quote! {
        impl #impl_generics_ts #error_occurence_lib_sc::#to_std_string_string_ucc for #ident_ts #ident_generics_ts {
            fn #to_std_string_string_sc(&#self_sc) -> #std_string_string_ts {
                #content_ts
            }
        }
    }
}
