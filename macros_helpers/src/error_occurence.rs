use crate::attribute_ident_str::AttributeIdentStr;
use naming::{CodeOccurenceSc, HashMapUcc, WithSerializeDeserializeUcc};
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use std::str::FromStr;

#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy)]
pub enum ErrorOccurenceFieldAttribute {
    EoToStdStringString,
    EoToStdStringStringSerializeDeserialize,
    EoErrorOccurence,
    EoVecToStdStringString,
    EoVecToStdStringStringSerializeDeserialize,
    EoVecErrorOccurence,
    EoHashMapKeyStdStringStringValueToStdStringString,
    EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize,
    EoHashMapKeyStdStringStringValueErrorOccurence,
}
impl FromStr for ErrorOccurenceFieldAttribute {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "eo_to_std_string_string" {
            Ok(Self::EoToStdStringString)
        } else if value == "eo_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoToStdStringStringSerializeDeserialize)
        } else if value == "eo_error_occurence" {
            Ok(Self::EoErrorOccurence)
        } else if value == "eo_vec_to_std_string_string" {
            Ok(Self::EoVecToStdStringString)
        } else if value == "eo_vec_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoVecToStdStringStringSerializeDeserialize)
        } else if value == "eo_vec_error_occurence" {
            Ok(Self::EoVecErrorOccurence)
        } else if value == "eo_hashmap_key_std_string_string_value_to_std_string_string" {
            Ok(Self::EoHashMapKeyStdStringStringValueToStdStringString)
        } else if value
            == "eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize"
        {
            Ok(Self::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize)
        } else if value == "eo_hashmap_key_std_string_string_value_error_occurence" {
            Ok(Self::EoHashMapKeyStdStringStringValueErrorOccurence)
        } else {
            Err(())
        }
    }
}
impl TryFrom<&syn::Field> for ErrorOccurenceFieldAttribute {
    type Error = String;
    fn try_from(syn_field: &syn::Field) -> Result<Self, Self::Error> {
        let mut option_attribute = None;
        for el_adfb232c in &syn_field.attrs {
            if el_adfb232c.path().segments.len() == 1 {
                let first_segment_ident = match el_adfb232c.path().segments.first() {
                    Some(value) => &value.ident,
                    None => {
                        return Err("no first value in punctuated".to_owned());
                    }
                };
                if let Ok(value) = FromStr::from_str(&first_segment_ident.to_string()) {
                    if option_attribute.is_some() {
                        return Err("two or more supported attributes!".to_owned());
                    }
                    option_attribute = Some(value);
                }
            } //other attributes are not for this proc_macro
        }
        option_attribute.map_or_else(|| Err("option attribute is None".to_owned()), Ok)
    }
}
impl AttributeIdentStr for ErrorOccurenceFieldAttribute {
    fn attribute_ident_str(&self) -> &str {
        match *self {
            Self::EoToStdStringString => "eo_to_std_string_string",
            Self::EoToStdStringStringSerializeDeserialize => {
                "eo_to_std_string_string_serialize_deserialize"
            }
            Self::EoErrorOccurence => "eo_error_occurence",
            Self::EoVecToStdStringString => "eo_vec_to_std_string_string",
            Self::EoVecToStdStringStringSerializeDeserialize => {
                "eo_vec_to_std_string_string_serialize_deserialize"
            }
            Self::EoVecErrorOccurence => "eo_vec_error_occurence",
            Self::EoHashMapKeyStdStringStringValueToStdStringString => {
                "eo_hashmap_key_std_string_string_value_to_std_string_string"
            }
            Self::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                "eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize"
            }
            Self::EoHashMapKeyStdStringStringValueErrorOccurence => {
                "eo_hashmap_key_std_string_string_value_error_occurence"
            }
        }
    }
}
impl ErrorOccurenceFieldAttribute {
    #[must_use]
    pub fn to_attribute_view_ts(&self) -> Ts2 {
        let value = format!("#[{}]", AttributeIdentStr::attribute_ident_str(self));
        value
            .parse::<Ts2>()
            .expect("147c39e9-4e64-4080-9426-f66520a414d6")
    }
}

#[must_use]
pub fn gen_serialize_deserialize_version_of_named_syn_variant(value: &syn::Variant) -> Ts2 {
    let el_ident = &value.ident;
    let fields = if let syn::Fields::Named(fields) = &value.fields {
        &fields.named
    } else {
        panic!("79b0f231-02b9-4770-8052-5f6cc3debf97");
    };
    let std_string_string = token_patterns::StdStringString;
    let code_occurence_sc = CodeOccurenceSc;
    let fields_idents_idents_with_serialize_deserialize_excluding_code_occurence_ts = fields.iter()
    .filter(|el_5782b638| *el_5782b638.ident.as_ref().expect("3078fd99-5fac-4d57-83ec-93f808b7444b") != *code_occurence_sc.to_string())
    .map(|el_c25b655e| {
        fn get_type_path_third_segment_second_argument_check_if_hashmap(
            value: &syn::Field,
            std_string_string: token_patterns::StdStringString
        ) -> &syn::GenericArgument {
            let segments = if let syn::Type::Path(syn_type_path) = &value.ty {
                &syn_type_path.path.segments
            } else {
                panic!("55136128-fe0b-4599-978d-8577ae049c98");
            };
            assert!(segments.len() == 1, "114c28f3-61af-4602-bc2f-c3688050e7cd");
            let first_segment = segments.iter().next().expect("a037b0ba-efa7-42ea-b024-fb446c16ebc1");
            {
                let hashmap_ucc = HashMapUcc;
                assert!(first_segment.ident == hashmap_ucc.to_string(), "5e1bc6b1-d997-489a-b903-cfc7f8618fe9");
            };
            let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &first_segment.arguments else {
                panic!("f464b7a1-e00e-4d99-8ca3-3fdc93be3d26");
            };
            assert!(args.len() == 2, "47cde1b8-93d7-4945-935d-6cb710bb2b0b");
            let first_argument_str = {
                let first_argument = args.iter().next().expect("f9d97146-c9ba-48f6-9f80-3540f7f7aa60");
                quote! {#first_argument}.to_string()
            };
            assert!(quote! {#std_string_string}.to_string() == first_argument_str, "bbdda4ab-809d-45c7-92f4-245f23318458");
            args.iter().nth(1).expect("f4e88416-5417-405a-9c0d-6035f815bbdd")
        }
        let el_c25b655e_ident = el_c25b655e.ident.as_ref().expect("438aa90e-d1f3-4b89-a61a-e2d9f6a7e653");
        let el_type_ts = {
            let el_type = &el_c25b655e.ty;
            quote! {#el_type}
        };
        let el_type_with_serialize_deserialize_ts = match ErrorOccurenceFieldAttribute::try_from(el_c25b655e).expect("2db209a8-2f57-4474-a9c6-9743aaaed57d") {
            ErrorOccurenceFieldAttribute::EoToStdStringString => {
                quote! {
                    #std_string_string
                }
            }
            ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize | ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => el_type_ts,
            ErrorOccurenceFieldAttribute::EoErrorOccurence => format!(
                "{el_type_ts}{WithSerializeDeserializeUcc}"
            ).parse::<Ts2>().expect("201dc0a4-4563-4e51-a228-ba085b767775"),
            ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
                quote! {
                    Vec<#std_string_string>
                }
            }
            ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                let segments = if let syn::Type::Path(path_value) = &el_c25b655e.ty {
                    &path_value.path.segments
                } else {
                    panic!("8d93bf20-5034-4dcb-9dcc-0d7056278dae");
                };
                assert!(segments.len() == 1, "0c65bbaa-59bd-4abd-9967-bea844983ee0");
                let first_segment = segments.iter().next().expect("595050cf-f859-49c8-b57c-35c322c25da8");
                let el_vec_type_with_serialize_deserialize_ts = if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &first_segment.arguments {
                    assert!(args.len() == 1, "572a9da8-209f-42d0-9fac-da7e42e3829d");
                    format!(
                        "{}{}",
                        {
                            let first_arg = args.iter().next().expect("e9b33787-870e-4520-a364-816c0f47f508");
                            quote! {#first_arg}
                        },
                        WithSerializeDeserializeUcc,
                    ).parse::<Ts2>().expect("22c364b9-c645-46ec-984e-cf0b911feb84")
                } else {
                    panic!("07c6ab44-5e5e-4fca-96a8-5786fb2d2f48");
                };
                quote! {
                    Vec<#el_vec_type_with_serialize_deserialize_ts>
                }
            }
            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => {
                let _: &syn::GenericArgument = get_type_path_third_segment_second_argument_check_if_hashmap(el_c25b655e, std_string_string);
                quote! {
                    std::collections::HashMap<#std_string_string, #std_string_string>
                }
            }
            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                let _: &syn::GenericArgument = get_type_path_third_segment_second_argument_check_if_hashmap(el_c25b655e, std_string_string);
                el_type_ts
            }
            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                let second_argument = get_type_path_third_segment_second_argument_check_if_hashmap(el_c25b655e, std_string_string);
                let el_hashmap_value_type_with_serialize_deserialize_ts = format!(
                    "{}{}",
                    quote! {#second_argument},
                    WithSerializeDeserializeUcc
                ).parse::<Ts2>().expect("86307dbc-484e-4012-ac70-2d593b1f99e6");
                quote! {
                    std::collections::HashMap<#std_string_string, #el_hashmap_value_type_with_serialize_deserialize_ts>
                }
            }
        };
        quote! {#el_c25b655e_ident: #el_type_with_serialize_deserialize_ts,}
    });
    quote! {
        #el_ident {
            #(#fields_idents_idents_with_serialize_deserialize_excluding_code_occurence_ts)*
            #code_occurence_sc: error_occurence_lib::code_occurence::CodeOccurence,
        }
    }
}
