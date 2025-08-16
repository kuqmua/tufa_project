#[proc_macro_derive(TryFromEnv)]
pub fn try_from_env(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input)
        .unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_try_from_env_error_named_upper_camel_case =
        naming::parameter::SelfTryFromEnvErrorNamedUpperCamelCase::from_tokens(&ident);
    let data_struct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("only works on Struct"),
    };
    let fields_named = match data_struct.fields {
        syn::Fields::Named(value) => value.named,
        syn::Fields::Unnamed(_) | syn::Fields::Unit => panic!("only works with syn::Fields::Named"),
    };
    let ident_in_none_stringified = "ident is None";
    let dotenv_upper_camel_case = naming::DotenvUpperCamelCase;
    let dotenv_snake_case = naming::DotenvSnakeCase;
    let std_env_var_error_upper_camel_case = naming::StdEnvVarErrorUpperCamelCase;
    let std_env_var_error_snake_case = naming::StdEnvVarErrorSnakeCase;
    let env_var_name_snake_case = naming::EnvVarNameSnakeCase;
    let try_from_std_env_var_ok_upper_camel_case = naming::TryFromStdEnvVarOkUpperCamelCase;
    let error_named_token_stream = {
        let variants_token_stream = fields_named.iter().map(|element| {
            let element_ident = &element.ident.as_ref().expect(ident_in_none_stringified);
            let element_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element_ident);
            let try_from_std_env_var_ok_self_error_named_upper_camel_case = naming::parameter::TryFromStdEnvVarOkSelfErrorNamedUpperCamelCase::from_tokens(&element_ident);
            quote::quote! {
                #element_ident_upper_camel_case_token_stream {
                    #element_ident: config_lib::#try_from_std_env_var_ok_self_error_named_upper_camel_case,
                }
            }
        });
        quote::quote! {
            #[derive(Debug, thiserror::Error)]
            pub enum #ident_try_from_env_error_named_upper_camel_case {
                #dotenv_upper_camel_case {
                    #dotenv_snake_case: dotenv::Error,
                },
                #std_env_var_error_upper_camel_case {
                    #std_env_var_error_snake_case: std::env::VarError,
                    env_var_name: std::string::String,
                },
                #(#variants_token_stream),*
            }
        }
    };
    let display_error_named_token_stream = {
        let variants_token_stream = fields_named.iter().map(|element| {
            let element_ident = &element.ident.as_ref().expect(ident_in_none_stringified);
            let element_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element_ident);
            quote::quote! {
                Self::#element_ident_upper_camel_case_token_stream { #element_ident } => write!(formatter, "{}", #element_ident)
            }
        });
        macros_helpers::generate_impl_std_fmt_display_token_stream(
            &proc_macro2::TokenStream::new(),
            &ident_try_from_env_error_named_upper_camel_case,
            &proc_macro2::TokenStream::new(),
            &quote::quote! {
                match self {
                    Self::#dotenv_upper_camel_case {
                        #dotenv_snake_case
                    } => write!(formatter, "{}", #dotenv_snake_case),
                    Self::#std_env_var_error_upper_camel_case {
                        #std_env_var_error_snake_case,
                        env_var_name
                    } => write!(formatter, "{} {}", #std_env_var_error_snake_case, env_var_name),
                    #(#variants_token_stream),*
                }
            },
        )
    };
    let try_from_env_token_stream = {
        let fields_initialization_token_stream = fields_named.iter().map(|element| {
            let element_ident = &element.ident.as_ref().expect(ident_in_none_stringified);
            let element_ident_quotes_screaming_snake_case_string = syn::LitStr::new(&naming::ToTokensToScreamingSnakeCaseStringified::case(&element_ident), ident.span());
            let element_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element_ident);
            let element_ident_wrapper_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element_ident);
            quote::quote! {
                let #element_ident = {
                    let env_var_name = std::string::String::from(#element_ident_quotes_screaming_snake_case_string);
                    match std::env::var(&env_var_name) {
                        Err(error) => {
                            return Err(#ident_try_from_env_error_named_upper_camel_case::#std_env_var_error_upper_camel_case {
                                #std_env_var_error_snake_case: error,
                                #env_var_name_snake_case,
                            });
                        }
                        Ok(value) => match <
                            config_lib::#element_ident_wrapper_upper_camel_case_token_stream as
                            config_lib::#try_from_std_env_var_ok_upper_camel_case
                        >::try_from_std_env_var_ok(value) {
                            Err(error) => {
                                return Err(#ident_try_from_env_error_named_upper_camel_case::#element_ident_upper_camel_case_token_stream {
                                    #element_ident: error,
                                });
                            }
                            Ok(value) => value.0,
                        },
                    }
                };
            }
        });
        let fields_token_stream = fields_named.iter().map(|element| &element.ident);
        quote::quote! {
            impl #ident {
                pub fn try_from_env() -> Result<Self, #ident_try_from_env_error_named_upper_camel_case> {
                    if let Err(error) = dotenv::dotenv() {
                        return Err(#ident_try_from_env_error_named_upper_camel_case::#dotenv_upper_camel_case {
                            #dotenv_snake_case: error,
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
    let generated = quote::quote! {
        #error_named_token_stream
        #display_error_named_token_stream
        #try_from_env_token_stream
    };
    // println!("{generated}");
    generated.into()
}
