#[proc_macro_derive(ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource)]
pub fn derive_impl_get_where_was_one_or_many_for_struct_with_hasmap_or_vec_source(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input)
        .expect("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource syn::parse(input) failed");
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Union(_) => {
            panic!("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource only work on structs and enums!")
        }
        syn::Data::Struct(_) => {
            panic!("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource only work on structs and enums!")
        }
        syn::Data::Enum(data_enum) => {
            let variants = data_enum.variants.into_iter().map(|v| {
                let variant_ident = v.ident;
                match v.fields {
                    syn::Fields::Unit => {
                        panic!(
                            "ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource still not work with syn::Fields::Unit"
                        )
                    }
                    syn::Fields::Named(fields_named) => {
                        let fields_idents = fields_named.named.iter().map(|field| {
                            let field_ident = field
                                .ident
                                .clone()
                                .expect("some of named fields doesnt have ident");
                            quote::quote! { #field_ident }
                        });
                        let where_was_one_or_many_vec = fields_named.named.iter().map(|field| {
                            let field_ident = field
                                .ident
                                .clone()
                                .expect("some of named fields doesnt have ident");
                            quote::quote! {
                                #field_ident
                                .get_where_was_one_or_many()
                                .into_vec()
                                .into_iter()
                                .for_each(|w| {
                                    vec.push(w);
                                });
                            }
                        });
                        quote::quote! {
                            #ident::#variant_ident{
                                #(#fields_idents,)*
                            } => {
                                let mut vec = Vec::new();
                                #(#where_was_one_or_many_vec)*
                                tufa_common::where_was::WhereWasOneOrMany::Many(vec)
                            }
                        }
                    }
                    syn::Fields::Unnamed(_) => quote::quote! {
                        #ident::#variant_ident(e) => e.get_where_was_one_or_many()
                    },
                }
            });
            let gen = quote::quote! {
                // impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for #ident {
                //     fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
                //         match self {
                //             #(#variants,)*
                //         }
                //     }
                // }
            };
            gen.into()
        }
    }
}
