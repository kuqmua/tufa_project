#[proc_macro_derive(GenerateGetterTraitsForStructFields)]
pub fn generate_getter_traits_for_struct_fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).expect("49780295-2350-409b-979d-ebd653dd223b");
    let ident = &syn_derive_input.ident;
    let datastruct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("only works on Struct"),
    };
    let generated_traits_implementations = datastruct.fields.into_iter().map(|field| {
        let (field_ident, upper_camel_case_field_ident) = {
            let field_ident = field.ident.as_ref().unwrap_or_else(|| panic!("{ident} {}", naming::FIELD_IDENT_IS_NONE));
            (field_ident, naming::ToTokensToUpperCamelCaseStringified::case(&field_ident))
        };
        let field_type = field.ty;
        let path_trait_ident = format!("app_state::Get{upper_camel_case_field_ident}").parse::<proc_macro2::TokenStream>().expect("8fb2cb27-69ec-4462-bb91-301ce5e9520e");
        let function_name_ident = format!("get_{field_ident}").parse::<proc_macro2::TokenStream>().expect("a349efd0-9b62-426b-a473-b9d3a3201424");
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
    let syn_derive_input: syn::DeriveInput = syn::parse(input).expect("195b48f5-8dda-4735-a580-86e5db9cdcf3");
    let ident = &syn_derive_input.ident;
    let data_struct = match syn_derive_input.data {
        syn::Data::Struct(value) => value,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("only works on Struct"),
    };
    let fields_unnamed = match data_struct.fields {
        syn::Fields::Unnamed(value) => value.unnamed,
        syn::Fields::Named(_) | syn::Fields::Unit => panic!("only works with syn::Fields::Unnamed"),
    };
    assert!(fields_unnamed.len() == 1, "fields_unnamed !== 1");
    let first_field_unnamed = fields_unnamed.iter().next().unwrap_or_else(|| panic!("fields_unnamed.iter().nth(0) is None"));
    let first_field_unnamed_type = &first_field_unnamed.ty;
    let get_ident_upper_camel_case = naming::parameter::GetSelfUpperCamelCase::from_tokens(&ident);
    let get_ident_snake_case = naming::parameter::GetSelfSnakeCase::from_tokens(&ident);
    let generated = quote::quote! {
        pub trait #get_ident_upper_camel_case {
            fn #get_ident_snake_case(&self) -> &#first_field_unnamed_type;
        }
    };
    // println!("{generated}");
    generated.into()
}
