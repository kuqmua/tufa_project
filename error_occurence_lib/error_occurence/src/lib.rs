#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
enum ErrorOccurenceFieldAttribute {
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

fn get_type_path_third_segment_second_argument_check_if_hashmap<'a>(
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
    assert!(!(first_segment.arguments != syn::PathArguments::None), "{proc_macro_name_upper_camel_case_ident_stringified} first_segment.arguments != PathArguments::None");
    let second_segment = segments.iter().nth(1).expect("no .nth(1) element");
    {
        let collections_snake_case_stringified = <naming_constants::Collections as naming_constants::Naming>::snake_case_stringified();
        assert!(second_segment.ident == collections_snake_case_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} second_segment.ident != {collections_snake_case_stringified} {}", second_segment.ident);
    };
    assert!(!(second_segment.arguments != syn::PathArguments::None), "{proc_macro_name_upper_camel_case_ident_stringified} second_segment.arguments != PathArguments::None");
    let third_segment = segments.iter().nth(2).expect("no .nth(2) element");
    {
        let hashmap_upper_camel_case_stringified = <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified();
        assert!(third_segment.ident == hashmap_upper_camel_case_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} third_segment.ident != {hashmap_upper_camel_case_stringified} {}", third_segment.ident);
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
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
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
    let std_string_string_token_stream = quote::quote!{std::string::String};
    let code_occurence_snake_case_stringified = proc_macro_helpers::naming_conventions::code_occurence_snake_case_stringified();
    let code_occurence_snake_case_token_stream = proc_macro_helpers::naming_conventions::code_occurence_snake_case_token_stream();
    let ident_in_none_stringified = "ident is None";
    let into_serialize_deserialize_version_snake_case_token_stream = quote::quote!{into_serialize_deserialize_version};
    let error_occurence_lib_lines_space_backslash_lines_space_backslash_lines_space_backslash_token_stream = quote::quote!{
        error_occurence_lib::lines_space_backslash::LinesSpaceBackslash::lines_space_backslash
    };
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
    let std_snake_case_stringified = <naming_constants::Std as naming_constants::Naming>::snake_case_stringified();
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
                                *element.ident.as_ref().expect(ident_in_none_stringified) != *code_occurence_snake_case_stringified
                            ).map(|element|{
                                let element_ident = &element.ident;
                                quote::quote! {#element_ident,}
                            });
                            let fields_format_excluding_code_occurence_token_stream = proc_macro_common::generate_quotes::token_stream(
                                &fields.iter().filter(|element|
                                    *element.ident.as_ref().expect(ident_in_none_stringified) != *code_occurence_snake_case_stringified
                                ).fold(std::string::String::new(), |mut acc, element| {
                                    let element_ident = &element.ident.as_ref().expect(ident_in_none_stringified);
                                    acc.push_str(&format!("{element_ident}: {{}}\n"));
                                    acc
                                }),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            let fields_format_values_excluding_code_occurence_token_stream = fields.iter().filter(|element|
                                *element.ident.as_ref().expect(ident_in_none_stringified) != *code_occurence_snake_case_stringified
                            ).map(|element|{
                                let element_ident = &element.ident.as_ref().expect(ident_in_none_stringified);
                                let ident_colon_to_string_with_config_format_token_stream = proc_macro_common::generate_quotes::token_stream(
                                    &format!("{element_ident}:\n"),
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                match ErrorOccurenceFieldAttribute::from(element) {
                                    ErrorOccurenceFieldAttribute::EoToStdStringString |
                                    ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize => {
                                        quote::quote!{
                                            error_occurence_lib::ToStdStringString::to_std_string_string(#element_ident)
                                        }
                                    },
                                    ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                        quote::quote!{
                                            #element_ident.to_string().lines().fold(
                                                std::string::String::new(),
                                                |mut acc, element| {
                                                    acc.push_str(&format!("\n {element}"));
                                                    acc
                                                }
                                            )
                                        }
                                    },
                                    ErrorOccurenceFieldAttribute::EoVecToStdStringString | ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => {
                                        quote::quote!{
                                            #element_ident.iter().fold(
                                                std::string::String::new(),
                                                |mut acc, element| {
                                                    acc.push_str(
                                                        &error_occurence_lib::ToStdStringString::to_std_string_string(element)
                                                        .lines()
                                                        .fold(
                                                            std::string::String::new(), 
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
                                    ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                        quote::quote!{
                                            #element_ident.iter().fold(
                                                std::string::String::new(),
                                                |mut acc, element| {
                                                    acc.push_str(&element.to_string().lines().fold(
                                                        std::string::String::new(),
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
                                    ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString | ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                        quote::quote!{
                                            #element_ident
                                                .iter()
                                                .fold(
                                                    std::string::String::new(), 
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
                                    ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                                        quote::quote!{
                                            #element_ident
                                                .iter()
                                                .fold(
                                                    std::string::String::new(), 
                                                    |mut acc, (key, value)| {
                                                        acc.push_str(&format!(
                                                            "\n {key}: {}",
                                                            value.to_string().lines().fold(
                                                                std::string::String::new(),
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
                        *element.ident.as_ref().expect(ident_in_none_stringified) != *code_occurence_snake_case_stringified
                    ).map(|element|{
                        let element_ident = &element.ident.as_ref().expect(ident_in_none_stringified);
                        let conversion_token_stream = match ErrorOccurenceFieldAttribute::from(element) {
                            ErrorOccurenceFieldAttribute::EoToStdStringString => {
                                quote::quote!{
                                    #element_ident: {
                                        error_occurence_lib::ToStdStringString::to_std_string_string(&#element_ident)
                                    }
                                }
                            },
                            ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize | 
                            ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize | 
                            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                quote::quote!{
                                    #element_ident
                                }
                            },
                            ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                quote::quote!{
                                    #element_ident: {
                                        #element_ident.into_serialize_deserialize_version()
                                    }
                                }
                            },
                            ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
                                quote::quote!{
                                    #element_ident: { 
                                        #element_ident.into_iter().map(|element|error_occurence_lib::ToStdStringString::to_std_string_string(&element)).collect()
                                    }
                                }
                            },
                            ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                quote::quote!{
                                    #element_ident: {
                                        #element_ident.into_iter().map(|element|element.into_serialize_deserialize_version()).collect()
                                    }
                                }
                            },
                            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => {
                                quote::quote!{
                                    #element_ident: {
                                        #element_ident.into_iter().map(|(key, value)|
                                            (key, error_occurence_lib::ToStdStringString::to_std_string_string(&value))
                                        ).collect()
                                    }
                                }
                            },
                            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
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
                    let fields_idents_idents_with_serialize_deserialize_excluding_code_occurence_token_stream = fields.iter().filter(|element|
                        *element.ident.as_ref().expect(ident_in_none_stringified) != *code_occurence_snake_case_stringified
                    ).map(|element|{
                        let element_ident = element.ident.as_ref().expect(ident_in_none_stringified);
                        let element_type_token_stream = {
                            let element_type = &element.ty;
                            quote::quote!{#element_type}
                        };
                        let element_type_with_serialize_deserialize_token_stream = match ErrorOccurenceFieldAttribute::from(element) {
                            ErrorOccurenceFieldAttribute::EoToStdStringString => {
                                quote::quote!{
                                    #std_string_string_token_stream
                                }
                            },
                            ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize | ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => element_type_token_stream,
                            ErrorOccurenceFieldAttribute::EoErrorOccurence => {
                                let element_type_with_serialize_deserialize_token_stream = {
                                    let value = format!(
                                        "{}{}",
                                        {
                                            let element_type = &element.ty;
                                            quote::quote!{#element_type}
                                        },
                                        proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
                                    );
                                    value
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                quote::quote!{
                                    #element_type_with_serialize_deserialize_token_stream
                                }
                            },
                            ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
                                quote::quote!{
                                    std::vec::Vec<#std_string_string_token_stream>
                                }
                            },
                            ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                                let segments = if let syn::Type::Path(value) = &element.ty {
                                    &value.path.segments
                                }
                                else {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} Type::Path(value) != &element.ty");
                                };
                                assert!(segments.len() == 3, "{proc_macro_name_upper_camel_case_ident_stringified} segments.len() != 3");
                                let first_segment = segments.iter().next().expect("no .next() element");
                                assert!(first_segment.ident == std_snake_case_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} first_segment.ident != {std_snake_case_stringified} {}", first_segment.ident);
                                assert!(!(first_segment.arguments != syn::PathArguments::None), "{proc_macro_name_upper_camel_case_ident_stringified} first_segment.arguments != PathArguments::None");
                                let second_segment = segments.iter().nth(1).expect("no .nth(1) element");
                                {
                                    let vec_snake_case_stringified = <naming_constants::Vec as naming_constants::Naming>::snake_case_stringified();
                                    assert!(second_segment.ident == vec_snake_case_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} second_segment.ident != {vec_snake_case_stringified} {}", second_segment.ident);
                                };
                                assert!(!(second_segment.arguments != syn::PathArguments::None), "{proc_macro_name_upper_camel_case_ident_stringified} second_segment.arguments != PathArguments::None");
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
                                        proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified(),
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
                            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => {
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
                            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                                let _: &syn::GenericArgument = get_type_path_third_segment_second_argument_check_if_hashmap(
                                    element,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                    std_snake_case_stringified,
                                    &std_string_string_token_stream,
                                );
                                element_type_token_stream
                            },
                            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
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
                                        proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified(),
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
                    quote::quote! {
                        #element_ident {
                            #(#fields_idents_idents_with_serialize_deserialize_excluding_code_occurence_token_stream)*
                            #code_occurence_snake_case_token_stream: error_occurence_lib::code_occurence::CodeOccurence,
                        }
                    }
                });
                generate_enum_ident_with_serialize_deserialize_token_stream(
                    &quote::quote! {#(#variants_token_stream),*}
                )
            };
            let impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream = generate_impl_std_fmt_display_handle_token_stream(&ident_with_serialize_deserialize_token_stream);
            quote::quote! {
                #impl_std_fmt_display_for_ident_token_stream
                #impl_ident_into_serialize_deserialize_version_token_stream
                #enum_ident_with_serialize_deserialize_token_stream
                #impl_std_fmt_display_for_ident_with_serialize_deserialize_token_stream
            }
        },
        proc_macro_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant::Unnamed => {
            let generate_display_formatter_unnamed_token_stream = |ident_token_stream: &proc_macro2::TokenStream|{
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
                    let display_formatter_unnamed_token_stream = generate_display_formatter_unnamed_token_stream(&quote::quote!{#ident});
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
                            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
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
                    let display_formatter_unnamed_token_stream = generate_display_formatter_unnamed_token_stream(&ident_with_serialize_deserialize_token_stream);
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
