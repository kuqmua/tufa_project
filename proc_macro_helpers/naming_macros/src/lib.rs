#[proc_macro]
pub fn generate_upper_camel_and_snake_case_stringified_and_token_stream_from_naming_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_snake_case_stringified = "generate_upper_camel_and_snake_case_stringified_and_token_stream_from_naming_constants";
    let implementations_token_stream = serde_json::from_str::<std::vec::Vec<std::vec::Vec<std::string::String>>>(&input.to_string())
    .expect("failed to convert tokens input into valid json string[][] pattern")
    .into_iter()
    .map(|element| {
        //todo check of elements contains only Aa-Zz symbols
        let prefix_upper_camel_case = "_upper_camel_case";
        let prefix_snake_case = "_snake_case";
        let prefix_stringified = "_stringified";
        let prefix_token_stream = "_token_stream";
        let prefix_upper_camel_case_stringified = format!("{prefix_upper_camel_case}{prefix_stringified}");
        let prefix_snake_case_stringified = format!("{prefix_snake_case}{prefix_stringified}");
        let prefix_upper_camel_case_token_stream = format!("{prefix_upper_camel_case}{prefix_token_stream}");
        let prefix_snake_case_token_stream = format!("{prefix_snake_case}{prefix_token_stream}");
        let phrase_part_stringified = element.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, element)| {
            let element_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(*&element);
            if index == 0 {
                acc.push_str(&element_snake_case_stringified);
            }
            else {
                acc.push_str(&format!("_{element_snake_case_stringified}"));
            }
            acc
        });
        let phrase_upper_camel_case_stringified_token_stream = {
            let value = format!("{phrase_part_stringified}{prefix_upper_camel_case_stringified}");
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let phrase_snake_case_stringified_token_stream = {
            let value = format!("{phrase_part_stringified}{prefix_snake_case_stringified}");
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let phrase_upper_camel_case_token_stream_token_stream = {
            let value = format!("{phrase_part_stringified}{prefix_upper_camel_case_token_stream}");
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let phrase_snake_case_token_stream_token_stream = {
            let value = format!("{phrase_part_stringified}{prefix_snake_case_token_stream}");
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let std_string_string_token_stream = proc_macro_common::std_string_string_token_stream();
        let proc_macro2_token_stream = quote::quote!{proc_macro2::TokenStream};//todo maybe reuse
        let upper_camel_case_stringified_format_parameters_places_token_stream = {
            let value = element.iter().fold(std::string::String::from(""), |mut acc, _| {
                acc.push_str("{}");
                acc
            });
            proc_macro_common::generate_quotes::token_stream(
                &value,
                &proc_macro_name_snake_case_stringified
            )
        };
        let snake_case_stringified_format_parameters_places_token_stream = {
            let value = element.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, _)| {
                if index == 0 {
                    acc.push_str("{}");
                }
                else {
                    acc.push_str(&format!("_{{}}"));
                }
                acc
            });
            proc_macro_common::generate_quotes::token_stream(
                &value,
                &proc_macro_name_snake_case_stringified
            )
        };
        let upper_camel_case_stringified_format_parameters_calls_token_stream = element.iter().map(|element|{
            let element_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(*&element);
            quote::quote!{
                <naming_constants::#element_upper_camel_case_token_stream as naming_constants::Naming>::upper_camel_case_stringified()
            }
        });
        let snake_case_stringified_format_parameters_calls_token_stream = element.iter().map(|element|{
            let element_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(*&element);
            quote::quote!{
                <naming_constants::#element_upper_camel_case_token_stream as naming_constants::Naming>::snake_case_stringified()
            }
        });
        let value_parse_token_stream = quote::quote!{
            value.parse::<#proc_macro2_token_stream>()
            .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        quote::quote!{
            pub fn #phrase_upper_camel_case_stringified_token_stream() -> #std_string_string_token_stream {
                format!(
                    #upper_camel_case_stringified_format_parameters_places_token_stream,
                    #(#upper_camel_case_stringified_format_parameters_calls_token_stream),*
                )
            }
            pub fn #phrase_snake_case_stringified_token_stream() -> #std_string_string_token_stream {
                format!(
                    #snake_case_stringified_format_parameters_places_token_stream,
                    #(#snake_case_stringified_format_parameters_calls_token_stream),*
                )
            }
            pub fn #phrase_upper_camel_case_token_stream_token_stream() -> #proc_macro2_token_stream {
                let value = #phrase_upper_camel_case_stringified_token_stream();
                #value_parse_token_stream
            }
            pub fn #phrase_snake_case_token_stream_token_stream() -> #proc_macro2_token_stream {
                let value = #phrase_snake_case_stringified_token_stream();
                #value_parse_token_stream
            }
        }
    });
    let gen = quote::quote!{#(#implementations_token_stream)*};
    // println!("{gen}");
    gen.into()
}