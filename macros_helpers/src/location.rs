use crate::attr_ident_str::AttrIdentStr;
use naming::{HashMapUcc, LocSc, WithSerdeUcc};
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use std::str::FromStr;
use syn::{
    AngleBracketedGenericArguments, Field, Fields, GenericArgument, PathArguments, Type, Variant,
};
use token_patterns::StringTs;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy)]
pub enum LocationFieldAttr {
    EoToErrString,
    EoToErrStringSerde,
    EoLocation,
    EoVecToErrString,
    EoVecToErrStringSerde,
    EoVecLocation,
    EoHashMapKStringValueToErrString,
    EoHashMapKStringValueToErrStringSerde,
    EoHashMapKStringValueLocation,
}
impl FromStr for LocationFieldAttr {
    type Err = ();
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        if v == "eo_to_err_string" {
            Ok(Self::EoToErrString)
        } else if v == "eo_to_err_string_serde" {
            Ok(Self::EoToErrStringSerde)
        } else if v == "eo_location" {
            Ok(Self::EoLocation)
        } else if v == "eo_vec_to_err_string" {
            Ok(Self::EoVecToErrString)
        } else if v == "eo_vec_to_err_string_serde" {
            Ok(Self::EoVecToErrStringSerde)
        } else if v == "eo_vec_location" {
            Ok(Self::EoVecLocation)
        } else if v == "eo_hashmap_k_string_v_to_err_string" {
            Ok(Self::EoHashMapKStringValueToErrString)
        } else if v == "eo_hashmap_k_string_v_to_err_string_serde" {
            Ok(Self::EoHashMapKStringValueToErrStringSerde)
        } else if v == "eo_hashmap_k_string_v_location" {
            Ok(Self::EoHashMapKStringValueLocation)
        } else {
            Err(())
        }
    }
}
impl TryFrom<&Field> for LocationFieldAttr {
    type Error = String;
    fn try_from(syn_field: &Field) -> Result<Self, Self::Error> {
        let mut opt_attr = None;
        for el in &syn_field.attrs {
            if el.path().segments.len() == 1 {
                let first_segment_ident = match el.path().segments.first() {
                    Some(v) => &v.ident,
                    None => {
                        return Err("no first in punct".to_owned());
                    }
                };
                if let Ok(v) = FromStr::from_str(&first_segment_ident.to_string()) {
                    if opt_attr.is_some() {
                        return Err("two or more supported attrs!".to_owned());
                    }
                    opt_attr = Some(v);
                }
            } //other attrs are not for this proc_macro
        }
        opt_attr.map_or_else(|| Err("opt attr is None".to_owned()), Ok)
    }
}
impl AttrIdentStr for LocationFieldAttr {
    fn attr_ident_str(&self) -> &str {
        match *self {
            Self::EoToErrString => "eo_to_err_string",
            Self::EoToErrStringSerde => "eo_to_err_string_serde",
            Self::EoLocation => "eo_location",
            Self::EoVecToErrString => "eo_vec_to_err_string",
            Self::EoVecToErrStringSerde => "eo_vec_to_err_string_serde",
            Self::EoVecLocation => "eo_vec_location",
            Self::EoHashMapKStringValueToErrString => "eo_hashmap_k_string_v_to_err_string",
            Self::EoHashMapKStringValueToErrStringSerde => {
                "eo_hashmap_k_string_v_to_err_string_serde"
            }
            Self::EoHashMapKStringValueLocation => "eo_hashmap_k_string_v_location",
        }
    }
}
impl LocationFieldAttr {
    #[must_use]
    pub fn to_attr_view_ts(&self) -> Ts2 {
        format!("#[{}]", AttrIdentStr::attr_ident_str(self))
            .parse::<Ts2>()
            .expect("147c39e9")
    }
}
#[must_use]
pub fn gen_serde_version_of_named_syn_vrt(v: &Variant) -> Ts2 {
    let el_ident = &v.ident;
    let fields = if let Fields::Named(fields) = &v.fields {
        &fields.named
    } else {
        panic!("79b0f231");
    };
    let fields_idents_idents_with_serde_excluding_loc_ts = fields
        .iter()
        .filter(|el| *el.ident.as_ref().expect("3078fd99") != *LocSc.to_string())
        .map(|el| {
            let get_type_path_third_segment_second_argument_check_if_hashmap = || {
                let segments = if let Type::Path(syn_type_path) = &el.ty {
                    &syn_type_path.path.segments
                } else {
                    panic!("55136128");
                };
                assert!(segments.len() == 1, "114c28f3");
                let first_segment = segments.iter().next().expect("a037b0ba");
                {
                    assert!(first_segment.ident == HashMapUcc.to_string(), "5e1bc6b1");
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
            let el_c25b655e_ident = el.ident.as_ref().expect("438aa90e");
            let el_type_ts = {
                let el_type = &el.ty;
                quote! {#el_type}
            };
            let el_type_with_serde_ts = match LocationFieldAttr::try_from(el).expect("2db209a8") {
                LocationFieldAttr::EoToErrString => quote! {#StringTs},
                LocationFieldAttr::EoToErrStringSerde
                | LocationFieldAttr::EoVecToErrStringSerde => el_type_ts,
                LocationFieldAttr::EoLocation => format!("{el_type_ts}{WithSerdeUcc}")
                    .parse::<Ts2>()
                    .expect("201dc0a4"),
                LocationFieldAttr::EoVecToErrString => {
                    quote! {
                        Vec<#StringTs>
                    }
                }
                LocationFieldAttr::EoVecLocation => {
                    let segments = if let Type::Path(v0) = &el.ty {
                        &v0.path.segments
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
                LocationFieldAttr::EoHashMapKStringValueToErrString => {
                    let _: &GenericArgument =
                        get_type_path_third_segment_second_argument_check_if_hashmap();
                    quote! {
                        std::collections::HashMap<#StringTs, #StringTs>
                    }
                }
                LocationFieldAttr::EoHashMapKStringValueToErrStringSerde => {
                    let _: &GenericArgument =
                        get_type_path_third_segment_second_argument_check_if_hashmap();
                    el_type_ts
                }
                LocationFieldAttr::EoHashMapKStringValueLocation => {
                    let second_argument =
                        get_type_path_third_segment_second_argument_check_if_hashmap();
                    let el_hashmap_v_type_with_serde_ts =
                        format!("{}{}", quote! {#second_argument}, WithSerdeUcc)
                            .parse::<Ts2>()
                            .expect("86307dbc");
                    quote! {
                        std::collections::HashMap<#StringTs, #el_hashmap_v_type_with_serde_ts>
                    }
                }
            };
            quote! {#el_c25b655e_ident: #el_type_with_serde_ts,}
        });
    quote! {
        #el_ident {
            #(#fields_idents_idents_with_serde_excluding_loc_ts)*
            #LocSc: location_lib::loc::Loc,
        }
    }
}
