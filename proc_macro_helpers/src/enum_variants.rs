pub fn code_occurence_syn_field(proc_macro_name_upper_camel_case_ident_stringified: &str) -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        ident: Some(
            syn::Ident::new("code_occurence", proc_macro2::Span::call_site())
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
                    segments: generate_simple_syn_punctuated_punctuated(
                        &["error_occurence_lib","code_occurence","CodeOccurence"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                },
            },
        ),
    }
}

pub fn postgres_error_syn_variants(
    proc_macro_name_upper_camel_case_ident_stringified: &str
) -> [syn::Variant;15] {
    let std_string_string_syn_punctuated_punctuated = generate_simple_syn_punctuated_punctuated(
        &["std","string","String"],
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let code_occurence_field = code_occurence_syn_field(&proc_macro_name_upper_camel_case_ident_stringified);
    let configuration_error_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::configuration_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    //todo move it into custom macro attribute
    let database_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::database_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let io_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::io_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplay, 
                    &variant_name_snake_case_stringified,
                    generate_simple_syn_punctuated_punctuated(
                        &["std","io","Error"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                )
            ]
        )
    };
    let tls_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::tls_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let protocol_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::protocol_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let row_not_found_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::row_not_found_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr404NotFound,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let type_not_found_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::type_not_found_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr400BadRequest,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let column_index_out_of_bounds_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::column_index_out_of_bounds_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified,
                    generate_simple_syn_punctuated_punctuated(
                        &["usize"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                ),
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    "len",
                    generate_simple_syn_punctuated_punctuated(
                        &["usize"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                )
            ]
        )
    };
    let column_not_found_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::column_not_found_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr400BadRequest,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let column_decode_syn_variant = construct_syn_variant(
        crate::status_code::StatusCode::Tvfrr500InternalServerError,
        &crate::naming_conventions::column_decode_upper_camel_case_stringified(),
        &code_occurence_field,
        vec![
            (
                crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                "column_decode_index", 
                std_string_string_syn_punctuated_punctuated.clone()
            ),
            (
                crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                "source_handle", 
                std_string_string_syn_punctuated_punctuated.clone()
            )
        ]
    );
    let decode_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::decode_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let pool_timed_out_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::pool_timed_out_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr408RequestTimeout,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let pool_closed_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::pool_closed_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let worker_crashed_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::worker_crashed_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let migrate_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::migrate_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplay, 
                    &variant_name_snake_case_stringified,
                    generate_simple_syn_punctuated_punctuated(
                        &["sqlx","migrate","MigrateError"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                )
            ]
        )
    };
    [
        configuration_error_syn_variant,
        database_syn_variant,
        io_syn_variant,
        tls_syn_variant,
        protocol_syn_variant,
        row_not_found_syn_variant,
        type_not_found_syn_variant,
        column_index_out_of_bounds_syn_variant,
        column_not_found_syn_variant,
        column_decode_syn_variant,
        decode_syn_variant,
        pool_timed_out_syn_variant,
        pool_closed_syn_variant,
        worker_crashed_syn_variant,
        migrate_syn_variant
    ]
}

fn generate_simple_syn_punctuated_punctuated(
    parts_vec: &[&str],
    proc_macro_name_upper_camel_case_ident_stringified: &str
) -> syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2> {
    let parts_vec_len = parts_vec.len();
    match parts_vec_len >= 1 {
        true => {
            let parts_vec_len_minus_one = parts_vec_len - 1;
            let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();
            for (index, element) in parts_vec.iter().enumerate() {
                handle.push_value(
                    syn::PathSegment {
                        ident: proc_macro2::Ident::new(element, proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    }
                );
                match index < parts_vec_len_minus_one {
                    true => {
                        handle.push_punct(syn::token::Colon2{
                            spans: [proc_macro2::Span::call_site(),proc_macro2::Span::call_site()],
                        });
                    }
                    false => ()
                }
            }
            handle
        },
        false => panic!("{proc_macro_name_upper_camel_case_ident_stringified} generate_simple_syn_punctuated_punctuated parts_vec_len.len() > 1 == false for {parts_vec:?}")
    }
}

pub fn construct_syn_variant(
    tvfrr_status_code: crate::status_code::StatusCode,
    variant_name: &str,
    code_occurence_field: &syn::Field,
    fields: std::vec::Vec<(crate::error_occurence::named_attribute::NamedAttribute, &str, syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>)>
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
                                    syn::Ident::new(element.1, proc_macro2::Span::call_site())
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