use proc_macro2::TokenStream as Ts2;
use quote::quote;
#[proc_macro_derive(ImplDisplayAsDebug)]
pub fn impl_display_as_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).expect("d5385b71");
    let ident = &syn_derive_input.ident;
    let generated = macros_helpers::gen_impl_std_fmt_display_ts(
        &Ts2::new(),
        &ident,
        &Ts2::new(),
        &quote! {write!(f, "{:#?}", self)},
    );
    // println!("{generated}");
    generated.into()
}
