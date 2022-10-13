#[proc_macro_derive(InitError)]
pub fn derive_init_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("InitError syn::parse(input) failed");
    let fields = match ast.data {
        syn::Data::Struct(struct_item) => struct_item.fields,
        _ => panic!("InitError only works on structs"),
    };
    let ident = &ast.ident;
    let source_type_ident = match fields {
        syn::Fields::Named(fields_named) => {
            match fields_named.named.len() {
                2 => {
                    match &fields_named.named[0].ty {
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
                        syn::Type::Path(type_path) => type_path.clone(),
                        _ => panic!(
                            "InitError only works on structs fields with  syn::Type::Path type"
                        ),
                    }
                }
                _ => panic!("InitError fields_named.len() != 2"),
            }
        }
        // syn::Fields::Unnamed(_) => todo!(),
        // syn::Fields::Unit => todo!(),
        _ => panic!("InitError only works with named fields"),
    };
    let gen = quote::quote! {
        impl tufa_common::traits::new_error::NewError<#source_type_ident> for #ident {
            fn new(source: #source_type_ident, where_was: tufa_common::where_was::WhereWas) -> Self {
                Self { source, where_was }
            }
        }
    };
    gen.into()
}
