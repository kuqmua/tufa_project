use convert_case::{Case, Casing};
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, parse};
#[proc_macro_derive(FromStr)]
pub fn from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: DeriveInput = parse(input).expect("f83fcd2d");
    let ident = &syn_derive_input.ident;
    let Data::Enum(data_enum) = syn_derive_input.data else {
        panic!("d35db256");
    };
    let variant_idents = data_enum
        .variants
        .into_iter()
        .map(|variant| match variant.fields {
            Fields::Unit => variant.ident,
            Fields::Named(_) | Fields::Unnamed(_) => {
                panic!("23575b02")
            }
        })
        .collect::<Vec<Ident>>();
    let variants_ts = variant_idents.iter().map(|variant_ident| {
        let variant_ident_sc_ts = {
            let variant_ident_sc_str =
                Casing::to_case(&format!("\"{variant_ident}\""), Case::Snake);
            variant_ident_sc_str.parse::<Ts2>().expect("791603c1")
        };
        quote! {
            #variant_ident_sc_ts => Ok(Self::#variant_ident),
        }
    });
    let error_variants_str =
        variant_idents
            .iter()
            .fold(String::default(), |mut acc_d6966473, variant_ident| {
                use std::fmt::Write as _;
                let variant_ident_sc_str =
                    Casing::to_case(&format!("{variant_ident}"), Case::Snake);
                assert!(
                    write!(acc_d6966473, "\'{variant_ident_sc_str}\',").is_ok(),
                    "09c49558"
                );
                acc_d6966473
            });
    let error_ts = {
        let error_str =
            format!("\"Invalid {ident}, expected one of {error_variants_str} found {{value}}\"");
        error_str.parse::<Ts2>().expect("1b778757")
    };
    let generated = quote! {
        impl std::str::FromStr for #ident {
            type Err = String;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    #(#variants_ts)*
                    _ => Err(format!(#error_ts)),
                }
            }
        }
    };
    // if ident == "" {
    //    println!("{generated}");
    // }
    generated.into()
}
