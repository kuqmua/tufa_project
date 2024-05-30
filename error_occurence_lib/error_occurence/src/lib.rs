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
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "ErrorOccurence";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let ident_with_serialize_deserialize_token_stream = {
        let value = format!(
            "{ident}{}",
            proc_macro_helpers::naming_conventions::WithSerializeDeserializeUpperCamelCase
        );
        value
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!(
            "{proc_macro_name_upper_camel_case_ident_stringified} {} syn::Data::Enum",
            naming_constants::SUPPORTS_ONLY_STRINGIFIED
        );
    };
    let supported_enum_variant = proc_macro_helpers::error_occurence::supported_enum_variant::create(
        &data_enum,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let std_fmt_display_token_stream = quote::quote! {std::fmt::Display};
    let std_string_string = token_patterns::StdStringString;
    let code_occurence_snake_case_stringified = proc_macro_helpers::naming_conventions::CodeOccurenceSnakeCase;
    let code_occurence_snake_case_token_stream = proc_macro_helpers::naming_conventions::CodeOccurenceSnakeCase;
    let into_serialize_deserialize_version_snake_case_token_stream = proc_macro_helpers::naming_conventions::IntoSerializeDeserializeVersionSnakeCase;
    let generate_impl_std_fmt_display_token_stream = |ident_token_stream: &proc_macro2::TokenStream, content_token_stream: &proc_macro2::TokenStream|{
        quote::quote! {
            impl #std_fmt_display_token_stream for #ident_token_stream {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    #content_token_stream
                }
            }
        }
    };
    let generate_enum_ident_with_serialize_deserialize_token_stream = |variants_token_stream: &proc_macro2::TokenStream|{
        quote::quote! {
            #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
            pub enum #ident_with_serialize_deserialize_token_stream {
                #variants_token_stream
            }
        }
    };
    let generate_impl_ident_into_serialize_deserialize_version_token_stream = |variants: &proc_macro2::TokenStream|{
        quote::quote! {
            impl #ident {
                pub fn #into_serialize_deserialize_version_snake_case_token_stream(self) -> #ident_with_serialize_deserialize_token_stream {
                    #[allow(clippy::redundant_closure_for_method_calls)]
                    match self {
                        #variants
                    }
                }
            }
        }
    };
    let tokens = match supported_enum_variant {
        proc_macro_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant::Named => {
            let generate_impl_std_fmt_display_handle_token_stream = |ident_token_stream: &proc_macro2::TokenStream|{
                generate_impl_std_fmt_display_token_stream(
                    ident_token_stream,
                    &{
                        let variants_token_stream = data_enum.variants.iter().map(|element| {
                            let element_ident = &element.ident;
                            let fields = if let syn::Fields::Named(fields) = &element.fields {
                                &fields.named
                            }
                            else {
                                panic!(
                                    "{proc_macro_name_upper_camel_case_ident_stringified} {} syn::Data::Enum",
                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                );
                            };
                            let fields_idents_excluding_code_occurence_token_stream = fields.iter().filter(|element|
                                *element.ident.as_ref().expect(proc_macro_common::constants::IDENT_IS_NONE) != *code_occurence_snake_case_stringified.to_string()
                            ).map(|element|{
                                let element_ident = &element.ident;
                                quote::quote! {#element_ident,}
                            });
                            let fields_format_excluding_code_occurence_token_stream = proc_macro_common::generate_quotes::token_stream(
                                &fields.iter().filter(|element|
                                    *element.ident.as_ref().expect(proc_macro_common::constants::IDENT_IS_NONE) != *code_occurence_snake_case_stringified.to_string()
                                ).fold(std::string::String::new(), |mut acc, element| {
                                    let element_ident = &element.ident.as_ref().expect(proc_macro_common::constants::IDENT_IS_NONE);
                                    acc.push_str(&format!("{element_ident}: {{}}\n"));
                                    acc
                                }),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            let fields_format_values_excluding_code_occurence_token_stream = fields.iter().filter(|element|
                                *element.ident.as_ref().expect(proc_macro_common::constants::IDENT_IS_NONE) != *code_occurence_snake_case_stringified.to_string()
                            ).map(|element|{
                                let element_ident = &element.ident.as_ref().expect(proc_macro_common::constants::IDENT_IS_NONE);
                                match proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::from(element) {
                                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString |
                                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize => {
                                        quote::quote!{
                                            error_occurence_lib::ToStdStringString::to_std_string_string(#element_ident)
                                        }
                                    },
                                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                        quote::quote!{
                                            #element_ident.to_string().lines().fold(
                                                #std_string_string::new(),
                                                |mut acc, element| {
                                                    acc.push_str(&format!("\n {element}"));
                                                    acc
                                                }
                                            )
                                        }
                                    },
                                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString | proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => {
                                        quote::quote!{
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
                                    },
                                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                        quote::quote!{
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
                                    },
                                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString | proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                        quote::quote!{
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
                                    },
                                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                                        quote::quote!{
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
                                    },
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
                            }
                            else {
                                quote::quote! {
                                    | Self::#element_ident {
                                        #code_occurence_snake_case_token_stream,
                                        ..
                                    }
                                }
                            }
                        });
                        quote::quote!{
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
                    }

                )
            };
            let impl_std_fmt_display_for_ident_token_stream = generate_impl_std_fmt_display_handle_token_stream(&quote::quote!{#ident});
            let impl_ident_into_serialize_deserialize_version_token_stream = {
                let variants_token_stream = data_enum.variants.iter().map(|element| {
                    let element_ident = &element.ident;
                    let fields = if let syn::Fields::Named(fields) = &element.fields {
                        &fields.named
                    }
                    else {
                        panic!(
                            "{proc_macro_name_upper_camel_case_ident_stringified} {} syn::Data::Enum",
                            naming_constants::SUPPORTS_ONLY_STRINGIFIED
                        );
                    };
                    let fields_idents_token_stream = fields.iter().map(|element|&element.ident);
                    let fields_into_serialize_deserialize_version_excluding_code_occurence_token_stream = fields.iter().filter(|element|
                        *element.ident.as_ref().expect(proc_macro_common::constants::IDENT_IS_NONE) != *code_occurence_snake_case_stringified.to_string()
                    ).map(|element|{
                        let element_ident = &element.ident.as_ref().expect(proc_macro_common::constants::IDENT_IS_NONE);
                        let conversion_token_stream = match proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::from(element) {
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString => {
                                quote::quote!{
                                    #element_ident: {
                                        error_occurence_lib::ToStdStringString::to_std_string_string(&#element_ident)
                                    }
                                }
                            },
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize | 
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize | 
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                quote::quote!{
                                    #element_ident
                                }
                            },
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                quote::quote!{
                                    #element_ident: {
                                        #element_ident.into_serialize_deserialize_version()
                                    }
                                }
                            },
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
                                quote::quote!{
                                    #element_ident: { 
                                        #element_ident.into_iter().map(|element|error_occurence_lib::ToStdStringString::to_std_string_string(&element)).collect()
                                    }
                                }
                            },
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                quote::quote!{
                                    #element_ident: {
                                        #element_ident.into_iter().map(|element|element.into_serialize_deserialize_version()).collect()
                                    }
                                }
                            },
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => {
                                quote::quote!{
                                    #element_ident: {
                                        #element_ident.into_iter().map(|(key, value)|
                                            (key, error_occurence_lib::ToStdStringString::to_std_string_string(&value))
                                        ).collect()
                                    }
                                }
                            },
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                                quote::quote!{
                                    #element_ident: {
                                        #element_ident.into_iter().map(
                                            |(key, value)|(key, value.into_serialize_deserialize_version())
                                        ).collect()
                                    }
                                }
                            },
                        };
                        quote::quote!{#conversion_token_stream,}
                    });
                    quote::quote! {
                        Self::#element_ident {
                            #(#fields_idents_token_stream),*
                        } => #ident_with_serialize_deserialize_token_stream::#element_ident {
                            #(#fields_into_serialize_deserialize_version_excluding_code_occurence_token_stream)*
                            #code_occurence_snake_case_token_stream,
                        }
                    }
                });
                generate_impl_ident_into_serialize_deserialize_version_token_stream(
                    &quote::quote!{#(#variants_token_stream),*}
                )
            };
            let enum_ident_with_serialize_deserialize_token_stream = {
                let variants_token_stream = data_enum.variants.iter().map(|element|proc_macro_helpers::error_occurence::generate_serialize_deserialize_version_of_named_syn_variant(
                    &element,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ));
                generate_enum_ident_with_serialize_deserialize_token_stream(
                    &quote::quote! {#(#variants_token_stream),*}
                )
            };
            let impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream = generate_impl_std_fmt_display_handle_token_stream(&ident_with_serialize_deserialize_token_stream);
            let impl_error_occurence_lib_to_std_string_string_to_std_string_string_for_ident_with_serialize_deserialize_token_stream = {
                quote::quote! {
                    impl error_occurence_lib::ToStdStringString for #ident_with_serialize_deserialize_token_stream {
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
        },
        proc_macro_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant::Unnamed => {
            let generate_display_formatter_unnamed_token_stream = ||{
                let variants_token_stream = data_enum.variants.iter().map(|element| {
                    let element_ident = &element.ident;
                    quote::quote! {
                        Self::#element_ident(value) => value
                    }
                });
                quote::quote!{
                    match self {
                        #(#variants_token_stream),*
                    }
                }
            };
            let impl_std_fmt_display_for_ident_token_stream = generate_impl_std_fmt_display_token_stream(
                &quote::quote!{#ident},
                &{
                    let display_formatter_unnamed_token_stream = generate_display_formatter_unnamed_token_stream();
                    quote::quote! {
                        write!(
                            formatter, 
                            "{}", 
                            #display_formatter_unnamed_token_stream
                        )
                    }
                }
            );
            let impl_ident_into_serialize_deserialize_version_token_stream = {
                let variants_token_stream = data_enum.variants.iter().map(|element| {
                    let element_ident = &element.ident;
                    quote::quote! {
                        Self::#element_ident(value) => #ident_with_serialize_deserialize_token_stream::#element_ident(
                            value.#into_serialize_deserialize_version_snake_case_token_stream(),
                        )
                    }
                });
                generate_impl_ident_into_serialize_deserialize_version_token_stream(
                    &quote::quote!{#(#variants_token_stream),*}
                )
            };
            let enum_ident_with_serialize_deserialize_token_stream = {
                let variants_token_stream = data_enum.variants.iter().map(|element| {
                    let element_ident = &element.ident;
                    let fields = if let syn::Fields::Unnamed(fields) = &element.fields {
                        &fields.unnamed
                    }
                    else {
                        panic!(
                            "{proc_macro_name_upper_camel_case_ident_stringified} {} syn::Data::Enum",
                            naming_constants::SUPPORTS_ONLY_STRINGIFIED
                        );
                    };
                    let inner_type_with_serialize_deserialize_token_stream = {
                        let value = format!(
                            "{}{}",
                            {
                                assert!(fields.len() == 1, "{proc_macro_name_upper_camel_case_ident_stringified} fields.len() != 1");
                                let field_type = &fields.iter().next().expect("no first field type").ty;
                                quote::quote!{#field_type}.to_string()
                            },
                            proc_macro_helpers::naming_conventions::WithSerializeDeserializeUpperCamelCase
                        );
                        value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!(
                            "{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", 
                            proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE
                        ))
                    };
                    quote::quote! {
                        #element_ident(#inner_type_with_serialize_deserialize_token_stream)
                    }
                });
                generate_enum_ident_with_serialize_deserialize_token_stream(
                    &quote::quote! {#(#variants_token_stream),*}
                )
            };
            let impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream = generate_impl_std_fmt_display_token_stream(
                &ident_with_serialize_deserialize_token_stream,
                &{
                    let display_formatter_unnamed_token_stream = generate_display_formatter_unnamed_token_stream();
                    quote::quote! {
                        write!(
                            formatter, 
                            "{}", 
                            #display_formatter_unnamed_token_stream
                        )
                    }
                }
            );
            quote::quote! {
                #impl_std_fmt_display_for_ident_token_stream
                #impl_ident_into_serialize_deserialize_version_token_stream
                #enum_ident_with_serialize_deserialize_token_stream
                #impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream
            }
        }
    };
    let gen = quote::quote! {
        #tokens
    };
    // println!("{gen} ");
    // if ident == "" {
    //     proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         &proc_macro_name_upper_camel_case,
    //         &gen,
    //         &proc_macro_name_upper_camel_case_ident_stringified
    //     );
    // }
    gen.into()
}
