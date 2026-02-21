use proc_macro::TokenStream as Ts;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse};
#[proc_macro_derive(EnumExtension)]
pub fn enum_extension(v: Ts) -> Ts {
    panic_location::panic_location();
    //it only supported for enums without values
    let di: DeriveInput = parse(v).expect("c6b8e80e");
    //todo to implement into_arr() and into_vec - must implement Default for all inner vrt types
    let len = match di.data.clone() {
        Data::Enum(enum_item) => enum_item.variants.len(),
        Data::Struct(_) | Data::Union(_) => panic!("bcbaca28"),
    };
    let vrts = match di.data {
        Data::Enum(enum_item) => enum_item.variants.into_iter().map(|el_f0467eb6| {
            let vrt_ident = el_f0467eb6.ident;
            match el_f0467eb6.fields {
                Fields::Named(fields_named) => {
                    let generated = fields_named.named.into_iter().map(|field| {
                        let field_ident = field.ident; //todo maybe unwrap_or_else panic?
                        quote! {#field_ident: Default::default()}
                    });
                    quote! {#vrt_ident { #(#generated),* }}
                }
                Fields::Unnamed(_) => quote! {#vrt_ident(Default::default())},
                Fields::Unit => quote! {#vrt_ident},
            }
        }),
        Data::Struct(_) | Data::Union(_) => panic!("4ba8c781"),
    };
    let ident = &di.ident;
    let generated = quote! {
        impl #ident {
            pub fn get_length() -> usize {
                #len
            }
            pub fn into_arr() -> [#ident; #len] {
                [ #(#ident::#vrts),* ]
            }
            pub fn into_vec() -> Vec<Self> {
                let mut self_vec = Vec::with_capacity(Self::get_length());
                for el_976865e3 in {
                    use enum_extension_lib::IntoEnumIterator;
                    Self::iter()
                } {
                    self_vec.push(el_976865e3);
                }
                self_vec
            }
            pub fn into_string_name_and_vrt_hashmap() -> std::collections::HashMap<String, Self> {
                let mut vrts_hashmap: std::collections::HashMap<String, Self> =
                    std::collections::HashMap::with_capacity(Self::get_length());
                for el_dfeecd17 in {
                    use enum_extension_lib::IntoEnumIterator;
                    Self::iter()
                } {
                    vrts_hashmap.insert(format!("{}", el_dfeecd17), el_dfeecd17);
                }
                vrts_hashmap
            }
            pub fn into_string_name_and_vrt_tuple_vec() -> Vec<(String, Self)> {
                let mut vrts_vec = Vec::with_capacity(Self::get_length());
                for el_538a15df in {
                    use enum_extension_lib::IntoEnumIterator;
                    Self::iter()
                } {
                    vrts_vec.push((format!("{}", el_538a15df), el_538a15df));
                }
                vrts_vec
            }
            //todo - it can be done in compile time
            pub fn to_ucc(&self) -> String {
                enum_extension_lib::Casing::to_case(&format!("{:?}", self),enum_extension_lib::Case::UpperCamel)
            }
            //todo - it can be done in compile time
            pub fn to_sc(&self) -> String {
                enum_extension_lib::Casing::to_case(&format!("{:?}", self),enum_extension_lib::Case::Snake)
            }
        }
    };
    // if name == "" {
    //     println!("{generated}");
    // }
    generated.into()
}
