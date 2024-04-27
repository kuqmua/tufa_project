#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

#[proc_macro_derive(InitFromEnv)]
pub fn init_from_env(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let ast: syn::DeriveInput =
        syn::parse(input).expect("InitFromEnvWithPanicIfFailed syn::parse(input) failed");
    let datastruct = match ast.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | 
        syn::Data::Union(_) => panic!("InitFromEnvWithPanicIfFailed only works on Struct"),
    };
    let ident = &ast.ident;
    let error_ident = syn::Ident::new(&format!("{ident}Error"), ident.span());
    let error_enum_ident = syn::Ident::new(&format!("{ident}ErrorEnum"), ident.span());
    let error_std_env_var_ident =
        syn::Ident::new(&format!("{ident}StdEnvVar"), ident.span());
    let error_std_env_var_enum_ident =
        syn::Ident::new(&format!("{ident}ErrorStdEnvEnum"), ident.span());
    let error_parse_ident = syn::Ident::new(&format!("{ident}Parse"), ident.span());
    let error_parse_enum_ident =
        syn::Ident::new(&format!("{ident}ErrorParseEnum"), ident.span());
    let capacity = datastruct.fields.len();
    let mut generated_init_struct_fields: Vec<proc_macro2::TokenStream> =
        Vec::with_capacity(capacity);
    let mut generated_functions: Vec<proc_macro2::TokenStream> =
        Vec::with_capacity(capacity);
    let mut generated_enum_error_std_env_var_variants: Vec<proc_macro2::TokenStream> =
        Vec::with_capacity(capacity);
    let mut generated_enum_error_parse_variants: Vec<proc_macro2::TokenStream> =
        Vec::with_capacity(capacity);
    datastruct.fields.into_iter().for_each(|field| {
        let (
            enum_variant_ident_value,
            env_var_name,
            enum_variant_ident
        ) = field.ident.as_ref().map_or_else(|| panic!(
            "InitFromEnvWithPanicIfFailed {}",
            naming_constants::FIELD_IDENT_IS_NONE
        ), |field_ident| {
            generated_init_struct_fields.push(quote::quote! {
                #field_ident: #field_ident
            });
            (
                field_ident,
                syn::Ident::new(
                    &convert_case::Casing::to_case(&format!("{field_ident}"), convert_case::Case::Snake)
                        .to_uppercase(),
                    ident.span(),
                ),
                syn::Ident::new(
                    &convert_case::Casing::to_case(&format!("{field_ident}"),convert_case::Case::UpperCamel),
                    ident.span(),
                )
            )
        });
        let (
            enum_variant_type,
            enum_variant_type_as_string,
        ) = match &field.ty {
            //todo: add different types support
            syn::Type::Path(type_path) => {
                let enum_variant_type = &type_path.path;
                let string_handle = if type_path.path.segments.len() == 1 {
                    format!("{}", type_path.path.segments.first().unwrap_or_else(||panic!("failed to get first type_path.path.segments element")).ident)
                } else {
                    let mut value = std::string::String::from("");
                    for seg in &type_path.path.segments {
                        value.push_str(&format!("{}:", seg.ident));
                    }
                    if !value.is_empty() {
                        let _: std::option::Option<std::primitive::char> = value.pop();
                    }
                    value
                };
                (
                    enum_variant_type,
                    syn::LitStr::new(&string_handle, ident.span())
                )
            }
            syn::Type::Array(_) | 
            syn::Type::BareFn(_) | 
            syn::Type::Group(_) | 
            syn::Type::ImplTrait(_) | 
            syn::Type::Infer(_) | 
            syn::Type::Macro(_) | 
            syn::Type::Never(_) | 
            syn::Type::Paren(_) | 
            syn::Type::Ptr(_) | 
            syn::Type::Reference(_) | 
            syn::Type::Slice(_) | 
            syn::Type::TraitObject(_) | 
            syn::Type::Tuple(_) | 
            syn::Type::Verbatim(_) => panic!("InitFromEnvWithPanicIfFailed field.ty supports only syn::Type::Path!"),
            _ => panic!("InitFromEnvWithPanicIfFailed field.ty supports only syn::Type::Path!(exhaustive)"),
        };
        let env_var_name_as_screaming_snake_case_string =
            syn::LitStr::new(&format!("{env_var_name}"), ident.span());
        generated_functions.push(quote::quote! {
            let #enum_variant_ident_value: #enum_variant_type;
            match std::env::var(#env_var_name_as_screaming_snake_case_string) {
                Err(error) => {
                    println!("error {error:#?}");
                    return Err(#error_ident {
                        source: #error_enum_ident::#error_std_env_var_ident(#error_std_env_var_enum_ident::#enum_variant_ident {
                            source: error,
                            env_var_name: #env_var_name_as_screaming_snake_case_string,
                        }),
                        was_dotenv_enable,
                    });
                },
                Ok(string_handle) => {
                    match string_handle.parse::<#enum_variant_type>() {
                        Err(_) => {
                            return Err(#error_ident {
                                source: #error_enum_ident::#error_parse_ident(#error_parse_enum_ident::#enum_variant_ident{
                                    env_var_name: #env_var_name_as_screaming_snake_case_string,
                                    expected_env_var_type: #enum_variant_type_as_string,
                                }),
                                was_dotenv_enable,
                            });
                        },
                        Ok(handle) => {
                            #enum_variant_ident_value = handle;
                        },
                    }
                },
            }
        });
        generated_enum_error_std_env_var_variants.push(quote::quote! {
            #enum_variant_ident {
                source: std::env::VarError,
                env_var_name: &'static str,
            },
        });
        generated_enum_error_parse_variants.push(quote::quote! {
            #enum_variant_ident {
                env_var_name: &'static str,
                expected_env_var_type: &'static str,
            },
        });
    });
    let gen = quote::quote! {
        #[derive(Debug)]
        pub struct #error_ident {
            pub source: #error_enum_ident,
            pub was_dotenv_enable: bool,
        }
        #[derive(Debug)]
        pub enum #error_enum_ident {
            #error_std_env_var_ident(#error_std_env_var_enum_ident),
            #error_parse_ident(#error_parse_enum_ident),
        }
        #[derive(Debug)]
        pub enum #error_std_env_var_enum_ident {
            #(#generated_enum_error_std_env_var_variants)*
        }
        #[derive(Debug)]
        pub enum #error_parse_enum_ident {
            #(#generated_enum_error_parse_variants)*
        }
        impl #ident {
            #[must_use]
            pub fn new() -> Result<Self, #error_ident> {
                let was_dotenv_enable = dotenv::dotenv().is_ok();
                #(#generated_functions)*
                Ok(
                    Self {
                        #(#generated_init_struct_fields,)*
                    }
                )
            }
        }
    };
    // println!("{gen}");
    gen.into()
}

//////////////////
#[proc_macro_derive(TryFromEnv)]
pub fn try_from_env(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "TryFromEnv";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case_stringified} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    // let ident_error = syn::Ident::new(&format!("{ident}Error"), ident.span());
    let data_struct = match ast.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | 
        syn::Data::Union(_) => panic!("{proc_macro_name_upper_camel_case_stringified} only works on Struct"),
    };
    let fields_named = match data_struct.fields {
        syn::Fields::Named(value) => value.named,
        syn::Fields::Unnamed(_) | 
        syn::Fields::Unit => panic!("{proc_macro_name_upper_camel_case_stringified} only works with syn::Fields::Named"),
    };
    let error_named_token_stream = {
        let variants_token_stream = fields_named.iter().map(|element|{
            let element_ident = &element.ident;
            let element_ident_upper_camel_case_token_stream = {
                let value = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(
                    &element_ident.as_ref().expect("ident is None").to_string()
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let error_upper_camel_case_token_stream = {
                let element_type_stringified = {
                    let value_type = &element.ty;
                    let value_token_stream = quote::quote!{#value_type};
                    value_token_stream.to_string()
                };
                let value = format!("TryFromStdEnvVarOk{element_type_stringified}ErrorNamed");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #element_ident_upper_camel_case_token_stream {
                    #[eo_error_occurence]
                    #element_ident: #error_upper_camel_case_token_stream,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                }
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote!{
            #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum ConfigUncheckedTryFromEnvErrorNamed {
                Dotenv {
                    #[eo_display]
                    dotenv: dotenv::Error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                StdEnvVarError {
                    #[eo_display]
                    std_env_var_error: std::env::VarError,
                    #[eo_display_with_serialize_deserialize]
                    env_var_name: std::string::String,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                #(#variants_token_stream),*
            }
        }
    };
    // println!("error_named_token_stream {error_named_token_stream}");
    // let fie
    // let error_std_env_var_ident =
    //     syn::Ident::new(&format!("{ident}StdEnvVar"), ident.span());
    // let error_std_env_var_enum_ident =
    //     syn::Ident::new(&format!("{ident}ErrorStdEnvEnum"), ident.span());
    // let error_parse_ident = syn::Ident::new(&format!("{ident}Parse"), ident.span());
    // let error_parse_enum_ident =
    //     syn::Ident::new(&format!("{ident}ErrorParseEnum"), ident.span());
    // let capacity = datastruct.fields.len();
    // let mut generated_init_struct_fields: Vec<proc_macro2::TokenStream> =
    //     Vec::with_capacity(capacity);
    // let mut generated_functions: Vec<proc_macro2::TokenStream> =
    //     Vec::with_capacity(capacity);
    // let mut generated_enum_error_std_env_var_variants: Vec<proc_macro2::TokenStream> =
    //     Vec::with_capacity(capacity);
    // let mut generated_enum_error_parse_variants: Vec<proc_macro2::TokenStream> =
    //     Vec::with_capacity(capacity);
    // datastruct.fields.into_iter().for_each(|field| {
    //     let (
    //         enum_variant_ident_value,
    //         env_var_name,
    //         enum_variant_ident
    //     ) = field.ident.as_ref().map_or_else(|| panic!(
    //         "InitFromEnvWithPanicIfFailed {}",
    //         naming_constants::FIELD_IDENT_IS_NONE
    //     ), |field_ident| {
    //         generated_init_struct_fields.push(quote::quote! {
    //             #field_ident: #field_ident
    //         });
    //         (
    //             field_ident,
    //             syn::Ident::new(
    //                 &convert_case::Casing::to_case(&format!("{field_ident}"), convert_case::Case::Snake)
    //                     .to_uppercase(),
    //                 ident.span(),
    //             ),
    //             syn::Ident::new(
    //                 &convert_case::Casing::to_case(&format!("{field_ident}"),convert_case::Case::UpperCamel),
    //                 ident.span(),
    //             )
    //         )
    //     });
    //     let (
    //         enum_variant_type,
    //         enum_variant_type_as_string,
    //     ) = match &field.ty {
    //         //todo: add different types support
    //         syn::Type::Path(type_path) => {
    //             let enum_variant_type = &type_path.path;
    //             let string_handle = if type_path.path.segments.len() == 1 {
    //                 format!("{}", type_path.path.segments.first().unwrap_or_else(||panic!("failed to get first type_path.path.segments element")).ident)
    //             } else {
    //                 let mut value = std::string::String::from("");
    //                 for seg in &type_path.path.segments {
    //                     value.push_str(&format!("{}:", seg.ident));
    //                 }
    //                 if !value.is_empty() {
    //                     let _: std::option::Option<std::primitive::char> = value.pop();
    //                 }
    //                 value
    //             };
    //             (
    //                 enum_variant_type,
    //                 syn::LitStr::new(&string_handle, ident.span())
    //             )
    //         }
    //         syn::Type::Array(_) | 
    //         syn::Type::BareFn(_) | 
    //         syn::Type::Group(_) | 
    //         syn::Type::ImplTrait(_) | 
    //         syn::Type::Infer(_) | 
    //         syn::Type::Macro(_) | 
    //         syn::Type::Never(_) | 
    //         syn::Type::Paren(_) | 
    //         syn::Type::Ptr(_) | 
    //         syn::Type::Reference(_) | 
    //         syn::Type::Slice(_) | 
    //         syn::Type::TraitObject(_) | 
    //         syn::Type::Tuple(_) | 
    //         syn::Type::Verbatim(_) => panic!("InitFromEnvWithPanicIfFailed field.ty supports only syn::Type::Path!"),
    //         _ => panic!("InitFromEnvWithPanicIfFailed field.ty supports only syn::Type::Path!(exhaustive)"),
    //     };
    //     let env_var_name_as_screaming_snake_case_string =
    //         syn::LitStr::new(&format!("{env_var_name}"), ident.span());
    //     generated_functions.push(quote::quote! {
    //         let #enum_variant_ident_value: #enum_variant_type;
    //         match std::env::var(#env_var_name_as_screaming_snake_case_string) {
    //             Err(error) => {
    //                 return Err(#error_enum_ident::#error_std_env_var_ident(#error_std_env_var_enum_ident::#enum_variant_ident {
    //                     source: error,
    //                     env_var_name: #env_var_name_as_screaming_snake_case_string,
    //                 }));
    //             },
    //             Ok(string_handle) => {
    //                 match string_handle.parse::<#enum_variant_type>() {
    //                     Err(_) => {
    //                         return Err(#error_enum_ident::#error_parse_ident(#error_parse_enum_ident::#enum_variant_ident{
    //                             env_var_name: #env_var_name_as_screaming_snake_case_string,
    //                             expected_env_var_type: #enum_variant_type_as_string,
    //                         }));
    //                     },
    //                     Ok(handle) => {
    //                         #enum_variant_ident_value = handle;
    //                     },
    //                 }
    //             },
    //         }
    //     });
    //     generated_enum_error_std_env_var_variants.push(quote::quote! {
    //         #enum_variant_ident {
    //             source: std::env::VarError,
    //             env_var_name: &'static str,
    //         },
    //     });
    //     generated_enum_error_parse_variants.push(quote::quote! {
    //         #enum_variant_ident {
    //             env_var_name: &'static str,
    //             expected_env_var_type: &'static str,
    //         },
    //     });
    // });
    // let dotenv_variant_name_upper_camel_case_token_stream = quote::quote!{Dotenv};
    // let dotenv_variant_name_snake_case_token_stream = quote::quote!{dotenv};
    // let dotenv_error_token_stream = quote::quote!{dotenv::Error};//implements Display
    let gen = quote::quote! {
        #error_named_token_stream
        // #[derive(Debug)]
        // pub enum #error_enum_ident {
        //     #error_std_env_var_ident(#error_std_env_var_enum_ident),
        //     #error_parse_ident(#error_parse_enum_ident),
        // }
        // #[derive(Debug)]
        // pub enum #error_std_env_var_enum_ident {
        //     #(#generated_enum_error_std_env_var_variants)*
        // }
        // #[derive(Debug)]
        // pub enum #error_parse_enum_ident {
        //     #(#generated_enum_error_parse_variants)*
        // }
        // impl #ident {
        //     #[must_use]
        //     pub fn new() -> Result<Self, #error_enum_ident> {
        //         if let Err(error) = dotenv::dotenv() {
        //             return Err(#error_enum_ident::#dotenv_variant_name_upper_camel_case_token_stream {
        //                 #dotenv_variant_name_snake_case_token_stream: error,
        //             });
        //         }
        //         #(#generated_functions)*
        //         Ok(
        //             Self {
        //                 #(#generated_init_struct_fields,)*
        //             }
        //         )
        //     }
        // }
    };
    // println!("{gen}");
    gen.into()
}
