use crate::attr_ident_str::AttrIdentStr;
use naming::{CodeOccurenceSc, HashMapUcc, WithSerdeUcc};
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use std::str::FromStr;
use syn::{
    AngleBracketedGenericArguments, Field, Fields, GenericArgument, PathArguments, Type, Variant,
};
use token_patterns::StringTs;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy)]
pub enum ErrorOccurenceFieldAttr {
    EoToErrString,
    EoToErrStringSerde,
    EoErrorOccurence,
    EoVecToErrString,
    EoVecToErrStringSerde,
    EoVecErrorOccurence,
    EoHashMapKeyStringValueToErrString,
    EoHashMapKeyStringValueToErrStringSerde,
    EoHashMapKeyStringValueErrorOccurence,
}
impl FromStr for ErrorOccurenceFieldAttr {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "eo_to_err_string" {
            Ok(Self::EoToErrString)
        } else if value == "eo_to_err_string_serde" {
            Ok(Self::EoToErrStringSerde)
        } else if value == "eo_error_occurence" {
            Ok(Self::EoErrorOccurence)
        } else if value == "eo_vec_to_err_string" {
            Ok(Self::EoVecToErrString)
        } else if value == "eo_vec_to_err_string_serde" {
            Ok(Self::EoVecToErrStringSerde)
        } else if value == "eo_vec_error_occurence" {
            Ok(Self::EoVecErrorOccurence)
        } else if value == "eo_hashmap_key_string_value_to_err_string" {
            Ok(Self::EoHashMapKeyStringValueToErrString)
        } else if value == "eo_hashmap_key_string_value_to_err_string_serde" {
            Ok(Self::EoHashMapKeyStringValueToErrStringSerde)
        } else if value == "eo_hashmap_key_string_value_error_occurence" {
            Ok(Self::EoHashMapKeyStringValueErrorOccurence)
        } else {
            Err(())
        }
    }
}
impl TryFrom<&Field> for ErrorOccurenceFieldAttr {
    type Error = String;
    fn try_from(syn_field: &Field) -> Result<Self, Self::Error> {
        let mut option_attr = None;
        for el_adfb232c in &syn_field.attrs {
            if el_adfb232c.path().segments.len() == 1 {
                let first_segment_ident = match el_adfb232c.path().segments.first() {
                    Some(value) => &value.ident,
                    None => {
                        return Err("no first value in punctuated".to_owned());
                    }
                };
                if let Ok(value) = FromStr::from_str(&first_segment_ident.to_string()) {
                    if option_attr.is_some() {
                        return Err("two or more supported attrs!".to_owned());
                    }
                    option_attr = Some(value);
                }
            } //other attrs are not for this proc_macro
        }
        option_attr.map_or_else(|| Err("option attr is None".to_owned()), Ok)
    }
}
impl AttrIdentStr for ErrorOccurenceFieldAttr {
    fn attr_ident_str(&self) -> &str {
        match *self {
            Self::EoToErrString => "eo_to_err_string",
            Self::EoToErrStringSerde => "eo_to_err_string_serde",
            Self::EoErrorOccurence => "eo_error_occurence",
            Self::EoVecToErrString => "eo_vec_to_err_string",
            Self::EoVecToErrStringSerde => "eo_vec_to_err_string_serde",
            Self::EoVecErrorOccurence => "eo_vec_error_occurence",
            Self::EoHashMapKeyStringValueToErrString => "eo_hashmap_key_string_value_to_err_string",
            Self::EoHashMapKeyStringValueToErrStringSerde => {
                "eo_hashmap_key_string_value_to_err_string_serde"
            }
            Self::EoHashMapKeyStringValueErrorOccurence => {
                "eo_hashmap_key_string_value_error_occurence"
            }
        }
    }
}
impl ErrorOccurenceFieldAttr {
    #[must_use]
    pub fn to_attr_view_ts(&self) -> Ts2 {
        let value = format!("#[{}]", AttrIdentStr::attr_ident_str(self));
        value.parse::<Ts2>().expect("147c39e9")
    }
}

#[must_use]
pub fn gen_serde_version_of_named_syn_variant(value: &Variant) -> Ts2 {
    let el_ident = &value.ident;
    let fields = if let Fields::Named(fields) = &value.fields {
        &fields.named
    } else {
        panic!("79b0f231");
    };
    let code_occurence_sc = CodeOccurenceSc;
    let fields_idents_idents_with_serde_excluding_code_occurence_ts = fields
        .iter()
        .filter(|el_5782b638| {
            *el_5782b638.ident.as_ref().expect("3078fd99") != *code_occurence_sc.to_string()
        })
        .map(|el_c25b655e| {
            let get_type_path_third_segment_second_argument_check_if_hashmap = || {
                let segments = if let Type::Path(syn_type_path) = &el_c25b655e.ty {
                    &syn_type_path.path.segments
                } else {
                    panic!("55136128");
                };
                assert!(segments.len() == 1, "114c28f3");
                let first_segment = segments.iter().next().expect("a037b0ba");
                {
                    let hashmap_ucc = HashMapUcc;
                    assert!(first_segment.ident == hashmap_ucc.to_string(), "5e1bc6b1");
                };
                let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
                    &first_segment.arguments
                else {
                    panic!("f464b7a1");
                };
                assert!(args.len() == 2, "47cde1b8");
                let first_argument_str = {
                    let first_argument = args.iter().next().expect("f9d97146");
                    quote! {#first_argument}.to_string()
                };
                assert!(
                    quote! {#StringTs}.to_string() == first_argument_str,
                    "bbdda4ab"
                );
                args.iter().nth(1).expect("f4e88416")
            };
            let el_c25b655e_ident = el_c25b655e.ident.as_ref().expect("438aa90e");
            let el_type_ts = {
                let el_type = &el_c25b655e.ty;
                quote! {#el_type}
            };
            let el_type_with_serde_ts = match ErrorOccurenceFieldAttr::try_from(el_c25b655e)
                .expect("2db209a8")
            {
                ErrorOccurenceFieldAttr::EoToErrString => quote! {#StringTs},
                ErrorOccurenceFieldAttr::EoToErrStringSerde
                | ErrorOccurenceFieldAttr::EoVecToErrStringSerde => el_type_ts,
                ErrorOccurenceFieldAttr::EoErrorOccurence => format!("{el_type_ts}{WithSerdeUcc}")
                    .parse::<Ts2>()
                    .expect("201dc0a4"),
                ErrorOccurenceFieldAttr::EoVecToErrString => {
                    quote! {
                        Vec<#StringTs>
                    }
                }
                ErrorOccurenceFieldAttr::EoVecErrorOccurence => {
                    let segments = if let Type::Path(path_value) = &el_c25b655e.ty {
                        &path_value.path.segments
                    } else {
                        panic!("8d93bf20");
                    };
                    assert!(segments.len() == 1, "0c65bbaa");
                    let first_segment = segments.iter().next().expect("595050cf");
                    let el_vec_type_with_serde_ts =
                        if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            args,
                            ..
                        }) = &first_segment.arguments
                        {
                            assert!(args.len() == 1, "572a9da8");
                            format!(
                                "{}{}",
                                {
                                    let first_arg = args.iter().next().expect("e9b33787");
                                    quote! {#first_arg}
                                },
                                WithSerdeUcc,
                            )
                            .parse::<Ts2>()
                            .expect("22c364b9")
                        } else {
                            panic!("07c6ab44");
                        };
                    quote! {
                        Vec<#el_vec_type_with_serde_ts>
                    }
                }
                ErrorOccurenceFieldAttr::EoHashMapKeyStringValueToErrString => {
                    let _: &GenericArgument =
                        get_type_path_third_segment_second_argument_check_if_hashmap();
                    quote! {
                        std::collections::HashMap<#StringTs, #StringTs>
                    }
                }
                ErrorOccurenceFieldAttr::EoHashMapKeyStringValueToErrStringSerde => {
                    let _: &GenericArgument =
                        get_type_path_third_segment_second_argument_check_if_hashmap();
                    el_type_ts
                }
                ErrorOccurenceFieldAttr::EoHashMapKeyStringValueErrorOccurence => {
                    let second_argument =
                        get_type_path_third_segment_second_argument_check_if_hashmap();
                    let el_hashmap_value_type_with_serde_ts =
                        format!("{}{}", quote! {#second_argument}, WithSerdeUcc)
                            .parse::<Ts2>()
                            .expect("86307dbc");
                    quote! {
                        std::collections::HashMap<#StringTs, #el_hashmap_value_type_with_serde_ts>
                    }
                }
            };
            quote! {#el_c25b655e_ident: #el_type_with_serde_ts,}
        });
    quote! {
        #el_ident {
            #(#fields_idents_idents_with_serde_excluding_code_occurence_ts)*
            #code_occurence_sc: error_occurence_lib::code_occurence::CodeOccurence,
        }
    }
}
