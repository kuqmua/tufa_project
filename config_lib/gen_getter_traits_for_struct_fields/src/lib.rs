use naming::ToTokensToUccStr;
use naming::parameter::{GetSelfSc, GetSelfUcc};
use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse};
#[proc_macro_derive(GenGetterTraitsForStructFields)]
pub fn gen_getter_traits_for_struct_fields(input: Ts) -> Ts {
    panic_location::panic_location();
    let di: DeriveInput = parse(input).expect("49780295");
    let ident = &di.ident;
    let datastruct = match di.data {
        Data::Struct(v) => v,
        Data::Enum(_) | Data::Union(_) => panic!("15cd72a2"),
    };
    let generated_traits_impls_ts = datastruct.fields.into_iter().map(|field| {
        let (fi, ucc_fi) = {
            let fi = field.ident.as_ref().expect("e5c23c45");
            (fi, ToTokensToUccStr::case(&fi))
        };
        let field_type = field.ty;
        let path_trait_ident = format!("app_state::Get{ucc_fi}")
            .parse::<Ts2>()
            .expect("8fb2cb27");
        let function_name_ident = format!("get_{fi}").parse::<Ts2>().expect("a349efd0");
        quote! {
            impl #path_trait_ident for #ident {
                fn #function_name_ident (&self) -> &#field_type {
                    &self.#fi
                }
            }
            impl #path_trait_ident for &#ident {
                fn #function_name_ident (&self) -> &#field_type {
                    &self.#fi
                }
            }
        }
    });
    let generated = quote! {#(#generated_traits_impls_ts)*};
    generated.into()
}
#[proc_macro_derive(GenGetterTrait)]
pub fn gen_getter_trait(input: Ts) -> Ts {
    panic_location::panic_location();
    let di: DeriveInput = parse(input).expect("195b48f5");
    let ident = &di.ident;
    let data_struct = match di.data {
        Data::Struct(v) => v,
        Data::Enum(_) | Data::Union(_) => panic!("cd6bbc4e"),
    };
    let fields_unnamed = match data_struct.fields {
        Fields::Unnamed(v) => v.unnamed,
        Fields::Named(_) | Fields::Unit => panic!("577cb86a"),
    };
    assert!(fields_unnamed.len() == 1, "1e82dc7e");
    let first_field_unnamed = fields_unnamed.iter().next().expect("7c2531fd");
    let first_field_unnamed_type = &first_field_unnamed.ty;
    let get_ident_ucc = GetSelfUcc::from_tokens(&ident);
    let get_ident_sc = GetSelfSc::from_tokens(&ident);
    let generated = quote! {
        pub trait #get_ident_ucc {
            fn #get_ident_sc(&self) -> &#first_field_unnamed_type;
        }
    };
    // println!("{generated}");
    generated.into()
}
