pub mod form_last_arg_lifetime_vec;
pub mod generate_path_from_segments;
pub mod hashmap_key_type;
pub mod hashmap_value_type;
pub mod lifetime;
pub mod panic_if_not_str;
pub mod panic_if_not_string;
pub mod possible_lifetime_addition;
pub mod supported_enum_variant;
pub mod vec_element_type;
pub mod vec_lifetime_to_string;

#[allow(clippy::enum_variant_names)]
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
impl std::str::FromStr for ErrorOccurenceFieldAttribute {
    type Err = ();
    fn from_str(value: &std::primitive::str) -> Result<Self, Self::Err> {
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
        } else if value == "eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize)
        } else if value == "eo_hashmap_key_std_string_string_value_error_occurence" {
            Ok(Self::EoHashMapKeyStdStringStringValueErrorOccurence)
        } else {
            Err(())
        }
    }
}
impl std::convert::From<&syn::Field> for ErrorOccurenceFieldAttribute {
    fn from(value: &syn::Field) -> Self {
        let mut option_attribute = None;
        for attr in &value.attrs {
            if attr.path().segments.len() == 1 {
                let first_segment_ident = &attr.path().segments.first().expect("no first value in punctuated").ident;
                if let Ok(value) = {
                    use std::str::FromStr;
                    ErrorOccurenceFieldAttribute::from_str(&first_segment_ident.to_string())
                } {
                    if option_attribute.is_some() {
                        panic!("two or more supported attributes!");
                    } else {
                        option_attribute = Some(value);
                    }
                }
            } //other attributes are not for this proc_macro
        }
        option_attribute.unwrap_or_else(|| panic!("option attribute {}", naming::IS_NONE_STRINGIFIED))
    }
}
impl crate::attribute_ident_stringified::AttributeIdentStringified for ErrorOccurenceFieldAttribute {
    fn attribute_ident_stringified(&self) -> &str {
        match self {
            Self::EoToStdStringString => "eo_to_std_string_string",
            Self::EoToStdStringStringSerializeDeserialize => "eo_to_std_string_string_serialize_deserialize",
            Self::EoErrorOccurence => "eo_error_occurence",
            Self::EoVecToStdStringString => "eo_vec_to_std_string_string",
            Self::EoVecToStdStringStringSerializeDeserialize => "eo_vec_to_std_string_string_serialize_deserialize",
            Self::EoVecErrorOccurence => "eo_vec_error_occurence",
            Self::EoHashMapKeyStdStringStringValueToStdStringString => "eo_hashmap_key_std_string_string_value_to_std_string_string",
            Self::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => "eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize",
            Self::EoHashMapKeyStdStringStringValueErrorOccurence => "eo_hashmap_key_std_string_string_value_error_occurence",
        }
    }
}
impl ErrorOccurenceFieldAttribute {
    pub fn to_attribute_view_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!("#[{}]", crate::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(self));
        value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}
pub fn attribute_view(attribute: &str) -> std::string::String {
    format!("attribute #[{attribute}]")
}

pub fn get_type_path_third_segment_second_argument_check_if_hashmap<'a>(value: &'a syn::Field, std_snake_case: &naming::StdSnakeCase, std_string_string: &token_patterns::StdStringString) -> &'a syn::GenericArgument {
    let segments = if let syn::Type::Path(value) = &value.ty {
        &value.path.segments
    } else {
        panic!("Type::Path(value) != &element.ty");
    };
    assert!(segments.len() == 3, "segments.len() != 3");
    let first_segment = segments.iter().next().expect("no .next()) element");
    assert!(first_segment.ident == std_snake_case.to_string(), "first_segment.ident != {std_snake_case} {}", first_segment.ident);
    match first_segment.arguments {
        syn::PathArguments::None => (),
        _ => panic!("first_segment.arguments != PathArguments::None"),
    }
    let second_segment = segments.iter().nth(1).expect("no .nth(1) element");
    {
        let collections_snake_case = naming::CollectionsSnakeCase;
        assert!(second_segment.ident == collections_snake_case.to_string(), "second_segment.ident != {collections_snake_case} {}", second_segment.ident);
    };
    match second_segment.arguments {
        syn::PathArguments::None => (),
        _ => panic!("second_segment.arguments != PathArguments::None"),
    }
    let third_segment = segments.iter().nth(2).expect("no .nth(2) element");
    {
        let hashmap_upper_camel_case = naming::HashMapUpperCamelCase;
        assert!(third_segment.ident == hashmap_upper_camel_case.to_string(), "third_segment.ident != {hashmap_upper_camel_case} {}", third_segment.ident);
    };
    let args = if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &third_segment.arguments {
        args
    } else {
        panic!("third_segment.arguments != syn::PathArguments::AngleBracketed");
    };
    assert!(args.len() == 2, "args.len() != 2");
    let first_argument_stringified = {
        let first_argument = args.iter().next().expect("args.iter().next() is None");
        quote::quote! {#first_argument}.to_string()
    };
    assert!(quote::quote! {#std_string_string}.to_string() == first_argument_stringified, "{} != {first_argument_stringified}", quote::quote! {#std_string_string});
    args.iter().nth(1).expect("args.iter().nth(1) is None")
}

pub fn generate_serialize_deserialize_version_of_named_syn_variant(value: &syn::Variant) -> proc_macro2::TokenStream {
    let element_ident = &value.ident;
    let fields = if let syn::Fields::Named(fields) = &value.fields {
        &fields.named
    } else {
        panic!("{} syn::Data::Enum", naming::SUPPORTS_ONLY_STRINGIFIED);
    };
    let std_string_string = token_patterns::StdStringString;
    let fields_idents_idents_with_serialize_deserialize_excluding_code_occurence_token_stream = fields.iter().filter(|element| *element.ident.as_ref().expect(constants::IDENT_IS_NONE) != *naming::CodeOccurenceSnakeCase.to_string()).map(|element| {
        let element_ident = element.ident.as_ref().expect(constants::IDENT_IS_NONE);
        let element_type_token_stream = {
            let element_type = &element.ty;
            quote::quote! {#element_type}
        };
        let std_snake_case = naming::StdSnakeCase;
        let element_type_with_serialize_deserialize_token_stream = match crate::error_occurence::ErrorOccurenceFieldAttribute::from(element) {
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString => {
                quote::quote! {
                    #std_string_string
                }
            }
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize | crate::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => element_type_token_stream,
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                let element_type_with_serialize_deserialize_token_stream = {
                    let value = format!(
                        "{}{}",
                        {
                            let element_type = &element.ty;
                            quote::quote! {#element_type}
                        },
                        naming::WithSerializeDeserializeUpperCamelCase
                    );
                    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    #element_type_with_serialize_deserialize_token_stream
                }
            }
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
                quote::quote! {
                    std::vec::Vec<#std_string_string>
                }
            }
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                let segments = if let syn::Type::Path(value) = &element.ty {
                    &value.path.segments
                } else {
                    panic!("Type::Path(value) != &element.ty");
                };
                assert!(segments.len() == 3, "segments.len() != 3");
                let first_segment = segments.iter().next().expect("no .next() element");
                assert!(first_segment.ident == std_snake_case.to_string(), "first_segment.ident != {std_snake_case} {}", first_segment.ident);
                match first_segment.arguments {
                    syn::PathArguments::None => (),
                    _ => panic!("first_segment.arguments != PathArguments::None"),
                }
                let second_segment = segments.iter().nth(1).expect("no .nth(1) element");
                {
                    let vec_snake_case = naming::VecSnakeCase;
                    assert!(second_segment.ident == vec_snake_case.to_string(), "second_segment.ident != {vec_snake_case} {}", second_segment.ident);
                };
                match second_segment.arguments {
                    syn::PathArguments::None => (),
                    _ => panic!("second_segment.arguments != PathArguments::None"),
                }
                let third_segment = segments.iter().nth(2).expect("no .nth(2) element");
                {
                    let vec_upper_camel_case = naming::VecUpperCamelCase;
                    assert!(third_segment.ident == vec_upper_camel_case.to_string(), "third_segment.ident != {vec_upper_camel_case} {}", third_segment.ident);
                };
                let element_vec_type_with_serialize_deserialize_token_stream = if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &third_segment.arguments {
                    assert!(args.len() == 1, "args.len() != 1");
                    let value = format!(
                        "{}{}",
                        {
                            let first_arg = args.iter().next().expect("args.iter().next() is None");
                            quote::quote! {#first_arg}
                        },
                        naming::WithSerializeDeserializeUpperCamelCase,
                    );
                    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                } else {
                    panic!("third_segment.arguments != syn::PathArguments::AngleBracketed");
                };
                quote::quote! {
                    std::vec::Vec<#element_vec_type_with_serialize_deserialize_token_stream>
                }
            }
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => {
                let _: &syn::GenericArgument = get_type_path_third_segment_second_argument_check_if_hashmap(element, &std_snake_case, &std_string_string);
                quote::quote! {
                    std::collections::HashMap<#std_string_string, #std_string_string>
                }
            }
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                let _: &syn::GenericArgument = get_type_path_third_segment_second_argument_check_if_hashmap(element, &std_snake_case, &std_string_string);
                element_type_token_stream
            }
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                let second_argument = get_type_path_third_segment_second_argument_check_if_hashmap(element, &std_snake_case, &std_string_string);
                let element_hashmap_value_type_with_serialize_deserialize_token_stream = {
                    let value = format!("{}{}", quote::quote! {#second_argument}, naming::WithSerializeDeserializeUpperCamelCase,);
                    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    std::collections::HashMap<#std_string_string, #element_hashmap_value_type_with_serialize_deserialize_token_stream>
                }
            }
        };
        quote::quote! {
            #element_ident: #element_type_with_serialize_deserialize_token_stream,
        }
    });
    let code_occurence_snake_case_token_stream = naming::CodeOccurenceSnakeCase;
    quote::quote! {
        #element_ident {
            #(#fields_idents_idents_with_serialize_deserialize_excluding_code_occurence_token_stream)*
            #code_occurence_snake_case_token_stream: error_occurence_lib::code_occurence::CodeOccurence,
        }
    }
}
