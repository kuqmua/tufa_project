pub fn construct_syn_variant_with_status_code(
    status_code: crate::status_code::StatusCode,
    variant_name: &impl std::fmt::Display,
    fields: std::vec::Vec<(
        crate::error_occurence::ErrorOccurenceFieldAttribute,
        &impl std::fmt::Display,
        syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
    )>,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> syn::Variant {
    syn::Variant {
        attrs: vec![syn::Attribute {
            pound_token: syn::token::Pound {
                spans: [proc_macro2::Span::call_site()],
            },
            style: syn::AttrStyle::Outer,
            bracket_token: syn::token::Bracket::default(),
            meta: syn::Meta::Path(syn::Path {
                leading_colon: None,
                segments: {
                    let mut handle = syn::punctuated::Punctuated::new();
                    handle.push(syn::PathSegment {
                            ident: proc_macro2::Ident::new(&proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&status_code), proc_macro2::Span::call_site()),
                            arguments: syn::PathArguments::None,
                        });
                    handle
                },
            }),
        }],
        ident: syn::Ident::new(&variant_name.to_string(), proc_macro2::Span::call_site()),
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: {
                let mut handle = fields.into_iter().fold(
                    syn::punctuated::Punctuated::new(),
                    |mut acc, element| {
                        acc.push_value(syn::Field {
                            attrs: vec![syn::Attribute {
                                pound_token: syn::token::Pound {
                                    spans: [proc_macro2::Span::call_site()],
                                },
                                style: syn::AttrStyle::Outer,
                                bracket_token: syn::token::Bracket::default(),
                                meta: syn::Meta::Path(syn::Path {
                                    leading_colon: None,
                                    segments: {
                                        let mut handle = syn::punctuated::Punctuated::new();
                                        handle.push(syn::PathSegment {
                                            ident: proc_macro2::Ident::new(
                                                proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&element.0),
                                                proc_macro2::Span::call_site(),
                                            ),
                                            arguments: syn::PathArguments::None,
                                        });
                                        handle
                                    },
                                }),
                            }],
                            vis: syn::Visibility::Inherited,
                            mutability: syn::FieldMutability::None,
                            ident: Some(syn::Ident::new(&element.1.to_string(), proc_macro2::Span::call_site())),
                            colon_token: Some(syn::token::Colon {
                                spans: [proc_macro2::Span::call_site()],
                            }),
                            ty: syn::Type::Path(syn::TypePath {
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    segments: element.2,
                                },
                            }),
                        });
                        acc.push_punct(syn::token::Comma {
                            spans: [proc_macro2::Span::call_site()],
                        });
                        acc
                    },
                );
                handle.push_value(crate::code_occurence_syn_field::code_occurence_syn_field(&proc_macro_name_upper_camel_case_ident_stringified));
                handle
            },
        }),
        discriminant: None,
    }
}

pub fn construct_syn_variant(
    variant_name: &impl std::fmt::Display,
    fields: std::vec::Vec<(
        crate::error_occurence::ErrorOccurenceFieldAttribute,
        &impl std::fmt::Display,
        syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
    )>,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> syn::Variant {
    syn::Variant {
        attrs: vec![],
        ident: syn::Ident::new(&variant_name.to_string(), proc_macro2::Span::call_site()),
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: {
                let mut handle = fields.into_iter().fold(
                    syn::punctuated::Punctuated::new(),
                    |mut acc, element| {
                        acc.push_value(syn::Field {
                            attrs: vec![syn::Attribute {
                                pound_token: syn::token::Pound {
                                    spans: [proc_macro2::Span::call_site()],
                                },
                                style: syn::AttrStyle::Outer,
                                bracket_token: syn::token::Bracket::default(),
                                meta: syn::Meta::Path(syn::Path {
                                    leading_colon: None,
                                    segments: {
                                        let mut handle = syn::punctuated::Punctuated::new();
                                        handle.push(syn::PathSegment {
                                            ident: proc_macro2::Ident::new(
                                                proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&element.0),
                                                proc_macro2::Span::call_site(),
                                            ),
                                            arguments: syn::PathArguments::None,
                                        });
                                        handle
                                    },
                                }),
                            }],
                            vis: syn::Visibility::Inherited,
                            mutability: syn::FieldMutability::None,
                            ident: Some(syn::Ident::new(&element.1.to_string(), proc_macro2::Span::call_site())),
                            colon_token: Some(syn::token::Colon {
                                spans: [proc_macro2::Span::call_site()],
                            }),
                            ty: syn::Type::Path(syn::TypePath {
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    segments: element.2,
                                },
                            }),
                        });
                        acc.push_punct(syn::token::Comma {
                            spans: [proc_macro2::Span::call_site()],
                        });
                        acc
                    },
                );
                handle.push_value(crate::code_occurence_syn_field::code_occurence_syn_field(&proc_macro_name_upper_camel_case_ident_stringified));
                handle
            },
        }),
        discriminant: None,
    }
}

pub fn construct_syn_variant_with_status_code_only_code_occurence(
    status_code: crate::status_code::StatusCode,
    variant_name: &impl std::fmt::Display,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> syn::Variant {
    syn::Variant {
        attrs: vec![syn::Attribute {
            pound_token: syn::token::Pound {
                spans: [proc_macro2::Span::call_site()],
            },
            style: syn::AttrStyle::Outer,
            bracket_token: syn::token::Bracket::default(),
            meta: syn::Meta::Path(syn::Path {
                leading_colon: None,
                segments: {
                    let mut handle = syn::punctuated::Punctuated::new();
                    handle.push(syn::PathSegment {
                            ident: proc_macro2::Ident::new(&proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&status_code), proc_macro2::Span::call_site()),
                            arguments: syn::PathArguments::None,
                        });
                    handle
                },
            }),
        }],
        ident: syn::Ident::new(&variant_name.to_string(), proc_macro2::Span::call_site()),
        fields: syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: {
                let mut handle = syn::punctuated::Punctuated::new();
                handle.push_value(crate::code_occurence_syn_field::code_occurence_syn_field(&proc_macro_name_upper_camel_case_ident_stringified));
                handle
            },
        }),
        discriminant: None,
    }
}

#[derive(Debug)]
pub struct ErrorSynVariant {
    variant: syn::Variant,
    status_code: std::option::Option<crate::status_code::StatusCode>,
}
impl ErrorSynVariant {
    pub fn new(
        variant_name: &dyn std::fmt::Display,
        status_code: std::option::Option<crate::status_code::StatusCode>,
        fields: std::vec::Vec<(
            crate::error_occurence::ErrorOccurenceFieldAttribute,
            &dyn std::fmt::Display,
            syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
        )>,
        proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
    ) -> Self {
        Self {
            variant: syn::Variant {
                attrs: match &status_code {
                    Some(value) => vec![syn::Attribute {
                        pound_token: syn::token::Pound {
                            spans: [proc_macro2::Span::call_site()],
                        },
                        style: syn::AttrStyle::Outer,
                        bracket_token: syn::token::Bracket::default(),
                        meta: syn::Meta::Path(syn::Path {
                            leading_colon: None,
                            segments: {
                                let mut handle = syn::punctuated::Punctuated::new();
                                handle.push(syn::PathSegment {
                                        ident: proc_macro2::Ident::new(&proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(*&value), proc_macro2::Span::call_site()),
                                        arguments: syn::PathArguments::None,
                                    });
                                handle
                            },
                        }),
                    }],
                    None => vec![]
                },
                ident: syn::Ident::new(&variant_name.to_string(), proc_macro2::Span::call_site()),
                fields: syn::Fields::Named(syn::FieldsNamed {
                    brace_token: syn::token::Brace::default(),
                    named: {
                        let mut handle = fields.into_iter().fold(
                            syn::punctuated::Punctuated::new(),
                            |mut acc, element| {
                                acc.push_value(syn::Field {
                                    attrs: vec![syn::Attribute {
                                        pound_token: syn::token::Pound {
                                            spans: [proc_macro2::Span::call_site()],
                                        },
                                        style: syn::AttrStyle::Outer,
                                        bracket_token: syn::token::Bracket::default(),
                                        meta: syn::Meta::Path(syn::Path {
                                            leading_colon: None,
                                            segments: {
                                                let mut handle = syn::punctuated::Punctuated::new();
                                                handle.push(syn::PathSegment {
                                                    ident: proc_macro2::Ident::new(
                                                        proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&element.0),
                                                        proc_macro2::Span::call_site(),
                                                    ),
                                                    arguments: syn::PathArguments::None,
                                                });
                                                handle
                                            },
                                        }),
                                    }],
                                    vis: syn::Visibility::Inherited,
                                    mutability: syn::FieldMutability::None,
                                    ident: Some(syn::Ident::new(&element.1.to_string(), proc_macro2::Span::call_site())),
                                    colon_token: Some(syn::token::Colon {
                                        spans: [proc_macro2::Span::call_site()],
                                    }),
                                    ty: syn::Type::Path(syn::TypePath {
                                        qself: None,
                                        path: syn::Path {
                                            leading_colon: None,
                                            segments: element.2,
                                        },
                                    }),
                                });
                                acc.push_punct(syn::token::Comma {
                                    spans: [proc_macro2::Span::call_site()],
                                });
                                acc
                            },
                        );
                        handle.push_value(crate::code_occurence_syn_field::code_occurence_syn_field(&proc_macro_name_upper_camel_case_ident_stringified));
                        handle
                    },
                }),
                discriminant: None,
            },
            status_code,
        }
    }
    pub fn get_syn_variant(&self) -> &syn::Variant {
        &self.variant
    }
    pub fn get_option_status_code(&self) -> &std::option::Option<crate::status_code::StatusCode> {
        &self.status_code
    }
    pub fn generate_initialization_token_stream(
        &self,
        file: &'static str,
        line: std::primitive::u32,
        column: std::primitive::u32,
        proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
    ) -> proc_macro2::TokenStream {
        let variant_ident = &self.variant.ident;
        let fields_token_stream = if let syn::Fields::Named(value) = &self.variant.fields {
            value.named.iter().map(|element|{
                let field_ident = &element.ident;
                if &field_ident.as_ref().unwrap_or_else(|| {
                    panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                        naming_constants::FIELD_IDENT_IS_NONE
                    )
                }).to_string() == &naming_conventions::CodeOccurenceSnakeCase.to_string() {
                    crate::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file,
                        line,
                        column,
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    )
                }
                else {
                    quote::quote!{#field_ident}
                }
            })
        }
        else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} syn::Fields::Named(value) != &self.variant.fields {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE);
        };
        quote::quote!{
            #variant_ident {
                #(#fields_token_stream),*
            }
        }
    }
}

//
    // let (
    //     bind_query_syn_variant, 
    //     bind_query_syn_variant_initialization_token_stream,
    //     bind_query_syn_variant_status_code
    // ) = {
    //     let bind_query_upper_camel_case = naming_conventions::BindQueryUpperCamelCase;
    //     let bind_query_snake_case = naming_conventions::BindQuerySnakeCase;
    //     let bind_query_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::InternalServerError500;
    //     (
    //         proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
    //             bind_query_syn_variant_status_code.clone(),
    //             &bind_query_upper_camel_case,
    //             vec![(
    //                 proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
    //                 &bind_query_snake_case,
    //                 proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
    //                     &[postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, &naming_conventions::TryGenerateBindIncrementsErrorNamedUpperCamelCase.to_string()],
    //                     &proc_macro_name_upper_camel_case_ident_stringified
    //                 ),
    //             )],
    //             &proc_macro_name_upper_camel_case_ident_stringified,
    //         ),
    //         {
    //             let field_code_occurence_new_d61d7616_3336_43be_aaa8_2144ff2d2158_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
    //                 file!(),
    //                 line!(),
    //                 column!(),
    //                 &proc_macro_name_upper_camel_case_ident_stringified,
    //             );
    //             quote::quote! {
    //                 #bind_query_upper_camel_case {
    //                     #bind_query_snake_case: #error_snake_case,
    //                     #field_code_occurence_new_d61d7616_3336_43be_aaa8_2144ff2d2158_token_stream
    //                 }
    //             }
    //         },
    //         bind_query_syn_variant_status_code
    //     )
    // };
//