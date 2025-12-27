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
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum SuportedEnumVariant {
        Named,
        Unnamed,
    }
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| panic!("{}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let generic_parameters = &syn_derive_input
        .generics
        .params
        .iter()
        .map(|element| match &element {
            syn::GenericParam::Type(value) => &value.ident,
            syn::GenericParam::Lifetime(_) | syn::GenericParam::Const(_) => {
                panic!("does support only syn::GenericParam::Type")
            }
        })
        .collect::<Vec<&syn::Ident>>();
    let ident_with_serialize_deserialize_upper_camel_case = naming::parameter::SelfWithSerializeDeserializeUpperCamelCase::from_tokens(&ident);
    let syn::Data::Enum(data_enum) = syn_derive_input.data else {
        panic!("{} syn::Data::Enum", naming::SUPPORTS_ONLY_STRINGIFIED);
    };
    let supported_enum_variant = {
        let mut all_equal: Option<SuportedEnumVariant> = None;
        assert!(!data_enum.variants.is_empty(), "enum variants are empty");
        let error_message = format!(
            "{} enums where all variants are {}::{} or all variants are {}::{}",
            naming::SUPPORTS_ONLY_STRINGIFIED, naming::SYN_FIELDS,
            naming::SYN_FIELDS,
            naming::NamedUpperCamelCase,
            naming::UnnamedUpperCamelCase
        );
        data_enum.variants.iter().for_each(|variant| match &variant.fields {
            syn::Fields::Named(_) => match &all_equal {
                Some(supported_variant) => {
                    assert!(!(*supported_variant == SuportedEnumVariant::Unnamed), "{error_message}");
                }
                None => {
                    all_equal = Some(SuportedEnumVariant::Named);
                }
            },
            syn::Fields::Unnamed(_) => match &all_equal {
                Some(supported_variant) => {
                    assert!(!(*supported_variant == SuportedEnumVariant::Named), "{error_message}");
                }
                None => {
                    all_equal = Some(SuportedEnumVariant::Unnamed);
                }
            },
            syn::Fields::Unit => panic!("{error_message}"),
        });
        all_equal.unwrap_or_else(|| {
            panic!("{} with enums where all variants are named or unnamed", naming::SUPPORTS_ONLY_STRINGIFIED);
        })
    };
    let acc_snake_case = naming::AccSnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let element_snake_case = naming::ElementSnakeCase;
    let std_string_string = token_patterns::StdStringString;
    let code_occurence_snake_case = naming::CodeOccurenceSnakeCase;
    let code_occurence_snake_case_stringified = code_occurence_snake_case.to_string();
    let code_occurence_snake_case_token_stream = naming::CodeOccurenceSnakeCase;
    let into_serialize_deserialize_version_snake_case_token_stream = naming::IntoSerializeDeserializeVersionSnakeCase;
    let maybe_generic_parameters_token_stream = if generic_parameters.is_empty() {
        proc_macro2::TokenStream::new()
    } else {
        quote::quote! {<#(#generic_parameters),*>}
    };
    let maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_token_stream = if generic_parameters.is_empty() {
        proc_macro2::TokenStream::new()
    } else {
        let value = generic_parameters.iter().map(|element| quote::quote! {#element: error_occurence_lib::ToStdStringString});
        quote::quote! {<#(#value),*>}
    };
    let generate_enum_ident_with_serialize_deserialize_token_stream = |variants_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
            pub enum #ident_with_serialize_deserialize_upper_camel_case #maybe_generic_parameters_token_stream {
                #variants_token_stream
            }
        }
    };
    let generate_impl_ident_into_serialize_deserialize_version_token_stream = |variants: &dyn quote::ToTokens| {
        quote::quote! {
            impl #maybe_generic_parameters_token_stream #ident #maybe_generic_parameters_token_stream {
                pub fn #into_serialize_deserialize_version_snake_case_token_stream(self) -> #ident_with_serialize_deserialize_upper_camel_case #maybe_generic_parameters_token_stream {
                    #[allow(clippy::redundant_closure_for_method_calls)]
                    match self {
                        #variants
                    }
                }
            }
        }
    };
    let tokens = match supported_enum_variant {
        SuportedEnumVariant::Named => {
            //todo maybe impl display was a bad idea. .to_string() casts is dangerous
            let impl_std_fmt_display_handle_content_token_stream = {
                let variants_token_stream = data_enum.variants.iter().map(|element| {
                    let element_ident = &element.ident;
                    let fields = if let syn::Fields::Named(fields) = &element.fields {
                        &fields.named
                    } else {
                        panic!("{} syn::Data::Enum", naming::SUPPORTS_ONLY_STRINGIFIED);
                    };
                    let fields_idents_excluding_code_occurence_token_stream = {
                        let acc_token_stream = fields.iter()
                        .filter(|current_element| *current_element.ident.as_ref().expect("07504636-310e-43cf-aa3b-afd7443987f0") != *code_occurence_snake_case_stringified)
                        .map(|current_element| current_element.ident.as_ref().expect("971ace15-e8cb-4780-8589-2da5e99e5587"))
                        .collect::<Vec<&syn::Ident>>();
                        if acc_token_stream.is_empty() {
                            proc_macro2::TokenStream::new()
                        }
                        else {
                            quote::quote!{#(#acc_token_stream),*,}
                        }
                    };
                    let fields_format_excluding_code_occurence_token_stream = generate_quotes::double_quotes_token_stream(
                        &fields.iter()
                        .filter(|current_element| *current_element.ident.as_ref().expect("3d70a4f4-046d-4f37-af70-d6c7b1c46b9d") != *code_occurence_snake_case_stringified)
                        .fold(
                            String::new(),
                            |mut acc, current_element| {
                                use std::fmt::Write as _;
                                let current_element_ident = &current_element.ident.as_ref().expect("2e7cd5fe-7653-4c10-8977-526b061d6748");
                                assert!(writeln!(acc, "{current_element_ident}: {{}}").is_ok(), "error ab44c70f-092e-46a0-8daa-56fe44395228");
                                acc
                            }
                        )
                    );
                    let fields_format_values_excluding_code_occurence_token_stream = fields.iter()
                    .filter(|current_element| *current_element.ident.as_ref().expect("f6f6fb24-bdf2-4bb6-a2be-32462758dba9") != *code_occurence_snake_case_stringified)
                    .map(|current_element| {
                        let current_element_ident = &current_element.ident.as_ref().expect("e97b25b9-49d3-4f89-a18a-4e77355c4c9c");
                        match macros_helpers::ErrorOccurenceFieldAttribute::try_from(current_element).expect("8ff56aeb-8636-43d6-b8c1-f8fb0486f817") {
                            macros_helpers::ErrorOccurenceFieldAttribute::EoToStdStringString | macros_helpers::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize => {
                                quote::quote! {
                                    error_occurence_lib::ToStdStringString::to_std_string_string(#current_element_ident)
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(&quote::quote! {#acc_snake_case, "\n {element}"}, &quote::quote! {panic!("error c751d54a-b008-493f-a97d-2f8e381780d5");});
                                quote::quote! {
                                    #current_element_ident.to_string().lines().fold(
                                        #std_string_string::new(),
                                        |mut #acc_snake_case, element| {
                                            #if_write_is_err_token_stream
                                            #acc_snake_case
                                        }
                                    )
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoVecToStdStringString | macros_helpers::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => {
                                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(&quote::quote! {#acc_snake_case, "\n {element}"}, &quote::quote! {panic!("error b35ed9f5-525b-4287-9d6e-0be1d72a0874");});
                                quote::quote! {
                                    #current_element_ident.iter().fold(
                                        #std_string_string::new(),
                                        |mut #acc_snake_case, #element_snake_case| {
                                            #acc_snake_case.push_str(
                                                &error_occurence_lib::ToStdStringString::to_std_string_string(#element_snake_case)
                                                .lines()
                                                .fold(
                                                    #std_string_string::new(),
                                                    |mut #acc_snake_case, #element_snake_case| {
                                                        #if_write_is_err_token_stream
                                                        #acc_snake_case
                                                    }
                                                )
                                            );
                                            #acc_snake_case
                                        }
                                    )
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(&quote::quote! {#acc_snake_case, "\n {element}"}, &quote::quote! {panic!("error 4dfdd18d-5fca-41ba-b556-36ceb1b18b60");});
                                quote::quote! {
                                    #current_element_ident.iter().fold(
                                        #std_string_string::new(),
                                        |mut #acc_snake_case, #element_snake_case| {
                                            #acc_snake_case.push_str(&#element_snake_case.to_string().lines().fold(
                                                #std_string_string::new(),
                                                |mut #acc_snake_case, #element_snake_case| {
                                                    #if_write_is_err_token_stream
                                                    #acc_snake_case
                                                },
                                            ));
                                            #acc_snake_case
                                        }
                                    )
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString | macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(&quote::quote! {#acc_snake_case, "\n {key}: {}", &error_occurence_lib::ToStdStringString::to_std_string_string(#value_snake_case)}, &quote::quote! {panic!("error d030580a-6c03-4913-9088-b77316b9f285");});
                                quote::quote! {
                                    #current_element_ident.iter().fold(
                                        #std_string_string::new(),
                                        |mut #acc_snake_case, (key, #value_snake_case)| {
                                            #if_write_is_err_token_stream
                                            #acc_snake_case
                                        }
                                    )
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(
                                    &{
                                        let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(&quote::quote! {#acc_snake_case, "\n  {element}"}, &quote::quote! {panic!("error d0492fbf-2da0-4b02-bec3-9d011bf08999");});
                                        quote::quote! {
                                            #acc_snake_case,
                                            "\n {key}: {}",
                                            #value_snake_case.to_string().lines().fold(
                                                #std_string_string::new(),
                                                |mut #acc_snake_case, #element_snake_case| {
                                                    #if_write_is_err_token_stream
                                                    #acc_snake_case
                                                }
                                            )
                                        }
                                    },
                                    &quote::quote! {panic!("error 75f6432a-9854-48cc-9a0d-c1dbf6774433");},
                                );
                                quote::quote! {
                                    #current_element_ident.iter().fold(
                                        #std_string_string::new(),
                                        |mut #acc_snake_case, (key, #value_snake_case)| {
                                            #if_write_is_err_token_stream
                                            #acc_snake_case
                                        }
                                    )
                                }
                            }
                        }
                    });
                    quote::quote! {
                        Self::#element_ident {
                            #fields_idents_excluding_code_occurence_token_stream
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
                        f,
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
            };
            let impl_std_fmt_display_for_ident_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_token_stream, &ident, &maybe_generic_parameters_token_stream, &impl_std_fmt_display_handle_content_token_stream);
            let impl_ident_into_serialize_deserialize_version_token_stream = {
                let variants_token_stream = data_enum.variants.iter().map(|element| {
                    let element_ident = &element.ident;
                    let fields = if let syn::Fields::Named(fields) = &element.fields {
                        &fields.named
                    } else {
                        panic!("{} syn::Data::Enum", naming::SUPPORTS_ONLY_STRINGIFIED);
                    };
                    let fields_idents_token_stream = fields.iter().map(|current_element| &current_element.ident);
                    let fields_into_serialize_deserialize_version_excluding_code_occurence_token_stream = fields.iter()
                    .filter(|current_element| *current_element.ident.as_ref().expect("0488fc4c-be15-431b-904b-8bf6a5b2e8e6") != *code_occurence_snake_case_stringified)
                    .map(|current_element| {
                        let current_element_ident = &current_element.ident.as_ref().expect("9a672ac2-5184-4427-9d88-70cb2a0bd199");
                        let conversion_token_stream = match macros_helpers::ErrorOccurenceFieldAttribute::try_from(current_element).expect("449c3781-1900-4ed4-b784-485db5a08508") {
                            macros_helpers::ErrorOccurenceFieldAttribute::EoToStdStringString => {
                                quote::quote! {
                                    #current_element_ident: {
                                        error_occurence_lib::ToStdStringString::to_std_string_string(&#current_element_ident)
                                    }
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize | macros_helpers::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize | macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                quote::quote! {
                                    #current_element_ident
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                quote::quote! {
                                    #current_element_ident: {
                                        #current_element_ident.into_serialize_deserialize_version()
                                    }
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
                                quote::quote! {
                                    #current_element_ident: {
                                        #current_element_ident.into_iter().map(|element|error_occurence_lib::ToStdStringString::to_std_string_string(&element)).collect()
                                    }
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                quote::quote! {
                                    #current_element_ident: {
                                        #current_element_ident.into_iter().map(|element|element.into_serialize_deserialize_version()).collect()
                                    }
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => {
                                quote::quote! {
                                    #current_element_ident: {
                                        #current_element_ident.into_iter().map(|(key, value)|
                                            (key, error_occurence_lib::ToStdStringString::to_std_string_string(&value))
                                        ).collect()
                                    }
                                }
                            }
                            macros_helpers::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                                quote::quote! {
                                    #current_element_ident: {
                                        #current_element_ident.into_iter().map(
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
                let variants_token_stream = data_enum.variants.iter().map(macros_helpers::generate_serialize_deserialize_version_of_named_syn_variant);
                generate_enum_ident_with_serialize_deserialize_token_stream(&quote::quote! {#(#variants_token_stream),*})
            };
            let impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_token_stream, &ident_with_serialize_deserialize_upper_camel_case, &maybe_generic_parameters_token_stream, &impl_std_fmt_display_handle_content_token_stream);
            let impl_error_occurence_lib_to_std_string_string_to_std_string_string_for_ident_with_serialize_deserialize_token_stream =
                macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_token_stream, &ident_with_serialize_deserialize_upper_camel_case, &maybe_generic_parameters_token_stream, &quote::quote! {format!("{self}")});
            quote::quote! {
                #impl_std_fmt_display_for_ident_token_stream
                #impl_ident_into_serialize_deserialize_version_token_stream
                #enum_ident_with_serialize_deserialize_token_stream
                #impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream
                #impl_error_occurence_lib_to_std_string_string_to_std_string_string_for_ident_with_serialize_deserialize_token_stream
            }
        }
        SuportedEnumVariant::Unnamed => {
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
            let impl_std_fmt_display_for_ident_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_token_stream, &ident, &maybe_generic_parameters_token_stream, &{
                let display_formatter_unnamed_token_stream = generate_display_formatter_unnamed_token_stream();
                quote::quote! {
                    write!(
                        f,
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
                        panic!("{} syn::Data::Enum", naming::SUPPORTS_ONLY_STRINGIFIED);
                    };
                    let inner_type_with_serialize_deserialize_token_stream = {
                        let value = format!(
                            "{}{}",
                            {
                                assert!(fields.len() == 1, "fields.len() != 1");
                                let field_type = &fields.iter().next().expect("8a80c36d-0b80-4ade-a4aa-2febb8079bd9").ty;
                                quote::quote! {#field_type}.to_string()
                            },
                            naming::WithSerializeDeserializeUpperCamelCase
                        );
                        value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote! {
                        #element_ident(#inner_type_with_serialize_deserialize_token_stream)
                    }
                });
                generate_enum_ident_with_serialize_deserialize_token_stream(&quote::quote! {#(#variants_token_stream),*})
            };
            let impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&maybe_generic_parameters_error_occurence_lib_to_std_string_string_annotations_token_stream, &ident_with_serialize_deserialize_upper_camel_case, &maybe_generic_parameters_token_stream, &{
                let display_formatter_unnamed_token_stream = generate_display_formatter_unnamed_token_stream();
                quote::quote! {
                    write!(
                        f,
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
    //     macros_helpers::write_token_stream_into_file(
    //         "ErrorOccurence",
    //         &generated,
    //         &macros_helpers::FormatWithRustfmt::True
    //     );
    // }
    generated.into()
}
