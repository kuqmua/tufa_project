#[proc_macro_derive(ImplDisplayAsDebug)]
pub fn impl_display_as_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input).expect("d5385b71-64b8-40b9-9e77-c18e585b7fb5");
    let ident = &syn_derive_input.ident;
    let generated = macros_helpers::generate_impl_std_fmt_display_token_stream(
        &proc_macro2::TokenStream::new(),
        &ident,
        &proc_macro2::TokenStream::new(),
        &quote::quote! {write!(f, "{:#?}", self)},
    );
    // println!("{generated}");
    generated.into()
}
