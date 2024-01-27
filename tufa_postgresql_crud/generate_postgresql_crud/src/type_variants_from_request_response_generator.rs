pub fn type_variants_from_request_response_generator(
    desirable_status_code: &proc_macro_helpers::status_code::StatusCode,
    desirable_token_stream: &proc_macro2::TokenStream,
    desirable_type_token_stream: &proc_macro2::TokenStream,
    try_operation_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    operation_snake_case_stringified: &str,
    code_occurence_snake_case_double_dot_space_crate_common_code_occurence_code_occurence_token_stream: &proc_macro2::TokenStream,
    code_occurence_snake_case_crate_code_occurence_tufa_common_macro_call_token_stream: &proc_macro2::TokenStream,
    error_named_derive_token_stream: &proc_macro2::TokenStream,
    eo_display_token_stream: &proc_macro2::TokenStream,
    eo_display_foreign_type_token_stream: &proc_macro2::TokenStream,
    eo_display_with_serialize_deserialize_token_stream: &proc_macro2::TokenStream,
    derive_debug_serialize_deserialize_token_stream: &proc_macro2::TokenStream,
    derive_debug_serialize_deserialize_to_schema_token_stream: &proc_macro2::TokenStream,
    type_variants_from_request_response_syn_variants: std::vec::Vec<&syn::Variant>,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    code_occurence_upper_camel_case_stringified: &str,
    code_occurence_snake_case_stringified: &str,
    operation: &crate::Operation,
) -> proc_macro2::TokenStream {
    let try_operation_response_variants_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseStringified::try_self_response_variants_upper_camel_case_stringified(operation);
    let try_operation_response_variants_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(operation);
    let try_operation_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfWithSerializeDeserializeUpperCamelCaseTokenStream::try_self_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let try_operation_response_variants_desirable_status_code_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsStatusCodeTokenStream::try_self_response_variants_status_code_token_stream(operation, &desirable_status_code);
    let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(operation);
    let try_operation_with_serialize_deserialize_token_stream = proc_macro_helpers::naming_conventions::TrySelfWithSerializeDeserializeTokenStream::try_self_with_serialize_deserialize_token_stream(operation);
    let axum_http_status_code_quote_token_stream = desirable_status_code.to_axum_http_status_code_token_stream();
    let http_status_code_quote_token_stream = desirable_status_code.to_http_status_code_token_stream();
    let axum_http_status_code_token_stream = quote::quote!{axum::http::StatusCode};
    let http_status_code_token_stream = quote::quote!{http::StatusCode};
    let reqwest_header_header_map_token_stream = quote::quote!{reqwest::header::HeaderMap};
    let reqwest_error_token_stream = quote::quote!{reqwest::Error};
    let type_variants_from_request_response_syn_variants_len = type_variants_from_request_response_syn_variants.len();
    let crate_common_api_request_unexpected_error_api_request_unexpected_error_token_stream = quote::quote! {crate::common::api_request_unexpected_error::ApiRequestUnexpectedError};
    let crate_common_api_request_unexpected_error_response_text_result_token_stream = quote::quote! {crate::common::api_request_unexpected_error::ResponseTextResult};
    let try_operation_token_stream = {
        let try_operation_mapped_token_stream = type_variants_from_request_response_syn_variants.iter().map(|error_variant| {
            let variant_ident = &error_variant.ident;
            let fields_named = if let syn::Fields::Named(fields_named) = &error_variant.fields {
                fields_named
            }
            else {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
            };
            let fields_mapped_into_token_stream = fields_named.named.iter().map(|field|{
                let field_ident = field.ident.clone().unwrap_or_else(|| panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} field.ident {}",
                    proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                ));
                let error_occurence_attribute = match field_ident == code_occurence_snake_case_stringified {
                    true => quote::quote! {},
                    false => {
                        let mut error_occurence_attribute: Option<proc_macro_helpers::error_occurence::named_attribute::NamedAttribute> = None;
                        for element in &field.attrs {
                            if let true = element.path.segments.len() == 1 {
                                let segment = element.path.segments.first().unwrap_or_else(|| {panic!("{proc_macro_name_upper_camel_case_ident_stringified} element.path.segments.get(0) is None")});
                                if let Ok(value) = {
                                    use std::str::FromStr;
                                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::from_str(&segment.ident.to_string())
                                } {
                                    match error_occurence_attribute {
                                        Some(value) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} duplicated attributes ({}) are not supported", value.to_string()),
                                        None => {
                                            error_occurence_attribute = Some(value);
                                        }
                                    }
                                }
                            }
                        }
                        match error_occurence_attribute {
                            Some(value) => value.to_attribute_view_token_stream(),
                            None => panic!("{proc_macro_name_upper_camel_case_ident_stringified} {variant_ident} no supported attribute"),
                        }
                    }
                };
                let field_type = &field.ty;
                quote::quote! {
                    #error_occurence_attribute
                    #field_ident: #field_type
                }
            }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            quote::quote! {
                #variant_ident {
                    #(#fields_mapped_into_token_stream),*
                }
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            #[derive(
                Debug,
                thiserror::Error,
                error_occurence::ErrorOccurence,
                from_sqlx_postgres_error::FromSqlxPostgresError,
            )]
            pub enum #try_operation_upper_camel_case_token_stream {
                #(#try_operation_mapped_token_stream),*
            }
        }
    };
    let enum_with_serialize_deserialize_logic_token_stream_handle_token_stream = {
        let enum_with_serialize_deserialize_logic_mapped_token_stream = type_variants_from_request_response_syn_variants.iter().map(|error_variant| {
            let variant_ident = &error_variant.ident;
            let fields_named = if let syn::Fields::Named(fields_named) = &error_variant.fields {
                fields_named
            }
            else {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
            };
            let fields_mapped_into_token_stream = fields_named.named.iter().map(|field|{
                let field_ident = field.ident.clone().unwrap_or_else(|| panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} field.ident {}",
                    proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                ));
                let field_type_with_serialize_deserialize = match field_ident == code_occurence_snake_case_stringified {
                    true => {
                        let code_occurence_type_token_stream = {
                            if let syn::Type::Path(type_path) = &field.ty {
                                let mut code_occurence_type_repeat_checker = false;
                                let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
                                .fold(String::from(""), |mut acc, path_segment| {
                                    let path_segment_ident = &path_segment.ident;
                                    match *path_segment_ident == code_occurence_upper_camel_case_stringified {
                                        true => {
                                            if code_occurence_type_repeat_checker {
                                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} code_occurence_ident detected more than one {code_occurence_upper_camel_case_stringified} inside type path");
                                            }
                                            acc.push_str(&path_segment_ident.to_string());
                                            code_occurence_type_repeat_checker = true;
                                        },
                                        false => acc.push_str(&format!("{path_segment_ident}::")),
                                    }
                                    acc
                                });
                                if !code_occurence_type_repeat_checker {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} no {code_occurence_upper_camel_case_stringified} named field");
                                }
                                code_occurence_segments_stringified_handle.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_segments_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            }
                            else {
                                let syn_type_path_stringified = proc_macro_helpers::naming_conventions::syn_type_path_stringified();
                                panic!(
                                    "{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_snake_case_stringified} {} {syn_type_path_stringified}",
                                    proc_macro_helpers::naming_conventions::SUPPORTS_ONLY_STRINGIFIED
                                );
                            }
                        };
                        code_occurence_type_token_stream
                    },
                    false => {
                        let attribute = {
                            let mut option_attribute = None;
                            field.attrs.iter().for_each(|attr|{
                                if let true = attr.path.segments.len() == 1 {
                                    let error_message = format!("{proc_macro_name_upper_camel_case_ident_stringified} two or more supported attributes!");
                                    let attr_ident = match attr.path.segments.iter().next() {
                                        Some(path_segment) => &path_segment.ident,
                                        None => panic!("attr.path.segments.iter().next() is None"),
                                    };
                                    match {
                                        use std::str::FromStr;
                                        proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::from_str(&attr_ident.to_string())
                                    } {
                                        Ok(value) => {
                                            if let true = option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(value);
                                            }
                                        },
                                        Err(_) => ()//other attributes are not for this proc_macro
                                    }
                                }//other attributes are not for this proc_macro
                            });
                            option_attribute.unwrap_or_else(|| panic!(
                                "{proc_macro_name_upper_camel_case_ident_stringified} option attribute {}",
                                proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                            ))
                        };
                        let supported_container = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_supported_container(
                            &field,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let field_type_with_serialize_deserialize = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_field_type_with_serialize_deserialize_version(
                            attribute,
                            supported_container,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        field_type_with_serialize_deserialize
                    },
                };
                quote::quote! {#field_ident: #field_type_with_serialize_deserialize}
            }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            quote::quote! {
                #variant_ident {
                    #(#fields_mapped_into_token_stream),*
                }
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();      
        quote::quote! {
            #derive_debug_serialize_deserialize_token_stream
            pub enum #try_operation_response_variants_upper_camel_case_token_stream {
                #desirable_token_stream(#desirable_type_token_stream),
                #(#enum_with_serialize_deserialize_logic_mapped_token_stream),*
            }
        }
    };
    let from_logic_token_stream_handle_token_stream = {
        let from_logic_token_stream_mapped_token_stream = type_variants_from_request_response_syn_variants.iter().map(|error_variant| {
            let variant_ident = &error_variant.ident;
            let fields_named = if let syn::Fields::Named(fields_named) = &error_variant.fields {
                fields_named
            }
            else {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
            };
            let fields_name_mapped_into_token_stream = fields_named.named.iter().map(|field|{
                let field_ident = field.ident.clone().unwrap_or_else(|| panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} field.ident {}",
                    proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                ));
                quote::quote! {#field_ident}
            }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            quote::quote! {
                #try_operation_with_serialize_deserialize_upper_camel_case_token_stream::#variant_ident {
                    #(#fields_name_mapped_into_token_stream),*
                } => Self::#variant_ident {
                    #(#fields_name_mapped_into_token_stream),*
                }
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            impl std::convert::From<#try_operation_upper_camel_case_token_stream> for #try_operation_response_variants_upper_camel_case_token_stream {
                fn from(value: #try_operation_upper_camel_case_token_stream) -> Self {
                    match value.into_serialize_deserialize_version() {
                        #(#from_logic_token_stream_mapped_token_stream),*
                    }
                }
            }
        }
    };
    let impl_std_convert_from_ident_response_variants_token_stream_for_http_status_code_logic_token_stream_handle_token_stream = {
        let impl_std_convert_from_ident_response_variants_token_stream_for_http_status_code_logic_token_stream_handle_mapped_token_stream = type_variants_from_request_response_syn_variants
            .iter()
            .map(|error_variant| {
                let variant_ident = &error_variant.ident;
                let fields_named = if let syn::Fields::Named(fields_named) = &error_variant.fields {
                    fields_named
                }
                else {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
                };
                let fields_anonymous_types_mapped_into_token_stream = fields_named.named.iter().map(|field|{
                    let field_ident = field.ident.clone().unwrap_or_else(|| panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} field.ident {}",
                        proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                    ));
                    quote::quote! {#field_ident: _}
                }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                quote::quote! {
                    #try_operation_response_variants_upper_camel_case_token_stream::#variant_ident {
                        #(#fields_anonymous_types_mapped_into_token_stream),*
                    } => #axum_http_status_code_quote_token_stream
                }
            }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            impl std::convert::From<&#try_operation_response_variants_upper_camel_case_token_stream> for #axum_http_status_code_token_stream {
                fn from(value: &#try_operation_response_variants_upper_camel_case_token_stream) -> Self {
                    match value {
                        #try_operation_response_variants_upper_camel_case_token_stream::#desirable_token_stream(_) => #axum_http_status_code_quote_token_stream,
                        #(#impl_std_convert_from_ident_response_variants_token_stream_for_http_status_code_logic_token_stream_handle_mapped_token_stream),*//todo maybe something wrong with status codes? check it later (for all variants was axum::http::StatusCode::OK)
                    }
                }
            }
        }
    };
    let generated_status_code_enums_with_from_impls_logic_token_stream_handle_token_stream = {
        let generated_status_code_enums_with_from_impls_logic_token_stream = {
            let status_code_enums_with_from_impls_logic_token_stream = type_variants_from_request_response_syn_variants.iter().fold(
                std::collections::HashMap::<proc_macro_helpers::status_code::StatusCode, std::vec::Vec<(
                    &syn::Ident,
                    std::vec::Vec<(syn::Ident, proc_macro2::TokenStream)>,
                )>>::with_capacity(type_variants_from_request_response_syn_variants_len),
                |mut acc, element| {
                    let variant_ident = &element.ident;
                    let error_variant_status_code = proc_macro_helpers::status_code::StatusCode::try_from(element)
                    .unwrap_or_else(|e| {panic!("{proc_macro_name_upper_camel_case_ident_stringified} variant {variant_ident} failed: {e}")});
                    let fields_named = if let syn::Fields::Named(fields_named) = &element.fields {
                        fields_named
                    }
                    else {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
                    };
                    let error_variant_fields = fields_named.named.iter().map(|field|{
                        let field_ident = field.ident.clone().unwrap_or_else(|| panic!(
                            "{proc_macro_name_upper_camel_case_ident_stringified} field.ident {}",
                            proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                        ));
                        let field_type_with_serialize_deserialize = match field_ident == code_occurence_snake_case_stringified {
                            true => {
                                let code_occurence_type_token_stream = {
                                    if let syn::Type::Path(type_path) = &field.ty {
                                        let mut code_occurence_type_repeat_checker = false;
                                        let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
                                        .fold(String::from(""), |mut acc, path_segment| {
                                            let path_segment_ident = &path_segment.ident;
                                            match *path_segment_ident == code_occurence_upper_camel_case_stringified {
                                                true => {
                                                    if code_occurence_type_repeat_checker {
                                                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} code_occurence_ident detected more than one {code_occurence_upper_camel_case_stringified} inside type path");
                                                    }
                                                    acc.push_str(&path_segment_ident.to_string());
                                                    code_occurence_type_repeat_checker = true;
                                                },
                                                false => acc.push_str(&format!("{path_segment_ident}::")),
                                            }
                                            acc
                                        });
                                        if !code_occurence_type_repeat_checker {
                                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} no {code_occurence_upper_camel_case_stringified} named field");
                                        }
                                        code_occurence_segments_stringified_handle.parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_segments_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                    }
                                    else {
                                        let syn_type_path_stringified = proc_macro_helpers::naming_conventions::syn_type_path_stringified();
                                        panic!(
                                            "{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_snake_case_stringified} {} {syn_type_path_stringified}",
                                            proc_macro_helpers::naming_conventions::SUPPORTS_ONLY_STRINGIFIED
                                        );
                                    }
                                };
                                code_occurence_type_token_stream
                            },
                            false => {
                                let attribute = {
                                    let mut option_attribute = None;
                                    field.attrs.iter().for_each(|attr|{
                                        if let true = attr.path.segments.len() == 1 {
                                            let error_message = format!("{proc_macro_name_upper_camel_case_ident_stringified} two or more supported attributes!");
                                            let attr_ident = match attr.path.segments.iter().next() {
                                                Some(path_segment) => &path_segment.ident,
                                                None => panic!("attr.path.segments.iter().next() is None"),
                                            };
                                            match {
                                                use std::str::FromStr;
                                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::from_str(&attr_ident.to_string())
                                            } {
                                                Ok(value) => {
                                                    if let true = option_attribute.is_some() {
                                                        panic!("{error_message}");
                                                    }
                                                    else {
                                                        option_attribute = Some(value);
                                                    }
                                                },
                                                Err(_) => ()//other attributes are not for this proc_macro
                                            }
                                        }//other attributes are not for this proc_macro
                                    });
                                    option_attribute.unwrap_or_else(|| panic!(
                                        "{proc_macro_name_upper_camel_case_ident_stringified} option attribute {}",
                                        proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                                    ))
                                };
                                let supported_container = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_supported_container(
                                    &field,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_field_type_with_serialize_deserialize_version(
                                    attribute,
                                    supported_container,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                )
                            },
                        };
                        (field_ident.clone(), field_type_with_serialize_deserialize)
                    }).collect::<Vec<(syn::Ident, proc_macro2::TokenStream)>>();
                    let error_variant = (
                        variant_ident,
                        error_variant_fields
                    );
                    match acc.get_mut(&error_variant_status_code) {
                        Some(value) => {
                            value.push(error_variant);
                        },
                        None => {
                            acc.insert(error_variant_status_code, vec![error_variant]);
                        }
                    }
                    acc
                },
            )
            .into_iter().map(|(key,value)|{
                let try_operation_response_variants_status_code_token_stream = {
                    let try_operation_response_variants_status_code_stingified = format!("{try_operation_response_variants_upper_camel_case_stringified}{key}");
                    try_operation_response_variants_status_code_stingified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {try_operation_response_variants_status_code_stingified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                // let try_operation_response_variants_desirable_attribute_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsStatusCodeTokenStream::try_self_response_variants_status_code_token_stream(operation, &status_code_attribute);
                // let try_operation_response_variants_attribute_token_stream = crate::generate_try_operation_response_variants_desirable_attribute_token_stream(
                //     &try_upper_camel_case_stringified,
                //     &operation_name_upper_camel_case_stringified,
                //     &response_variants_upper_camel_case_stringified,
                //     &key,
                //     &proc_macro_name_upper_camel_case_ident_stringified
                // );
                let enum_variants_token_stream = value.iter().map(|element|{
                    let error_variant_ident = &element.0;
                    let fields_mapped_into_token_stream = element.1.iter().map(|element| {
                        let field_name_token_stream = &element.0;
                        let field_type_token_stream = &element.1;
                        quote::quote! {#field_name_token_stream: #field_type_token_stream}
                    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                    quote::quote!{
                        #error_variant_ident {
                            #(#fields_mapped_into_token_stream),*
                        }
                    }
                });
                let std_convert_from_match_variants_token_stream = value.iter().map(|element|{
                    let error_variant_ident = &element.0;
                    let fields_name_mapped_into_token_stream = element.1.iter().map(|element| {
                        let field_name_token_stream = &element.0;
                        quote::quote! {#field_name_token_stream}
                    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                    quote::quote!{
                        #try_operation_response_variants_status_code_token_stream::#error_variant_ident {
                            #(#fields_name_mapped_into_token_stream),*
                        } => Self::#error_variant_ident {
                            #(#fields_name_mapped_into_token_stream),*
                        }
                    }
                });
                quote::quote!{
                    #derive_debug_serialize_deserialize_to_schema_token_stream
                    pub enum #try_operation_response_variants_status_code_token_stream {
                        #(#enum_variants_token_stream),*
                    }
                    impl std::convert::From<#try_operation_response_variants_status_code_token_stream> for #try_operation_response_variants_upper_camel_case_token_stream {
                        fn from(value: #try_operation_response_variants_status_code_token_stream) -> Self {
                            match value {
                                #(#std_convert_from_match_variants_token_stream),*
                            }
                        }
                    }
                }
            });
            quote::quote! {
                #(#status_code_enums_with_from_impls_logic_token_stream)*
            }
        };
        //todo need to add table name prefix to enum names
        quote::quote! {
            #derive_debug_serialize_deserialize_to_schema_token_stream
            pub enum #try_operation_response_variants_desirable_status_code_token_stream {
                #desirable_token_stream(#desirable_type_token_stream),
            }
            impl std::convert::From<#try_operation_response_variants_desirable_status_code_token_stream> for #try_operation_response_variants_upper_camel_case_token_stream {
                fn from(value: #try_operation_response_variants_desirable_status_code_token_stream) -> Self {
                    match value {
                        #try_operation_response_variants_desirable_status_code_token_stream::#desirable_token_stream(i) => Self::#desirable_token_stream(i),
                    }
                }
            }
            #generated_status_code_enums_with_from_impls_logic_token_stream
        }
    };
    let try_from_response_logic_token_stream_handle_token_stream = {
        let (
            unique_status_codes,
            unique_status_codes_len,
            unique_status_codes_len_minus_one
         ) = {
            let hashmap_unique_status_codes = type_variants_from_request_response_syn_variants.iter().fold(//todo maybe not need hashmap here? maybe just unique vec?
                std::collections::HashMap::<proc_macro_helpers::status_code::StatusCode, std::vec::Vec<(
                    &syn::Ident,
                    std::vec::Vec<(syn::Ident, proc_macro2::TokenStream)>,
                )>>::with_capacity(type_variants_from_request_response_syn_variants_len),
                |mut acc, element| {
                    let variant_ident = &element.ident;
                    let error_variant_attribute = proc_macro_helpers::status_code::StatusCode::try_from(element)
                    .unwrap_or_else(|e| {panic!("{proc_macro_name_upper_camel_case_ident_stringified} variant {variant_ident} failed: {e}")});
                    let fields_named = if let syn::Fields::Named(fields_named) = &element.fields {
                        fields_named
                    }
                    else {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
                    };
                    let error_variant_fields = fields_named.named.iter().map(|field|{
                        let field_ident = field.ident.clone().unwrap_or_else(|| panic!(
                            "{proc_macro_name_upper_camel_case_ident_stringified} field.ident {}",
                            proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                        ));
                        let field_type_with_serialize_deserialize = match field_ident == code_occurence_snake_case_stringified {
                            true => {
                                let code_occurence_type_token_stream = {
                                    if let syn::Type::Path(type_path) = &field.ty {
                                        let mut code_occurence_type_repeat_checker = false;
                                        let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
                                        .fold(String::from(""), |mut acc, path_segment| {
                                            let path_segment_ident = &path_segment.ident;
                                            match *path_segment_ident == code_occurence_upper_camel_case_stringified {
                                                true => {
                                                    if code_occurence_type_repeat_checker {
                                                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} code_occurence_ident detected more than one {code_occurence_upper_camel_case_stringified} inside type path");
                                                    }
                                                    acc.push_str(&path_segment_ident.to_string());
                                                    code_occurence_type_repeat_checker = true;
                                                },
                                                false => acc.push_str(&format!("{path_segment_ident}::")),
                                            }
                                            acc
                                        });
                                        if !code_occurence_type_repeat_checker {
                                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} no {code_occurence_upper_camel_case_stringified} named field");
                                        }
                                        code_occurence_segments_stringified_handle.parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_segments_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                    }
                                    else {
                                        let syn_type_path_stringified = proc_macro_helpers::naming_conventions::syn_type_path_stringified();
                                        panic!(
                                            "{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_snake_case_stringified} {} {syn_type_path_stringified}",
                                            proc_macro_helpers::naming_conventions::SUPPORTS_ONLY_STRINGIFIED
                                        );
                                    }
                                };
                                code_occurence_type_token_stream
                            },
                            false => {
                                let attribute = {
                                    let mut option_attribute = None;
                                    field.attrs.iter().for_each(|attr|{
                                        if let true = attr.path.segments.len() == 1 {
                                            let error_message = format!("{proc_macro_name_upper_camel_case_ident_stringified} two or more supported attributes!");
                                            let attr_ident = match attr.path.segments.iter().next() {
                                                Some(path_segment) => &path_segment.ident,
                                                None => panic!("attr.path.segments.iter().next() is None"),
                                            };
                                            match {
                                                use std::str::FromStr;
                                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::from_str(&attr_ident.to_string())
                                            } {
                                                Ok(value) => {
                                                    if let true = option_attribute.is_some() {
                                                        panic!("{error_message}");
                                                    }
                                                    else {
                                                        option_attribute = Some(value);
                                                    }
                                                },
                                                Err(_) => ()//other attributes are not for this proc_macro
                                            }
                                        }//other attributes are not for this proc_macro
                                    });
                                    option_attribute.unwrap_or_else(|| panic!(
                                        "{proc_macro_name_upper_camel_case_ident_stringified} option attribute {}",
                                        proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                                    ))
                                };
                                let supported_container = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_supported_container(
                                    &field,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                let field_type_with_serialize_deserialize = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_field_type_with_serialize_deserialize_version(
                                    attribute,
                                    supported_container,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                field_type_with_serialize_deserialize
                            },
                        };
                        (field_ident, field_type_with_serialize_deserialize)
                    }).collect::<Vec<(syn::Ident, proc_macro2::TokenStream)>>();
                    let error_variant = (
                        variant_ident,
                        error_variant_fields,
                    );
                    match acc.get_mut(&error_variant_attribute) {
                        Some(value) => {
                            value.push(error_variant);
                        },
                        None => {
                            acc.insert(error_variant_attribute, vec![error_variant]);
                        }
                    }
                    acc
                },
            );
            let unique_status_codes_len = hashmap_unique_status_codes.len();
            if unique_status_codes_len < 1 {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} unique_status_codes_len < 1 {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE);
            }
            let unique_status_codes_len_minus_one = unique_status_codes_len - 1;
            let unique_status_codes = hashmap_unique_status_codes.into_iter().map(|(key, _)|key).collect::<std::vec::Vec<proc_macro_helpers::status_code::StatusCode>>();
            (
                unique_status_codes,
                unique_status_codes_len,
                unique_status_codes_len_minus_one
            )
        };
        let desirable_enum_name = {
            let status_code_enum_name_stingified = format!("{try_operation_response_variants_upper_camel_case_token_stream}{desirable_status_code}");
            status_code_enum_name_stingified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {status_code_enum_name_stingified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let api_request_unexpected_error_module_path_token_stream = quote::quote! { crate::common::api_request_unexpected_error };
        let api_request_unexpected_error_path_token_stream = quote::quote! { #api_request_unexpected_error_module_path_token_stream::ApiRequestUnexpectedError };
        let try_from_response_operation_snake_case_token_stream = {
            let ident_response_variants_attribute_stingified = format!("try_from_response_try_{operation_snake_case_stringified}");
            ident_response_variants_attribute_stingified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_response_variants_attribute_stingified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let status_code_enums_try_from = {
            let mut is_last_element_found = false;
            let desirable_status_code_case_token_stream = quote::quote! {
                match response.text().await {
                    Ok(response_text) => match serde_json::from_str::<#desirable_enum_name>(&response_text){
                        Ok(value) => Ok(#try_operation_response_variants_upper_camel_case_token_stream::from(value)), 
                        Err(e) => Err(
                            #api_request_unexpected_error_path_token_stream::DeserializeBody{ 
                                serde: e,
                                status_code,
                                headers,response_text
                            }
                        ),
                    },
                    Err(e) => Err(
                        #api_request_unexpected_error_path_token_stream::FailedToGetResponseText {
                            reqwest: e,
                            status_code,
                            headers,
                        }
                    ),
                }
            };
            let mut status_code_enums_try_from_variants = std::vec::Vec::with_capacity(unique_status_codes_len + 1);
            status_code_enums_try_from_variants.push(quote::quote! {
                if status_code == #http_status_code_quote_token_stream {
                    #desirable_status_code_case_token_stream
                }
            });
            unique_status_codes
            .into_iter()
            .enumerate()
            .for_each(|(index, status_code_attribute)|{
                // let status_code_enum_name_stringified = format!("{try_operation_response_variants_upper_camel_case_token_stream}{status_code_attribute}");
                // let status_code_enum_name_token_stream = status_code_enum_name_stringified
                //     .parse::<proc_macro2::TokenStream>()
                //     .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {status_code_enum_name_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                let try_operation_response_variants_desirable_attribute_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsStatusCodeTokenStream::try_self_response_variants_status_code_token_stream(operation, &status_code_attribute);
                
                // crate::generate_try_operation_response_variants_desirable_attribute_token_stream(
                //     &try_upper_camel_case_stringified,
                //     &operation_name_upper_camel_case_stringified,
                //     &response_variants_upper_camel_case_stringified,
                //     &status_code_attribute,
                //     &proc_macro_name_upper_camel_case_ident_stringified
                // );
                let http_status_code_token_stream = status_code_attribute.to_http_status_code_token_stream();
                match index == unique_status_codes_len_minus_one{
                    true => {
                        is_last_element_found = true;
                        status_code_enums_try_from_variants.push(quote::quote! {
                            else {
                                match response.text().await {
                                    Ok(response_text) => Err(
                                        #api_request_unexpected_error_path_token_stream::StatusCode {
                                            status_code,
                                            headers,
                                            response_text_result: #api_request_unexpected_error_module_path_token_stream::ResponseTextResult::ResponseText(response_text)
                                        },
                                    ),
                                    Err(e) => Err(
                                        #api_request_unexpected_error_path_token_stream::StatusCode {
                                            status_code,
                                            headers,
                                            response_text_result: #api_request_unexpected_error_module_path_token_stream::ResponseTextResult::ReqwestError(e),
                                        },
                                    ),
                                }
                            }
                        });
                    },
                    false => {
                        if let false = *desirable_status_code == status_code_attribute {
                            status_code_enums_try_from_variants.push(quote::quote! {
                                else if status_code == #http_status_code_token_stream {
                                    match response.text().await {
                                        Ok(response_text) => match serde_json::from_str::<#try_operation_response_variants_desirable_attribute_token_stream>(&response_text){
                                            Ok(value) => Ok(#try_operation_response_variants_upper_camel_case_token_stream::from(value)), 
                                            Err(e) => Err(
                                                #api_request_unexpected_error_path_token_stream::DeserializeBody{ 
                                                    serde: e,
                                                    status_code,
                                                    headers,response_text
                                                }
                                            ),
                                        },
                                        Err(e) => Err(
                                            #api_request_unexpected_error_path_token_stream::FailedToGetResponseText {
                                                reqwest: e,
                                                status_code,
                                                headers,
                                            }
                                        ),
                                    }
                                }
                            });
                        }
                    },
                }
            });
            if let false = is_last_element_found {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} false = is_last_element_found");
            }
            status_code_enums_try_from_variants
        };
        quote::quote! {
            async fn #try_from_response_operation_snake_case_token_stream(response: reqwest::Response) -> Result<#try_operation_response_variants_upper_camel_case_token_stream, #api_request_unexpected_error_path_token_stream> {
                let status_code = response.status();
                let headers = response.headers().clone();
                #(#status_code_enums_try_from)*
            }
        }
    };
    let impl_try_from_ident_response_variants_token_stream_for_desirable_logic_token_stream_handle_token_stream = {
        let impl_try_from_ident_response_variants_token_stream_for_desirable_logic_handle_mapped_token_stream = type_variants_from_request_response_syn_variants
            .iter()
            .map(
                |error_variant_attribute| {
                    let variant_ident = &error_variant_attribute.ident;
                    let fields_named = if let syn::Fields::Named(fields_named) = &error_variant_attribute.fields {
                        fields_named
                    }
                    else {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
                    };
                    let fields_name_mapped_into_token_stream = fields_named.named.iter().map(|field|{
                        let field_ident = field.ident.clone().unwrap_or_else(|| panic!(
                            "{proc_macro_name_upper_camel_case_ident_stringified} field.ident {}",
                            proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                        ));
                        quote::quote! {#field_ident}
                    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                    quote::quote! {
                        #try_operation_response_variants_upper_camel_case_token_stream::#variant_ident {
                            #(#fields_name_mapped_into_token_stream),*
                        } => Err(#try_operation_with_serialize_deserialize_upper_camel_case_token_stream::#variant_ident {
                            #(#fields_name_mapped_into_token_stream),*
                        })
                    }
                },
            )
            .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            impl TryFrom<#try_operation_response_variants_upper_camel_case_token_stream> for #desirable_type_token_stream {
                type Error = #try_operation_with_serialize_deserialize_upper_camel_case_token_stream;
                fn try_from(value: #try_operation_response_variants_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                    match value {
                        #try_operation_response_variants_upper_camel_case_token_stream::#desirable_token_stream(i) => Ok(i),
                        #(#impl_try_from_ident_response_variants_token_stream_for_desirable_logic_handle_mapped_token_stream),*
                    }
                }
            }
        }
    };
    let ident_request_error_logic_token_stream_handle_token_stream = {
        quote::quote! {
            #error_named_derive_token_stream
            pub enum #try_operation_request_error_upper_camel_case_token_stream {
                ExpectedType {
                    #eo_display_with_serialize_deserialize_token_stream
                    expected_type: #try_operation_with_serialize_deserialize_token_stream,
                    #code_occurence_snake_case_double_dot_space_crate_common_code_occurence_code_occurence_token_stream,
                },
                UnexpectedStatusCode {
                    #eo_display_token_stream
                    status_code: #http_status_code_token_stream,
                    #eo_display_foreign_type_token_stream
                    headers: #reqwest_header_header_map_token_stream,
                    #eo_display_foreign_type_token_stream
                    response_text_result: #crate_common_api_request_unexpected_error_response_text_result_token_stream,
                    #code_occurence_snake_case_double_dot_space_crate_common_code_occurence_code_occurence_token_stream,
                },
                FailedToGetResponseText {
                    #eo_display_foreign_type_token_stream
                    reqwest: #reqwest_error_token_stream,
                    #eo_display_token_stream
                    status_code: #http_status_code_token_stream,
                    #eo_display_foreign_type_token_stream
                    headers: #reqwest_header_header_map_token_stream,
                    #code_occurence_snake_case_double_dot_space_crate_common_code_occurence_code_occurence_token_stream,
                },
                DeserializeResponse {
                    #eo_display_token_stream
                    serde: serde_json::Error,
                    #eo_display_token_stream
                    status_code: #http_status_code_token_stream,
                    #eo_display_foreign_type_token_stream
                    headers: #reqwest_header_header_map_token_stream,
                    #eo_display_with_serialize_deserialize_token_stream
                    response_text: std::string::String,
                    #code_occurence_snake_case_double_dot_space_crate_common_code_occurence_code_occurence_token_stream,
                },
                Reqwest {
                    #eo_display_foreign_type_token_stream
                    reqwest: #reqwest_error_token_stream,
                    #code_occurence_snake_case_double_dot_space_crate_common_code_occurence_code_occurence_token_stream,
                },
            }
        }
    };
    let extraction_logic_token_stream_handle_token_stream = {
        let tvfrr_extraction_logic_try_operation_snake_case_token_stream = {
            let tvfrr_extraction_logic_try_operation_snake_case_stringified = format!("tvfrr_extraction_logic_try_{operation_snake_case_stringified}");
            tvfrr_extraction_logic_try_operation_snake_case_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {tvfrr_extraction_logic_try_operation_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let try_from_response_try_operation_snake_case_token_stream = {
            let try_from_response_try_operation_snake_case_stringified =
                format!("try_from_response_try_{operation_snake_case_stringified}");
            try_from_response_try_operation_snake_case_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {try_from_response_try_operation_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let try_from_response_try_operation_snake_case_token_stream_result_ok_token_stream = quote::quote!{
            match #desirable_type_token_stream::try_from(variants){
                Ok(value) => Ok(value),
                Err(e) => Err(#try_operation_request_error_upper_camel_case_token_stream::ExpectedType {
                    expected_type: e,
                    #code_occurence_snake_case_crate_code_occurence_tufa_common_macro_call_token_stream,
                }),
            }
        };
        quote::quote! {
            async fn #tvfrr_extraction_logic_try_operation_snake_case_token_stream<'a>(
                future: impl std::future::Future<Output = Result<reqwest::Response, #reqwest_error_token_stream>>,
            ) -> Result<
                #desirable_type_token_stream,
                #try_operation_request_error_upper_camel_case_token_stream,
            > {
                match future.await {
                    Ok(response) => match #try_from_response_try_operation_snake_case_token_stream(response).await {
                        Ok(variants) => #try_from_response_try_operation_snake_case_token_stream_result_ok_token_stream,
                        Err(e) => match e {
                            #crate_common_api_request_unexpected_error_api_request_unexpected_error_token_stream::StatusCode {
                                status_code,
                                headers,
                                response_text_result,
                            } => Err(#try_operation_request_error_upper_camel_case_token_stream::UnexpectedStatusCode {
                                status_code,
                                headers,
                                response_text_result,
                                #code_occurence_snake_case_crate_code_occurence_tufa_common_macro_call_token_stream
                            }),
                            #crate_common_api_request_unexpected_error_api_request_unexpected_error_token_stream::FailedToGetResponseText {
                                reqwest,
                                status_code,
                                headers
                            } => Err(#try_operation_request_error_upper_camel_case_token_stream::FailedToGetResponseText {
                                reqwest,
                                status_code,
                                headers,
                                #code_occurence_snake_case_crate_code_occurence_tufa_common_macro_call_token_stream
                            }),
                            #crate_common_api_request_unexpected_error_api_request_unexpected_error_token_stream::DeserializeBody {
                                serde,
                                status_code,
                                headers,
                                response_text,
                            } => Err(#try_operation_request_error_upper_camel_case_token_stream::DeserializeResponse {
                                serde,
                                status_code,
                                headers,
                                response_text,
                                #code_occurence_snake_case_crate_code_occurence_tufa_common_macro_call_token_stream
                            }),
                        },
                    },
                    Err(e) => Err(#try_operation_request_error_upper_camel_case_token_stream::Reqwest {
                        reqwest: e,
                        #code_occurence_snake_case_crate_code_occurence_tufa_common_macro_call_token_stream,
                    }),
                }
            }
        }
    };
    let enum_status_codes_checker_name_logic_token_stream_handle_token_stream = {
        let enum_status_codes_checker_upper_camel_case_token_stream = {
            let enum_status_codes_checker_upper_camel_case_stringified = format!("{try_operation_upper_camel_case_token_stream}StatusCodesChecker");
            enum_status_codes_checker_upper_camel_case_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {enum_status_codes_checker_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let enum_status_codes_checker_name_logic_token_stream_handle_mapped_token_stream = type_variants_from_request_response_syn_variants.iter().map(|error_variant_attribute| {
                let variant_ident = &error_variant_attribute.ident;
                let error_variant_attribute = proc_macro_helpers::status_code::StatusCode::try_from(error_variant_attribute)
                .unwrap_or_else(|e| {panic!("{proc_macro_name_upper_camel_case_ident_stringified} variant {variant_ident} failed: {e}")});
                let variant_ident_attribute_upper_camel_case_token_stream = {
                    let variant_ident_attribute_upper_camel_case_stringified = format!("{variant_ident}{error_variant_attribute}");
                    variant_ident_attribute_upper_camel_case_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {variant_ident_attribute_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    #variant_ident_attribute_upper_camel_case_token_stream,
                }
            },
        ).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            pub enum #enum_status_codes_checker_upper_camel_case_token_stream {
                #(#enum_status_codes_checker_name_logic_token_stream_handle_mapped_token_stream)*
            }
        }
    };
    let axum_response_into_response_logic_token_stream_handle_token_stream = {
        let axum_response_into_response_logic_token_stream_handle_mapped_token_stream = type_variants_from_request_response_syn_variants.iter().map(
            |error_variant_attribute| {
                let variant_ident = &error_variant_attribute.ident;
                let fields_named = if let syn::Fields::Named(fields_named) = &error_variant_attribute.fields {
                    fields_named
                }
                else {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
                };
                let fields_anonymous_types_mapped_into_token_stream = fields_named.named.iter().map(|field|{
                    let field_ident = field.ident.clone().unwrap_or_else(|| panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} field.ident {}",
                        proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
                    ));
                    quote::quote! {#field_ident: _}
                }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                quote::quote! {
                    #try_operation_response_variants_upper_camel_case_token_stream::#variant_ident {
                        #(#fields_anonymous_types_mapped_into_token_stream),*
                    } => {
                        let mut res = axum::Json(self).into_response();
                        *res.status_mut() = #axum_http_status_code_quote_token_stream;
                        res
                    }
                }
            }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            impl axum::response::IntoResponse for #try_operation_response_variants_upper_camel_case_token_stream {
                fn into_response(self) -> axum::response::Response {
                    match &self {
                        #try_operation_response_variants_upper_camel_case_token_stream::#desirable_token_stream(_) => {
                            let mut res = axum::Json(self).into_response();
                            *res.status_mut() = #axum_http_status_code_quote_token_stream;
                            res
                        }
                        #(#axum_response_into_response_logic_token_stream_handle_mapped_token_stream),*
                    }
                }
            }
        }
    };
    // println!("{}");
    quote::quote! {
        #try_operation_token_stream
        #enum_with_serialize_deserialize_logic_token_stream_handle_token_stream
        #from_logic_token_stream_handle_token_stream
        #impl_std_convert_from_ident_response_variants_token_stream_for_http_status_code_logic_token_stream_handle_token_stream
        #generated_status_code_enums_with_from_impls_logic_token_stream_handle_token_stream
        #try_from_response_logic_token_stream_handle_token_stream
        #impl_try_from_ident_response_variants_token_stream_for_desirable_logic_token_stream_handle_token_stream
        #ident_request_error_logic_token_stream_handle_token_stream
        #extraction_logic_token_stream_handle_token_stream
        #enum_status_codes_checker_name_logic_token_stream_handle_token_stream
        #axum_response_into_response_logic_token_stream_handle_token_stream
    }
}

pub fn construct_syn_variant(
    tvfrr_status_code: proc_macro_helpers::status_code::StatusCode,
    variant_name: &str,
    code_occurence_field: &syn::Field,
    fields: std::vec::Vec<(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute, &str, syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>)>
) -> syn::Variant {
    syn::Variant {
        attrs: vec![
            syn::Attribute {
                pound_token: syn::token::Pound {
                    spans: [proc_macro2::Span::call_site()],
                },
                style: syn::AttrStyle::Outer,
                bracket_token: syn::token::Bracket {
                    span: proc_macro2::Span::call_site(),
                },
                path: syn::Path {
                    leading_colon: None,
                    segments: {
                        let mut handle = syn::punctuated::Punctuated::new();
                        handle.push(syn::PathSegment {
                            ident: proc_macro2::Ident::new(&proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&tvfrr_status_code), proc_macro2::Span::call_site()),
                            arguments: syn::PathArguments::None,
                        });
                       handle
                    },
                },
                tokens: proc_macro2::TokenStream::new(),
            },
        ],
        ident: syn::Ident::new(variant_name, proc_macro2::Span::call_site()),
        fields: syn::Fields::Named(
            syn::FieldsNamed {
                brace_token: syn::token::Brace {
                    span: proc_macro2::Span::call_site(),
                },
                named: {
                    let mut handle = fields.into_iter().fold(syn::punctuated::Punctuated::new(), |mut acc, element| {
                        acc.push_value(
                            syn::Field {
                                attrs: vec![
                                    syn::Attribute {
                                        pound_token: syn::token::Pound {
                                            spans: [proc_macro2::Span::call_site()],
                                        },
                                        style: syn::AttrStyle::Outer,
                                        bracket_token: syn::token::Bracket {
                                            span: proc_macro2::Span::call_site(),
                                        },
                                        path: syn::Path {
                                            leading_colon: None,
                                            segments: {
                                                let mut handle = syn::punctuated::Punctuated::new();
                                                handle.push(
                                                    syn::PathSegment {
                                                        ident: proc_macro2::Ident::new(&element.0.to_string(), proc_macro2::Span::call_site()),
                                                        arguments: syn::PathArguments::None,
                                                    }
                                                );
                                                handle
                                            },
                                        },
                                        tokens: proc_macro2::TokenStream::new(),
                                    },
                                ],
                                vis: syn::Visibility::Inherited,
                                ident: Some(
                                    syn::Ident::new(&element.1, proc_macro2::Span::call_site())
                                ),
                                colon_token: Some(
                                    syn::token::Colon {
                                        spans: [proc_macro2::Span::call_site()],
                                    },
                                ),
                                ty: syn::Type::Path(
                                    syn::TypePath {
                                        qself: None,
                                        path: syn::Path {
                                            leading_colon: None,
                                            segments: element.2
                                        },
                                    },
                                ),
                            }
                        );
                        acc.push_punct(
                            syn::token::Comma {
                                spans: [proc_macro2::Span::call_site()],
                            }
                        );
                        acc
                    });
                    handle.push_value(code_occurence_field.clone());
                    handle
                },
            },
        ),
        discriminant: None,
    }
}