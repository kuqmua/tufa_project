use quote::quote;
#[proc_macro_derive(TryFromEnv)]
pub fn try_from_env(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use naming::parameter::{
        SelfTryFromEnvErrorNamedUpperCamelCase, TryFromStdEnvVarOkSelfErrorNamedUpperCamelCase,
    };
    use naming::{
        DotenvSnakeCase, DotenvUpperCamelCase, EnvVarNameSnakeCase, StdEnvVarErrorSnakeCase,
        StdEnvVarErrorUpperCamelCase, ToTokensToUpperCamelCaseTokenStream,
        ToTokensToUpperSnakeCaseStringified, TryFromStdEnvVarOkUpperCamelCase,
    };
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input).expect("e45f75c2-92ea-4f80-9962-a2438ac0b3fe");
    let ident = &syn_derive_input.ident;
    let ident_try_from_env_error_named_upper_camel_case =
        SelfTryFromEnvErrorNamedUpperCamelCase::from_tokens(&ident);
    let data_struct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("54289ad5-e5f5-4062-bead-69242ae601a4"),
    };
    let fields_named = match data_struct.fields {
        syn::Fields::Named(value) => value.named,
        syn::Fields::Unnamed(_) | syn::Fields::Unit => {
            panic!("330b2512-2672-4aab-a447-27aa15a56f1c")
        }
    };
    let dotenv_upper_camel_case = DotenvUpperCamelCase;
    let dotenv_snake_case = DotenvSnakeCase;
    let std_env_var_error_upper_camel_case = StdEnvVarErrorUpperCamelCase;
    let std_env_var_error_snake_case = StdEnvVarErrorSnakeCase;
    let env_var_name_snake_case = EnvVarNameSnakeCase;
    let try_from_std_env_var_ok_upper_camel_case = TryFromStdEnvVarOkUpperCamelCase;
    let error_named_ts = {
        let variants_ts = fields_named.iter().map(|el_f931deb2| {
            let el_ident = &el_f931deb2.ident.as_ref().expect("2ecb63c1-675f-489a-af65-a6a17c778bd6");
            let el_ident_upper_camel_case_ts = ToTokensToUpperCamelCaseTokenStream::case_or_panic(&el_ident);
            let try_from_std_env_var_ok_self_error_named_upper_camel_case = TryFromStdEnvVarOkSelfErrorNamedUpperCamelCase::from_tokens(&el_ident);
            quote! {
                #el_ident_upper_camel_case_ts {
                    #el_ident: config_lib::#try_from_std_env_var_ok_self_error_named_upper_camel_case,
                }
            }
        });
        quote! {
            #[derive(Debug, thiserror::Error)]
            pub enum #ident_try_from_env_error_named_upper_camel_case {
                #dotenv_upper_camel_case {
                    #dotenv_snake_case: dotenv::Error,
                },
                #std_env_var_error_upper_camel_case {
                    #std_env_var_error_snake_case: std::env::VarError,
                    env_var_name: String,
                },
                #(#variants_ts),*
            }
        }
    };
    let display_error_named_ts = {
        let variants_ts = fields_named.iter().map(|el_f931deb2| {
            let el_ident = &el_f931deb2
                .ident
                .as_ref()
                .expect("8b79a379-2073-4415-82c6-bf7ea4b05165");
            let el_ident_upper_camel_case_ts =
                ToTokensToUpperCamelCaseTokenStream::case_or_panic(&el_ident);
            quote! {
                Self::#el_ident_upper_camel_case_ts { #el_ident } => write!(f, "{}", #el_ident)
            }
        });
        macros_helpers::generate_impl_std_fmt_display_ts(
            &proc_macro2::TokenStream::new(),
            &ident_try_from_env_error_named_upper_camel_case,
            &proc_macro2::TokenStream::new(),
            &quote! {
                match self {
                    Self::#dotenv_upper_camel_case {
                        #dotenv_snake_case
                    } => write!(f, "{}", #dotenv_snake_case),
                    Self::#std_env_var_error_upper_camel_case {
                        #std_env_var_error_snake_case,
                        env_var_name
                    } => write!(f, "{} {}", #std_env_var_error_snake_case, env_var_name),
                    #(#variants_ts),*
                }
            },
        )
    };
    let try_from_env_ts = {
        let fields_initialization_ts = fields_named.iter().map(|el_0b2240f0| {
            let el_ident = &el_0b2240f0.ident.as_ref().expect("ebf4e1b2-f07a-40ee-b885-fc8be3444d9a");
            let el_ident_quotes_upper_snake_case_string = syn::LitStr::new(&ToTokensToUpperSnakeCaseStringified::case(&el_ident), ident.span());
            let el_ident_upper_camel_case_ts = ToTokensToUpperCamelCaseTokenStream::case_or_panic(&el_ident);
            let el_ident_wrapper_upper_camel_case_ts = ToTokensToUpperCamelCaseTokenStream::case_or_panic(&el_ident);
            quote! {
                let #el_ident = {
                    let env_var_name = String::from(#el_ident_quotes_upper_snake_case_string);
                    match std::env::var(&env_var_name) {
                        Err(error) => {
                            return Err(#ident_try_from_env_error_named_upper_camel_case::#std_env_var_error_upper_camel_case {
                                #std_env_var_error_snake_case: error,
                                #env_var_name_snake_case,
                            });
                        }
                        Ok(value) => match <
                            config_lib::#el_ident_wrapper_upper_camel_case_ts as
                            config_lib::#try_from_std_env_var_ok_upper_camel_case
                        >::try_from_std_env_var_ok(value) {
                            Err(error) => {
                                return Err(#ident_try_from_env_error_named_upper_camel_case::#el_ident_upper_camel_case_ts {
                                    #el_ident: error,
                                });
                            }
                            Ok(value) => value.0,
                        },
                    }
                };
            }
        });
        let fields_ts = fields_named.iter().map(|el_dd7dea0c| &el_dd7dea0c.ident);
        quote! {
            impl #ident {
                pub fn try_from_env() -> Result<Self, #ident_try_from_env_error_named_upper_camel_case> {
                    if let Err(error) = dotenv::dotenv() {
                        return Err(#ident_try_from_env_error_named_upper_camel_case::#dotenv_upper_camel_case {
                            #dotenv_snake_case: error,
                        });
                    }
                    #(#fields_initialization_ts)*
                    Ok(Self {
                        #(#fields_ts),*
                    })
                }
            }
        }
    };
    let generated = quote! {
        #error_named_ts
        #display_error_named_ts
        #try_from_env_ts
    };
    // println!("{generated}");
    generated.into()
}
