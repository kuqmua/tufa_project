#[proc_macro_derive(GenEnumWithoutValues)]
pub fn derive_gen_enum_without_values(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use convert_case::Case;
    use convert_case::Casing;
    let ast: syn::DeriveInput =
        syn::parse(input).expect("GenEnumWithoutValues syn::parse(input) failed");
    let ident = &ast.ident;
    let generated = match ast.data {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let enum_variant_ident = match field.ident {
                None => panic!("field.ident is None"),
                Some(field_ident) => syn::Ident::new(
                    &format!("{}", field_ident).to_case(Case::Pascal),
                    ident.span(),
                ),
            };
            quote::quote! {
                #enum_variant_ident,
            }
        }),
        _ => panic!("GenEnum only works on Struct"),
    };
    let enum_ident = syn::Ident::new(&format!("{}EnumWithoutValues", ident), ident.span());
    let gen = quote::quote! {
        use convert_case;
         #[derive(Debug, strum_macros::Display, strum_macros::EnumIter, enum_extension::EnumExtension)]
        pub enum #enum_ident {
            #(#generated)*
        }
    };
    gen.into()
}
