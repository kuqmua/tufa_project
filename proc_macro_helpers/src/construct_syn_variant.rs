#[derive(Debug)]
pub struct SynVariantWrapper {
    variant: syn::Variant,
    status_code: std::option::Option<crate::status_code::StatusCode>,
}
impl SynVariantWrapper {
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
            value.named.iter().enumerate().map(|(index, element)|{
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
                    let error_increment_token_stream = {
                        let value = format!("{}_{index}", naming_constants::ErrorSnakeCase);
                        value.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote!{#field_ident: #error_increment_token_stream}
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