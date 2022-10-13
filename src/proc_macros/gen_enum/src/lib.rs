#[proc_macro_derive(GenEnum)]
pub fn derive_gen_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use convert_case::Case;
    use convert_case::Casing;
    let ast: syn::DeriveInput = syn::parse(input).expect("GenEnum syn::parse(input) failed");
    let ident = &ast.ident;
    let generated = match ast.data {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let enum_variant_ident = match field.ident {
                None => panic!("field.ident is None"),
                Some(field_ident) => syn::Ident::new(
                    &format!("{field_ident}").to_case(Case::Pascal),
                    ident.span(),
                ),
            };
            let enum_variant_type = match field.ty {
                syn::Type::Path(type_path) => type_path.path,
                _ => panic!("field.ty is not a syn::Type::Path!"),
            };
            quote::quote! {
                #enum_variant_ident(#enum_variant_type),
            }
        }),
        _ => panic!("GenEnum only works on Struct"),
    };
    let enum_ident = syn::Ident::new(&format!("{ident}Enum"), ident.span());
    let gen = quote::quote! {
        #[derive(Debug, strum_macros::EnumIter, strum_macros::Display, enum_extension::EnumExtension)]
        pub enum #enum_ident {
            #(#generated)*
        }
    };
    gen.into()
}
