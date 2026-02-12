use quote::quote;

const REGEX_VALUE: &str = "^[a-zA-Z]+$";

#[proc_macro]
pub fn gen_upper_camel_and_sc_str_and_ts(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let implementations_ts = serde_json::from_str::<Vec<Vec<String>>>(&input_ts.to_string())
        .expect("90e5793b-d813-44aa-a124-c738772030c2")
        .into_iter()
        .map(|el_020a8657| {
            {
                let regex =
                    regex::Regex::new(REGEX_VALUE).expect("20948d87-2c38-4896-96b6-239d9c9a0a38");
                for el_d68254e8 in &el_020a8657 {
                    assert!(
                        regex.is_match(el_d68254e8),
                        "faadba8a-ff38-40f9-af05-e4f95bba896a"
                    );
                }
            }
            let phrase_part_ucc_str =
                el_020a8657
                    .iter()
                    .fold(String::new(), |mut acc_3d60efa0, el_132cd6b1| {
                        acc_3d60efa0.push_str(&naming_common::AsRefStrToUccStr::case(el_132cd6b1));
                        acc_3d60efa0
                    });
            let phrase_part_sc_str = el_020a8657.iter().enumerate().fold(
                String::new(),
                |mut acc_7a8bd950, (index, el_b9981760)| {
                    let el_sc_str = naming_common::AsRefStrToScStr::case(el_b9981760);
                    if index == 0 {
                        acc_7a8bd950.push_str(&el_sc_str);
                    } else {
                        use std::fmt::Write as _;
                        assert!(
                            write!(acc_7a8bd950, "_{el_sc_str}").is_ok(),
                            "ef718915-7a99-45a6-b3c5-496262804976"
                        );
                    }
                    acc_7a8bd950
                },
            );
            let phrase_part_ucc_double_quotes_ts =
                gen_quotes::double_quotes_ts(&phrase_part_ucc_str);
            let phrase_part_sc_double_quotes_ts = gen_quotes::double_quotes_ts(&phrase_part_sc_str);
            let phrase_part_ucc_ts = phrase_part_ucc_str
                .parse::<proc_macro2::TokenStream>()
                .expect("7cf3ffc0-e9c9-4d91-b42f-beb77350d743");
            let phrase_part_sc_ts = phrase_part_sc_str
                .parse::<proc_macro2::TokenStream>()
                .expect("114a573a-3df3-4e4a-96c4-043eed3a358c");
            let phrase_part_ucc_ucc_ts = {
                let value = format!("{phrase_part_ucc_str}Ucc");
                value
                    .parse::<proc_macro2::TokenStream>()
                    .expect("4ab6a54c-892b-4f8f-a6b6-aead9c3671fe")
            };
            let phrase_part_sc_ucc_ts = {
                let value = format!("{phrase_part_ucc_str}Sc");
                value
                    .parse::<proc_macro2::TokenStream>()
                    .expect("0cc47b2e-03e2-48b8-8df3-7bbbe09de244")
            };
            let gen_struct_declaration = |struct_name_ts: &dyn quote::ToTokens| {
                quote! {
                    #[derive(Debug)]
                    pub struct #struct_name_ts;
                }
            };
            let ucc_struct_declaration_ts = gen_struct_declaration(&phrase_part_ucc_ucc_ts);
            let sc_struct_declaration_ts = gen_struct_declaration(&phrase_part_sc_ucc_ts);
            let gen_display_implementation_ts =
                |struct_name_ts: &dyn quote::ToTokens, write_content_ts: &dyn quote::ToTokens| {
                    quote! {
                        impl std::fmt::Display for #struct_name_ts {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                write!(f, #write_content_ts)
                            }
                        }
                    }
                };
            let ucc_display_implementation_ts = gen_display_implementation_ts(
                &phrase_part_ucc_ucc_ts,
                &phrase_part_ucc_double_quotes_ts,
            );
            let sc_display_implementation_ts = gen_display_implementation_ts(
                &phrase_part_sc_ucc_ts,
                &phrase_part_sc_double_quotes_ts,
            );
            let gen_to_tokens_implementation_ts =
                |struct_name_ts: &dyn quote::ToTokens, quote_content_ts: &dyn quote::ToTokens| {
                    quote! {
                        impl quote::ToTokens for #struct_name_ts {
                            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                                quote!{#quote_content_ts}.to_tokens(tokens);
                            }
                        }
                    }
                };
            let ucc_to_tokens_implementation_ts =
                gen_to_tokens_implementation_ts(&phrase_part_ucc_ucc_ts, &phrase_part_ucc_ts);
            let snake_to_tokens_implementation_ts =
                gen_to_tokens_implementation_ts(&phrase_part_sc_ucc_ts, &phrase_part_sc_ts);
            quote! {
                #ucc_struct_declaration_ts
                #ucc_display_implementation_ts
                #ucc_to_tokens_implementation_ts
                #sc_struct_declaration_ts
                #sc_display_implementation_ts
                #snake_to_tokens_implementation_ts
            }
        });
    let gend = quote! {#(#implementations_ts)*};
    // println!("{gend}");
    gend.into()
}

#[proc_macro]
pub fn gen_self_upper_camel_and_sc_str_and_ts(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let implementations_ts = serde_json::from_str::<Vec<Vec<String>>>(&input_ts.to_string()).expect("9d6a20af-a7b3-4fce-b11f-92b57a8fdb57").into_iter().map(|el_a5ccbaa7| {
        {
            let regex = regex::Regex::new(REGEX_VALUE).expect("cba1b5fb-6833-416b-96d9-b64b7a308008");
            for el_6d4f29dd in &el_a5ccbaa7 {
                assert!(regex.is_match(el_6d4f29dd), "4a12d90f-6428-4494-8305-40c149a1509a");
            }
        }
        let self_match_name = "self";
        {
            let mut is_self_exists_and_only_one = false;
            for el_3eac2cfb in &el_a5ccbaa7 {
                if el_3eac2cfb == self_match_name {
                    is_self_exists_and_only_one = true;
                    break;
                }
            }
            assert!(is_self_exists_and_only_one, "5680dd63-c98d-4e98-a67e-e13a3710a7c2");
        };
        let (elements_concat_value_ucc_double_quotes_ts, elements_concat_value_sc_double_quotes_ts, struct_ucc_ucc_ts, struct_sc_token_ucc_ts, trait_ucc_ucc_ts, trait_sc_token_ucc_ts) = {
            let ucc_ucc_str = "Ucc";
            let sc_ucc_str = "Sc";
            let elements_concat_ucc_str = el_a5ccbaa7.iter().fold(String::new(), |mut acc_34997d76, el_98881b7d| {
                acc_34997d76.push_str(&naming_common::AsRefStrToUccStr::case(el_98881b7d));
                acc_34997d76
            });
            let elements_concat_value_ucc_double_quotes_ts = gen_quotes::double_quotes_ts(&el_a5ccbaa7.iter().fold(String::new(), |mut acc_ae77cbd3, el_626f2b61| {
                if el_626f2b61 == "self" {
                    acc_ae77cbd3.push_str("{value}");
                } else {
                    acc_ae77cbd3.push_str(&naming_common::AsRefStrToUccStr::case(el_626f2b61));
                }
                acc_ae77cbd3
            }));
            let elements_concat_value_sc_double_quotes_ts = gen_quotes::double_quotes_ts(&{
                let mut value = el_a5ccbaa7.iter().fold(String::new(), |mut acc_cbcae5e1, el_73b0c851| {
                    use std::fmt::Write as _;
                    let symbol = '_';
                    if el_73b0c851 == "self" {
                        assert!(write!(acc_cbcae5e1, "{{value}}{symbol}").is_ok(), "6a02a2ff-1cb0-488d-85c0-32ea2d1291ac");
                    } else {
                        assert!(write!(acc_cbcae5e1, "{}{symbol}", naming_common::AsRefStrToScStr::case(el_73b0c851)).is_ok(), "d915980a-3aa3-4220-abfd-d5800927eef0");
                    }
                    acc_cbcae5e1
                });
                let _: Option<char> = value.pop();
                value
            });
            let struct_ucc_ucc_ts = {
                let value = format!("{elements_concat_ucc_str}{ucc_ucc_str}");
                value.parse::<proc_macro2::TokenStream>().expect("82f4ac08-08bd-4152-9633-7fb0ad2f59a9")
            };
            let struct_sc_token_ucc_ts = {
                let value = format!("{elements_concat_ucc_str}{sc_ucc_str}");
                value.parse::<proc_macro2::TokenStream>().expect("21044eba-c2c2-4c48-b84a-f7af8777436f")
            };
            let (trait_ucc_ucc_ts, trait_sc_token_ucc_ts) = {
                let trait_ucc_str = "Trait";
                let trait_ucc_ucc_ts = {
                    let value = format!("{elements_concat_ucc_str}{ucc_ucc_str}{trait_ucc_str}");
                    value.parse::<proc_macro2::TokenStream>().expect("1066857a-b509-4b94-937f-8a72af6482fe")
                };
                let trait_sc_token_ucc_ts = {
                    let value = format!("{elements_concat_ucc_str}{sc_ucc_str}{trait_ucc_str}");
                    value.parse::<proc_macro2::TokenStream>().expect("8db74cfd-cc35-4e38-83fa-3e0497504821")
                };
                (trait_ucc_ucc_ts, trait_sc_token_ucc_ts)
            };
            (
                elements_concat_value_ucc_double_quotes_ts,
                elements_concat_value_sc_double_quotes_ts,
                struct_ucc_ucc_ts,
                struct_sc_token_ucc_ts,
                trait_ucc_ucc_ts,
                trait_sc_token_ucc_ts,
            )
        };
        let gen_struct_ts = |elements_concat_value_case_double_quotes_ts: &dyn quote::ToTokens, is_ucc: bool, trait_ident_ts: &dyn quote::ToTokens| {
            let struct_ident_ts = if is_ucc {
                quote! {#struct_ucc_ucc_ts}
            } else {
                quote! {#struct_sc_token_ucc_ts}
            };
            let casing_ts = if is_ucc {
                quote! {naming_common::AsRefStrToUccStr::case}
            } else {
                quote! {naming_common::AsRefStrToScStr::case}
            };
            quote! {
                #[derive(Debug)]
                pub struct #struct_ident_ts(String);
                impl #struct_ident_ts {
                    fn wrap(value: &dyn std::fmt::Display) -> Self {
                        Self(Self::format(value))
                    }
                    fn format(value: &dyn std::fmt::Display) -> String {
                        format!(#elements_concat_value_case_double_quotes_ts)
                    }
                    pub fn from_display(value: &dyn std::fmt::Display) -> Self {
                        Self::wrap(&#casing_ts(&value.to_string()))
                    }
                    pub fn from_tokens(value: &dyn quote::ToTokens) -> Self {
                        Self::wrap(&#casing_ts(&{
                            let mut tokens = proc_macro2::TokenStream::new();
                            quote::ToTokens::to_tokens(&value, &mut tokens);
                            tokens
                        }.to_string()))
                    }
                    pub fn from_type_last_segment(value: &syn::Type) -> Self {
                        if let syn::Type::Path(type_path) = value {
                            let path_before_str = type_path.path.segments.iter().take(
                                type_path.path.segments.len().checked_sub(1).expect("e1f5a332-80ab-4a8a-8cbe-882e658185b7")
                            )
                            .fold(String::new(), |mut acc_f0a77378, el_2b05e58f| {
                                use std::fmt::Write as _;
                                assert!(write!(acc_f0a77378, "{}::", el_2b05e58f.ident).is_ok(), "67c90ce9-beea-4a81-99a2-874b8f04aa0a");
                                acc_f0a77378
                            });
                            let last = type_path.path.segments.iter().last().expect("19f6e1a6-2e06-4043-8732-03f3807d58c4");
                            Self(format!("{path_before_str}{}", Self::format(&#casing_ts(&last.ident.to_string()))))
                        }
                        else {
                            panic!("518933f8-c5b4-4452-908d-0fff899e7a25");
                        }
                    }
                }
                impl std::fmt::Display for #struct_ident_ts {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.0)
                    }
                }
                impl quote::ToTokens for #struct_ident_ts {
                    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                        let value_str = self.to_string();
                        let value_ts = value_str.parse::<proc_macro2::TokenStream>()
                            .expect("71c8d26b-18c4-4bbe-a07e-3114a15932d2");
                        value_ts.to_tokens(tokens);
                    }
                }
                pub trait #trait_ident_ts: std::fmt::Display + quote::ToTokens {}
                impl #trait_ident_ts for #struct_ident_ts {}
            }
        };
        let pub_struct_ucc_ts = gen_struct_ts(&elements_concat_value_ucc_double_quotes_ts, true, &trait_ucc_ucc_ts);
        let pub_struct_sc_ts = gen_struct_ts(&elements_concat_value_sc_double_quotes_ts, false, &trait_sc_token_ucc_ts);
        quote! {
            #pub_struct_ucc_ts
            #pub_struct_sc_ts
        }
    });
    let gend = quote! {#(#implementations_ts)*};
    // println!("{gend}");
    gend.into()
}

////////////////////
/*
only works if all enum variants without fields like this
#[derive(macros_assistants::AsRefStrToUccStr)]
enum Operation {
     One,
     Two,
     Three,
}
*/
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToUccStr)]
pub fn as_ref_str_enum_with_unit_fields_to_ucc_str(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input_ts).expect("a8f22481-4162-4372-97ef-91a012d80a8c");
    let ident = &syn_derive_input.ident;
    let syn::Data::Enum(data_enum) = syn_derive_input.data else {
        panic!("d26bf85e-20cf-4ee1-97bc-d61f59eb11bc")
    };
    let std_string_string_ts = token_patterns::StdStringString;
    let variants_matching_values_ts = data_enum
        .variants
        .iter()
        .map(|variant| match variant.fields {
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                let variant_ident_ucc_str = naming_common::ToTokensToUccStr::case(&variant_ident);
                let variant_ident_ucc_double_quotes_ts = gen_quotes::double_quotes_ts(&variant_ident_ucc_str);
                quote! {Self::#variant_ident => #std_string_string_ts::from(#variant_ident_ucc_double_quotes_ts)}
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("4955c50d-3db7-4881-a085-64b08a1ef413"),
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    let trait_path_ts = trait_path_ts();
    let gend = quote! {
        impl #trait_path_ts::AsRefStrToUccStr for #ident {
            fn case(&self) -> #std_string_string_ts {//todo maybe write duplicate Trait with &str instead of String
                match self {
                    #(#variants_matching_values_ts),*
                }
            }
        }
    };
    // println!("{gend}");
    gend.into()
}

/*
only works if all enum variants without fields like this
 #[derive(macros_assistants::AsRefStrToScStr)]
 enum Operation {
     One,
     Two,
     Three,
 }
*/
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToScStr)]
pub fn as_ref_str_enum_with_unit_fields_to_sc_str(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input_ts).expect("dea5cbcf-77f9-4861-9993-6ea53e01020b");
    let ident = &syn_derive_input.ident;
    let syn::Data::Enum(data_enum) = syn_derive_input.data else {
        panic!("ed6efe2e-ded2-4b61-807d-7b14ba0e2031");
    };
    let std_string_string_ts = token_patterns::StdStringString;
    let variants_matching_values_ts = data_enum
        .variants
        .iter()
        .map(|variant| match variant.fields {
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                let variant_ident_sc_str = naming_common::ToTokensToScStr::case(&variant_ident);
                let variant_ident_sc_double_quotes_ts = gen_quotes::double_quotes_ts(&variant_ident_sc_str);
                quote! {Self::#variant_ident => #std_string_string_ts::from(#variant_ident_sc_double_quotes_ts)}
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("b3ef2657-22f2-4df6-a58c-263a50e3c247"),
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    let trait_path_ts = trait_path_ts();
    let gend = quote! {
        impl #trait_path_ts::AsRefStrToScStr for #ident {
            fn case(&self) -> #std_string_string_ts {
                match self {
                    #(#variants_matching_values_ts),*
                }
            }
        }
    };
    // println!("{gend}");
    gend.into()
}
/*
only works if all enum variants without fields like this
 #[derive(macros_assistants::AsRefStrToUpperScStr)]
 enum Operation {
     One,
     Two,
     Three,
 }
*/
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToUpperScStr)]
pub fn as_ref_str_enum_with_unit_fields_to_upper_sc_str(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input_ts).expect("edabbc24-fdff-40d6-a83e-5b6a108689f6");
    let ident = &syn_derive_input.ident;
    let syn::Data::Enum(data_enum) = syn_derive_input.data else {
        panic!("b2263e7e-0e5e-48ef-8d91-32864281d4aa");
    };
    let std_string_string = token_patterns::StdStringString;
    let variants_matching_values_ts = data_enum
        .variants
        .iter()
        .map(|variant| match variant.fields {
            syn::Fields::Unit => {
                let variant_ident = &variant.ident;
                let variant_ident_sc_str = naming_common::ToTokensToUpperScStr::case(&variant_ident);
                let variant_ident_sc_double_quotes_ts = gen_quotes::double_quotes_ts(&variant_ident_sc_str);
                quote! {Self::#variant_ident => #std_string_string::from(#variant_ident_sc_double_quotes_ts)}
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("b6fedcff-1a88-455f-bd93-219ec45a1fce"),
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    let trait_path_ts = trait_path_ts();
    let gend = quote! {
        impl #trait_path_ts::ToUpperScStr for #ident {
            fn case(&self) -> #std_string_string {
                match self {
                    #(#variants_matching_values_ts),*
                }
            }
        }
    };
    // println!("{gend}");
    gend.into()
}

fn trait_path_ts() -> proc_macro2::TokenStream {
    quote! {naming}
}
