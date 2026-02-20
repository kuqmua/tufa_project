use convert_case::{Case, Casing};
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, parse};
#[proc_macro_derive(FromStr)]
pub fn from_str(input: Ts) -> Ts {
    panic_location::panic_location();
    let di: DeriveInput = parse(input).expect("f83fcd2d");
    let ident = &di.ident;
    let Data::Enum(data_enum) = di.data else {
        panic!("d35db256");
    };
    let vrt_idents = data_enum
        .variants
        .into_iter()
        .map(|vrt| match vrt.fields {
            Fields::Unit => vrt.ident,
            Fields::Named(_) | Fields::Unnamed(_) => {
                panic!("23575b02")
            }
        })
        .collect::<Vec<Ident>>();
    let vrts_ts = vrt_idents.iter().map(|vrt_ident| {
        let vrt_ident_sc_ts = {
            let vrt_ident_sc_str = Casing::to_case(&format!("\"{vrt_ident}\""), Case::Snake);
            vrt_ident_sc_str.parse::<Ts2>().expect("791603c1")
        };
        quote! {
            #vrt_ident_sc_ts => Ok(Self::#vrt_ident),
        }
    });
    let er_vrts_str = vrt_idents
        .iter()
        .fold(String::default(), |mut acc_d6966473, vrt_ident| {
            use std::fmt::Write as _;
            let vrt_ident_sc_str = Casing::to_case(&format!("{vrt_ident}"), Case::Snake);
            assert!(
                write!(acc_d6966473, "\'{vrt_ident_sc_str}\',").is_ok(),
                "09c49558"
            );
            acc_d6966473
        });
    let er_ts = {
        let er_str = format!("\"Invalid {ident}, expected one of {er_vrts_str} found {{v}}\"");
        er_str.parse::<Ts2>().expect("1b778757")
    };
    let generated = quote! {
        impl std::str::FromStr for #ident {
            type Err = String;
            fn from_str(v: &str) -> Result<Self, Self::Err> {
                match v {
                    #(#vrts_ts)*
                    _ => Err(format!(#er_ts)),
                }
            }
        }
    };
    // if ident == "" {
    //    println!("{generated}");
    // }
    generated.into()
}
