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
            let (
                handle_upper_camel_case_upper_camel_case_token_stream,
                handle_snake_case_token_upper_camel_case_stream,
                elements_concat_value_upper_camel_case_double_quotes_token_stream,
                elements_concat_value_snake_case_double_quotes_token_stream,
            ) = {
                let upper_camel_case_upper_camel_case_stringified = "UpperCamelCase";
                let snake_case_upper_camel_case_stringified = "SnakeCase";
                let elements_concat_upper_camel_case_stringified = element.iter().fold(std::string::String::from(""), |mut acc, element| {
                    acc.push_str(&proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(element));
                    acc
                });
                let elements_concat_value_upper_camel_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &element.iter().fold(std::string::String::from(""), |mut acc, element| {
                        if element == "self" {
                            acc.push_str("{value}");
                        }
                        else {
                            acc.push_str(&proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(element));
                        }
                        acc
                    }),
                    &proc_macro_name_snake_case_stringified
                );
                let elements_concat_value_snake_case_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &{
                        let mut value = element.iter().fold(std::string::String::from(""), |mut acc, element| {
                            let symbol = '_';
                            if element == "self" {
                                acc.push_str(&format!("{{value}}{symbol}"));
                            }
                            else {
                                acc.push_str(&format!("{}{symbol}", proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(element)));
                            }
                            acc
                        });
                        let _ = value.pop();
                        value
                    },
                    &proc_macro_name_snake_case_stringified
                );
                let handle_upper_camel_case_upper_camel_case_token_stream = {
                    let value = format!("{elements_concat_upper_camel_case_stringified}{upper_camel_case_upper_camel_case_stringified}");
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let handle_snake_case_token_upper_camel_case_stream = {
                    let value = format!("{elements_concat_upper_camel_case_stringified}{snake_case_upper_camel_case_stringified}");
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                (
                    handle_upper_camel_case_upper_camel_case_token_stream,
                    handle_snake_case_token_upper_camel_case_stream,
                    elements_concat_value_upper_camel_case_double_quotes_token_stream,
                    elements_concat_value_snake_case_double_quotes_token_stream,
                )
            };
            let generate_struct_token_stream = |
                struct_ident_token_stream: &proc_macro2::TokenStream,
                elements_concat_value_case_double_quotes_token_stream: &proc_macro2::TokenStream,
                is_upper_camel_case: std::primitive::bool,
            |{
                let panic_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                    &format!("failed to parse stringified {struct_ident_token_stream} into proc_macro2::TokenStream: {{value_stringified}}"),
                    &proc_macro_name_snake_case_stringified
                );
                let casing_token_stream = if is_upper_camel_case {
                    quote::quote!{proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified}
                }
                else {
                    quote::quote!{proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified}
                };
                quote::quote!{
                    #[derive(Debug)]
                    pub struct #struct_ident_token_stream(std::string::String);
                    impl #struct_ident_token_stream {
                        fn wrap(value: &dyn std::fmt::Display) -> Self {
                            Self(format!(#elements_concat_value_case_double_quotes_token_stream))
                        }
                        pub fn from_dyn_std_fmt_display(value: &dyn std::fmt::Display) -> Self {
                            Self::wrap(&#casing_token_stream(&value.to_string()))
                        }
                        pub fn from_quote_to_tokens(value: &dyn quote::ToTokens) -> Self {
                            Self::wrap(&#casing_token_stream(&{
                                let mut tokens = proc_macro2::TokenStream::new();
                                quote::ToTokens::to_tokens(&value, &mut tokens);
                                tokens
                            }.to_string()))
                        }
                    }
                    impl std::fmt::Display for #struct_ident_token_stream {
                        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(formatter, "{}", self.0)
                        }
                    }
                    impl quote::ToTokens for #struct_ident_token_stream {
                        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                            let value_stringified = self.to_string();
                            let value_token_stream = value_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!(#panic_handle_token_stream));
                            value_token_stream.to_tokens(tokens)
                        }
                    }
                }
            };
            let pub_struct_upper_camel_case_token_stream = generate_struct_token_stream(
                &handle_upper_camel_case_upper_camel_case_token_stream,
                &elements_concat_value_upper_camel_case_double_quotes_token_stream,
                true,
            );
            let pub_struct_snake_case_token_stream = generate_struct_token_stream(
                &handle_snake_case_token_upper_camel_case_stream,
                &elements_concat_value_snake_case_double_quotes_token_stream,
                false
            );
            quote::quote! {
                #pub_struct_upper_camel_case_token_stream
                #pub_struct_snake_case_token_stream
            }
        });
    let generated = quote::quote! {#(#implementations_token_stream)*};
    // println!("{generated}");
    generated.into()
}