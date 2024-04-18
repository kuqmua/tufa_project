#![deny(
    clippy::indexing_slicing,
    // clippy::arithmetic_side_effects,//detects inside quote!{}
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

//todo maybe reexport from crates
///you need to install this crates to use this macro(check on work was only with this versions)
/// strum = {version = "=0.24.1"}
/// strum_macros = {version = "=0.24.3"}
/// convert_case = {version = "=0.6.0"}
#[proc_macro_derive(EnumExtension)]
pub fn enum_extension(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    //it only supported for enums without values
    let ast: syn::DeriveInput = syn::parse(input).expect("EnumExtension syn::parse(input) failed");
    //todo to implement into_array() and into_vec - must implement Default for all inner variant types
    let len = match ast.data.clone() {
        syn::Data::Enum(enum_item) => enum_item.variants.len(),
        _ => panic!("EnumVariantCount only works on Enums"),
    };
    let variants = match ast.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| {
            let variant_ident = v.ident;
            match v.fields {
                syn::Fields::Named(fields_named) => {
                    let generated = fields_named.named.into_iter().map(|field| {
                        let field_ident = field.ident; //todo maybe unwrap_or_else panic?
                        quote::quote! { #field_ident: Default::default() }
                    });
                    quote::quote! {
                       #variant_ident {
                           #(#generated),*
                       }
                    }
                }
                syn::Fields::Unnamed(_) => quote::quote! { #variant_ident(Default::default()) },
                syn::Fields::Unit => quote::quote! { #variant_ident },
            }
        }),
        _ => panic!("EnumIntoArray works only on enums"),
    };
    let ident = &ast.ident;
    let gen = quote::quote! {
        impl #ident {
            pub fn get_length() -> std::primitive::usize {
                #len
            }
            pub fn into_array() -> [#ident; #len] {
                [ #(#ident::#variants),* ]
            }
            pub fn into_vec() -> std::vec::Vec<Self> {
                let mut self_vec = std::vec::Vec::with_capacity(Self::get_length());
                for element in {
                    use enum_extension_lib::IntoEnumIterator;
                    Self::iter()
                } {
                    self_vec.push(element);
                }
                self_vec
            }
            pub fn into_string_name_and_variant_hashmap() -> std::collections::HashMap<std::string::String, Self> {
                let mut variants_hashmap: std::collections::HashMap<std::string::String, Self> =
                    std::collections::HashMap::with_capacity(Self::get_length());
                for element in {
                    use enum_extension_lib::IntoEnumIterator;
                    Self::iter()
                } {
                    variants_hashmap.insert(format!("{}", element), element);
                }
                variants_hashmap
            }
            pub fn into_string_name_and_variant_tuple_vec() -> std::vec::Vec<(std::string::String, Self)> {
                let mut variants_vec = std::vec::Vec::with_capacity(Self::get_length());
                for element in {
                    use enum_extension_lib::IntoEnumIterator;
                    Self::iter()
                } {
                    variants_vec.push((format!("{}", element), element));
                }
                variants_vec
            }
            //todo - it can be done in compile time
            pub fn to_upper_camel_case(&self) -> std::string::String {
                enum_extension_lib::Casing::to_case(&format!("{:?}", self),enum_extension_lib::Case::UpperCamel)
            }
            //todo - it can be done in compile time
            pub fn to_snake_case(&self) -> std::string::String {
                enum_extension_lib::Casing::to_case(&format!("{:?}", self),enum_extension_lib::Case::Snake)
            }
        }
    };
    // if name == "" {
    //     println!("{gen}");
    // }
    gen.into()
}
