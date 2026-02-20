use macros_helpers::gen_impl_display_ts;
use naming::parameter::{SelfTryFromEnvErUcc, TryFromStdEnvVarOkSelfErUcc};
use naming::{
    DotenvSc, DotenvUcc, EnvVarNameSc, StdEnvVarErSc, StdEnvVarErUcc, ToTokensToUccTs,
    ToTokensToUpperScStr, TryFromStdEnvVarOkUcc,
};
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse};
#[proc_macro_derive(TryFromEnv)]
pub fn try_from_env(input: Ts) -> Ts {
    panic_location::panic_location();
    let di: DeriveInput = parse(input).expect("e45f75c2");
    let ident = &di.ident;
    let ident_try_from_env_er_ucc = SelfTryFromEnvErUcc::from_tokens(&ident);
    let data_struct = match di.data {
        Data::Struct(v) => v,
        Data::Enum(_) | Data::Union(_) => panic!("54289ad5"),
    };
    let fields_named = match data_struct.fields {
        Fields::Named(v) => v.named,
        Fields::Unnamed(_) | Fields::Unit => {
            panic!("330b2512")
        }
    };
    let er_ts = {
        let variants_ts = fields_named.iter().map(|el_f931deb2| {
            let el_ident = &el_f931deb2.ident.as_ref().expect("2ecb63c1");
            let el_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&el_ident);
            let try_from_std_env_var_ok_self_er_ucc =
                TryFromStdEnvVarOkSelfErUcc::from_tokens(&el_ident);
            quote! {
                #el_ident_ucc_ts {
                    #el_ident: config_lib::#try_from_std_env_var_ok_self_er_ucc,
                }
            }
        });
        quote! {
            #[derive(Debug, Error)]
            pub enum #ident_try_from_env_er_ucc {
                #DotenvUcc {
                    #DotenvSc: dotenv::Error,
                },
                #StdEnvVarErUcc {
                    #StdEnvVarErSc: std::env::VarError,
                    env_var_name: String,
                },
                #(#variants_ts),*
            }
        }
    };
    let display_er_ts = {
        let variants_ts = fields_named.iter().map(|el_f931deb2| {
            let el_ident = &el_f931deb2.ident.as_ref().expect("8b79a379");
            let el_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&el_ident);
            quote! {
                Self::#el_ident_ucc_ts { #el_ident } => write!(f, "{}", #el_ident)
            }
        });
        gen_impl_display_ts(
            &Ts2::new(),
            &ident_try_from_env_er_ucc,
            &Ts2::new(),
            &quote! {
                match self {
                    Self::#DotenvUcc {
                        #DotenvSc
                    } => write!(f, "{}", #DotenvSc),
                    Self::#StdEnvVarErUcc {
                        #StdEnvVarErSc,
                        env_var_name
                    } => write!(f, "{} {}", #StdEnvVarErSc, env_var_name),
                    #(#variants_ts),*
                }
            },
        )
    };
    let try_from_env_ts = {
        let fields_init_ts = fields_named.iter().map(|el_0b2240f0| {
            let el_ident = &el_0b2240f0.ident.as_ref().expect("ebf4e1b2");
            let el_ident_quotes_upper_sc_string =
                LitStr::new(&ToTokensToUpperScStr::case(&el_ident), ident.span());
            let el_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&el_ident);
            let el_ident_wrapper_ucc_ts = ToTokensToUccTs::case_or_panic(&el_ident);
            quote! {
                let #el_ident = {
                    let env_var_name = String::from(#el_ident_quotes_upper_sc_string);
                    match std::env::var(&env_var_name) {
                        Err(er) => {
                            return Err(#ident_try_from_env_er_ucc::#StdEnvVarErUcc {
                                #StdEnvVarErSc: er,
                                #EnvVarNameSc,
                            });
                        }
                        Ok(v) => match <
                            config_lib::#el_ident_wrapper_ucc_ts as
                            config_lib::#TryFromStdEnvVarOkUcc
                        >::try_from_std_env_var_ok(v) {
                            Err(er) => {
                                return Err(#ident_try_from_env_er_ucc::#el_ident_ucc_ts {
                                    #el_ident: er,
                                });
                            }
                            Ok(v) => v.0,
                        },
                    }
                };
            }
        });
        let fields_ts = fields_named.iter().map(|el_dd7dea0c| &el_dd7dea0c.ident);
        quote! {
            impl #ident {
                pub fn try_from_env() -> Result<Self, #ident_try_from_env_er_ucc> {
                    if let Err(er) = dotenv::dotenv() {
                        return Err(#ident_try_from_env_er_ucc::#DotenvUcc {
                            #DotenvSc: er,
                        });
                    }
                    #(#fields_init_ts)*
                    Ok(Self {
                        #(#fields_ts),*
                    })
                }
            }
        }
    };
    let generated = quote! {
        #er_ts
        #display_er_ts
        #try_from_env_ts
    };
    // println!("{generated}");
    generated.into()
}
