use naming::ToTokensToUpperCamelCaseStringified;
use naming::parameter::{GetSelfSnakeCase, GetSelfUpperCamelCase};

#[proc_macro_derive(GenerateGetterTraitsForStructFields)]
pub fn generate_getter_traits_for_struct_fields(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input).expect("49780295-2350-409b-979d-ebd653dd223b");
    let ident = &syn_derive_input.ident;
    let datastruct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("15cd72a2-2f8c-4d47-91b8-e86530856966"),
    };
    let generated_traits_implementations = datastruct.fields.into_iter().map(|field| {
        let (field_ident, upper_camel_case_field_ident) = {
            let field_ident = field
                .ident
                .as_ref()
                .expect("e5c23c45-9bcf-485b-a6d7-0fcb99f9346b");
            (
                field_ident,
                ToTokensToUpperCamelCaseStringified::case(&field_ident),
            )
        };
        let field_type = field.ty;
        let path_trait_ident = format!("app_state::Get{upper_camel_case_field_ident}")
            .parse::<proc_macro2::TokenStream>()
            .expect("8fb2cb27-69ec-4462-bb91-301ce5e9520e");
        let function_name_ident = format!("get_{field_ident}")
            .parse::<proc_macro2::TokenStream>()
            .expect("a349efd0-9b62-426b-a473-b9d3a3201424");
        quote::quote! {
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
    let generated = quote::quote! {
        #(#generated_traits_implementations)*
    };
    generated.into()
}

#[proc_macro_derive(GenerateGetterTrait)]
pub fn generate_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput =
        syn::parse(input).expect("195b48f5-8dda-4735-a580-86e5db9cdcf3");
    let ident = &syn_derive_input.ident;
    let data_struct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("cd6bbc4e-0cb8-4eff-bc88-69db5909f534"),
    };
    let fields_unnamed = match data_struct.fields {
        syn::Fields::Unnamed(value) => value.unnamed,
        syn::Fields::Named(_) | syn::Fields::Unit => panic!("577cb86a-9071-40ca-9afd-4e0bfeb60cc1"),
    };
    assert!(
        fields_unnamed.len() == 1,
        "1e82dc7e-724c-4599-93aa-442b262cbcf5"
    );
    let first_field_unnamed = fields_unnamed
        .iter()
        .next()
        .expect("7c2531fd-3a78-43fa-8990-44d8e8438fa3");
    let first_field_unnamed_type = &first_field_unnamed.ty;
    let get_ident_upper_camel_case = GetSelfUpperCamelCase::from_tokens(&ident);
    let get_ident_snake_case = GetSelfSnakeCase::from_tokens(&ident);
    let generated = quote::quote! {
        pub trait #get_ident_upper_camel_case {
            fn #get_ident_snake_case(&self) -> &#first_field_unnamed_type;
        }
    };
    // println!("{generated}");
    generated.into()
}
