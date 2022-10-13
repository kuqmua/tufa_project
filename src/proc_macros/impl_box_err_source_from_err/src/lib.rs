#[proc_macro_derive(ImplBoxErrSourceForErr)]
pub fn derive_impl_box_err_source_from_err(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ImplBoxErrSourceForErr syn::parse(input) failed");
    let ident = &ast.ident;
    let error_type_ident: syn::TypePath;
    match &ast.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                if fields_named.named.len() != 1 {
                    panic!(
                        "fields_named.named != 1, length is {}",
                        fields_named.named.len()
                    );
                }
                match &fields_named.named[0].ty {
                    syn::Type::Path(type_path) => {
                        if type_path.path.segments.len() != 1 {
                            panic!(
                                "type_path.path.segments != 1, length is {}",
                                type_path.path.segments.len()
                            );
                        }
                        match &type_path.path.segments[0].arguments {
                            syn::PathArguments::AngleBracketed(abga) => {
                                if abga.args.len() != 1 {
                                    panic!(
                                        "abga.args != 1, length is {}",
                                        abga.args.len()
                                    );
                                }
                                match &abga.args[0] {
                                    syn::GenericArgument::Type(type_handle) => {
                                        match type_handle {
                                            syn::Type::Path(type_path) => {
                                                error_type_ident = type_path.clone();
                                            },
                                            _ => panic!("type_handle is not a syn::Type::Path!"),
                                        }
                                    },
                                    _ => panic!("abga.args[0] is not a syn::GenericArgument::Type!"),
                                }
                            },
                            _ => panic!("type_path.path.segments[0].arguments is not a syn::PathArguments::AngleBracketed!"),
                        }
                    }
                    _ => panic!("fields_named.named[0].ty is not a syn::Type::Path!"),
                }
            }
            _ => panic!("data_struct.fields is not a syn::Fields::Named!"),
        },
        _ => panic!("data is not a Struct!"),
    }
    let gen = quote::quote! {
        impl From<#error_type_ident> for #ident {
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn from(error: #error_type_ident) -> Self {
                #ident {
                    source: Box::new(error),
                }
            }
        }
    };
    gen.into()
}
