#[proc_macro_derive(ImplGetWhereWasOneOrManyOneForErrorStruct)]
pub fn derive_impl_get_where_was_one_or_many_one_for_error_struct(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input)
        .expect("ImplGetWhereWasOneOrManyOneForErrorStruct syn::parse(input) failed");
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Union(_) => {
            panic!("ImplGetWhereWasOneOrManyOneForErrorStruct only work on structs and structs!")
        }
        syn::Data::Enum(_) => {
            panic!("ImplGetWhereWasOneOrManyOneForErrorStruct only work on structs and structs!")
        }
        syn::Data::Struct(_) => {
            let gen = quote::quote! {
                impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for #ident {
                    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
                        tufa_common::where_was::WhereWasOneOrMany::One(
                            tufa_common::where_was::WhereWasWithAddition {
                                additional_info: None,
                                where_was: self.where_was.clone(),
                            },
                        )
                    }
                }
            };
            gen.into()
        }
    }
}
