#[proc_macro_derive(ImplGetSourceForStructWithMethod)]
pub fn derive_impl_get_source_for_struct_with_method(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ImplGetSourceForStructWithMethod syn::parse(input) failed");
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Union(_) => {
            panic!("ImplGetSourceForStructWithMethod only work on structs!")
        }
        syn::Data::Enum(_) => {
            panic!("ImplGetSourceForStructWithMethod only work on structs!")
        }
        syn::Data::Struct(data_struct) => {
            match data_struct.fields {
                syn::Fields::Named(fields_named) => {
                    match fields_named.named.len() {
                        2 => {
                            let source_field_ident = fields_named.named[0].ident.clone().expect("ImplGetSourceForStructWithMethod - there is no first field ident!");
                            if format!("{}", source_field_ident) != *"source" {
                                panic!("ImplGetSourceForStructWithMethod - no 'source'-named field found!");
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
                                                    impl tufa_common::traits::get_source::GetSource for #ident {
                                                        self.source.get_source()
                                                    }
                                                };
                                                gen.into()
                                            }
                                            else if possible_vec_or_hashmap_ident_as_string == *"HashMap" {
                                                let gen = quote::quote! {
                                                    impl tufa_common::traits::get_source::GetSource for #ident {
                                                        let mut formatted = self
                                                        .source
                                                        .iter()
                                                        .map(|(key, error)| format!("{} {},", key, error.get_source()))
                                                        .collect::<Vec<String>>()
                                                        .iter()
                                                        .fold(String::from(""), |mut acc, elem| {
                                                            acc.push_str(elem);
                                                            acc
                                                        });
                                                        if !formatted.is_empty() {
                                                            formatted.pop();
                                                        }
                                                        formatted
                                                    }
                                                };
                                                gen.into()
                                            }
                                            else {
                                                let gen = quote::quote! {
                                                    impl tufa_common::traits::get_source::GetSource for #ident {
                                                        let mut formatted = self
                                                        .source
                                                        .iter()
                                                        .map(|error| format!("{},", error.get_source()))
                                                        .fold(String::from(""), |mut acc, elem| {
                                                            acc.push_str(&elem);
                                                            acc
                                                        });
                                                        if !formatted.is_empty() {
                                                            formatted.pop();
                                                        }
                                                        formatted
                                                    }
                                                };
                                                gen.into()
                                            }
                                        }
                                        _ => panic!("ImplGetSourceForStructWithMethod only work with type_path.path.segments.len() == 1!"),
                                    }
                                },
                                _ => panic!("ImplGetSourceForStructWithMethod only work on Type::Path!")
                            }
                        }
                        _ => panic!("ImplGetSourceForStructWithMethod only work on structs with 2 named fields!")
                    }
                }
                // syn::Fields::Unnamed(_) => {},
                // syn::Fields::Unit(_) => {},
                _ => panic!("ImplGetSourceForStructWithMethod only work with syn::Fields::Named!"),
            }
        }
    }
}
