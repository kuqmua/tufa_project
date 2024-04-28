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
    let ident_try_from_env_error_named = syn::Ident::new(&format!(
        "{ident}{}{}{}{}{}",
        <naming_constants::Try as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::From as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Env as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
        <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
    ), ident.span());
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
    let ident_in_none_stringified = "ident is None";
    let dotenv_upper_camel_case_token_stream = quote::quote!{Dotenv};
    let dotenv_snake_case_token_stream = quote::quote!{dotenv};
    let std_env_var_error_upper_camel_case_token_stream = quote::quote!{StdEnvVarError};
    let std_env_var_error_snake_case_token_stream = quote::quote!{std_env_var_error};
    let env_var_name_snake_case_token_stream = quote::quote!{env_var_name};
    let try_from_std_env_var_ok_upper_camel_case_stringified = "TryFromStdEnvVarOk";
    let try_from_std_env_var_ok_upper_camel_case_token_stream = {
        try_from_std_env_var_ok_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {try_from_std_env_var_ok_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let try_from_std_env_var_ok_snake_case_token_stream = {
        let value = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&try_from_std_env_var_ok_upper_camel_case_stringified);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let error_named_token_stream = {
        let variants_token_stream = fields_named.iter().map(|element|{
            let element_ident = &element.ident;
            let element_ident_upper_camel_case_token_stream = {
                let value = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(
                    &element_ident.as_ref().expect(ident_in_none_stringified).to_string()
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let error_upper_camel_case_token_stream = {
                // let element_type_stringified = {
                //     let value_type = &element.ty;
                //     let value_token_stream = quote::quote!{#value_type};
                //     value_token_stream.to_string()
                // };
                let value = format!(
                    "{try_from_std_env_var_ok_upper_camel_case_stringified}{element_ident_upper_camel_case_token_stream}{}{}",
                    <naming_constants::Error as naming_constants::Naming>::upper_camel_case_stringified(),
                    <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified(),
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #element_ident_upper_camel_case_token_stream {
                    #element_ident: config_lib::#error_upper_camel_case_token_stream,
                }
            }
        });
        quote::quote!{
            #[derive(Debug, thiserror::Error)]
            pub enum #ident_try_from_env_error_named {
                #dotenv_upper_camel_case_token_stream {
                    #dotenv_snake_case_token_stream: dotenv::Error,
                },
                #std_env_var_error_upper_camel_case_token_stream {
                    #std_env_var_error_snake_case_token_stream: std::env::VarError,
                    env_var_name: std::string::String,
                },
                #(#variants_token_stream),*
            }
        }
    };
    let display_error_named_token_stream = {
        let variants_token_stream = fields_named.iter().map(|element|{
            let element_ident = &element.ident;
            let element_ident_upper_camel_case_token_stream = {
                let value = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(
                    &element_ident.as_ref().expect(ident_in_none_stringified).to_string()
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                Self::#element_ident_upper_camel_case_token_stream { #element_ident } => write!(formatter, "{}", #element_ident)
            }
        });
        quote::quote!{
            impl std::fmt::Display for #ident_try_from_env_error_named {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        Self::#dotenv_upper_camel_case_token_stream {
                            #dotenv_snake_case_token_stream
                        } => write!(formatter, "{}", #dotenv_snake_case_token_stream),
                        Self::#std_env_var_error_upper_camel_case_token_stream {
                            #std_env_var_error_snake_case_token_stream,
                            env_var_name
                        } => write!(formatter, "{} {}", #std_env_var_error_snake_case_token_stream, env_var_name),
                        #(#variants_token_stream),*
                    }
                }
            }
        }
    };
    let try_from_env_token_stream = {
        let fields_initialization_token_stream = fields_named.iter().map(|element|{
            let element_ident = &element.ident;
            let element_ident_quotes_screaming_snake_case_string = syn::LitStr::new(
                &convert_case::Casing::to_case(
                    &element_ident.as_ref().expect(ident_in_none_stringified).to_string(), 
                    convert_case::Case::ScreamingSnake
                ),
                ident.span()
            );
            let element_ident_upper_camel_case_token_stream = {
                let value = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(
                    &element_ident.as_ref().expect(ident_in_none_stringified).to_string()
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let element_type = &element.ty;
            quote::quote!{
                let #element_ident: #element_type = {
                    let env_var_name = std::string::String::from(#element_ident_quotes_screaming_snake_case_string);
                    match std::env::var(&env_var_name) {
                        Err(error) => {
                            return Err(#ident_try_from_env_error_named::#std_env_var_error_upper_camel_case_token_stream {
                                #std_env_var_error_snake_case_token_stream: error,
                                #env_var_name_snake_case_token_stream,
                            });
                        }
                        Ok(value) => match config_lib::#try_from_std_env_var_ok_upper_camel_case_token_stream::try_from_std_env_var_ok(value) {
                            Err(error) => {
                                return Err(#ident_try_from_env_error_named::#element_ident_upper_camel_case_token_stream {
                                    #element_ident: error,
                                });
                            }
                            Ok(value) => value,
                        },
                    }
                };
            }
        });
        let fields_token_stream = fields_named.iter().map(|element|&element.ident);
        quote::quote!{
            impl #ident {
                fn try_from_env() -> Result<Self, #ident_try_from_env_error_named> {
                    if let Err(error) = dotenv::dotenv() {
                        return Err(#ident_try_from_env_error_named::#dotenv_upper_camel_case_token_stream {
                            #dotenv_snake_case_token_stream: error,
                        });
                    }
                    #(#fields_initialization_token_stream)*
                    Ok(Self {
                        #(#fields_token_stream),*
                    })
                }
            }
        }
    };
    let gen = quote::quote! {
        #error_named_token_stream
        #display_error_named_token_stream
        #try_from_env_token_stream
    };
    // println!("{gen}");
    gen.into()
}
