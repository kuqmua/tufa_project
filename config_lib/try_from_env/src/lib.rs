use macros_helpers::gen_impl_display_ts;
use naming::{
    DotenvSc, DotenvUcc, EnvVarNameSc, StdEnvVarErSc, StdEnvVarErUcc, ToTokensToUccTs,
    ToTokensToUpperScStr, TryFromStdEnvVarOkUcc,
    prm::{SelfTryFromEnvErUcc, TryFromStdEnvVarOkSelfErUcc},
};
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident, LitStr, parse};
#[allow(clippy::missing_const_for_fn)] // syn::Field access path is used for macro parsing only; const form is not useful here
fn field_ident<'field_lt>(field: &'field_lt Field, exp_id: &'static str) -> &'field_lt Ident {
    field.ident.as_ref().unwrap_or_else(|| panic!("{exp_id}"))
}
#[proc_macro_derive(TryFromEnv)]
pub fn try_from_env(v: Ts) -> Ts {
    panic_loc::panic_loc();
    let di: DeriveInput = parse(v).expect("e45f75c2");
    let ident = &di.ident;
    let ident_try_from_env_er_ucc = SelfTryFromEnvErUcc::from_tokens(&ident);
    let data_struct = match di.data {
        Data::Struct(v0) => v0,
        Data::Enum(_) | Data::Union(_) => panic!("54289ad5"),
    };
    let fields_named = match data_struct.fields {
        Fields::Named(v0) => v0.named,
        Fields::Unnamed(_) | Fields::Unit => panic!("330b2512"),
    };
    let er_ts = {
        let vrts_ts = fields_named.iter().map(|el| {
            let el_ident = field_ident(el, "2ecb63c1");
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
            #[derive(Debug, Error, optml::Optml)]
            pub enum #ident_try_from_env_er_ucc {
                #DotenvUcc {
                    #DotenvSc: dotenv::Error,
                },
                #StdEnvVarErUcc {
                    #StdEnvVarErSc: std::env::VarError,
                    env_var_name: String,
                },
                #(#vrts_ts),*
            }
        }
    };
    let display_er_ts = {
        let vrts_ts = fields_named.iter().map(|el| {
            let el_ident = field_ident(el, "8b79a379");
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
                    #(#vrts_ts),*
                }
            },
        )
    };
    let try_from_env_ts = {
        let fields_init_ts = fields_named.iter().map(|el| {
            let el_ident = field_ident(el, "ebf4e1b2");
            let el_ident_quotes_upper_sc_string =
                LitStr::new(&ToTokensToUpperScStr::case(el_ident), ident.span());
            let el_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&el_ident);
            quote! {
                let #el_ident = config_lib::parse_required_env_var(
                    #el_ident_quotes_upper_sc_string,
                    |#StdEnvVarErSc, #EnvVarNameSc| #ident_try_from_env_er_ucc::#StdEnvVarErUcc {
                        #StdEnvVarErSc,
                        #EnvVarNameSc,
                    },
                    |v| <
                        config_lib::#el_ident_ucc_ts as
                        config_lib::#TryFromStdEnvVarOkUcc
                    >::try_from_std_env_var_ok(v),
                    |#el_ident| #ident_try_from_env_er_ucc::#el_ident_ucc_ts {
                        #el_ident,
                    },
                )?.0;
            }
        });
        let fields_ts = fields_named.iter().map(|el| &el.ident);
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
