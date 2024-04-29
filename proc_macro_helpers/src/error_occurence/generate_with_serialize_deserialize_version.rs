pub fn generate_with_serialize_deserialize_version(
    supported_enum_variant: &crate::error_occurence::supported_enum_variant::SuportedEnumVariant,
    variants: &syn::punctuated::Punctuated<&syn::Variant, syn::token::Comma>, //&syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>
    proc_macro_name_ident_stringified: &str,
    generics_len: usize,
    ident_with_serialize_deserialize_token_stream: &proc_macro2::TokenStream,
    optional_additional_named_variant: Option<proc_macro2::TokenStream>,
    implements_this_error: bool,
    is_pub: bool,
) -> proc_macro2::TokenStream {
    let this_error_token_stream = if implements_this_error {
        quote::quote! { thiserror::Error, }
    }
    else {
        proc_macro2::TokenStream::new()
    };
    let is_pub_token_stream = if is_pub {
        quote::quote! { pub }
    }
    else {
        proc_macro2::TokenStream::new()
    };
    let variants_len = variants.len();
    let with_serialize_deserialize_upper_camel_case =
        crate::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified();
    // let error_occurence_snake_case = crate::naming_conventions::error_occurence_snake_case();
    let vec_snake_case = <naming_constants::Vec as naming_constants::Naming>::snake_case_stringified();
    let hashmap_snake_case = <naming_constants::HashMap as naming_constants::Naming>::snake_case_stringified();
    let key_snake_case = <naming_constants::Key as naming_constants::Naming>::upper_camel_case_stringified();
    let value_snake_case = <naming_constants::Value as naming_constants::Naming>::upper_camel_case_stringified();
    let syn_type_path_stringified = crate::naming_conventions::syn_type_path_stringified();
    let token_stream = match supported_enum_variant {
        crate::error_occurence::supported_enum_variant::SuportedEnumVariant::Named => {
            let code_occurence_upper_camel_case =
                crate::naming_conventions::code_occurence_upper_camel_case_stringified();
            let foreign_type_upper_camel_case = "ForeignType";
            let display_upper_camel_case = "Display";
            let to_std_string_string_upper_camel_case =
                format!("{display_upper_camel_case}{foreign_type_upper_camel_case}");
            let to_std_string_string_snake_case =
                proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
                    &to_std_string_string_upper_camel_case,
                );
            let display_snake_case =
                proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
                    &display_upper_camel_case,
                );
            let attribute_prefix_stringified = "eo_";
            let with_serialize_deserialize_snake_case =
                crate::naming_conventions::with_serialize_deserialize_snake_case_stringified();
            let attribute_display_with_serialize_deserialize_stringified = format!(
                "{attribute_prefix_stringified}{display_snake_case}_{with_serialize_deserialize_snake_case}");
            let attribute_vec_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{vec_snake_case}_{display_snake_case}_{with_serialize_deserialize_snake_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_snake_case}_{with_serialize_deserialize_snake_case}_{value_snake_case}_{display_snake_case}_{with_serialize_deserialize_snake_case}");
            let attribute_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{to_std_string_string_snake_case}_{value_snake_case}_{display_snake_case}_{with_serialize_deserialize_snake_case}");
            let variants_vec = variants.iter().map(|variant| {
                let fields_named = if let syn::Fields::Named(fields_named) = &variant.fields {
                    fields_named
                }
                else {
                    panic!("{proc_macro_name_ident_stringified} expected fields would be named");
                };
                let variant_fields_vec = fields_named.named.iter().map(|field|{
                    let field_ident = field.ident.as_ref().unwrap_or_else(|| {
                        panic!(
                            "{proc_macro_name_ident_stringified} {}",
                            naming_constants::FIELD_IDENT_IS_NONE
                        )
                    });
                    let code_occurence_snake_case = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&code_occurence_upper_camel_case);
                    let error_or_code_occurence = if *field_ident == *code_occurence_snake_case {
                        let (code_occurence_type_stringified, code_occurence_lifetime) = {
                            if let syn::Type::Path(type_path) = &field.ty {
                                (
                                    {
                                        let mut code_occurence_type_repeat_checker = false;
                                        let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
                                        .fold(std::string::String::new(), |mut acc, path_segment| {
                                            let path_segment_ident = &path_segment.ident;
                                            if *path_segment_ident == code_occurence_upper_camel_case {
                                                assert!(!code_occurence_type_repeat_checker, "{proc_macro_name_ident_stringified} code_occurence_ident detected more than one {code_occurence_upper_camel_case} inside type path");
                                                acc.push_str(&path_segment_ident.to_string());
                                                code_occurence_type_repeat_checker = true;
                                            }
                                            else {
                                                acc.push_str(&format!("{path_segment_ident}::"));
                                            }
                                            acc
                                        });
                                        assert!(code_occurence_type_repeat_checker, "{proc_macro_name_ident_stringified} no {code_occurence_upper_camel_case} named field");
                                        code_occurence_segments_stringified_handle
                                    },
                                    crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                        &type_path.path.segments,
                                        proc_macro_name_ident_stringified
                                    ),
                                )
                            }
                            else {
                                panic!(
                                    "{proc_macro_name_ident_stringified} {code_occurence_snake_case} {} {syn_type_path_stringified}",
                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                );
                            }
                        };
                        crate::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence::CodeOccurence {
                            field_type: code_occurence_type_stringified,
                            vec_lifetime: code_occurence_lifetime
                        }
                    }
                    else {
                        let attribute = {
                            let mut option_attribute = None;
                            field.attrs.iter().for_each(|attr|{
                                if attr.path().segments.len() == 1 {
                                    let error_message = format!("{proc_macro_name_ident_stringified} two or more supported attributes!");
                                    let attr_ident = attr.path().segments.iter().next().map_or_else(|| panic!("attr.path().segments.iter().next() is None"), |path_segment| &path_segment.ident);
                                    if let Ok(value) = {
                                        use std::str::FromStr;
                                        crate::error_occurence::named_attribute::NamedAttribute::from_str(&attr_ident.to_string())
                                    } {
                                        if option_attribute.is_some() {
                                            panic!("{error_message}");
                                        }
                                        else {
                                            option_attribute = Some(value);
                                        }
                                    }
                                }//other attributes are not for this proc_macro
                            });
                            option_attribute.unwrap_or_else(|| panic!(
                                "{proc_macro_name_ident_stringified} option attribute {}",
                                naming_constants::IS_NONE_STRINGIFIED
                            ))
                        };
                        let supported_container = generate_supported_container(
                            field,
                            proc_macro_name_ident_stringified,
                        );
                        crate::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence::ErrorField {
                            attribute,
                            supported_container,
                        }
                    };
                    (
                        field_ident.clone(),
                        error_or_code_occurence,
                    )
                })
                .collect::<Vec<(
                    proc_macro2::Ident,
                    crate::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence
                )>>();
                (
                    &variant.ident,
                    variant_fields_vec,
                )
            });
            let mut lifetimes_for_serialize_deserialize = Vec::with_capacity(generics_len);
            let mut logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> =
                Vec::with_capacity(variants_len);
            let mut should_generate_impl_compile_time_check_error_occurence_members = false;
            variants_vec.into_iter().for_each(|(
                variant_ident,
                fields_vec
            )|{
                let mut enum_fields_logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                for (field_ident, error_or_code_occurence) in fields_vec {
                    match error_or_code_occurence {
                        crate::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence::ErrorField {
                            attribute,
                            supported_container,
                        } => {
                            match &attribute {
                                crate::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence |
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecErrorOccurence
                                //  |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueErrorOccurence
                                 => {
                                    if !should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoDisplay |
                                crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize |
                                crate::error_occurence::named_attribute::NamedAttribute::EoToStdStringString |
                                crate::error_occurence::named_attribute::NamedAttribute::EoToStdStringStringWithSerializeDeserialize |
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplay |
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplayWithSerializeDeserialize |
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecToStdStringString |
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecToStdStringStringWithSerializeDeserialize
                                //  |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringString |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringStringWithSerializeDeserialize |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueDisplay |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueDisplayWithSerializeDeserialize |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueToStdStringString |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueToStdStringStringWithSerializeDeserialize
                                 => (),
                            }
                            match &attribute {
                                crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize => {
                                    if let crate::error_occurence::supported_container::SupportedContainer::Reference{ lifetime_ident, .. } = &supported_container {
                                        crate::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                                            lifetime_ident.to_string(),
                                            &mut lifetimes_for_serialize_deserialize
                                        );
                                    }
                                },
                                crate::error_occurence::named_attribute::NamedAttribute::EoDisplay |
                                crate::error_occurence::named_attribute::NamedAttribute::EoToStdStringString |
                                crate::error_occurence::named_attribute::NamedAttribute::EoToStdStringStringWithSerializeDeserialize |
                                crate::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence |
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplay |
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplayWithSerializeDeserialize |
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecToStdStringString |
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecToStdStringStringWithSerializeDeserialize |
                                crate::error_occurence::named_attribute::NamedAttribute::EoVecErrorOccurence 
                                // |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringString |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringStringWithSerializeDeserialize |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueDisplay |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueDisplayWithSerializeDeserialize |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueToStdStringString |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueToStdStringStringWithSerializeDeserialize |
                                // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueErrorOccurence
                                 => (),
                            }
                            {
                                let must_be_used_with_stringified = "must be used with";
                                let str_stringified = "str";
                                let string_string_stringified: std::string::String = format!(
                                    "{}::{}",
                                    <naming_constants::String as naming_constants::Naming>::snake_case_stringified(),
                                    <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified(),
                                );
                                let std_string_string_stringified = format!(
                                    "{}::{}::{}",
                                    naming_constants::STD_STRINGIFIED,
                                    <naming_constants::String as naming_constants::Naming>::snake_case_stringified(),
                                    <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                                );
                                match &attribute {
                                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplay => {
                                        if let crate::error_occurence::supported_container::SupportedContainer::Path { path, ..} = &supported_container {
                                            inform_use_str_string_in_different_attribute(
                                                path,
                                                proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                                                &attribute_display_with_serialize_deserialize_stringified,
                                                str_stringified,
                                                proc_macro_name_ident_stringified,
                                                must_be_used_with_stringified,
                                                &std_string_string_stringified,
                                                &string_string_stringified,
                                            );
                                        }
                                    },
                                    crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplay => {
                                        if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                                            vec_element_type: crate::error_occurence::vec_element_type::VecElementType::Path { element_path, ..},
                                            ..
                                        } = &supported_container {
                                            inform_use_str_string_in_different_attribute(
                                                element_path,
                                                proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                                                &attribute_vec_display_with_serialize_deserialize_stringified,
                                                str_stringified,
                                                proc_macro_name_ident_stringified,
                                                must_be_used_with_stringified,
                                                &std_string_string_stringified,
                                                &string_string_stringified,
                                            );
                                        }
                                    },
                                    // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay => {
                                    //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                    //         hashmap_key_type,
                                    //         hashmap_value_type,
                                    //         ..
                                    //     } = &supported_container {
                                    //         if let (
                                    //             crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..} | 
                                    //             crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
                                    //             crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                    //                 value_segments_stringified,
                                    //                 ..
                                    //             }
                                    //         ) = (hashmap_key_type, hashmap_value_type) {
                                    //             inform_use_str_string_in_different_attribute(
                                    //                 value_segments_stringified,
                                    //                 proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                                    //                 &attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified,
                                    //                 str_stringified,
                                    //                 proc_macro_name_ident_stringified,
                                    //                 must_be_used_with_stringified,
                                    //                 &std_string_string_stringified,
                                    //                 &string_string_stringified,
                                    //             );
                                    //         }
                                    //     }
                                    // },
                                    // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueDisplay => {
                                    //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
                                    //         hashmap_key_type: crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
                                    //         hashmap_value_type: crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                    //             value_segments_stringified,
                                    //             ..
                                    //         },
                                    //         ..
                                    //     } = &supported_container {
                                    //         inform_use_str_string_in_different_attribute(
                                    //             value_segments_stringified,
                                    //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                                    //             &attribute_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize_stringified,
                                    //             str_stringified,
                                    //             proc_macro_name_ident_stringified,
                                    //             must_be_used_with_stringified,
                                    //             &std_string_string_stringified,
                                    //             &string_string_stringified,
                                    //         );
                                    //     }
                                    // },
                                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize |
                                    crate::error_occurence::named_attribute::NamedAttribute::EoToStdStringString |
                                    crate::error_occurence::named_attribute::NamedAttribute::EoToStdStringStringWithSerializeDeserialize |
                                    crate::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence |
                                    crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplayWithSerializeDeserialize |
                                    crate::error_occurence::named_attribute::NamedAttribute::EoVecToStdStringString |
                                    crate::error_occurence::named_attribute::NamedAttribute::EoVecToStdStringStringWithSerializeDeserialize |
                                    crate::error_occurence::named_attribute::NamedAttribute::EoVecErrorOccurence
                                    //  |    
                                    // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize |
                                    // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringString |
                                    // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringStringWithSerializeDeserialize |
                                    // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence |
                                    // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueDisplayWithSerializeDeserialize |
                                    // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueToStdStringString |
                                    // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueToStdStringStringWithSerializeDeserialize |
                                    // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueErrorOccurence
                                     => (),
                                }
                            }
                            let field_type_with_serialize_deserialize_token_stream = generate_field_type_with_serialize_deserialize_version(
                                attribute,
                                supported_container,
                                proc_macro_name_ident_stringified,
                            );
                            enum_fields_logic_for_enum_with_serialize_deserialize.push(quote::quote!{
                                #field_ident: #field_type_with_serialize_deserialize_token_stream
                            });
                        },
                        crate::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence::CodeOccurence {
                            field_type,
                            ..
                         } => {
                            let code_occurence_type_token_stream = {
                                let code_occurence_type_stringified = field_type.as_str();
                                code_occurence_type_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {code_occurence_type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            enum_fields_logic_for_enum_with_serialize_deserialize.push(quote::quote!{
                                #field_ident: #code_occurence_type_token_stream
                            });
                        },
                    }
                }
                logic_for_enum_with_serialize_deserialize.push(quote::quote! {
                    #variant_ident {
                        #(#enum_fields_logic_for_enum_with_serialize_deserialize),*
                    }
                });
            });
            let additional_variant_token_stream =optional_additional_named_variant.map_or_else(|| quote::quote! {}, |additional_variant_token_stream| quote::quote! { #additional_variant_token_stream,});
            quote::quote! {
                #[derive(Debug, #this_error_token_stream serde::Serialize, serde::Deserialize)]
                #is_pub_token_stream enum #ident_with_serialize_deserialize_token_stream {
                    #additional_variant_token_stream
                    #(#logic_for_enum_with_serialize_deserialize),*
                }
            }
        }
        crate::error_occurence::supported_enum_variant::SuportedEnumVariant::Unnamed => {
            let mut logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> =
                Vec::with_capacity(variants_len);
            variants.iter().for_each(|variant|{
                let variant_ident = &variant.ident;
                let field_type = if let syn::Fields::Unnamed(fields_unnamed) = &variant.fields {
                    let unnamed = &fields_unnamed.unnamed;
                    assert!(unnamed.len() == 1, 
                        "{proc_macro_name_ident_stringified} {}::{} variant fields unnamed len != 1",
                        naming_constants::SUPPORTED_ENUM_VARIANT_STRINGIFIED,
                        <naming_constants::Unnamed as naming_constants::Naming>::upper_camel_case_stringified()
                    );
                    unnamed.iter().next().map_or_else(|| panic!("unnamed.iter().next() is None"), |field| &field.ty)
                }
                else {
                    panic!(
                        "{proc_macro_name_ident_stringified} {} {}::{}",
                        naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                        naming_constants::SYN_FIELDS,
                        <naming_constants::Unnamed as naming_constants::Naming>::upper_camel_case_stringified()
                    );
                };
                let type_token_stream = if let syn::Type::Path(type_path) = field_type {
                    let type_stringified = format!(
                        "{}{with_serialize_deserialize_upper_camel_case}",
                        crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                    );
                    type_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                }
                else {
                    panic!(
                        "{proc_macro_name_ident_stringified} {} {syn_type_path_stringified}",
                        naming_constants::SUPPORTS_ONLY_STRINGIFIED
                    )
                };
                logic_for_enum_with_serialize_deserialize.push({
                    quote::quote!{
                        #variant_ident(#type_token_stream)
                    }
                });
            });
            let logic_for_enum_with_serialize_deserialize_generated =
                logic_for_enum_with_serialize_deserialize.iter();
            quote::quote! {
                #[derive(Debug, #this_error_token_stream serde::Serialize, serde::Deserialize)]
                #is_pub_token_stream enum #ident_with_serialize_deserialize_token_stream {
                    #(#logic_for_enum_with_serialize_deserialize_generated),*
                }
            }
        }
    };
    // if &ident_with_serialize_deserialize_token_stream.to_string() == "" {
    //     println!("{token_stream}");
    // }
    token_stream
}

fn inform_use_str_string_in_different_attribute(
    path: &str,
    wrong_attribute: &str,
    attribute_to_use: &str,
    str_stringified: &str,
    proc_macro_name_ident_stringified: &str,
    must_be_used_with_stringified: &str,
    std_string_string_stringified: &str,
    string_string_stringified: &str,
) {
    let wrong_attribute_view =
        crate::error_occurence::named_attribute::attribute_view(wrong_attribute);
    let attribute_to_use_view =
        crate::error_occurence::named_attribute::attribute_view(attribute_to_use);
    //maybe additional cases exists
    if path == str_stringified {
        panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {str_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
    } else if path == std_string_string_stringified {
        panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {std_string_string_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
    } else if path == string_string_stringified {
        panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {string_string_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
    } else if path == <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified() {
        panic!(
            "{proc_macro_name_ident_stringified} {wrong_attribute_view} {} {must_be_used_with_stringified} {attribute_to_use_view}",
            <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
        );
    }
    else {
        //clippy lint forces to add empty else
    }
}

pub fn generate_field_type_with_serialize_deserialize_version(
    attribute: crate::error_occurence::named_attribute::NamedAttribute,
    supported_container: crate::error_occurence::supported_container::SupportedContainer,
    proc_macro_name_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let with_serialize_deserialize_upper_camel_case =
        crate::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified();
    let supports_only_supported_container_stringified =
        crate::naming_conventions::supports_only_supported_container_stringified();
    let does_not_support_stringified = "does not support";
    let str_stringified = "str";
    let std_string_string_stringified = format!(
        "{}::{}::{}",
        naming_constants::STD_STRINGIFIED,
        <naming_constants::String as naming_constants::Naming>::snake_case_stringified(),
        <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
    );
    let std_string_string_token_stream = proc_macro_common::std_string_string_token_stream();
    let as_std_collections_hashmap_key_type_stringified = format!(
        "as {}::collections::{} key type",
        naming_constants::STD_STRINGIFIED,
        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
    );
    let type_upper_camel_case = "Type";
    let hashmap_value_type_stringified = format!(
        "{}{}{type_upper_camel_case}",
        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Value as naming_constants::Naming>::upper_camel_case_stringified(),
    );
    let hashmap_value_type_path_stringified = format!(
        "{hashmap_value_type_stringified}::{}",
        <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified(),
    );
    let hashmap_value_type_reference_stringified = format!(
        "{hashmap_value_type_stringified}::{}",
        <naming_constants::Reference as naming_constants::Naming>::upper_camel_case_stringified(),
    );
    let hashmap_key_type_stringified = format!(
        "{}{}{type_upper_camel_case}",
        <naming_constants::Key as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
    );
    let hashmap_key_type_path_stringified = format!(
        "{hashmap_key_type_stringified}::{}",
        <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified(),
    );
    let hashmap_key_type_reference_stringified = format!(
        "{hashmap_key_type_stringified}::{}",
        <naming_constants::Reference as naming_constants::Naming>::upper_camel_case_stringified(),
    );
    let vec_element_type_path_stringified = format!(
        "crate::error_occurence::vec_element_type::VecElementType::{}",
        <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified(),
    );
    match &attribute {
        crate::error_occurence::named_attribute::NamedAttribute::EoDisplay => {
            if let crate::error_occurence::supported_container::SupportedContainer::Path { .. } = supported_container {
                quote::quote! {
                    #std_string_string_token_stream
                }
            }
            else {
                panic!(
                    "{proc_macro_name_ident_stringified} {} {} {}{}", 
                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                    naming_constants::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
                    <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified(),
                )
            }
        },
        crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize => {
            match supported_container {
                crate::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime } => {
                    {
                        let type_stringified = format!("{path}{}",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                        type_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    }
                },
                crate::error_occurence::supported_container::SupportedContainer::Reference{ reference_ident, .. } => {
                    crate::error_occurence::panic_if_not_str::panic_if_not_str(
                        &reference_ident,
                        str_stringified,
                        proc_macro_name_ident_stringified,
                        naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                        &attribute
                    );
                    quote::quote!{#std_string_string_token_stream}
                },
                crate::error_occurence::supported_container::SupportedContainer::Vec{ .. } | 
                crate::error_occurence::supported_container::SupportedContainer::HashMap{ .. } => panic!(
                    "{proc_macro_name_ident_stringified} {} only supports {}{} and {}{}", 
                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                    naming_constants::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
                    <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified(),
                    naming_constants::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
                    <naming_constants::Reference as naming_constants::Naming>::upper_camel_case_stringified(),
                ),
            }
        },
        crate::error_occurence::named_attribute::NamedAttribute::EoToStdStringString => {
            if let crate::error_occurence::supported_container::SupportedContainer::Path {..} = supported_container {}
            else {
                panic!(
                    "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                    <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified(),
                );
            }
            quote::quote! {
                #std_string_string_token_stream
            }
        },
        crate::error_occurence::named_attribute::NamedAttribute::EoToStdStringStringWithSerializeDeserialize => {
            if let crate::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime } = &supported_container {
                {
                    let type_stringified = format!("{path}{}",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(vec_lifetime));
                        type_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}",proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                }
            }
            else {
                panic!(
                    "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                    <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified(),
                );
            }
        },
        crate::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence => {
            if let crate::error_occurence::supported_container::SupportedContainer::Path { path, ..} = &supported_container {
                {
                    let type_stringified = format!("{path}{with_serialize_deserialize_upper_camel_case}");
                    type_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                }
            }
            else {
                panic!(
                    "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                    <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified(),
                );
            }
        },
        crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplay => {
            if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                path,
                vec_element_type
            } = supported_container {
                if let crate::error_occurence::vec_element_type::VecElementType::Path { .. } = vec_element_type {
                    let type_stringified = format!("{path}<{std_string_string_stringified}>");
                    type_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                }
                else {
                    panic!(
                        "{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", 
                        proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                        naming_constants::SUPPORTS_ONLY_STRINGIFIED
                    );
                }
            }
            else {
                panic!(
                    "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                    <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified(),
                );
            }
        },
        crate::error_occurence::named_attribute::NamedAttribute::EoVecDisplayWithSerializeDeserialize => {
            if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                path,
                vec_element_type
            } = supported_container {
                match vec_element_type {
                    crate::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } => {
                        let type_stringified = format!("{path}<{element_path}{}>",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                        type_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    },
                    crate::error_occurence::vec_element_type::VecElementType::Reference { reference_ident, .. } => {
                        crate::error_occurence::panic_if_not_str::panic_if_not_str(
                            &reference_ident,
                            str_stringified,
                            proc_macro_name_ident_stringified,
                            naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                            &attribute
                        );
                        {
                            let type_stringified = format!("{path}<{std_string_string_stringified}>");
                            type_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        }
                    },
                }
            }
            else {
                panic!(
                    "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                    <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified(),
                );
            }
        },
        crate::error_occurence::named_attribute::NamedAttribute::EoVecToStdStringString => {
            if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                vec_element_type,
                ..
            } = supported_container {
                if let crate::error_occurence::vec_element_type::VecElementType::Path {..} = vec_element_type {}
                else {
                    panic!(
                        "{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", 
                        proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                        naming_constants::SUPPORTS_ONLY_STRINGIFIED
                    );
                }
            }
            else {
                panic!(
                    "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                    <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified(),
                );
            }
            quote::quote! {
                std::vec::Vec<#std_string_string_token_stream>
            }
        },
        crate::error_occurence::named_attribute::NamedAttribute::EoVecToStdStringStringWithSerializeDeserialize => {
            if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                path,
                vec_element_type
            } = supported_container {
                if let crate::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } = vec_element_type {
                    {
                        let type_stringified = format!("{path}<{element_path}{}>",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                        type_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    }
                }
                else {
                    panic!(
                        "{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", 
                        proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                        naming_constants::SUPPORTS_ONLY_STRINGIFIED
                    );
                }
            }
            else {
                panic!(
                    "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                    <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified(),
                );
            }
        },
        crate::error_occurence::named_attribute::NamedAttribute::EoVecErrorOccurence => {
            if let crate::error_occurence::supported_container::SupportedContainer::Vec {
                path,
                vec_element_type
            } = supported_container {
                if let crate::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } = vec_element_type  {
                    {
                        let type_stringified = format!("{path}<{element_path}{with_serialize_deserialize_upper_camel_case}{}>",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                        type_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    }
                }
                else {
                    panic!(
                        "{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", 
                        proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                        naming_constants::SUPPORTS_ONLY_STRINGIFIED
                    );
                }
            }
            else {
                panic!(
                    "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
                    <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified(),
                );
            }
        },
        // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay => {
        //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
        //         path,
        //         hashmap_key_type,
        //         hashmap_value_type,
        //     } = supported_container {
        //         let hashmap_key_type_path_case = |
        //             key_segments_stringified: std::string::String,
        //             key_vec_lifetime: Vec<crate::error_occurence::lifetime::Lifetime>,
        //         | -> proc_macro2::TokenStream {
        //             crate::error_occurence::panic_if_not_string::panic_if_not_string(
        //                 &key_segments_stringified,
        //                 &std_string_string_stringified,
        //                 proc_macro_name_ident_stringified,
        //                 naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                 &as_std_collections_hashmap_key_type_stringified,
        //                 &attribute
        //             );
        //             {
        //                 let type_stringified = format!(
        //                     "{path}<{key_segments_stringified}{}, {std_string_string_stringified}>",
        //                      crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
        //                 );
        //                 type_stringified
        //                 .parse::<proc_macro2::TokenStream>()
        //                 .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //             }
        //         };
        //         match (hashmap_key_type, hashmap_value_type) {
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
        //                     key_segments_stringified,
        //                     key_vec_lifetime
        //                 },
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => {
        //                 hashmap_key_type_path_case(
        //                     key_segments_stringified,
        //                     key_vec_lifetime,
        //                 )
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => {
        //                 {
        //                     let type_stringified = format!("{path}<{std_string_string_stringified}, {std_string_string_stringified}>");
        //                     type_stringified
        //                     .parse::<proc_macro2::TokenStream>()
        //                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //                 }
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..} |
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //         }
        //     }
        //     else {
        //         panic!(
        //             "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
        //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
        //             <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
        //         );
        //     }
        // },
        // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize => {
        //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
        //         path,
        //         hashmap_key_type,
        //         hashmap_value_type
        //     } = supported_container {
        //         match (hashmap_key_type, hashmap_value_type) {
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
        //                     key_segments_stringified,
        //                     key_vec_lifetime
        //                 },
        //                crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
        //                     value_segments_stringified,
        //                     value_vec_lifetime
        //                 }
        //             ) => {
        //                 crate::error_occurence::panic_if_not_string::panic_if_not_string(
        //                     &key_segments_stringified,
        //                     &std_string_string_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &as_std_collections_hashmap_key_type_stringified,
        //                     &attribute
        //                 );
        //                 {
        //                     let type_stringified = format!(
        //                         "{path}<{key_segments_stringified}{}, {value_segments_stringified}{}>",
        //                          crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
        //                          crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
        //                     );
        //                     type_stringified
        //                     .parse::<proc_macro2::TokenStream>()
        //                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //                 }
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
        //                     key_segments_stringified,
        //                     key_vec_lifetime
        //                 },
        //                crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
        //                     value_reference_ident,
        //                     ..
        //                 }
        //             ) => {
        //                 crate::error_occurence::panic_if_not_string::panic_if_not_string(
        //                     &key_segments_stringified,
        //                     &std_string_string_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &as_std_collections_hashmap_key_type_stringified,
        //                     &attribute
        //                 );
        //                 crate::error_occurence::panic_if_not_str::panic_if_not_str(
        //                     &value_reference_ident,
        //                     str_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &attribute
        //                 );
        //                 {
        //                     let type_stringified = format!(
        //                         "{path}<{key_segments_stringified}{}, {std_string_string_stringified}>",
        //                          crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime)
        //                     );
        //                     type_stringified
        //                     .parse::<proc_macro2::TokenStream>()
        //                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //                 }
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
        //                     key_reference_ident,
        //                     ..
        //                 },
        //                crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
        //                     value_segments_stringified,
        //                     value_vec_lifetime
        //                 }
        //             ) => {
        //                 crate::error_occurence::panic_if_not_str::panic_if_not_str(
        //                     &key_reference_ident,
        //                     str_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &attribute
        //                 );
        //                 {
        //                     let type_stringified = format!(
        //                         "{path}<{std_string_string_stringified}, {value_segments_stringified}{}>",
        //                          crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
        //                     );
        //                     type_stringified
        //                     .parse::<proc_macro2::TokenStream>()
        //                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}",proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //                 }
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
        //                     key_reference_ident,
        //                     ..
        //                 },
        //                crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
        //                     value_reference_ident,
        //                     ..
        //                 }
        //             ) => {
        //                 crate::error_occurence::panic_if_not_str::panic_if_not_str(
        //                     &key_reference_ident,
        //                     str_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &attribute
        //                 );
        //                 crate::error_occurence::panic_if_not_str::panic_if_not_str(
        //                     &value_reference_ident,
        //                     str_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &attribute
        //                 );
        //                 {
        //                     let type_stringified = format!("{path}<{std_string_string_stringified}, {std_string_string_stringified}>");
        //                     type_stringified
        //                     .parse::<proc_macro2::TokenStream>()
        //                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}",proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //                 }
        //             },
        //         }
        //     }
        //     else {
        //         panic!(
        //             "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
        //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
        //             <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
        //         );
        //     }
        // },
        // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringString => {
        //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
        //         path,
        //         hashmap_key_type,
        //         hashmap_value_type
        //     } = supported_container {
        //         let hashmap_key_type_path_case = |
        //             key_segments_stringified: std::string::String,
        //             key_vec_lifetime: Vec<crate::error_occurence::lifetime::Lifetime>,
        //         | -> proc_macro2::TokenStream {
        //             {
        //                 let type_stringified = format!(
        //                     "{path}<{key_segments_stringified}{},{std_string_string_stringified}>",
        //                      crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime)
        //                 );
        //                 type_stringified
        //                 .parse::<proc_macro2::TokenStream>()
        //                 .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //             }
        //         };
        //         match (hashmap_key_type, hashmap_value_type) {
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
        //                     key_segments_stringified,
        //                     key_vec_lifetime
        //                 },
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => hashmap_key_type_path_case(
        //                 key_segments_stringified,
        //                 key_vec_lifetime,
        //             ),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => {
        //                 let type_stringified = format!(
        //                     "{path}<{std_string_string_stringified},{std_string_string_stringified}>"
        //                 );
        //                 type_stringified
        //                 .parse::<proc_macro2::TokenStream>()
        //                 .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //         }
        //     }
        //     else {
        //         panic!(
        //             "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
        //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
        //             <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
        //         );
        //     }
        // },
        // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueToStdStringStringWithSerializeDeserialize => {
        //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
        //         path,
        //         hashmap_key_type,
        //         hashmap_value_type
        //     } = supported_container {
        //         match (hashmap_key_type, hashmap_value_type) {
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
        //                     key_segments_stringified,
        //                     key_vec_lifetime
        //                 },
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
        //                     value_segments_stringified,
        //                     value_vec_lifetime
        //                 }
        //             ) => {
        //                 crate::error_occurence::panic_if_not_string::panic_if_not_string(
        //                     &key_segments_stringified,
        //                     &std_string_string_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &as_std_collections_hashmap_key_type_stringified,
        //                     &attribute
        //                 );
        //                 {
        //                     let type_stringified = format!(
        //                         "{path}<{key_segments_stringified}{},{value_segments_stringified}{}>",
        //                          crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
        //                          crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime),
        //                     );
        //                     type_stringified
        //                     .parse::<proc_macro2::TokenStream>()
        //                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //                 }
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
        //                     key_reference_ident,
        //                     ..
        //                 },
        //                crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
        //                     value_segments_stringified,
        //                     value_vec_lifetime
        //                 }
        //             ) => {
        //                 crate::error_occurence::panic_if_not_str::panic_if_not_str(
        //                     &key_reference_ident,
        //                     str_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &attribute
        //                 );
        //                 {
        //                         let type_stringified = format!(
        //                             "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
        //                              crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime),
        //                         );
        //                         type_stringified
        //                         .parse::<proc_macro2::TokenStream>()
        //                         .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //                     }
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //         }
        //     }
        //     else {
        //         panic!(
        //             "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
        //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
        //             <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
        //         );
        //     }
        // },
        // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence => {
        //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
        //         path,
        //         hashmap_key_type,
        //         hashmap_value_type
        //     } = supported_container {
        //         match (hashmap_key_type, hashmap_value_type) {
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {
        //                     key_segments_stringified,
        //                     key_vec_lifetime
        //                 },
        //                crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
        //                     value_segments_stringified,
        //                     value_vec_lifetime
        //                 }
        //             ) => {
        //                 crate::error_occurence::panic_if_not_string::panic_if_not_string(
        //                     &key_segments_stringified,
        //                     &std_string_string_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &as_std_collections_hashmap_key_type_stringified,
        //                     &attribute
        //                 );
        //                 {
        //                     let type_stringified = format!(
        //                         "{path}<{key_segments_stringified}{}, {value_segments_stringified}{with_serialize_deserialize_upper_camel_case}{}>",
        //                          crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
        //                          crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
        //                     );
        //                     type_stringified
        //                     .parse::<proc_macro2::TokenStream>()
        //                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //                 }
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
        //                     key_reference_ident,
        //                     ..
        //                 },
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
        //                     value_segments_stringified,
        //                     value_vec_lifetime
        //                 }
        //             ) => {
        //                 crate::error_occurence::panic_if_not_str::panic_if_not_str(
        //                     &key_reference_ident,
        //                     str_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &attribute
        //                 );
        //                 {
        //                     let type_stringified = format!(
        //                         "{path}<{std_string_string_stringified}, {value_segments_stringified}{with_serialize_deserialize_upper_camel_case}{}>",
        //                          crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
        //                    );
        //                     type_stringified
        //                     .parse::<proc_macro2::TokenStream>()
        //                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //                 }
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //         }
        //     }
        //     else {
        //         panic!(
        //             "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
        //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
        //             <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
        //         );
        //     }
        // },
        // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueDisplay => {
        //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
        //         path,
        //         hashmap_key_type,
        //         hashmap_value_type
        //     } = supported_container {
        //         let hashmap_key_type_path_case = || -> proc_macro2::TokenStream {
        //             let type_stringified = format!("{path}<{std_string_string_stringified},{std_string_string_stringified}>");
        //             type_stringified
        //             .parse::<proc_macro2::TokenStream>()
        //             .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //         };
        //         match (hashmap_key_type, hashmap_value_type) {
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => {
        //                 hashmap_key_type_path_case()
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //         }
        //     }
        //     else {
        //         panic!(
        //             "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
        //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
        //             <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
        //         );
        //     }
        // },
        // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueDisplayWithSerializeDeserialize => {
        //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
        //         path,
        //         hashmap_key_type,
        //         hashmap_value_type
        //     } = supported_container {
        //         match (hashmap_key_type, hashmap_value_type) {
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
        //                     value_segments_stringified,
        //                     value_vec_lifetime
        //                 }
        //             ) => {
        //                 let type_stringified = format!(
        //                     "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
        //                      crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
        //                 );
        //                 type_stringified
        //                 .parse::<proc_macro2::TokenStream>()
        //                 .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
        //                     value_reference_ident,
        //                     ..
        //                 }
        //             ) => {
        //                 crate::error_occurence::panic_if_not_str::panic_if_not_str(
        //                     &value_reference_ident,
        //                     str_stringified,
        //                     proc_macro_name_ident_stringified,
        //                     naming_constants::SUPPORTS_ONLY_STRINGIFIED,
        //                     &attribute
        //                 );
        //                 {
        //                     let type_stringified = format!(
        //                         "{path}<{std_string_string_stringified},{std_string_string_stringified}>"
        //                     );
        //                     type_stringified
        //                     .parse::<proc_macro2::TokenStream>()
        //                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //                 }
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //         }
        //     }
        //     else {
        //         panic!(
        //             "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
        //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
        //             <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
        //         );
        //     }
        // },
        // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueToStdStringString => {
        //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
        //         path,
        //         hashmap_key_type,
        //         hashmap_value_type
        //     } = supported_container {
        //         match (hashmap_key_type, hashmap_value_type) {
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => {
        //                 let type_stringified = format!("{path}<{std_string_string_stringified},{std_string_string_stringified}>");
        //                 type_stringified
        //                 .parse::<proc_macro2::TokenStream>()
        //                 .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //         }
        //     }
        //     else {
        //         panic!(
        //             "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
        //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
        //             <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
        //         );
        //     }
        // },
        // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueToStdStringStringWithSerializeDeserialize => {
        //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
        //         path,
        //         hashmap_key_type,
        //         hashmap_value_type
        //     } = supported_container {
        //         match (hashmap_key_type, hashmap_value_type) {
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
        //                     value_segments_stringified,
        //                     value_vec_lifetime
        //                 }
        //             ) => {
        //                 let type_stringified = format!(
        //                     "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
        //                      crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime),
        //                 );
        //                 type_stringified
        //                 .parse::<proc_macro2::TokenStream>()
        //                 .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}",proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //         }
        //     }
        //     else {
        //         panic!(
        //             "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
        //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
        //             <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
        //         );
        //     }
        // },
        // crate::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyToStdStringStringValueErrorOccurence => {
        //     if let crate::error_occurence::supported_container::SupportedContainer::HashMap {
        //         path,
        //         hashmap_key_type,
        //         hashmap_value_type
        //     } = supported_container {
        //         match (hashmap_key_type, hashmap_value_type) {
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {
        //                     value_segments_stringified,
        //                     value_vec_lifetime
        //                 }
        //             ) => {
        //                 let type_stringified = format!(
        //                     "{path}<{std_string_string_stringified}, {value_segments_stringified}{with_serialize_deserialize_upper_camel_case}{}>",
        //                      crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
        //                 );
        //                 type_stringified
        //                 .parse::<proc_macro2::TokenStream>()
        //                 .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //             },
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Path {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Path {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //             (
        //                 crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {..},
        //                 crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {..}
        //             ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_key_type_reference_stringified}", proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute)),
        //         }
        //     }
        //     else {
        //         panic!(
        //             "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
        //             proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
        //             <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
        //         );
        //     }
        // },
    }
}

pub fn generate_supported_container(
    field: &syn::Field,
    proc_macro_name_ident_stringified: &str,
) -> crate::error_occurence::supported_container::SupportedContainer {
    let syn_type_path_stringified = crate::naming_conventions::syn_type_path_stringified();
    let syn_type_reference = format!(
        "syn::Type::{}",
        <naming_constants::Reference as naming_constants::Naming>::upper_camel_case_stringified(),
    );
    let error_message = format!(
        "{} {syn_type_path_stringified} and {syn_type_reference}",
        naming_constants::SUPPORTS_ONLY_STRINGIFIED
    );
    match &field.ty {
        syn::Type::Path(type_path) => {
            let path = crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments);
            let vec_lifetime = crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                &type_path.path.segments,
                proc_macro_name_ident_stringified
            );
            let path_segment = type_path.path.segments.iter().last()
            .unwrap_or_else(|| panic!(
                "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().last() {}",
                naming_constants::IS_NONE_STRINGIFIED
            ));
            if path_segment.ident == <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
            {
                let vec_element_type = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = &path_segment.arguments {
                    if angle_brackets_generic_arguments.args.len() == 1 {
                        if let syn::GenericArgument::Type(type_handle) =
                            angle_brackets_generic_arguments.args
                            .iter().next()
                            .unwrap_or_else(|| panic!(
                                "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.into_iter().nth(0) {}",
                                naming_constants::IS_NONE_STRINGIFIED
                            ))
                        {
                            match type_handle {
                                syn::Type::Path(type_path) => crate::error_occurence::vec_element_type::VecElementType::Path{
                                    element_path: crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                                    vec_lifetime: crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                        &type_path.path.segments,
                                        proc_macro_name_ident_stringified
                                    )
                                },
                                syn::Type::Reference(type_reference) => {
                                    let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
                                        if type_path.path.segments.len() == 1 {
                                            type_path.path.segments
                                            .into_iter().next()
                                            .unwrap_or_else(|| panic!(
                                                "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                naming_constants::IS_NONE_STRINGIFIED
                                            ))
                                            .ident
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}",
                                            naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                        );
                                    };
                                    crate::error_occurence::vec_element_type::VecElementType::Reference {
                                        reference_ident,
                                        lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!(
                                            "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                            naming_constants::IS_NONE_STRINGIFIED
                                        )).ident
                                    }
                                },
                                syn::Type::Array(_) | 
                                syn::Type::BareFn(_) | 
                                syn::Type::Group(_) | 
                                syn::Type::ImplTrait(_) | 
                                syn::Type::Infer(_) | 
                                syn::Type::Macro(_) | 
                                syn::Type::Never(_) | 
                                syn::Type::Paren(_) | 
                                syn::Type::Ptr(_) | 
                                syn::Type::Slice(_) | 
                                syn::Type::TraitObject(_) | 
                                syn::Type::Tuple(_) | 
                                syn::Type::Verbatim(_) => panic!(
                                    "{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and {syn_type_reference}",
                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                ),
                                _ => panic!(
                                    "{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and {syn_type_reference} (exhaustive)",
                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                ),
                            }
                        }
                        else {
                            panic!(
                                "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args[0] {} {}",
                                naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                naming_constants::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                            );
                        }
                    }
                    else {
                        panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() == 1");
                    }
                }
                else {
                    panic!(
                        "{proc_macro_name_ident_stringified} path_segment.arguments {} syn::PathArguments::AngleBracketed",
                        naming_constants::SUPPORTS_ONLY_STRINGIFIED
                    );
                };
                crate::error_occurence::supported_container::SupportedContainer::Vec{
                    path,
                    vec_element_type
                }
            }
            else if path_segment.ident == <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified() {
                let (
                    hashmap_key_type,
                    hashmap_value_type
                ) = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = &path_segment.arguments {
                    if angle_brackets_generic_arguments.args.len() == 2 {
                        let (
                            key_generic_argument,
                            value_generic_argument
                        ) = {
                            let mut key_generic_argument_option = None;
                            let mut value_generic_argument_option = None;
                            angle_brackets_generic_arguments.args
                            .iter()
                            .enumerate()
                            .for_each(|(index, generic_argument)|{
                                match index {
                                    0 => {
                                        key_generic_argument_option = Some(generic_argument);
                                    }
                                    1 => {
                                        value_generic_argument_option = Some(generic_argument);
                                    }
                                    _ => panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() != 2")
                                }
                            });
                            (
                                key_generic_argument_option.unwrap_or_else(|| panic!(
                                    "{proc_macro_name_ident_stringified} key_generic_argument_option {}",
                                    naming_constants::IS_NONE_STRINGIFIED
                                )),
                                value_generic_argument_option.unwrap_or_else(|| panic!(
                                    "{proc_macro_name_ident_stringified} value_generic_argument_option {}",
                                    naming_constants::IS_NONE_STRINGIFIED
                                ))
                            )
                        };
                        let hashmap_key_type
                        = if let syn::GenericArgument::Type(type_handle) =
                            key_generic_argument
                        {
                            match type_handle {
                                syn::Type::Path(type_path) => {
                                    crate::error_occurence::hashmap_value_type::HashMapKeyType::Path{
                                        key_segments_stringified: crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                                        key_vec_lifetime: crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                            &type_path.path.segments,
                                            proc_macro_name_ident_stringified
                                        )
                                    }
                                },
                                syn::Type::Reference(type_reference) => {
                                    let key_reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
                                        if type_path.path.segments.len() == 1 {
                                            type_path.path.segments
                                            .into_iter().next()
                                            .unwrap_or_else(|| panic!(
                                                "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                naming_constants::IS_NONE_STRINGIFIED
                                            ))
                                            .ident
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}",
                                            naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                        );
                                    };
                                    crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                        key_reference_ident,
                                        key_lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!(
                                            "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                            naming_constants::IS_NONE_STRINGIFIED
                                        )).ident
                                    }
                                },
                                syn::Type::Array(_) | 
                                syn::Type::BareFn(_) | 
                                syn::Type::Group(_) | 
                                syn::Type::ImplTrait(_) | 
                                syn::Type::Infer(_) | 
                                syn::Type::Macro(_) | 
                                syn::Type::Never(_) | 
                                syn::Type::Paren(_) | 
                                syn::Type::Ptr(_) | 
                                syn::Type::Slice(_) | 
                                syn::Type::TraitObject(_) | 
                                syn::Type::Tuple(_) | 
                                syn::Type::Verbatim(_) => panic!(
                                    "{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and {syn_type_reference}",
                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                ),
                                _ => panic!(
                                    "{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and {syn_type_reference} (exhaustive)",
                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                ),
                            }
                        }
                        else {
                            panic!(
                                "{proc_macro_name_ident_stringified} key_generic_argument {} {}",
                                naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                naming_constants::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                            );
                        };
                        let hashmap_value_type = if let syn::GenericArgument::Type(type_handle) = value_generic_argument {
                            match type_handle {
                                syn::Type::Path(type_path) => {
                                   crate::error_occurence::hashmap_key_type::HashMapValueType::Path{
                                        value_segments_stringified: crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                                        value_vec_lifetime: crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                            &type_path.path.segments,
                                            proc_macro_name_ident_stringified
                                        )
                                    }
                                },
                                syn::Type::Reference(type_reference) => {
                                    let value_reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
                                        if type_path.path.segments.len() == 1 {
                                            type_path.path.segments
                                            .into_iter().next()
                                            .unwrap_or_else(|| panic!(
                                                "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                naming_constants::IS_NONE_STRINGIFIED
                                            ))
                                            .ident
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}",
                                            naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                        );
                                    };
                                   crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                        value_reference_ident,
                                        value_lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!(
                                            "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                            naming_constants::IS_NONE_STRINGIFIED
                                        )).ident
                                    }
                                },
                                syn::Type::Array(_) | 
                                syn::Type::BareFn(_) | 
                                syn::Type::Group(_) | 
                                syn::Type::ImplTrait(_) | 
                                syn::Type::Infer(_) | 
                                syn::Type::Macro(_) | 
                                syn::Type::Never(_) | 
                                syn::Type::Paren(_) | 
                                syn::Type::Ptr(_) | 
                                syn::Type::Slice(_) | 
                                syn::Type::TraitObject(_) | 
                                syn::Type::Tuple(_) | 
                                syn::Type::Verbatim(_) => panic!(
                                    "{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and syn::Type::Reference",
                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                ),
                                _ => panic!(
                                    "{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and syn::Type::Reference (exhaustive)",
                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                ),
                            }
                        }
                        else {
                            panic!(
                                "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args[0] {} {}",
                                naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                naming_constants::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                            );
                        };
                        (
                            hashmap_key_type,
                            hashmap_value_type,
                        )
                    }
                    else {
                        panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() == 2");
                    }
                }
                else {
                    panic!(
                        "{proc_macro_name_ident_stringified} path_segment.arguments {} syn::PathArguments::AngleBracketed",
                        naming_constants::SUPPORTS_ONLY_STRINGIFIED
                    );
                };
                crate::error_occurence::supported_container::SupportedContainer::HashMap{
                    path,
                    hashmap_key_type,
                    hashmap_value_type
                }
            }
            else {
                crate::error_occurence::supported_container::SupportedContainer::Path{
                    path,
                    vec_lifetime,
                }
            }
        },
        syn::Type::Reference(type_reference) => {
            let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
                if type_path.path.segments.len() == 1 {
                    type_path.path.segments
                    .into_iter().next()
                    .unwrap_or_else(|| panic!(
                        "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                        naming_constants::IS_NONE_STRINGIFIED
                    ))
                    .ident
                }
                else {
                    panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                }
            }
            else {
                panic!(
                    "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}",
                    naming_constants::SUPPORTS_ONLY_STRINGIFIED
                );
            };
            crate::error_occurence::supported_container::SupportedContainer::Reference{
                reference_ident,
                lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!(
                    "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                    naming_constants::IS_NONE_STRINGIFIED
                )).ident,
            }
        },
        syn::Type::Array(_) | 
        syn::Type::BareFn(_) | 
        syn::Type::Group(_) | 
        syn::Type::ImplTrait(_) | 
        syn::Type::Infer(_) | 
        syn::Type::Macro(_) | 
        syn::Type::Never(_) | 
        syn::Type::Paren(_) | 
        syn::Type::Ptr(_) | 
        syn::Type::Slice(_) | 
        syn::Type::TraitObject(_) | 
        syn::Type::Tuple(_) | 
        syn::Type::Verbatim(_) => panic!("{proc_macro_name_ident_stringified} field.ty is not syn::Type::Path or syn::Type::Reference {error_message}"),
        _ => panic!("{proc_macro_name_ident_stringified} field.ty is not syn::Type::Path or syn::Type::Reference {error_message} (exhaustive)"),
    }
}
