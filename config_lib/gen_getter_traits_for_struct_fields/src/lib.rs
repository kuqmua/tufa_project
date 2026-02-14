use proc_macro2::TokenStream as Ts2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse};
#[proc_macro_derive(GenGetterTraitsForStructFields)]
pub fn gen_getter_traits_for_struct_fields(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use naming::ToTokensToUccStr;
    panic_location::panic_location();
    let syn_derive_input: DeriveInput = parse(input).expect("49780295");
    let ident = &syn_derive_input.ident;
    let datastruct = match syn_derive_input.data {
        Data::Struct(value) => value,
        Data::Enum(_) | Data::Union(_) => panic!("15cd72a2"),
    };
    let generated_traits_implementations = datastruct.fields.into_iter().map(|field| {
        let (field_ident, ucc_field_ident) = {
            let field_ident = field.ident.as_ref().expect("e5c23c45");
            (field_ident, ToTokensToUccStr::case(&field_ident))
        };
        let field_type = field.ty;
        let path_trait_ident = format!("app_state::Get{ucc_field_ident}")
            .parse::<Ts2>()
            .expect("8fb2cb27");
        let function_name_ident = format!("get_{field_ident}")
            .parse::<Ts2>()
            .expect("a349efd0");
        quote! {
            impl #path_trait_ident for #ident {
                fn #function_name_ident (&self) -> &#field_type {
                    &self.#field_ident
                }
            }
            impl #path_trait_ident for &#ident {
                fn #function_name_ident (&self) -> &#field_type {
                    &self.#field_ident
                }
            }
        }
    });
    let generated = quote! {
        #(#generated_traits_implementations)*
    };
    generated.into()
}

#[proc_macro_derive(GenGetterTrait)]
pub fn gen_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use naming::parameter::{GetSelfSc, GetSelfUcc};
    panic_location::panic_location();
    let syn_derive_input: DeriveInput = parse(input).expect("195b48f5");
    let ident = &syn_derive_input.ident;
    let data_struct = match syn_derive_input.data {
        Data::Struct(value) => value,
        Data::Enum(_) | Data::Union(_) => panic!("cd6bbc4e"),
    };
    let fields_unnamed = match data_struct.fields {
        Fields::Unnamed(value) => value.unnamed,
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
