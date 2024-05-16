#[proc_macro]
pub fn generate_upper_camel_and_snake_case_stringified_and_token_stream_from_naming_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_snake_case_stringified = "generate_upper_camel_and_snake_case_stringified_and_token_stream_from_naming_constants";
    let implementations_token_stream = serde_json::from_str::<std::vec::Vec<std::vec::Vec<std::string::String>>>(&input.to_string())
    .expect("failed to convert tokens input into valid json string[][] pattern")
    .into_iter()
    .map(|element| {
        let regex_value = r"^[a-zA-Z]+$";
        let regex = regex::Regex::new(regex_value).unwrap();
        for element in &element {
            if !regex.is_match(&element) {
                panic!("{proc_macro_name_snake_case_stringified} invalid element {element}, regex: {regex_value}");
            }
        }
        let prefix_upper_camel_case = format!(
            "_{}_{}_{}",
            <naming_constants::Upper as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Camel as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Case as naming_constants::Naming>::snake_case_stringified(),
        );
        let prefix_snake_case = format!(
            "_{}_{}",
            <naming_constants::Snake as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Case as naming_constants::Naming>::snake_case_stringified(),
        );
        let prefix_stringified = format!(
            "_{}",
            <naming_constants::Stringified as naming_constants::Naming>::snake_case_stringified(),
        );
        let prefix_token_stream = format!(
            "_{}_{}",
            <naming_constants::Token as naming_constants::Naming>::snake_case_stringified(),
            <naming_constants::Stream as naming_constants::Naming>::snake_case_stringified(),
        );
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
        let generate_pub_fn_token_stream = |
            name_token_stream: &proc_macro2::TokenStream,
            return_type_token_stream: &proc_macro2::TokenStream,
            content_token_stream: &proc_macro2::TokenStream,
        |{
            quote::quote!{
                pub fn #name_token_stream() -> #return_type_token_stream {
                    #content_token_stream
                }
            }
        };
        let pub_fn_upper_camel_case_stringified_token_stream = generate_pub_fn_token_stream(
            &phrase_upper_camel_case_stringified_token_stream,
            &std_string_string_token_stream,
            &quote::quote!{
                format!(
                    #upper_camel_case_stringified_format_parameters_places_token_stream,
                    #(#upper_camel_case_stringified_format_parameters_calls_token_stream),*
                )
            }
        );
        let pub_fn_snake_case_stringified_token_stream = generate_pub_fn_token_stream(
            &phrase_snake_case_stringified_token_stream,
            &std_string_string_token_stream,
            &quote::quote!{
                format!(
                    #snake_case_stringified_format_parameters_places_token_stream,
                    #(#snake_case_stringified_format_parameters_calls_token_stream),*
                )
            }
        );
        let pub_fn_upper_camel_case_token_stream_token_stream = generate_pub_fn_token_stream(
            &phrase_upper_camel_case_token_stream_token_stream,
            &proc_macro2_token_stream,
            &quote::quote!{
                let value = #phrase_upper_camel_case_stringified_token_stream();
                #value_parse_token_stream
            }
        );
        let pub_fn_snake_case_token_stream_token_stream = generate_pub_fn_token_stream(
            &phrase_snake_case_token_stream_token_stream,
            &proc_macro2_token_stream,
            &quote::quote!{
                let value = #phrase_snake_case_stringified_token_stream();
                #value_parse_token_stream
            }
        );
        quote::quote!{
            #pub_fn_upper_camel_case_stringified_token_stream
            #pub_fn_snake_case_stringified_token_stream
            #pub_fn_upper_camel_case_token_stream_token_stream
            #pub_fn_snake_case_token_stream_token_stream
        }
    });
    let gen = quote::quote!{#(#implementations_token_stream)*};
    // println!("{gen}");
    gen.into()
}

///////
#[proc_macro]
pub fn generate_self_upper_camel_and_snake_case_stringified_and_token_stream_from_naming_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_snake_case_stringified = "generate_self_upper_camel_and_snake_case_stringified_and_token_stream_from_naming_constants";
    let implementations_token_stream = serde_json::from_str::<std::vec::Vec<std::vec::Vec<std::string::String>>>(&input.to_string())
    .expect("failed to convert tokens input into valid json string[][] pattern")
    .into_iter()
    .map(|element| {
        let regex_value = r"^[a-zA-Z]+$";
        let regex = regex::Regex::new(regex_value).unwrap();
        for element in &element {
            if !regex.is_match(&element) {
                panic!("{proc_macro_name_snake_case_stringified} invalid element {element}, regex: {regex_value}");
            }
        }
        let self_match_name = "Self";
        {
            let mut is_self_exists_and_only_one = false;
            for element in element {
                if element == self_match_name {
                    if is_self_exists_and_only_one {
                        panic!("{proc_macro_name_snake_case_stringified} {self_match_name} written not written only once");
                    }
                    else {
                        is_self_exists_and_only_one = true;
                    }
                }
            }
            if !is_self_exists_and_only_one {
                panic!("{proc_macro_name_snake_case_stringified} cannot find {self_match_name}");
            }
        }
        let stringified_upper_camel_case_stringified = "Stringified";
        let token_stream_upper_camel_case_stringified = "TokenStream";

        // let f = "UpperCamelCaseStringified";

        // let prefix_upper_camel_case = "_upper_camel_case";
        // let prefix_snake_case = "_snake_case";
        // let prefix_stringified = "_stringified";
        // let prefix_token_stream = "_token_stream";
        // let prefix_upper_camel_case_stringified = format!("{prefix_upper_camel_case}{prefix_stringified}");
        // let prefix_snake_case_stringified = format!("{prefix_snake_case}{prefix_stringified}");
        // let prefix_upper_camel_case_token_stream = format!("{prefix_upper_camel_case}{prefix_token_stream}");
        // let prefix_snake_case_token_stream = format!("{prefix_snake_case}{prefix_token_stream}");



        // let phrase_part_stringified = element.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, element)| {
        //     let element_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(*&element);
        //     if index == 0 {
        //         acc.push_str(&element_snake_case_stringified);
        //     }
        //     else {
        //         acc.push_str(&format!("_{element_snake_case_stringified}"));
        //     }
        //     acc
        // });
        // let phrase_upper_camel_case_stringified_token_stream = {
        //     let value = format!("{phrase_part_stringified}{prefix_upper_camel_case_stringified}");
        //     value.parse::<proc_macro2::TokenStream>()
        //     .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        // };
        // let phrase_snake_case_stringified_token_stream = {
        //     let value = format!("{phrase_part_stringified}{prefix_snake_case_stringified}");
        //     value.parse::<proc_macro2::TokenStream>()
        //     .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        // };
        // let phrase_upper_camel_case_token_stream_token_stream = {
        //     let value = format!("{phrase_part_stringified}{prefix_upper_camel_case_token_stream}");
        //     value.parse::<proc_macro2::TokenStream>()
        //     .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        // };
        // let phrase_snake_case_token_stream_token_stream = {
        //     let value = format!("{phrase_part_stringified}{prefix_snake_case_token_stream}");
        //     value.parse::<proc_macro2::TokenStream>()
        //     .unwrap_or_else(|_| panic!("{proc_macro_name_snake_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        // };
        // let std_string_string_token_stream = proc_macro_common::std_string_string_token_stream();
        // let proc_macro2_token_stream = quote::quote!{proc_macro2::TokenStream};//todo maybe reuse
        // let upper_camel_case_stringified_format_parameters_places_token_stream = {
        //     let value = element.iter().fold(std::string::String::from(""), |mut acc, _| {
        //         acc.push_str("{}");
        //         acc
        //     });
        //     proc_macro_common::generate_quotes::token_stream(
        //         &value,
        //         &proc_macro_name_snake_case_stringified
        //     )
        // };
        // let snake_case_stringified_format_parameters_places_token_stream = {
        //     let value = element.iter().enumerate().fold(std::string::String::from(""), |mut acc, (index, _)| {
        //         if index == 0 {
        //             acc.push_str("{}");
        //         }
        //         else {
        //             acc.push_str(&format!("_{{}}"));
        //         }
        //         acc
        //     });
        //     proc_macro_common::generate_quotes::token_stream(
        //         &value,
        //         &proc_macro_name_snake_case_stringified
        //     )
        // };
        // let upper_camel_case_stringified_format_parameters_calls_token_stream = element.iter().map(|element|{
        //     let element_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(*&element);
        //     quote::quote!{
        //         <naming_constants::#element_upper_camel_case_token_stream as naming_constants::Naming>::upper_camel_case_stringified()
        //     }
        // });
        // let snake_case_stringified_format_parameters_calls_token_stream = element.iter().map(|element|{
        //     let element_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(*&element);
        //     quote::quote!{
        //         <naming_constants::#element_upper_camel_case_token_stream as naming_constants::Naming>::snake_case_stringified()
        //     }
        // });
        // let value_parse_token_stream = quote::quote!{
        //     value.parse::<#proc_macro2_token_stream>()
        //     .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        // };
        // let generate_pub_fn_token_stream = |
        //     name_token_stream: &proc_macro2::TokenStream,
        //     return_type_token_stream: &proc_macro2::TokenStream,
        //     content_token_stream: &proc_macro2::TokenStream,
        // |{
        //     quote::quote!{
        //         pub fn #name_token_stream() -> #return_type_token_stream {
        //             #content_token_stream
        //         }
        //     }
        // };
        // let pub_fn_upper_camel_case_stringified_token_stream = generate_pub_fn_token_stream(
        //     &phrase_upper_camel_case_stringified_token_stream,
        //     &std_string_string_token_stream,
        //     &quote::quote!{
        //         format!(
        //             #upper_camel_case_stringified_format_parameters_places_token_stream,
        //             #(#upper_camel_case_stringified_format_parameters_calls_token_stream),*
        //         )
        //     }
        // );
        // let pub_fn_snake_case_stringified_token_stream = generate_pub_fn_token_stream(
        //     &phrase_snake_case_stringified_token_stream,
        //     &std_string_string_token_stream,
        //     &quote::quote!{
        //         format!(
        //             #snake_case_stringified_format_parameters_places_token_stream,
        //             #(#snake_case_stringified_format_parameters_calls_token_stream),*
        //         )
        //     }
        // );
        // let pub_fn_upper_camel_case_token_stream_token_stream = generate_pub_fn_token_stream(
        //     &phrase_upper_camel_case_token_stream_token_stream,
        //     &proc_macro2_token_stream,
        //     &quote::quote!{
        //         let value = #phrase_upper_camel_case_stringified_token_stream();
        //         #value_parse_token_stream
        //     }
        // );
        // let pub_fn_snake_case_token_stream_token_stream = generate_pub_fn_token_stream(
        //     &phrase_snake_case_token_stream_token_stream,
        //     &proc_macro2_token_stream,
        //     &quote::quote!{
        //         let value = #phrase_snake_case_stringified_token_stream();
        //         #value_parse_token_stream
        //     }
        // );
        quote::quote!{
            // #pub_fn_upper_camel_case_stringified_token_stream
            // #pub_fn_snake_case_stringified_token_stream
            // #pub_fn_upper_camel_case_token_stream_token_stream
            // #pub_fn_snake_case_token_stream_token_stream
            //
            // pub trait SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified {
            //     fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(
            //         &self,
            //     ) -> std::string::String;
            // }

            // impl<T> SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified for T
            // where
            //     T: proc_macro_common::naming_conventions::ToUpperCamelCaseStringified,
            // {
            //     fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(
            //         &self,
            //     ) -> std::string::String {
            //         format!(
            //             "{}{}{}{}{}{}{}",
            //             self.to_upper_camel_case_stringified(),
            //             <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            //             <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
            //             <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
            //             self.to_upper_camel_case_stringified(),
            //             <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            //             with_serialize_deserialize_upper_camel_case_stringified()
            //         )
            //     }
            // }
            
            // pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseStringified {
            //     fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_stringified(
            //         &self,
            //     ) -> std::string::String;
            // }

            // impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseStringified for T
            // where
            //     T: proc_macro_common::naming_conventions::ToSnakeCaseStringified,
            // {
            //     fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_stringified(
            //         &self,
            //     ) -> std::string::String {
            //         format!(
            //             "{}_{}_{}_{}_{}_{}_{}",
            //             self.to_snake_case_stringified(),
            //             <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            //             with_serialize_deserialize_snake_case_stringified(),
            //             <naming_constants::Try as naming_constants::Naming>::snake_case_stringified(),
            //             <naming_constants::From as naming_constants::Naming>::snake_case_stringified(),
            //             self.to_snake_case_stringified(),
            //             <naming_constants::Payload as naming_constants::Naming>::upper_camel_case_stringified(),
            //         )
            //     }
            // }
            
            // pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseTokenStream {
            //     fn self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_token_stream(
            //         &self,
            //     ) -> proc_macro2::TokenStream;
            // }

            // impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseTokenStream for T
            // where
            //     T: SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseStringified,
            // {
            //     fn self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_token_stream(
            //         &self,
            //     ) -> proc_macro2::TokenStream {
            //         let value = self.self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_stringified();
            //         value.parse::<proc_macro2::TokenStream>()
            //         .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            //     }
            // }
            
            // pub trait SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseTokenStream {
            //     fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_token_stream(
            //         &self,
            //     ) -> proc_macro2::TokenStream;
            // }

            // impl<T> SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseTokenStream for T
            // where
            //     T: SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseStringified,
            // {
            //     fn self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_token_stream(
            //         &self,
            //     ) -> proc_macro2::TokenStream {
            //         let value = self.self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_stringified();
            //         value.parse::<proc_macro2::TokenStream>()
            //         .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            //     }
            // }
        }
    });
    let gen = quote::quote!{#(#implementations_token_stream)*};
    // println!("{gen}");
    gen.into()
}