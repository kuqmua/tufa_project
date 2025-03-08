const REGEX_VALUE: &str = r"^[a-zA-Z]+$";

#[proc_macro]
pub fn generate_upper_camel_and_snake_case_stringified_and_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let implementations_token_stream = serde_json::from_str::<std::vec::Vec<std::vec::Vec<std::string::String>>>(&input.to_string())
        .expect("failed to convert tokens input into valid json string[][] pattern")
        .into_iter()
        .map(|element| {
            {
                let regex = regex::Regex::new(REGEX_VALUE).unwrap();
                for element in &element {
                    if !regex.is_match(element) {
                        panic!("invalid element {element}, regex: {REGEX_VALUE}");
                    }
                }
            }
            let phrase_part_upper_camel_case_stringified = element.iter().fold(std::string::String::from(""), |mut acc, element| {
                let element_upper_camel_case_stringified = naming_common::AsRefStrToUpperCamelCaseStringified::case(element);
                acc.push_str(&element_upper_camel_case_stringified);
                acc
            });
            let phrase_part_snake_case_stringified = element.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, element)| {
                let element_snake_case_stringified = naming_common::AsRefStrToSnakeCaseStringified::case(element);
                if index == 0 {
                    acc.push_str(&element_snake_case_stringified);
                } else {
                    acc.push_str(&format!("_{element_snake_case_stringified}"));
                }
                acc
            });
            let phrase_part_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&phrase_part_upper_camel_case_stringified);
            let phrase_part_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&phrase_part_snake_case_stringified);
            let phrase_part_upper_camel_case_token_stream = {
                phrase_part_upper_camel_case_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{phrase_part_upper_camel_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let phrase_part_snake_case_token_stream = {
                phrase_part_snake_case_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{phrase_part_snake_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let phrase_part_upper_camel_case_upper_camel_case_token_stream = {
                let value = format!("{phrase_part_upper_camel_case_stringified}UpperCamelCase");
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let phrase_part_snake_case_upper_camel_case_token_stream = {
                let value = format!("{phrase_part_upper_camel_case_stringified}SnakeCase");
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let generate_struct_declaration = |struct_name_token_stream: &dyn quote::ToTokens| quote::quote! {pub struct #struct_name_token_stream;};
            let upper_camel_case_struct_declaration_token_stream = generate_struct_declaration(&phrase_part_upper_camel_case_upper_camel_case_token_stream);
            let snake_case_struct_declaration_token_stream = generate_struct_declaration(&phrase_part_snake_case_upper_camel_case_token_stream);
            let generate_display_implementation_token_stream = |struct_name_token_stream: &dyn quote::ToTokens, write_content_token_stream: &dyn quote::ToTokens| {
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
            let generate_to_tokens_implementation_token_stream = |struct_name_token_stream: &dyn quote::ToTokens, quote_content_token_stream: &dyn quote::ToTokens| {
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
    panic_location::panic_location();
    let implementations_token_stream = serde_json::from_str::<std::vec::Vec<std::vec::Vec<std::string::String>>>(&input.to_string())
        .expect("failed to convert tokens input into valid json string[][] pattern")
        .into_iter()
        .map(|element| {
            {
                let regex = regex::Regex::new(REGEX_VALUE).unwrap();
                for element in &element {
                    if !regex.is_match(element) {
                        panic!("invalid element {element}, regex: {REGEX_VALUE}");
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
                    panic!("cannot find {self_match_name}");
                }
            }
            let (
                elements_concat_value_upper_camel_case_double_quotes_token_stream,
                elements_concat_value_snake_case_double_quotes_token_stream,
                struct_upper_camel_case_upper_camel_case_token_stream,
                struct_snake_case_token_upper_camel_case_stream,
                trait_upper_camel_case_upper_camel_case_token_stream,
                trait_snake_case_token_upper_camel_case_stream,
            ) = {
                let upper_camel_case_upper_camel_case_stringified = "UpperCamelCase";
                let snake_case_upper_camel_case_stringified = "SnakeCase";
                let elements_concat_upper_camel_case_stringified = element.iter().fold(std::string::String::from(""), |mut acc, element| {
                    acc.push_str(&naming_common::AsRefStrToUpperCamelCaseStringified::case(element));
                    acc
                });
                let elements_concat_value_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.iter().fold(std::string::String::from(""), |mut acc, element| {
                    if element == "self" {
                        acc.push_str("{value}");
                    } else {
                        acc.push_str(&naming_common::AsRefStrToUpperCamelCaseStringified::case(element));
                    }
                    acc
                }));
                let elements_concat_value_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&{
                    let mut value = element.iter().fold(std::string::String::from(""), |mut acc, element| {
                        let symbol = '_';
                        if element == "self" {
                            acc.push_str(&format!("{{value}}{symbol}"));
                        } else {
                            acc.push_str(&format!("{}{symbol}", naming_common::AsRefStrToSnakeCaseStringified::case(element)));
                        }
                        acc
                    });
                    let _ = value.pop();
                    value
                });
                let struct_upper_camel_case_upper_camel_case_token_stream = {
                    let value = format!("{elements_concat_upper_camel_case_stringified}{upper_camel_case_upper_camel_case_stringified}");
                    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let struct_snake_case_token_upper_camel_case_stream = {
                    let value = format!("{elements_concat_upper_camel_case_stringified}{snake_case_upper_camel_case_stringified}");
                    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let (trait_upper_camel_case_upper_camel_case_token_stream, trait_snake_case_token_upper_camel_case_stream) = {
                    let trait_upper_camel_case_stringified = "Trait";
                    let trait_upper_camel_case_upper_camel_case_token_stream = {
                        let value = format!("{elements_concat_upper_camel_case_stringified}{upper_camel_case_upper_camel_case_stringified}{trait_upper_camel_case_stringified}");
                        value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let trait_snake_case_token_upper_camel_case_stream = {
                        let value = format!("{elements_concat_upper_camel_case_stringified}{snake_case_upper_camel_case_stringified}{trait_upper_camel_case_stringified}");
                        value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    (trait_upper_camel_case_upper_camel_case_token_stream, trait_snake_case_token_upper_camel_case_stream)
                };
                (
                    elements_concat_value_upper_camel_case_double_quotes_token_stream,
                    elements_concat_value_snake_case_double_quotes_token_stream,
                    struct_upper_camel_case_upper_camel_case_token_stream,
                    struct_snake_case_token_upper_camel_case_stream,
                    trait_upper_camel_case_upper_camel_case_token_stream,
                    trait_snake_case_token_upper_camel_case_stream,
                )
            };
            let generate_struct_token_stream = |elements_concat_value_case_double_quotes_token_stream: &dyn quote::ToTokens, is_upper_camel_case: std::primitive::bool, trait_ident_token_stream: &dyn quote::ToTokens| {
                let struct_ident_token_stream = if is_upper_camel_case {
                    quote::quote! {#struct_upper_camel_case_upper_camel_case_token_stream}
                } else {
                    quote::quote! {#struct_snake_case_token_upper_camel_case_stream}
                };
                let panic_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("failed to parse stringified {struct_ident_token_stream} into proc_macro2::TokenStream: {{value_stringified}}"));
                let casing_token_stream = if is_upper_camel_case {
                    quote::quote! {naming_common::AsRefStrToUpperCamelCaseStringified::case}
                } else {
                    quote::quote! {naming_common::AsRefStrToSnakeCaseStringified::case}
                };
                quote::quote! {
                    #[derive(Debug)]
                    pub struct #struct_ident_token_stream(std::string::String);
                    impl #struct_ident_token_stream {
                        fn wrap(value: &dyn std::fmt::Display) -> Self {
                            Self(Self::format(value))
                        }
                        fn format(value: &dyn std::fmt::Display) -> std::string::String {
                            format!(#elements_concat_value_case_double_quotes_token_stream)
                        }
                        pub fn from_display(value: &dyn std::fmt::Display) -> Self {
                            Self::wrap(&#casing_token_stream(&value.to_string()))
                        }
                        pub fn from_tokens(value: &dyn quote::ToTokens) -> Self {
                            Self::wrap(&#casing_token_stream(&{
                                let mut tokens = proc_macro2::TokenStream::new();
                                quote::ToTokens::to_tokens(&value, &mut tokens);
                                tokens
                            }.to_string()))
                        }
                        pub fn from_type_last_segment(value: &syn::Type) -> Self {
                            match value {
                                syn::Type::Path(type_path) => {
                                    let path_before_stringified = type_path.path.segments.iter().take(type_path.path.segments.len() - 1).fold(std::string::String::from(""), |mut acc, elem| {
                                        acc.push_str(&format!("{}::", elem.ident));
                                        acc
                                    });
                                    let last = type_path.path.segments.iter().last().unwrap();
                                    Self(format!("{path_before_stringified}{}", Self::format(&#casing_token_stream(&last.ident.to_string()))))
                                },
                                _ => panic!("syn::Type is not syn::Type::Path")
                            }
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
                    pub trait #trait_ident_token_stream: std::fmt::Display + quote::ToTokens {}
                    impl #trait_ident_token_stream for #struct_ident_token_stream {}
                }
            };
            let pub_struct_upper_camel_case_token_stream = generate_struct_token_stream(&elements_concat_value_upper_camel_case_double_quotes_token_stream, true, &trait_upper_camel_case_upper_camel_case_token_stream);
            let pub_struct_snake_case_token_stream = generate_struct_token_stream(&elements_concat_value_snake_case_double_quotes_token_stream, false, &trait_snake_case_token_upper_camel_case_stream);
            quote::quote! {
                #pub_struct_upper_camel_case_token_stream
                #pub_struct_snake_case_token_stream
            }
        });
    let generated = quote::quote! {#(#implementations_token_stream)*};
    // println!("{generated}");
    generated.into()
}

////////////////////
/*
only works if all enum variants without fields like this
#[derive(macros_assistants::AsRefStrToUpperCamelCaseStringified)]
enum Operation {
     One,
     Two,
     Three,
}
*/
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToUpperCamelCaseStringified)]
pub fn as_ref_str_enum_with_unit_fields_to_upper_camel_case_stringified(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let data_enum = if let syn::Data::Enum(data_enum) = &syn_derive_input.data {
        data_enum
    } else {
        panic!("does work only on structs!");
    };
    let std_string_string_token_stream = token_patterns::StdStringString;
    let variants_matching_values_token_stream = data_enum
        .variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                let variant_ident_upper_camel_case_stringified = naming_common::ToTokensToUpperCamelCaseStringified::case(&variant_ident);
                let variant_ident_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&variant_ident_upper_camel_case_stringified);
                quote::quote! {Self::#variant_ident => #std_string_string_token_stream::from(#variant_ident_upper_camel_case_double_quotes_token_stream)}
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("supported only syn::Fields::Unit"),
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let trait_path_token_stream = trait_path_token_stream();
    let generated = quote::quote! {
        impl #trait_path_token_stream::AsRefStrToUpperCamelCaseStringified for #ident {
            fn case(&self) -> #std_string_string_token_stream {//todo maybe write duplicate Trait with &str instead of std::string::String
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{generated}");
    generated.into()
}

/*
only works if all enum variants without fields like this
 #[derive(macros_assistants::AsRefStrToSnakeCaseStringified)]
 enum Operation {
     One,
     Two,
     Three,
 }
*/
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToSnakeCaseStringified)]
pub fn as_ref_str_enum_with_unit_fields_to_snake_case_stringified(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let data_enum = if let syn::Data::Enum(data_enum) = &syn_derive_input.data {
        data_enum
    } else {
        panic!("does work only on structs!");
    };
    let std_string_string = token_patterns::StdStringString;
    let variants_matching_values_token_stream = data_enum
        .variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                let variant_ident_snake_case_stringified = naming_common::ToTokensToSnakeCaseStringified::case(&variant_ident);
                let variant_ident_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&variant_ident_snake_case_stringified);
                quote::quote! {Self::#variant_ident => #std_string_string::from(#variant_ident_snake_case_double_quotes_token_stream)}
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("supported only syn::Fields::Unit"),
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let trait_path_token_stream = trait_path_token_stream();
    let generated = quote::quote! {
        impl #trait_path_token_stream::AsRefStrToSnakeCaseStringified for #ident {
            fn case(&self) -> #std_string_string {
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{generated}");
    generated.into()
}
/*
only works if all enum variants without fields like this
 #[derive(macros_assistants::AsRefStrToScreamingSnakeCaseStringified)]
 enum Operation {
     One,
     Two,
     Three,
 }
*/
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToScreamingSnakeCaseStringified)]
pub fn as_ref_str_enum_with_unit_fields_to_screaming_snake_case_stringified(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let data_enum = if let syn::Data::Enum(data_enum) = &syn_derive_input.data {
        data_enum
    } else {
        panic!("does work only on structs!");
    };
    let std_string_string = token_patterns::StdStringString;
    let variants_matching_values_token_stream = data_enum
        .variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                let variant_ident_snake_case_stringified = naming_common::ToTokensToScreamingSnakeCaseStringified::case(&variant_ident);
                let variant_ident_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&variant_ident_snake_case_stringified);
                quote::quote! {Self::#variant_ident => #std_string_string::from(#variant_ident_snake_case_double_quotes_token_stream)}
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("supported only syn::Fields::Unit"),
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let trait_path_token_stream = trait_path_token_stream();
    let generated = quote::quote! {
        impl #trait_path_token_stream::ToScreamingSnakeCaseStringified for #ident {
            fn case(&self) -> #std_string_string {
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{generated}");
    generated.into()
}

fn trait_path_token_stream() -> proc_macro2::TokenStream {
    quote::quote! {naming}
}
