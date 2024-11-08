pub fn token_stream(values: &[&proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    quote::quote! {#[derive(#(#values),*)]}
}
