const REGEX_VALUE: &str = r"^[a-zA-Z]+$";

#[proc_macro]
pub fn generate_upper_camel_and_snake_case_stringified_and_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_snake_case_stringified = "generate_upper_camel_and_snake_case_stringified_and_token_stream";
    let implementations_token_stream = serde_json::from_str::<std::vec::Vec<std::vec::Vec<std::string::String>>>(&input.to_string())
        .expect("failed to convert tokens input into valid json string[][] pattern")
        .into_iter()
        .map(|element| {
            {
                let regex = regex::Regex::new(REGEX_VALUE).unwrap();
                for element in &element {
                    if !regex.is_match(&element) {
                        panic!("{proc_macro_name_snake_case_stringified} invalid element {element}, regex: {REGEX_VALUE}");
                    }
                }
            }
            let phrase_part_upper_camel_case_stringified = element.iter().fold(std::string::String::from(""), |mut acc, element| {
                let element_upper_camel_case_stringified = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(*&element);
                acc.push_str(&element_upper_camel_case_stringified);
                acc
            });
            let phrase_part_snake_case_stringified = element.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, element)| {
                let element_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(*&element);
                if index == 0 {
                    acc.push_str(&element_snake_case_stringified);
                } else {
                    acc.push_str(&format!("_{element_snake_case_stringified}"));
                }
                acc
            });
            let phrase_part_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&phrase_part_upper_camel_case_stringified, &proc_macro_name_snake_case_stringified);
            let phrase_part_snake_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(&phrase_part_snake_case_stringified, &proc_macro_name_snake_case_stringified);
            let phrase_part_upper_camel_case_token_stream = {
                phrase_part_upper_camel_case_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {phrase_part_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let phrase_part_snake_case_token_stream = {
                phrase_part_snake_case_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {phrase_part_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let phrase_part_upper_camel_case_upper_camel_case_token_stream = {
                let value = format!("{phrase_part_upper_camel_case_stringified}UpperCamelCase");
                value
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let phrase_part_snake_case_upper_camel_case_token_stream = {
                let value = format!("{phrase_part_upper_camel_case_stringified}SnakeCase");
                value
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let generate_struct_declaration = |struct_name_token_stream: &proc_macro2::TokenStream| quote::quote! {pub struct #struct_name_token_stream;};
            let upper_camel_case_struct_declaration_token_stream = generate_struct_declaration(&phrase_part_upper_camel_case_upper_camel_case_token_stream);
            let snake_case_struct_declaration_token_stream = generate_struct_declaration(&phrase_part_snake_case_upper_camel_case_token_stream);
            let generate_display_implementation_token_stream = |struct_name_token_stream: &proc_macro2::TokenStream, write_content_token_stream: &proc_macro2::TokenStream| {
                quote::quote! {
                    impl std::fmt::Display for #struct_name_token_stream {
                        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            write!(formatter, #write_content_token_stream)
                        }
                    }
                }
            };
            let upper_camel_case_display_implementation_token_stream = generate_display_implementation_token_stream(&phrase_part_upper_camel_case_upper_camel_case_token_stream, &phrase_part_upper_camel_case_double_quotes_token_stream);
            let snake_case_display_implementation_token_stream = generate_display_implementation_token_stream(&phrase_part_snake_case_upper_camel_case_token_stream, &phrase_part_snake_case_double_quotes_token_stream);
            let generate_to_tokens_implementation_token_stream = |struct_name_token_stream: &proc_macro2::TokenStream, quote_content_token_stream: &proc_macro2::TokenStream| {
                quote::quote! {
                    impl quote::ToTokens for #struct_name_token_stream {
                        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                            quote::quote!{#quote_content_token_stream}.to_tokens(tokens)
                        }
                    }
                }
            };
            let upper_camel_case_to_tokens_implementation_token_stream = generate_to_tokens_implementation_token_stream(&phrase_part_upper_camel_case_upper_camel_case_token_stream, &phrase_part_upper_camel_case_token_stream);
            let snake_to_tokens_implementation_token_stream = generate_to_tokens_implementation_token_stream(&phrase_part_snake_case_upper_camel_case_token_stream, &phrase_part_snake_case_token_stream);
            quote::quote! {
                #upper_camel_case_struct_declaration_token_stream
                #upper_camel_case_display_implementation_token_stream
                #upper_camel_case_to_tokens_implementation_token_stream
                #snake_case_struct_declaration_token_stream
                #snake_case_display_implementation_token_stream
                #snake_to_tokens_implementation_token_stream
            }
        });
    let generated = quote::quote! {#(#implementations_token_stream)*};
    // println!("{generated}");
    generated.into()
}

#[proc_macro]
pub fn generate_self_upper_camel_and_snake_case_stringified_and_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_snake_case_stringified = "generate_self_upper_camel_and_snake_case_stringified_and_token_stream";
    let implementations_token_stream = serde_json::from_str::<std::vec::Vec<std::vec::Vec<std::string::String>>>(&input.to_string())
        .expect("failed to convert tokens input into valid json string[][] pattern")
        .into_iter()
        .map(|element| {
            {
                let regex = regex::Regex::new(REGEX_VALUE).unwrap();
                for element in &element {
                    if !regex.is_match(&element) {
                        panic!("{proc_macro_name_snake_case_stringified} invalid element {element}, regex: {REGEX_VALUE}");
                    }
                }
            }
            let self_match_name = "self";
            {
                let mut is_self_exists_and_only_one = false;
                for element in &element {
                    if element == self_match_name {
                        is_self_exists_and_only_one = true;
                        break;
                    }
                }
                if !is_self_exists_and_only_one {
                    panic!("{proc_macro_name_snake_case_stringified} cannot find {self_match_name}");
                }
            }
            let (upper_camel_case_stringified_trait_name_upper_camel_case_token_stream, snake_case_stringified_trait_name_upper_camel_case_token_stream, upper_camel_case_token_stream_trait_name_upper_camel_case_token_stream, snake_case_token_stream_trait_name_upper_camel_case_token_stream) = {
                let upper_camel_case_upper_camel_case_stringified = "UpperCamelCase";
                let snake_case_upper_camel_case_stringified = "SnakeCase";
                let stringified_upper_camel_case = "Stringified";
                let token_stream_upper_camel_case_stringified = "TokenStream";
                let elements_concat_upper_camel_case_stringified = element.iter().fold(std::string::String::from(""), |mut acc, element| {
                    acc.push_str(&proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(element));
                    acc
                });
                let upper_camel_case_stringified_trait_name_upper_camel_case_token_stream = {
                    let value = format!("{elements_concat_upper_camel_case_stringified}{upper_camel_case_upper_camel_case_stringified}{stringified_upper_camel_case}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let snake_case_stringified_trait_name_upper_camel_case_token_stream = {
                    let value = format!("{elements_concat_upper_camel_case_stringified}{snake_case_upper_camel_case_stringified}{stringified_upper_camel_case}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let upper_camel_case_token_stream_trait_name_upper_camel_case_token_stream = {
                    let value = format!("{elements_concat_upper_camel_case_stringified}{upper_camel_case_upper_camel_case_stringified}{token_stream_upper_camel_case_stringified}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let snake_case_token_stream_trait_name_upper_camel_case_token_stream = {
                    let value = format!("{elements_concat_upper_camel_case_stringified}{snake_case_upper_camel_case_stringified}{token_stream_upper_camel_case_stringified}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                (
                    upper_camel_case_stringified_trait_name_upper_camel_case_token_stream,
                    snake_case_stringified_trait_name_upper_camel_case_token_stream,
                    upper_camel_case_token_stream_trait_name_upper_camel_case_token_stream,
                    snake_case_token_stream_trait_name_upper_camel_case_token_stream,
                )
            };
            let std_string_string = token_patterns::StdStringString;
            let proc_macro2_token_stream = quote::quote! {proc_macro2::TokenStream}; //todo maybe reuse
            let (
                upper_camel_case_stringified_trait_function_name_snake_case_token_stream,
                snake_case_stringified_trait_function_name_snake_case_token_stream,
                upper_camel_case_token_stream_trait_function_name_snake_case_token_stream,
                snake_case_token_stream_trait_function_name_snake_case_token_stream,
            ) = {
                let prefix_upper_camel_case = "_upper_camel_case";
                let prefix_snake_case = "_snake_case";
                let prefix_stringified = "_stringified";
                let prefix_token_stream = "_token_stream";
                let elements_concat_snake_case_stringified = element.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, element)| {
                    let element_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(element);
                    if index == 0 {
                        acc.push_str(&element_snake_case_stringified);
                    } else {
                        acc.push_str(&format!("_{element_snake_case_stringified}"));
                    }
                    acc
                });
                let upper_camel_case_stringified_trait_function_name_snake_case_token_stream = {
                    let value = format!("{elements_concat_snake_case_stringified}{prefix_upper_camel_case}{prefix_stringified}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let snake_case_stringified_trait_function_name_snake_case_token_stream = {
                    let value = format!("{elements_concat_snake_case_stringified}{prefix_snake_case}{prefix_stringified}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let upper_camel_case_token_stream_trait_function_name_snake_case_token_stream = {
                    let value = format!("{elements_concat_snake_case_stringified}{prefix_upper_camel_case}{prefix_token_stream}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let snake_case_token_stream_trait_function_name_snake_case_token_stream = {
                    let value = format!("{elements_concat_snake_case_stringified}{prefix_snake_case}{prefix_token_stream}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                (
                    upper_camel_case_stringified_trait_function_name_snake_case_token_stream,
                    snake_case_stringified_trait_function_name_snake_case_token_stream,
                    upper_camel_case_token_stream_trait_function_name_snake_case_token_stream,
                    snake_case_token_stream_trait_function_name_snake_case_token_stream,
                )
            };
            let (
                upper_camel_case_stringified_trait_declaration_upper_camel_case_token_stream,
                snake_case_stringified_trait_declaration_upper_camel_case_token_stream,
                upper_camel_case_token_stream_trait_declaration_upper_camel_case_token_stream,
                snake_case_token_stream_trait_declaration_upper_camel_case_token_stream,
            ) = {
                let generate_pub_trait_declaration_token_stream = |trait_name_token_stream: &proc_macro2::TokenStream, trait_function_name_token_stream: &proc_macro2::TokenStream, return_type_token_stream: &dyn quote::ToTokens| -> proc_macro2::TokenStream {
                    quote::quote! {
                        pub trait #trait_name_token_stream {
                            fn #trait_function_name_token_stream(
                                &self,
                            ) -> #return_type_token_stream;
                        }
                    }
                };
                let upper_camel_case_stringified_trait_declaration_upper_camel_case_token_stream =
                    generate_pub_trait_declaration_token_stream(&upper_camel_case_stringified_trait_name_upper_camel_case_token_stream, &upper_camel_case_stringified_trait_function_name_snake_case_token_stream, &std_string_string);
                let snake_case_stringified_trait_declaration_upper_camel_case_token_stream = generate_pub_trait_declaration_token_stream(&snake_case_stringified_trait_name_upper_camel_case_token_stream, &snake_case_stringified_trait_function_name_snake_case_token_stream, &std_string_string);
                let upper_camel_case_token_stream_trait_declaration_upper_camel_case_token_stream =
                    generate_pub_trait_declaration_token_stream(&upper_camel_case_token_stream_trait_name_upper_camel_case_token_stream, &upper_camel_case_token_stream_trait_function_name_snake_case_token_stream, &proc_macro2_token_stream);
                let snake_case_token_stream_trait_declaration_upper_camel_case_token_stream =
                    generate_pub_trait_declaration_token_stream(&snake_case_token_stream_trait_name_upper_camel_case_token_stream, &snake_case_token_stream_trait_function_name_snake_case_token_stream, &proc_macro2_token_stream);
                (
                    upper_camel_case_stringified_trait_declaration_upper_camel_case_token_stream,
                    snake_case_stringified_trait_declaration_upper_camel_case_token_stream,
                    upper_camel_case_token_stream_trait_declaration_upper_camel_case_token_stream,
                    snake_case_token_stream_trait_declaration_upper_camel_case_token_stream,
                )
            };
            let (upper_camel_case_stringified_impl_trait_upper_camel_case_token_stream, snake_case_stringified_impl_trait_upper_camel_case_token_stream, upper_camel_case_token_stream_impl_trait_upper_camel_case_token_stream, snake_case_token_stream_impl_trait_upper_camel_case_token_stream) = {
                let generate_impl_trait_token_stream = |trait_name_token_stream: &proc_macro2::TokenStream,
                                                        generic_must_implement_token_stream: &proc_macro2::TokenStream,
                                                        trait_function_name_token_stream: &proc_macro2::TokenStream,
                                                        return_type_token_stream: &dyn quote::ToTokens,
                                                        content_token_stream: &proc_macro2::TokenStream|
                 -> proc_macro2::TokenStream {
                    quote::quote! {
                        impl<T> #trait_name_token_stream for T
                        where
                            T: #generic_must_implement_token_stream,
                        {
                            fn #trait_function_name_token_stream(
                                &self,
                            ) -> #return_type_token_stream {
                                #content_token_stream
                            }
                        }
                    }
                };
                let upper_camel_case_stringified_impl_trait_upper_camel_case_token_stream = generate_impl_trait_token_stream(
                    &upper_camel_case_stringified_trait_name_upper_camel_case_token_stream,
                    &quote::quote! {proc_macro_common::naming_conventions::ToUpperCamelCaseStringified},
                    &upper_camel_case_stringified_trait_function_name_snake_case_token_stream,
                    &std_string_string,
                    &{
                        let upper_camel_case_stringified_format_parameters_places_token_stream = {
                            let value = element.iter().fold(std::string::String::from(""), |mut acc, element| {
                                if element == self_match_name {
                                    acc.push_str("{}");
                                } else {
                                    acc.push_str(&proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(*&element));
                                }
                                acc
                            });
                            proc_macro_common::generate_quotes::double_quotes_token_stream(&value, &proc_macro_name_snake_case_stringified)
                        };
                        let upper_camel_case_stringified_format_parameters_calls_token_stream = element.iter().map(|element| {
                            if element == self_match_name {
                                quote::quote! {self.to_upper_camel_case_stringified(),}
                            } else {
                                proc_macro2::TokenStream::new()
                            }
                        });
                        quote::quote! {
                            format!(
                                #upper_camel_case_stringified_format_parameters_places_token_stream,
                                #(#upper_camel_case_stringified_format_parameters_calls_token_stream)*
                            )
                        }
                    },
                );
                let snake_case_stringified_impl_trait_upper_camel_case_token_stream = generate_impl_trait_token_stream(
                    &snake_case_stringified_trait_name_upper_camel_case_token_stream,
                    &quote::quote! {proc_macro_common::naming_conventions::ToSnakeCaseStringified},
                    &snake_case_stringified_trait_function_name_snake_case_token_stream,
                    &std_string_string,
                    &{
                        let snake_case_stringified_format_parameters_places_token_stream = {
                            let value = element.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, element)| {
                                if index == 0 {
                                    if element == self_match_name {
                                        acc.push_str("{}");
                                    } else {
                                        acc.push_str(&proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(*&element));
                                    }
                                } else {
                                    if element == self_match_name {
                                        acc.push_str("_{}");
                                    } else {
                                        acc.push_str(&format!("_{}", proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(*&element)));
                                    }
                                }
                                acc
                            });
                            proc_macro_common::generate_quotes::double_quotes_token_stream(&value, &proc_macro_name_snake_case_stringified)
                        };
                        let snake_case_stringified_format_parameters_calls_token_stream = element.iter().map(|element| {
                            if element == self_match_name {
                                quote::quote! {
                                    self.to_snake_case_stringified(),
                                }
                            } else {
                                proc_macro2::TokenStream::new()
                            }
                        });
                        quote::quote! {
                            format!(
                                #snake_case_stringified_format_parameters_places_token_stream,
                                #(#snake_case_stringified_format_parameters_calls_token_stream)*
                            )
                        }
                    },
                );
                let (upper_camel_case_token_stream_impl_trait_upper_camel_case_token_stream, snake_case_token_stream_impl_trait_upper_camel_case_token_stream) = {
                    let value_parse_token_stream = quote::quote! {
                        value.parse::<#proc_macro2_token_stream>()
                        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let upper_camel_case_token_stream_impl_trait_upper_camel_case_token_stream = generate_impl_trait_token_stream(
                        &upper_camel_case_token_stream_trait_name_upper_camel_case_token_stream,
                        &upper_camel_case_stringified_trait_name_upper_camel_case_token_stream,
                        &upper_camel_case_token_stream_trait_function_name_snake_case_token_stream,
                        &proc_macro2_token_stream,
                        &quote::quote! {
                            let value = self.#upper_camel_case_stringified_trait_function_name_snake_case_token_stream();
                            #value_parse_token_stream
                        },
                    );
                    let snake_case_token_stream_impl_trait_upper_camel_case_token_stream = generate_impl_trait_token_stream(
                        &snake_case_token_stream_trait_name_upper_camel_case_token_stream,
                        &snake_case_stringified_trait_name_upper_camel_case_token_stream,
                        &snake_case_token_stream_trait_function_name_snake_case_token_stream,
                        &proc_macro2_token_stream,
                        &quote::quote! {
                            let value = self.#snake_case_stringified_trait_function_name_snake_case_token_stream();
                            #value_parse_token_stream
                        },
                    );
                    (upper_camel_case_token_stream_impl_trait_upper_camel_case_token_stream, snake_case_token_stream_impl_trait_upper_camel_case_token_stream)
                };
                (
                    upper_camel_case_stringified_impl_trait_upper_camel_case_token_stream,
                    snake_case_stringified_impl_trait_upper_camel_case_token_stream,
                    upper_camel_case_token_stream_impl_trait_upper_camel_case_token_stream,
                    snake_case_token_stream_impl_trait_upper_camel_case_token_stream,
                )
            };
            quote::quote! {
                #upper_camel_case_stringified_trait_declaration_upper_camel_case_token_stream
                #upper_camel_case_stringified_impl_trait_upper_camel_case_token_stream

                #snake_case_stringified_trait_declaration_upper_camel_case_token_stream
                #snake_case_stringified_impl_trait_upper_camel_case_token_stream

                #upper_camel_case_token_stream_trait_declaration_upper_camel_case_token_stream
                #upper_camel_case_token_stream_impl_trait_upper_camel_case_token_stream

                #snake_case_token_stream_trait_declaration_upper_camel_case_token_stream
                #snake_case_token_stream_impl_trait_upper_camel_case_token_stream
            }
        });
    let generated = quote::quote! {#(#implementations_token_stream)*};
    // println!("{generated}");
    generated.into()
}
