use proc_macro2::TokenStream as Ts2;
use quote::quote;
#[proc_macro_derive(TryFromEnv)]
pub fn try_from_env(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use naming::parameter::{SelfTryFromEnvErrorNamedUcc, TryFromStdEnvVarOkSelfErrorNamedUcc};
    use naming::{
        DotenvSc, DotenvUcc, EnvVarNameSc, StdEnvVarErrorSc, StdEnvVarErrorUcc, ToTokensToUccTs,
        ToTokensToUpperScStr, TryFromStdEnvVarOkUcc,
    };
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input).expect("e45f75c2-92ea-4f80-9962-a2438ac0b3fe");
    let ident = &syn_derive_input.ident;
    let ident_try_from_env_error_named_ucc = SelfTryFromEnvErrorNamedUcc::from_tokens(&ident);
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
    let dotenv_ucc = DotenvUcc;
    let dotenv_sc = DotenvSc;
    let std_env_var_error_ucc = StdEnvVarErrorUcc;
    let std_env_var_error_sc = StdEnvVarErrorSc;
    let env_var_name_sc = EnvVarNameSc;
    let try_from_std_env_var_ok_ucc = TryFromStdEnvVarOkUcc;
    let error_named_ts = {
        let variants_ts = fields_named.iter().map(|el_f931deb2| {
            let el_ident = &el_f931deb2
                .ident
                .as_ref()
                .expect("2ecb63c1-675f-489a-af65-a6a17c778bd6");
            let el_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&el_ident);
            let try_from_std_env_var_ok_self_error_named_ucc =
                TryFromStdEnvVarOkSelfErrorNamedUcc::from_tokens(&el_ident);
            quote! {
                #el_ident_ucc_ts {
                    #el_ident: config_lib::#try_from_std_env_var_ok_self_error_named_ucc,
                }
            }
        });
        quote! {
            #[derive(Debug, thiserror::Error)]
            pub enum #ident_try_from_env_error_named_ucc {
                #dotenv_ucc {
                    #dotenv_sc: dotenv::Error,
                },
                #std_env_var_error_ucc {
                    #std_env_var_error_sc: std::env::VarError,
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
            let el_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&el_ident);
            quote! {
                Self::#el_ident_ucc_ts { #el_ident } => write!(f, "{}", #el_ident)
            }
        });
        macros_helpers::gen_impl_std_fmt_display_ts(
            &Ts2::new(),
            &ident_try_from_env_error_named_ucc,
            &Ts2::new(),
            &quote! {
                match self {
                    Self::#dotenv_ucc {
                        #dotenv_sc
                    } => write!(f, "{}", #dotenv_sc),
                    Self::#std_env_var_error_ucc {
                        #std_env_var_error_sc,
                        env_var_name
                    } => write!(f, "{} {}", #std_env_var_error_sc, env_var_name),
                    #(#variants_ts),*
                }
            },
        )
    };
    let try_from_env_ts = {
        let fields_initialization_ts = fields_named.iter().map(|el_0b2240f0| {
            let el_ident = &el_0b2240f0
                .ident
                .as_ref()
                .expect("ebf4e1b2-f07a-40ee-b885-fc8be3444d9a");
            let el_ident_quotes_upper_sc_string =
                syn::LitStr::new(&ToTokensToUpperScStr::case(&el_ident), ident.span());
            let el_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&el_ident);
            let el_ident_wrapper_ucc_ts = ToTokensToUccTs::case_or_panic(&el_ident);
            quote! {
                let #el_ident = {
                    let env_var_name = String::from(#el_ident_quotes_upper_sc_string);
                    match std::env::var(&env_var_name) {
                        Err(error) => {
                            return Err(#ident_try_from_env_error_named_ucc::#std_env_var_error_ucc {
                                #std_env_var_error_sc: error,
                                #env_var_name_sc,
                            });
                        }
                        Ok(value) => match <
                            config_lib::#el_ident_wrapper_ucc_ts as
                            config_lib::#try_from_std_env_var_ok_ucc
                        >::try_from_std_env_var_ok(value) {
                            Err(error) => {
                                return Err(#ident_try_from_env_error_named_ucc::#el_ident_ucc_ts {
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
                pub fn try_from_env() -> Result<Self, #ident_try_from_env_error_named_ucc> {
                    if let Err(error) = dotenv::dotenv() {
                        return Err(#ident_try_from_env_error_named_ucc::#dotenv_ucc {
                            #dotenv_sc: error,
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
    let gend = quote! {
        #error_named_ts
        #display_error_named_ts
        #try_from_env_ts
    };
    // println!("{gend}");
    gend.into()
}
