use macros_helpers::gen_impl_display_ts;
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use syn::{DeriveInput, parse};
#[proc_macro_derive(ImplDisplayAsDebug)]
pub fn impl_display_as_debug(v: Ts) -> Ts {
    panic_location::panic_location();
    let di: DeriveInput = parse(v).expect("d5385b71");
    let ident = &di.ident;
    let generated = gen_impl_display_ts(
        &Ts2::new(),
        &ident,
        &Ts2::new(),
        &quote! {write!(f, "{:#?}", self)},
    );
    // println!("{generated}");
    generated.into()
}
