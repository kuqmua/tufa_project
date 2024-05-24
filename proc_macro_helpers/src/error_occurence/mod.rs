pub mod error_field_or_code_occurence;
pub mod form_last_arg_lifetime_vec;
pub mod generate_path_from_segments;
pub mod generate_with_serialize_deserialize_version;
pub mod hashmap_key_type;
pub mod hashmap_value_type;
pub mod lifetime;
pub mod panic_if_not_str;
pub mod panic_if_not_string;
pub mod possible_lifetime_addition;
pub mod supported_container;
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
        }
        else if value == "eo_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoToStdStringStringSerializeDeserialize)
        }
        else if value == "eo_error_occurence" {
            Ok(Self::EoErrorOccurence)
        }
        else if value == "eo_vec_to_std_string_string" {
            Ok(Self::EoVecToStdStringString)
        }
        else if value == "eo_vec_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoVecToStdStringStringSerializeDeserialize)
        }
        else if value == "eo_vec_error_occurence" {
            Ok(Self::EoVecErrorOccurence)
        }
        else if value == "eo_hashmap_key_std_string_string_value_to_std_string_string" {
            Ok(Self::EoHashMapKeyStdStringStringValueToStdStringString)
        }
        else if value == "eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize)
        }
        else if value == "eo_hashmap_key_std_string_string_value_error_occurence" {
            Ok(Self::EoHashMapKeyStdStringStringValueErrorOccurence)
        }
        else {
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
                    }
                    else {
                        option_attribute = Some(value);
                    }
                }
            }//other attributes are not for this proc_macro
        };
        option_attribute.unwrap_or_else(|| panic!(
            "option attribute {}",
            naming_constants::IS_NONE_STRINGIFIED
        ))
    }
}
impl proc_macro_common::attribute_ident_stringified::AttributeIdentStringified for ErrorOccurenceFieldAttribute {
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
        let value = format!("#[{}]", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(self));
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}
pub fn attribute_view(attribute: &str) -> std::string::String {
    format!("attribute #[{attribute}]")
}

pub fn get_type_path_third_segment_second_argument_check_if_hashmap<'a>(
    value: &'a syn::Field,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
    std_snake_case_stringified: &std::primitive::str,
    std_string_string_token_stream: &proc_macro2::TokenStream,
) -> &'a syn::GenericArgument {
    let segments = if let syn::Type::Path(value) = &value.ty {
        &value.path.segments
    }
    else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} Type::Path(value) != &element.ty");
    };
    assert!(segments.len() == 3, "{proc_macro_name_upper_camel_case_ident_stringified} segments.len() != 3");
    let first_segment = segments.iter().next().expect("no .next()) element");
    assert!(first_segment.ident == std_snake_case_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} first_segment.ident != {std_snake_case_stringified} {}", first_segment.ident);
    match first_segment.arguments {
        syn::PathArguments::None => (),
        _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} first_segment.arguments != PathArguments::None")
    }
    let second_segment = segments.iter().nth(1).expect("no .nth(1) element");
    {
        let collections_snake_case_stringified = <naming_constants::Collections as naming_constants::Naming>::snake_case_stringified();
        assert!(second_segment.ident == collections_snake_case_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} second_segment.ident != {collections_snake_case_stringified} {}", second_segment.ident);
    };
    match second_segment.arguments {
        syn::PathArguments::None => (),
        _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} second_segment.arguments != PathArguments::None")
    }
    let third_segment = segments.iter().nth(2).expect("no .nth(2) element");
    {
        let hashmap_upper_camel_case = naming_constants::HashMapUpperCamelCase;
        assert!(third_segment.ident == &hashmap_upper_camel_case.to_string(), "{proc_macro_name_upper_camel_case_ident_stringified} third_segment.ident != {hashmap_upper_camel_case} {}", third_segment.ident);
    };
    let args = if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        args,
        ..
    }) = &third_segment.arguments {
        args
    }
    else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} third_segment.arguments != syn::PathArguments::AngleBracketed");
    };
    assert!(args.len() == 2, "{proc_macro_name_upper_camel_case_ident_stringified} args.len() != 2");
    let first_argument_stringified = {
        let first_argument = args.iter().next().expect("args.iter().next() is None");
        quote::quote! {#first_argument}.to_string()
    };
    assert!(std_string_string_token_stream.to_string() == first_argument_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} {std_string_string_token_stream} != {first_argument_stringified}");
    args.iter().nth(1).expect("args.iter().nth(1) is None")
}

pub fn generate_serialize_deserialize_version_of_named_syn_variant(
    value: &syn::Variant,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let element_ident = &value.ident;
    let fields = if let syn::Fields::Named(fields) = &value.fields {
        &fields.named
    }
    else {
        panic!(
            "{proc_macro_name_upper_camel_case_ident_stringified} {} syn::Data::Enum",
            naming_constants::SUPPORTS_ONLY_STRINGIFIED
        );
    };
    let std_string_string_token_stream = proc_macro_common::std_string_string_token_stream();
    let fields_idents_idents_with_serialize_deserialize_excluding_code_occurence_token_stream = fields.iter().filter(|element|
        *element.ident.as_ref().expect(proc_macro_common::constants::IDENT_IS_NONE) != *crate::naming_conventions::CodeOccurenceSnakeCase.to_string()
    ).map(|element|{
        let element_ident = element.ident.as_ref().expect(proc_macro_common::constants::IDENT_IS_NONE);
        let element_type_token_stream = {
            let element_type = &element.ty;
            quote::quote!{#element_type}
        };
        let std_snake_case_stringified = <naming_constants::Std as naming_constants::Naming>::snake_case_stringified();
        let element_type_with_serialize_deserialize_token_stream = match crate::error_occurence::ErrorOccurenceFieldAttribute::from(element) {
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString => {
                quote::quote!{
                    #std_string_string_token_stream
                }
            },
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize | crate::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => element_type_token_stream,
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                let element_type_with_serialize_deserialize_token_stream = {
                    let value = format!(
                        "{}{}",
                        {
                            let element_type = &element.ty;
                            quote::quote!{#element_type}
                        },
                        crate::naming_conventions::WithSerializeDeserializeUpperCamelCase
                    );
                    value
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote!{
                    #element_type_with_serialize_deserialize_token_stream
                }
            },
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
                quote::quote!{
                    std::vec::Vec<#std_string_string_token_stream>
                }
            },
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                let segments = if let syn::Type::Path(value) = &element.ty {
                    &value.path.segments
                }
                else {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} Type::Path(value) != &element.ty");
                };
                assert!(segments.len() == 3, "{proc_macro_name_upper_camel_case_ident_stringified} segments.len() != 3");
                let first_segment = segments.iter().next().expect("no .next() element");
                assert!(first_segment.ident == std_snake_case_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} first_segment.ident != {std_snake_case_stringified} {}", first_segment.ident);
                match first_segment.arguments {
                    syn::PathArguments::None => (),
                    _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} first_segment.arguments != PathArguments::None")
                }
                let second_segment = segments.iter().nth(1).expect("no .nth(1) element");
                {
                    let vec_snake_case_stringified = <naming_constants::Vec as naming_constants::Naming>::snake_case_stringified();
                    assert!(second_segment.ident == vec_snake_case_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} second_segment.ident != {vec_snake_case_stringified} {}", second_segment.ident);
                };
                match second_segment.arguments {
                    syn::PathArguments::None => (),
                    _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} second_segment.arguments != PathArguments::None")
                }
                let third_segment = segments.iter().nth(2).expect("no .nth(2) element");
                {
                    let vec_upper_camel_case_stringified = <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified();
                    assert!(third_segment.ident == vec_upper_camel_case_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} third_segment.ident != {vec_upper_camel_case_stringified} {}", third_segment.ident);
                };
                let element_vec_type_with_serialize_deserialize_token_stream = if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) = &third_segment.arguments {
                    assert!(args.len() == 1, "{proc_macro_name_upper_camel_case_ident_stringified} args.len() != 1");
                    let value = format!(
                        "{}{}",
                        {
                            let first_arg = args.iter().next().expect("args.iter().next() is None");
                            quote::quote! {#first_arg}
                        },
                        crate::naming_conventions::WithSerializeDeserializeUpperCamelCase,
                    );
                    value
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                }
                else {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} third_segment.arguments != syn::PathArguments::AngleBracketed");
                };
                quote::quote!{
                    std::vec::Vec<#element_vec_type_with_serialize_deserialize_token_stream>
                }
            },
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => {
                let _: &syn::GenericArgument = get_type_path_third_segment_second_argument_check_if_hashmap(
                    element,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                    std_snake_case_stringified,
                    &std_string_string_token_stream,
                );
                quote::quote!{
                    std::collections::HashMap<#std_string_string_token_stream, #std_string_string_token_stream>
                }
            },
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                let _: &syn::GenericArgument = get_type_path_third_segment_second_argument_check_if_hashmap(
                    element,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                    std_snake_case_stringified,
                    &std_string_string_token_stream,
                );
                element_type_token_stream
            },
            crate::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                let second_argument = get_type_path_third_segment_second_argument_check_if_hashmap(
                    element,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                    std_snake_case_stringified,
                    &std_string_string_token_stream,
                );
                let element_hashmap_value_type_with_serialize_deserialize_token_stream = {
                    let value = format!(
                        "{}{}",
                        quote::quote! {#second_argument},
                        crate::naming_conventions::WithSerializeDeserializeUpperCamelCase,
                    );
                    value
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote!{
                    std::collections::HashMap<#std_string_string_token_stream, #element_hashmap_value_type_with_serialize_deserialize_token_stream>
                }
            },
        }; 
        quote::quote! {
            #element_ident: #element_type_with_serialize_deserialize_token_stream,
        }
    });
    let code_occurence_snake_case_token_stream = crate::naming_conventions::CodeOccurenceSnakeCase;
    quote::quote! {
        #element_ident {
            #(#fields_idents_idents_with_serialize_deserialize_excluding_code_occurence_token_stream)*
            #code_occurence_snake_case_token_stream: error_occurence_lib::code_occurence::CodeOccurence,
        }
    }
}

