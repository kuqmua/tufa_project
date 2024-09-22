#[proc_macro_derive(ImplDisplayAsDebug)]
pub fn impl_display_as_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).expect("ImplDisplayAsDebug syn::parse(input) failed");
    let ident = &syn_derive_input.ident;
    let generated = quote::quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{:#?}", self)
            }
        }
    };
    // println!("{generated}");
    generated.into()
}
