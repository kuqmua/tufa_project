const REGEX_VALUE: &str = "^[a-zA-Z]+$";

#[proc_macro]
pub fn generate_upper_camel_and_snake_case_stringified_and_token_stream(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let implementations_token_stream = serde_json::from_str::<Vec<Vec<String>>>(&input_token_stream.to_string()).expect("90e5793b-d813-44aa-a124-c738772030c2").into_iter().map(|element| {
        {
            let regex = regex::Regex::new(REGEX_VALUE).expect("20948d87-2c38-4896-96b6-239d9c9a0a38");
            for inner_element in &element {
                assert!(regex.is_match(inner_element), "faadba8a-ff38-40f9-af05-e4f95bba896a");
            }
        }
        let phrase_part_upper_camel_case_stringified = element.iter().fold(String::new(), |mut acc_3d60efa0, current_element| {
            acc_3d60efa0.push_str(&naming_common::AsRefStrToUpperCamelCaseStringified::case(current_element));
            acc_3d60efa0
        });
        let phrase_part_snake_case_stringified = element.iter().enumerate().fold(String::new(), |mut acc_7a8bd950, (index, current_element)| {
            let element_snake_case_stringified = naming_common::AsRefStrToSnakeCaseStringified::case(current_element);
            if index == 0 {
                acc_7a8bd950.push_str(&element_snake_case_stringified);
            } else {
                use std::fmt::Write as _;
                assert!(write!(acc_7a8bd950, "_{element_snake_case_stringified}").is_ok(), "ef718915-7a99-45a6-b3c5-496262804976");
            }
            acc_7a8bd950
        });
        let phrase_part_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&phrase_part_upper_camel_case_stringified);
        let phrase_part_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&phrase_part_snake_case_stringified);
        let phrase_part_upper_camel_case_token_stream = phrase_part_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>().expect("7cf3ffc0-e9c9-4d91-b42f-beb77350d743");
        let phrase_part_snake_case_token_stream = phrase_part_snake_case_stringified.parse::<proc_macro2::TokenStream>().expect("114a573a-3df3-4e4a-96c4-043eed3a358c");
        let phrase_part_upper_camel_case_upper_camel_case_token_stream = {
            let value = format!("{phrase_part_upper_camel_case_stringified}UpperCamelCase");
            value.parse::<proc_macro2::TokenStream>().expect("4ab6a54c-892b-4f8f-a6b6-aead9c3671fe")
        };
        let phrase_part_snake_case_upper_camel_case_token_stream = {
            let value = format!("{phrase_part_upper_camel_case_stringified}SnakeCase");
            value.parse::<proc_macro2::TokenStream>().expect("0cc47b2e-03e2-48b8-8df3-7bbbe09de244")
        };
        let generate_struct_declaration = |struct_name_token_stream: &dyn quote::ToTokens| quote::quote! {pub struct #struct_name_token_stream;};
        let upper_camel_case_struct_declaration_token_stream = generate_struct_declaration(&phrase_part_upper_camel_case_upper_camel_case_token_stream);
        let snake_case_struct_declaration_token_stream = generate_struct_declaration(&phrase_part_snake_case_upper_camel_case_token_stream);
        let generate_display_implementation_token_stream = |struct_name_token_stream: &dyn quote::ToTokens, write_content_token_stream: &dyn quote::ToTokens| {
            quote::quote! {
                impl std::fmt::Display for #struct_name_token_stream {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, #write_content_token_stream)
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
pub fn generate_self_upper_camel_and_snake_case_stringified_and_token_stream(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let implementations_token_stream = serde_json::from_str::<Vec<Vec<String>>>(&input_token_stream.to_string()).expect("9d6a20af-a7b3-4fce-b11f-92b57a8fdb57").into_iter().map(|element| {
        {
            let regex = regex::Regex::new(REGEX_VALUE).expect("cba1b5fb-6833-416b-96d9-b64b7a308008");
            for inner_element in &element {
                assert!(regex.is_match(inner_element), "4a12d90f-6428-4494-8305-40c149a1509a");
            }
        }
        let self_match_name = "self";
        {
            let mut is_self_exists_and_only_one = false;
            for inner_element in &element {
                if inner_element == self_match_name {
                    is_self_exists_and_only_one = true;
                    break;
                }
            }
            assert!(is_self_exists_and_only_one, "5680dd63-c98d-4e98-a67e-e13a3710a7c2");
        };
        let (elements_concat_value_upper_camel_case_double_quotes_token_stream, elements_concat_value_snake_case_double_quotes_token_stream, struct_upper_camel_case_upper_camel_case_token_stream, struct_snake_case_token_upper_camel_case_stream, trait_upper_camel_case_upper_camel_case_token_stream, trait_snake_case_token_upper_camel_case_stream) = {
            let upper_camel_case_upper_camel_case_stringified = "UpperCamelCase";
            let snake_case_upper_camel_case_stringified = "SnakeCase";
            let elements_concat_upper_camel_case_stringified = element.iter().fold(String::new(), |mut acc_34997d76, current_element| {
                acc_34997d76.push_str(&naming_common::AsRefStrToUpperCamelCaseStringified::case(current_element));
                acc_34997d76
            });
            let elements_concat_value_upper_camel_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.iter().fold(String::new(), |mut acc_ae77cbd3, current_element| {
                if current_element == "self" {
                    acc_ae77cbd3.push_str("{value}");
                } else {
                    acc_ae77cbd3.push_str(&naming_common::AsRefStrToUpperCamelCaseStringified::case(current_element));
                }
                acc_ae77cbd3
            }));
            let elements_concat_value_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&{
                let mut value = element.iter().fold(String::new(), |mut acc_cbcae5e1, current_element| {
                    use std::fmt::Write as _;
                    let symbol = '_';
                    if current_element == "self" {
                        assert!(write!(acc_cbcae5e1, "{{value}}{symbol}").is_ok(), "6a02a2ff-1cb0-488d-85c0-32ea2d1291ac");
                    } else {
                        assert!(write!(acc_cbcae5e1, "{}{symbol}", naming_common::AsRefStrToSnakeCaseStringified::case(current_element)).is_ok(), "d915980a-3aa3-4220-abfd-d5800927eef0");
                    }
                    acc_cbcae5e1
                });
                let _: Option<char> = value.pop();
                value
            });
            let struct_upper_camel_case_upper_camel_case_token_stream = {
                let value = format!("{elements_concat_upper_camel_case_stringified}{upper_camel_case_upper_camel_case_stringified}");
                value.parse::<proc_macro2::TokenStream>().expect("82f4ac08-08bd-4152-9633-7fb0ad2f59a9")
            };
            let struct_snake_case_token_upper_camel_case_stream = {
                let value = format!("{elements_concat_upper_camel_case_stringified}{snake_case_upper_camel_case_stringified}");
                value.parse::<proc_macro2::TokenStream>().expect("21044eba-c2c2-4c48-b84a-f7af8777436f")
            };
            let (trait_upper_camel_case_upper_camel_case_token_stream, trait_snake_case_token_upper_camel_case_stream) = {
                let trait_upper_camel_case_stringified = "Trait";
                let trait_upper_camel_case_upper_camel_case_token_stream = {
                    let value = format!("{elements_concat_upper_camel_case_stringified}{upper_camel_case_upper_camel_case_stringified}{trait_upper_camel_case_stringified}");
                    value.parse::<proc_macro2::TokenStream>().expect("1066857a-b509-4b94-937f-8a72af6482fe")
                };
                let trait_snake_case_token_upper_camel_case_stream = {
                    let value = format!("{elements_concat_upper_camel_case_stringified}{snake_case_upper_camel_case_stringified}{trait_upper_camel_case_stringified}");
                    value.parse::<proc_macro2::TokenStream>().expect("8db74cfd-cc35-4e38-83fa-3e0497504821")
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
        let generate_struct_token_stream = |elements_concat_value_case_double_quotes_token_stream: &dyn quote::ToTokens, is_upper_camel_case: bool, trait_ident_token_stream: &dyn quote::ToTokens| {
            let struct_ident_token_stream = if is_upper_camel_case {
                quote::quote! {#struct_upper_camel_case_upper_camel_case_token_stream}
            } else {
                quote::quote! {#struct_snake_case_token_upper_camel_case_stream}
            };
            let casing_token_stream = if is_upper_camel_case {
                quote::quote! {naming_common::AsRefStrToUpperCamelCaseStringified::case}
            } else {
                quote::quote! {naming_common::AsRefStrToSnakeCaseStringified::case}
            };
            quote::quote! {
                #[derive(Debug)]
                pub struct #struct_ident_token_stream(String);
                impl #struct_ident_token_stream {
                    fn wrap(value: &dyn std::fmt::Display) -> Self {
                        Self(Self::format(value))
                    }
                    fn format(value: &dyn std::fmt::Display) -> String {
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
                                let path_before_stringified = type_path.path.segments.iter().take(type_path.path.segments.len() - 1).fold(String::new(), |mut acc_f0a77378, elem| {
                                    use std::fmt::Write as _;
                                    if write!(acc_f0a77378, "{}::", elem.ident).is_err() {
                                        panic!("67c90ce9-beea-4a81-99a2-874b8f04aa0a");
                                    }
                                    acc_f0a77378
                                });
                                let last = type_path.path.segments.iter().last().expect("19f6e1a6-2e06-4043-8732-03f3807d58c4");
                                Self(format!("{path_before_stringified}{}", Self::format(&#casing_token_stream(&last.ident.to_string()))))
                            },
                            _ => panic!("518933f8-c5b4-4452-908d-0fff899e7a25")
                        }
                    }
                }
                impl std::fmt::Display for #struct_ident_token_stream {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.0)
                    }
                }
                impl quote::ToTokens for #struct_ident_token_stream {
                    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                        let value_stringified = self.to_string();
                        let value_token_stream = value_stringified.parse::<proc_macro2::TokenStream>()
                            .expect("71c8d26b-18c4-4bbe-a07e-3114a15932d2");
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
pub fn as_ref_str_enum_with_unit_fields_to_upper_camel_case_stringified(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input_token_stream).expect("a8f22481-4162-4372-97ef-91a012d80a8c");
    let ident = &syn_derive_input.ident;
    let syn::Data::Enum(data_enum) = &syn_derive_input.data else {
        panic!("d26bf85e-20cf-4ee1-97bc-d61f59eb11bc")
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
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("4955c50d-3db7-4881-a085-64b08a1ef413"),
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    let trait_path_token_stream = trait_path_token_stream();
    let generated = quote::quote! {
        impl #trait_path_token_stream::AsRefStrToUpperCamelCaseStringified for #ident {
            fn case(&self) -> #std_string_string_token_stream {//todo maybe write duplicate Trait with &str instead of String
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
pub fn as_ref_str_enum_with_unit_fields_to_snake_case_stringified(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input_token_stream).expect("dea5cbcf-77f9-4861-9993-6ea53e01020b");
    let ident = &syn_derive_input.ident;
    let syn::Data::Enum(data_enum) = &syn_derive_input.data else {
        panic!("ed6efe2e-ded2-4b61-807d-7b14ba0e2031");
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
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("b3ef2657-22f2-4df6-a58c-263a50e3c247"),
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
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
 #[derive(macros_assistants::AsRefStrToUpperSnakeCaseStringified)]
 enum Operation {
     One,
     Two,
     Three,
 }
*/
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToUpperSnakeCaseStringified)]
pub fn as_ref_str_enum_with_unit_fields_to_upper_snake_case_stringified(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input_token_stream).expect("edabbc24-fdff-40d6-a83e-5b6a108689f6");
    let ident = &syn_derive_input.ident;
    let syn::Data::Enum(data_enum) = &syn_derive_input.data else {
        panic!("b2263e7e-0e5e-48ef-8d91-32864281d4aa");
    };
    let std_string_string = token_patterns::StdStringString;
    let variants_matching_values_token_stream = data_enum
        .variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                let variant_ident_snake_case_stringified = naming_common::ToTokensToUpperSnakeCaseStringified::case(&variant_ident);
                let variant_ident_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&variant_ident_snake_case_stringified);
                quote::quote! {Self::#variant_ident => #std_string_string::from(#variant_ident_snake_case_double_quotes_token_stream)}
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("b6fedcff-1a88-455f-bd93-219ec45a1fce"),
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    let trait_path_token_stream = trait_path_token_stream();
    let generated = quote::quote! {
        impl #trait_path_token_stream::ToUpperSnakeCaseStringified for #ident {
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
