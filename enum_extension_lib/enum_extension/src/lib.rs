use proc_macro::TokenStream as Ts;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse};

#[proc_macro_derive(EnumExtension)]
pub fn enum_extension(input: Ts) -> Ts {
    panic_location::panic_location();
    //it only supported for enums without values
    let syn_derive_input: DeriveInput = parse(input).expect("c6b8e80e");
    //todo to implement into_array() and into_vec - must implement Default for all inner variant types
    let len = match syn_derive_input.data.clone() {
        Data::Enum(enum_item) => enum_item.variants.len(),
        Data::Struct(_) | Data::Union(_) => {
            panic!("bcbaca28")
        }
    };
    let variants = match syn_derive_input.data {
        Data::Enum(enum_item) => enum_item.variants.into_iter().map(|el_f0467eb6| {
            let variant_ident = el_f0467eb6.ident;
            match el_f0467eb6.fields {
                Fields::Named(fields_named) => {
                    let generated = fields_named.named.into_iter().map(|field| {
                        let field_ident = field.ident; //todo maybe unwrap_or_else panic?
                        quote! { #field_ident: Default::default() }
                    });
                    quote! {
                       #variant_ident {
                           #(#generated),*
                       }
                    }
                }
                Fields::Unnamed(_) => quote! { #variant_ident(Default::default()) },
                Fields::Unit => quote! { #variant_ident },
            }
        }),
        Data::Struct(_) | Data::Union(_) => {
            panic!("4ba8c781")
        }
    };
    let ident = &syn_derive_input.ident;
    let generated = quote! {
        impl #ident {
            pub fn get_length() -> usize {
                #len
            }
            pub fn into_array() -> [#ident; #len] {
                [ #(#ident::#variants),* ]
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
            pub fn into_string_name_and_variant_hashmap() -> std::collections::HashMap<String, Self> {
                let mut variants_hashmap: std::collections::HashMap<String, Self> =
                    std::collections::HashMap::with_capacity(Self::get_length());
                for el_dfeecd17 in {
                    use enum_extension_lib::IntoEnumIterator;
                    Self::iter()
                } {
                    variants_hashmap.insert(format!("{}", el_dfeecd17), el_dfeecd17);
                }
                variants_hashmap
            }
            pub fn into_string_name_and_variant_tuple_vec() -> Vec<(String, Self)> {
                let mut variants_vec = Vec::with_capacity(Self::get_length());
                for el_538a15df in {
                    use enum_extension_lib::IntoEnumIterator;
                    Self::iter()
                } {
                    variants_vec.push((format!("{}", el_538a15df), el_538a15df));
                }
                variants_vec
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
