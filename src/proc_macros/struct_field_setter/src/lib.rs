#[proc_macro_derive(DeriveStructFieldSetter)]
pub fn derive_struct_field_setter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("DeriveStructFieldSetter syn::parse(input) failed");
    let fields = match ast.data {
        syn::Data::Struct(struct_item) => struct_item.fields,
        _ => panic!("DeriveStructFieldSetter only works on structs"),
    };
    let ident = &ast.ident;
    let generated = fields.into_iter().map(|v| {
        let field_ident = v
            .ident
            .unwrap_or_else(|| panic!("DeriveStructFieldSetter v.ident is None"));
        let field_type_ident = match v.ty {
            // syn::Type::Array(_) => todo!(),
            // syn::Type::BareFn(_) => todo!(),
            // syn::Type::Group(_) => todo!(),
            // syn::Type::ImplTrait(_) => todo!(),
            // syn::Type::Infer(_) => todo!(),
            // syn::Type::Macro(_) => todo!(),
            // syn::Type::Never(_) => todo!(),
            // syn::Type::Paren(_) => todo!(),
            // syn::Type::Ptr(_) => todo!(),
            // syn::Type::Reference(_) => todo!(),
            // syn::Type::Slice(_) => todo!(),
            // syn::Type::TraitObject(_) => todo!(),
            // syn::Type::Tuple(_) => todo!(),
            // syn::Type::Verbatim(_) => todo!(),
            syn::Type::Path(type_path) => {
                if type_path.path.segments.len() != 1 {
                    panic!(
                        "DeriveStructFieldSetter type_path.path.segments != 1, length is {}",
                        type_path.path.segments.len()
                    );
                }
                type_path.path.segments[0].ident.clone()
            }
            _ => panic!("DeriveStructFieldSetter only works on structs"),
        };
        let setter_function_ident = syn::Ident::new(&format!("set_{field_ident}"), ident.span());
        quote::quote! {
            pub fn #setter_function_ident(&mut self, value: #field_type_ident) {
                self.#field_ident = value;
            }
        }
    });
    let gen = quote::quote! {
        impl #ident {
            #(#generated)*
        }
    };
    gen.into()
}
