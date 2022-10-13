#[proc_macro_derive(ImplGetWhereWasOneOrManyWithMethod)]
pub fn derive_impl_get_where_was_one_or_many_with_method(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ImplGetWhereWasOneOrManyWithMethod syn::parse(input) failed");
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Union(_) => {
            panic!("ImplGetWhereWasOneOrManyWithMethod only work on structs!")
        }
        syn::Data::Enum(data_enum) => {
            let variants = data_enum.variants.into_iter().map(|v| {
                let variant_ident = v.ident;
                match v.fields {
                    syn::Fields::Unit => {
                        panic!(
                            "ImplGetWhereWasOneOrManyWithMethod still not work with syn::Fields::Unit"
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
                impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for #ident {
                    fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
                        match self {
                            #(#variants,)*
                        }
                    }
                }
            };
            gen.into()
        }
        syn::Data::Struct(data_struct) => {
            match data_struct.fields {
                syn::Fields::Named(fields_named) => {
                    match fields_named.named.len() {
                        2 => {
                            let source_field_ident = fields_named.named[0].ident.clone().expect("ImplGetWhereWasOneOrManyWithMethod - there is no first field ident!");
                            if format!("{}", source_field_ident) != *"source" {
                                panic!("ImplGetWhereWasOneOrManyWithMethod - no 'source'-named field found!");
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
                                                let gen = quote::quote! {
                                                    impl tufa_common::traits::get_where_was_one_or_many::GetWhereWasOneOrMany for #ident
                                                    {
                                                        fn get_where_was_one_or_many(&self) -> tufa_common::where_was::WhereWasOneOrMany {
                                                            let mut vec = Vec::new();
                                                            self.source
                                                            .get_where_was_one_or_many()
                                                            .into_vec()
                                                            .into_iter()
                                                            .for_each(|w| {
                                                              vec.push(w);
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
                                        }
                                        _ => panic!("ImplGetWhereWasOneOrManyWithMethod only work with type_path.path.segments.len() == 1!"),
                                    }
                                },
                                _ => panic!("ImplGetWhereWasOneOrManyWithMethod only work on Type::Path!")
                            }
                        }
                        _ => panic!("ImplGetWhereWasOneOrManyWithMethod only work on structs with 2 named fields!")
                    }
                }
                // syn::Fields::Unnamed(_) => {},
                // syn::Fields::Unit(_) => {},
                _ => {
                    panic!("ImplGetWhereWasOneOrManyWithMethod only work with syn::Fields::Named!")
                }
            }
        }
    }
}
