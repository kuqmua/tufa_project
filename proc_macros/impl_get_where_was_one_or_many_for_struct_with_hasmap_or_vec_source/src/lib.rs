#[proc_macro_derive(ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource)]
pub fn derive_impl_get_where_was_one_or_many_for_struct_with_hasmap_or_vec_source(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input)
        .expect("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource syn::parse(input) failed");
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Union(_) => {
            panic!("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource only work on structs!")
        }
        syn::Data::Enum(_) => {
            panic!("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource only work on structs!")
        }
        syn::Data::Struct(data_struct) => {
            match data_struct.fields {
                syn::Fields::Named(fields_named) => {
                    match fields_named.named.len() {
                        2 => {
                            let source_field_ident = fields_named.named[0].ident.clone().expect("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource - there is no first field ident!");
                            if format!("{}", source_field_ident) != *"source" {
                                panic!("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource - no 'source'-named field found!");
                            }
                            match fields_named.named[0].ty.clone() {
                                // syn::Type::Array(type_array) => {},
                                // syn::Type::BareFn(TypeBareFn),
                                // syn::Type::Group(TypeGroup),
                                // syn::Type::ImplTrait(TypeImplTrait),
                                // syn::Type::Infer(TypeInfer),
                                // syn::Type::Macro(TypeMacro),
                                // syn::Type::Never(TypeNever),
                                // syn::Type::Paren(TypeParen),
                                // syn::Type::Ptr(TypePtr),
                                // syn::Type::Reference(TypeReference),
                                // syn::Type::Slice(TypeSlice),
                                // syn::Type::TraitObject(TypeTraitObject),
                                // syn::Type::Tuple(TypeTuple),
                                // syn::Type::Verbatim(TokenStream),
                                syn::Type::Path(type_path) => {
                                    match type_path.path.segments.len() {
                                        1 => {
                                            let possible_vec_or_hashmap_ident_as_string = format!("{}", type_path.path.segments[0].ident);
                                            if possible_vec_or_hashmap_ident_as_string == *"Vec" {
                                                let gen = quote::quote! {
                                                    impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for #ident
                                                    {
                                                        fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
                                                            let mut vec = Vec::new();
                                                            self.source.iter().for_each(|e| {
                                                                e.get_where_was_one_or_many()
                                                                    .into_vec()
                                                                    .into_iter()
                                                                    .for_each(|w| {
                                                                        vec.push(w);
                                                                    });
                                                            });
                                                            vec.push(tufa_common::where_was::WhereWasWithAddition {
                                                                additional_info: None,
                                                                where_was: self.where_was.clone(),
                                                            });
                                                            tufa_common::where_was::WhereWasOneOrMany::Many(vec)
                                                        }
                                                    }
                                                };
                                                gen.into()
                                            }
                                            else if possible_vec_or_hashmap_ident_as_string == *"HashMap" {
                                                let gen = quote::quote! {
                                                    impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for #ident
                                                    {
                                                        fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
                                                            let mut vec = Vec::new();
                                                            self.source.iter().for_each(|(key, error)| {
                                                                error
                                                               .get_where_was_one_or_many()
                                                               .into_vec()
                                                               .into_iter()
                                                               .for_each(|mut w| {
                                                                    w.additional_info = Some(format!("{}", key)); //todo
                                                                    vec.push(w);
                                                                });
                                                            });
                                                            vec.push(tufa_common::where_was::WhereWasWithAddition {
                                                                additional_info: None,
                                                                where_was: self.where_was.clone(),
                                                            });
                                                            tufa_common::where_was::WhereWasOneOrMany::Many(vec)
                                                        }
                                                    }
                                                };
                                                gen.into()
                                            }
                                            else {
                                                panic!("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource only work on Type::Path!");
                                            }
                                        }
                                        _ => panic!("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource only work with type_path.path.segments.len() == 1!"),
                                    }
                                },
                                _ => panic!("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource only work on Type::Path!")
                            }
                        }
                        _ => panic!("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource only work on structs with 2 named fields!")
                    }
                },
                // syn::Fields::Unnamed(_) => {},
                // syn::Fields::Unit(_) => {},
                _ => panic!("ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSource only work with syn::Fields::Named!")
            }
        }
    }
}
