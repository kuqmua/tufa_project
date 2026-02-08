pub fn generate_if_write_is_err_ts(
    parameters_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        if {
            use std::fmt::Write as _;
            write!(#parameters_ts)
        }.is_err() {
            #content_ts
        }
    }
}
pub fn generate_if_write_is_err_curly_braces_ts(
    parameters_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let ts = generate_if_write_is_err_ts(parameters_ts, content_ts);
    quote::quote! {#ts}
}
