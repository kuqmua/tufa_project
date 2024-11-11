pub fn wrap_derive(values: &[&proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    quote::quote! {#[derive(#(#values),*)]}
}
