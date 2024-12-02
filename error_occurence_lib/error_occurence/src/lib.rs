#[proc_macro_derive(
    ErrorOccurence,
    attributes(
        eo_to_std_string_string,
        eo_to_std_string_string_serialize_deserialize,
        eo_error_occurence,
        eo_vec_to_std_string_string,
        eo_vec_to_std_string_string_serialize_deserialize,
        eo_vec_error_occurence,
        eo_hashmap_key_std_string_string_value_to_std_string_string,
        eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize,
        eo_hashmap_key_std_string_string_value_error_occurence,
    )
)]
pub fn error_occurence(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| panic!("{}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_with_serialize_deserialize_upper_camel_case = naming_conventions::self_constants::SelfWithSerializeDeserializeUpperCamelCase::from_dyn_quote_to_tokens(&ident);
    let data_enum = if let syn::Data::Enum(data_enum) = syn_derive_input.data {
        data_enum
    } else {
        panic!("{} syn::Data::Enum", naming_conventions::SUPPORTS_ONLY_STRINGIFIED);
    };
    let supported_enum_variant = macros_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant::new_or_panic(&data_enum);
    let std_fmt_display_token_stream = quote::quote! {std::fmt::Display};
    let std_string_string = token_patterns::StdStringString;
    let code_occurence_snake_case = naming_conventions::CodeOccurenceSnakeCase;
    let code_occurence_snake_case_stringified = code_occurence_snake_case.to_string();
    let code_occurence_snake_case_token_stream = naming_conventions::CodeOccurenceSnakeCase;
    let into_serialize_deserialize_version_snake_case_token_stream = naming_conventions::IntoSerializeDeserializeVersionSnakeCase;
    let generate_impl_std_fmt_display_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            impl #std_fmt_display_token_stream for #ident_token_stream {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    #content_token_stream
                }
            }
        }
    };
    let generate_enum_ident_with_serialize_deserialize_token_stream = |variants_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
            pub enum #ident_with_serialize_deserialize_upper_camel_case {
                #variants_token_stream
            }
        }
    };
    let generate_impl_ident_into_serialize_deserialize_version_token_stream = |variants: &dyn quote::ToTokens| {
        quote::quote! {
            impl #ident {
                pub fn #into_serialize_deserialize_version_snake_case_token_stream(self) -> #ident_with_serialize_deserialize_upper_camel_case {
                    #[allow(clippy::redundant_closure_for_method_calls)]
                    match self {
                        #variants
                    }
                }
            }
        }
    };
    let tokens = match supported_enum_variant {
        macros_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant::Named => {
            let generate_impl_std_fmt_display_handle_token_stream = |ident_token_stream: &dyn quote::ToTokens| {
                generate_impl_std_fmt_display_token_stream(ident_token_stream, &{
                    let variants_token_stream = data_enum.variants.iter().map(|element| {
                        let element_ident = &element.ident;
                        let fields = if let syn::Fields::Named(fields) = &element.fields {
                            &fields.named
                        } else {
                            panic!("{} syn::Data::Enum", naming_conventions::SUPPORTS_ONLY_STRINGIFIED);
                        };
                        let fields_idents_excluding_code_occurence_token_stream = fields.iter().filter(|element| *element.ident.as_ref().expect(constants::IDENT_IS_NONE) != *code_occurence_snake_case_stringified).map(|element| {
                            let element_ident = &element.ident;
                            quote::quote! {#element_ident,}
                        });
                        let fields_format_excluding_code_occurence_token_stream = generate_quotes::double_quotes_token_stream(
                            &fields.iter()
                                .filter(|element| *element.ident.as_ref().expect(constants::IDENT_IS_NONE) != *code_occurence_snake_case_stringified)
                                .fold(std::string::String::new(), |mut acc, element| {
                                    let element_ident = &element.ident.as_ref().expect(constants::IDENT_IS_NONE);
                                    acc.push_str(&format!("{element_ident}: {{}}\n"));
                                    acc
                                })
                        );
                        let fields_format_values_excluding_code_occurence_token_stream = fields.iter().filter(|element| *element.ident.as_ref().expect(constants::IDENT_IS_NONE) != *code_occurence_snake_case_stringified).map(|element| {
                            let element_ident = &element.ident.as_ref().expect(constants::IDENT_IS_NONE);
                            match macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::from(element) {
                                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString | macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize => {
                                    quote::quote! {
                                        error_occurence_lib::ToStdStringString::to_std_string_string(#element_ident)
                                    }
                                }
                                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                    quote::quote! {
                                        #element_ident.to_string().lines().fold(
                                            #std_string_string::new(),
                                            |mut acc, element| {
                                                acc.push_str(&format!("\n {element}"));
                                                acc
                                            }
                                        )
                                    }
                                }
                                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString | macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => {
                                    quote::quote! {
                                        #element_ident.iter().fold(
                                            #std_string_string::new(),
                                            |mut acc, element| {
                                                acc.push_str(
                                                    &error_occurence_lib::ToStdStringString::to_std_string_string(element)
                                                    .lines()
                                                    .fold(
                                                        #std_string_string::new(),
                                                        |mut acc, element| {
                                                            acc.push_str(&format!("\n {element}"));
                                                            acc
                                                        }
                                                    )
                                                );
                                                acc
                                            }
                                        )
                                    }
                                }
                                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                    quote::quote! {
                                        #element_ident.iter().fold(
                                            #std_string_string::new(),
                                            |mut acc, element| {
                                                acc.push_str(&element.to_string().lines().fold(
                                                    #std_string_string::new(),
                                                    |mut acc, element| {
                                                        acc.push_str(&format!("\n {element}"));
                                                        acc
                                                    },
                                                ));
                                                acc
                                            }
                                        )
                                    }
                                }
                                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString | macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                    quote::quote! {
                                        #element_ident
                                            .iter()
                                            .fold(
                                                #std_string_string::new(),
                                                |mut acc, (key, value)| {
                                                    acc.push_str(
                                                        &format!(
                                                            "\n {key}: {}",
                                                            &error_occurence_lib::ToStdStringString::to_std_string_string(value)
                                                        )
                                                    );
                                                    acc
                                                }
                                            )
                                    }
                                }
                                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                                    quote::quote! {
                                        #element_ident
                                            .iter()
                                            .fold(
                                                #std_string_string::new(),
                                                |mut acc, (key, value)| {
                                                    acc.push_str(&format!(
                                                        "\n {key}: {}",
                                                        value.to_string().lines().fold(
                                                            #std_string_string::new(),
                                                            |mut acc, element| {
                                                                acc.push_str(&format!("\n  {element}"));
                                                                acc
                                                            }
                                                        )
                                                    ));
                                                    acc
                                                }
                                            )
                                    }
                                }
                            }
                        });
                        quote::quote! {
                            Self::#element_ident {
                                #(#fields_idents_excluding_code_occurence_token_stream)*
                                ..
                            } => {
                                format!(
                                    #fields_format_excluding_code_occurence_token_stream,
                                    #(#fields_format_values_excluding_code_occurence_token_stream),*
                                )
                            }
                        }
                    });
                    let code_occurence_variants_token_stream = data_enum.variants.iter().enumerate().map(|(index, element)| {
                        let element_ident = &element.ident;
                        if index == 0 {
                            quote::quote! {
                                Self::#element_ident {
                                    #code_occurence_snake_case_token_stream,
                                    ..
                                }
                            }
                        } else {
                            quote::quote! {
                                | Self::#element_ident {
                                    #code_occurence_snake_case_token_stream,
                                    ..
                                }
                            }
                        }
                    });
                    quote::quote! {
                        write!(
                            formatter,
                            "{}{}",
                            match self {
                                #(#variants_token_stream),*
                            },
                            match self {
                                #(#code_occurence_variants_token_stream)*
                                => #code_occurence_snake_case_token_stream
                            }
                        )
                    }
                })
            };
            let impl_std_fmt_display_for_ident_token_stream = generate_impl_std_fmt_display_handle_token_stream(&quote::quote! {#ident});
            let impl_ident_into_serialize_deserialize_version_token_stream = {
                let variants_token_stream = data_enum.variants.iter().map(|element| {
                    let element_ident = &element.ident;
                    let fields = if let syn::Fields::Named(fields) = &element.fields {
                        &fields.named
                    } else {
                        panic!("{} syn::Data::Enum", naming_conventions::SUPPORTS_ONLY_STRINGIFIED);
                    };
                    let fields_idents_token_stream = fields.iter().map(|element| &element.ident);
                    let fields_into_serialize_deserialize_version_excluding_code_occurence_token_stream = fields.iter().filter(|element| *element.ident.as_ref().expect(constants::IDENT_IS_NONE) != *code_occurence_snake_case_stringified).map(|element| {
                        let element_ident = &element.ident.as_ref().expect(constants::IDENT_IS_NONE);
                        let conversion_token_stream = match macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::from(element) {
                            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString => {
                                quote::quote! {
                                    #element_ident: {
                                        error_occurence_lib::ToStdStringString::to_std_string_string(&#element_ident)
                                    }
                                }
                            }
                            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize
                            | macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize
                            | macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                quote::quote! {
                                    #element_ident
                                }
                            }
                            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                quote::quote! {
                                    #element_ident: {
                                        #element_ident.into_serialize_deserialize_version()
                                    }
                                }
                            }
                            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
                                quote::quote! {
                                    #element_ident: {
                                        #element_ident.into_iter().map(|element|error_occurence_lib::ToStdStringString::to_std_string_string(&element)).collect()
                                    }
                                }
                            }
                            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                quote::quote! {
                                    #element_ident: {
                                        #element_ident.into_iter().map(|element|element.into_serialize_deserialize_version()).collect()
                                    }
                                }
                            }
                            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => {
                                quote::quote! {
                                    #element_ident: {
                                        #element_ident.into_iter().map(|(key, value)|
                                            (key, error_occurence_lib::ToStdStringString::to_std_string_string(&value))
                                        ).collect()
                                    }
                                }
                            }
                            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                                quote::quote! {
                                    #element_ident: {
                                        #element_ident.into_iter().map(
                                            |(key, value)|(key, value.into_serialize_deserialize_version())
                                        ).collect()
                                    }
                                }
                            }
                        };
                        quote::quote! {#conversion_token_stream,}
                    });
                    quote::quote! {
                        Self::#element_ident {
                            #(#fields_idents_token_stream),*
                        } => #ident_with_serialize_deserialize_upper_camel_case::#element_ident {
                            #(#fields_into_serialize_deserialize_version_excluding_code_occurence_token_stream)*
                            #code_occurence_snake_case_token_stream,
                        }
                    }
                });
                generate_impl_ident_into_serialize_deserialize_version_token_stream(&quote::quote! {#(#variants_token_stream),*})
            };
            let enum_ident_with_serialize_deserialize_token_stream = {
                let variants_token_stream = data_enum
                    .variants
                    .iter()
                    .map(|element| macros_helpers::error_occurence::generate_serialize_deserialize_version_of_named_syn_variant(&element));
                generate_enum_ident_with_serialize_deserialize_token_stream(&quote::quote! {#(#variants_token_stream),*})
            };
            let impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream = generate_impl_std_fmt_display_handle_token_stream(&ident_with_serialize_deserialize_upper_camel_case);
            let impl_error_occurence_lib_to_std_string_string_to_std_string_string_for_ident_with_serialize_deserialize_token_stream = {
                quote::quote! {
                    impl error_occurence_lib::ToStdStringString for #ident_with_serialize_deserialize_upper_camel_case {
                        fn to_std_string_string(&self) -> #std_string_string {
                            format!("{self}")
                        }
                    }
                }
            };
            quote::quote! {
                #impl_std_fmt_display_for_ident_token_stream
                #impl_ident_into_serialize_deserialize_version_token_stream
                #enum_ident_with_serialize_deserialize_token_stream
                #impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream
                #impl_error_occurence_lib_to_std_string_string_to_std_string_string_for_ident_with_serialize_deserialize_token_stream
            }
        }
        macros_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant::Unnamed => {
            let generate_display_formatter_unnamed_token_stream = || {
                let variants_token_stream = data_enum.variants.iter().map(|element| {
                    let element_ident = &element.ident;
                    quote::quote! {
                        Self::#element_ident(value) => value
                    }
                });
                quote::quote! {
                    match self {
                        #(#variants_token_stream),*
                    }
                }
            };
            let impl_std_fmt_display_for_ident_token_stream = generate_impl_std_fmt_display_token_stream(&quote::quote! {#ident}, &{
                let display_formatter_unnamed_token_stream = generate_display_formatter_unnamed_token_stream();
                quote::quote! {
                    write!(
                        formatter,
                        "{}",
                        #display_formatter_unnamed_token_stream
                    )
                }
            });
            let impl_ident_into_serialize_deserialize_version_token_stream = {
                let variants_token_stream = data_enum.variants.iter().map(|element| {
                    let element_ident = &element.ident;
                    quote::quote! {
                        Self::#element_ident(value) => #ident_with_serialize_deserialize_upper_camel_case::#element_ident(
                            value.#into_serialize_deserialize_version_snake_case_token_stream(),
                        )
                    }
                });
                generate_impl_ident_into_serialize_deserialize_version_token_stream(&quote::quote! {#(#variants_token_stream),*})
            };
            let enum_ident_with_serialize_deserialize_token_stream = {
                let variants_token_stream = data_enum.variants.iter().map(|element| {
                    let element_ident = &element.ident;
                    let fields = if let syn::Fields::Unnamed(fields) = &element.fields {
                        &fields.unnamed
                    } else {
                        panic!("{} syn::Data::Enum", naming_conventions::SUPPORTS_ONLY_STRINGIFIED);
                    };
                    let inner_type_with_serialize_deserialize_token_stream = {
                        let value = format!(
                            "{}{}",
                            {
                                assert!(fields.len() == 1, "fields.len() != 1");
                                let field_type = &fields.iter().next().expect("no first field type").ty;
                                quote::quote! {#field_type}.to_string()
                            },
                            naming_conventions::WithSerializeDeserializeUpperCamelCase
                        );
                        value
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote! {
                        #element_ident(#inner_type_with_serialize_deserialize_token_stream)
                    }
                });
                generate_enum_ident_with_serialize_deserialize_token_stream(&quote::quote! {#(#variants_token_stream),*})
            };
            let impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream = generate_impl_std_fmt_display_token_stream(&ident_with_serialize_deserialize_upper_camel_case, &{
                let display_formatter_unnamed_token_stream = generate_display_formatter_unnamed_token_stream();
                quote::quote! {
                    write!(
                        formatter,
                        "{}",
                        #display_formatter_unnamed_token_stream
                    )
                }
            });
            //todo maybe make a trait?
            quote::quote! {
                #impl_std_fmt_display_for_ident_token_stream
                #impl_ident_into_serialize_deserialize_version_token_stream
                #enum_ident_with_serialize_deserialize_token_stream
                #impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream
            }
        }
    };
    let generated = quote::quote! {
        #tokens
    };
    // println!("{generated} ");
    // if ident == "" {
    //     macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         &proc_macro_name_upper_camel_case,
    //         &generated,
    //         &proc_macro_name_upper_camel_case_ident_stringified
    //     );
    // }
    generated.into()
}
