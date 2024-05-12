pub fn construct_syn_variant(
    status_code: crate::status_code::StatusCode,
    variant_name: &str,
    code_occurence_field: &syn::Field,
    fields: std::vec::Vec<(
        crate::error_occurence::ErrorOccurenceFieldAttribute,
        &str,
        syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
    )>,
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
        ident: syn::Ident::new(variant_name, proc_macro2::Span::call_site()),
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
                            ident: Some(syn::Ident::new(element.1, proc_macro2::Span::call_site())),
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
                handle.push_value(code_occurence_field.clone());
                handle
            },
        }),
        discriminant: None,
    }
}
