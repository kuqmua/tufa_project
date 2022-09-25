#[proc_macro_derive(ImplGetSourceForEnumWithMethod)]
pub fn derive_impl_get_source_for_enum_with_method(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ImplGetSourceForEnumWithMethod syn::parse(input) failed");
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Union(_) => panic!("ImplGetSourceForEnumWithMethod only work on enums!"),
        syn::Data::Struct(_) => panic!("ImplGetSourceForEnumWithMethod only work on enums!"),
        syn::Data::Enum(data_enum) => {
            let variants = data_enum.variants.into_iter().map(|v| {
                let variant_ident = v.ident;
                match v.fields {
                    syn::Fields::Unit => panic!(
                        "ImplGetSourceForEnumWithMethod still not work with syn::Fields::Unit"
                    ),
                    syn::Fields::Named(fields_named) => {
                        let fields_list = fields_named.named.iter().map(|field_named| {
                            let field_ident = &field_named.ident;
                            quote::quote! {
                                #field_ident
                            }
                        });
                        let fields_logic = fields_named.named.iter().map(|field_named| {
                            let field_ident = &field_named.ident;
                                match &field_named.ty {
                                    // Array(TypeArray),
                                    // BareFn(TypeBareFn),
                                    // Group(TypeGroup),
                                    // ImplTrait(TypeImplTrait),
                                    // Infer(TypeInfer),
                                    // Macro(TypeMacro),
                                    // Never(TypeNever),
                                    // Paren(TypeParen),
                                    // Ptr(TypePtr),
                                    // Reference(TypeReference),
                                    // Slice(TypeSlice),
                                    // TraitObject(TypeTraitObject),
                                    // Tuple(TypeTuple),
                                    // Verbatim(TokenStream),
                                    syn::Type::Path(type_path) => {
                                        match type_path.path.segments.len() {
                                            1 => {
                                                let segment_ident = format!("{}", type_path.path.segments[0].ident);
                                                if segment_ident == *"Vec" {
                                                    quote::quote! {
                                                        let mut #field_ident = #field_ident
                                                            .iter()
                                                            .map(|error| format!("{} ,", error.get_source()))
                                                            .fold(String::from(""), |mut acc, elem| {
                                                                acc.push_str(&elem);
                                                                acc
                                                            }
                                                        );
                                                        if !#field_ident.is_empty() {
                                                            #field_ident.pop();
                                                            #field_ident.pop();
                                                        }
                                                        #field_ident = format!("[{}]", #field_ident);
                                                    }
                                                }
                                                else if segment_ident == *"HashMap" {
                                                    quote::quote! {
                                                        let mut #field_ident = #field_ident
                                                            .iter()
                                                            .map(|(key, error)| format!("[{} {}],", key, error.get_source()))
                                                            .fold(String::from(""), |mut acc, elem| {
                                                                acc.push_str(&elem);
                                                                acc
                                                            }
                                                            );
                                                            if !#field_ident.is_empty() {
                                                                #field_ident.pop();
                                                            }
                                                            let #field_ident = format!("[{}]", #field_ident);
                                                        }
                                                }
                                                else if segment_ident.contains("Enum") {
                                                    quote::quote! {
                                                        let #field_ident = #field_ident.get_source();
                                                    }
                                                }
                                                else {
                                                    panic!("ImplGetSourceForEnumWithMethod only work on enums with HashMap<Key, StructOrEnumWithGetSourceMethod>, Vec<StructOrEnumWithGetSourceMethod> or EnumWithGetSourceMethod which name contains \"Enum\" keyword!")
                                                }
                                            },
                                            _ => panic!("ImplGetSourceForEnumWithMethod only work on enums with type_path.segments.len() == 1!")
                                        }
                                    },
                                    _ => panic!("ImplGetSourceForEnumWithMethod only work on enums with Path(type_path)!")
                                }
                        });
                        let mut fields_bracket_string = fields_named.named.iter().map(|_| {
                            String::from("{} ")
                        })
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !fields_bracket_string.is_empty() {
                            fields_bracket_string.pop();
                        }
                        fields_bracket_string = format!("[{}]", fields_bracket_string);
                        let fields_arguments = fields_named.named.iter().map(|field_named| {
                            let field_ident = &field_named.ident;
                            quote::quote! {
                                #field_ident
                            }
                        });
                        quote::quote! {
                             #ident::#variant_ident{
                                #(#fields_list,)*
                             } => {
                                #(#fields_logic)*
                                format!(#fields_bracket_string, #(#fields_arguments,)*)
                             }
                        }
                    }
                    syn::Fields::Unnamed(unnamed) => {
                        match unnamed.unnamed.len() {
                            1 => {
                                match &unnamed.unnamed[0].ty {
                                    // Array(TypeArray),
                                    // BareFn(TypeBareFn),
                                    // Group(TypeGroup),
                                    // ImplTrait(TypeImplTrait),
                                    // Infer(TypeInfer),
                                    // Macro(TypeMacro),
                                    // Never(TypeNever),
                                    // Paren(TypeParen),
                                    // Ptr(TypePtr),
                                    // Reference(TypeReference),
                                    // Slice(TypeSlice),
                                    // TraitObject(TypeTraitObject),
                                    // Tuple(TypeTuple),
                                    // Verbatim(TokenStream),
                                    syn::Type::Path(type_path) => {
                                        match type_path.path.segments.len() {
                                            1 => {
                                                let segment_ident = format!("{}", type_path.path.segments[0].ident);
                                                if segment_ident == *"Vec" {
                                                    quote::quote! {
                                                        #ident::#variant_ident(error_vec) => {
                                                            let mut formatted = error_vec
                                                                .iter()
                                                                .map(|error| error.get_source())
                                                                .fold(String::from(""), |mut acc, elem| {
                                                                        acc.push_str(&elem);
                                                                        acc
                                                                }
                                                            );
                                                            if !formatted.is_empty() {
                                                                formatted.pop();
                                                                formatted.pop();
                                                            }
                                                            format!("[{}]", formatted)
                                                        }
                                                    }
                                                }
                                                else if segment_ident == *"HashMap" {
                                                    quote::quote! {
                                                        #ident::#variant_ident(error_hashmap) => {
                                                            let mut formatted = error_hashmap
                                                                .iter()
                                                                .map(|(key, error)| format!("[{} {}],", key, error.get_source()))
                                                                .fold(String::from(""), |mut acc, elem| {
                                                                        acc.push_str(&elem);
                                                                        acc
                                                                }
                                                            );
                                                            if !formatted.is_empty() {
                                                                formatted.pop();
                                                            }
                                                            format!("[{}]", formatted)
                                                        }
                                                    }
                                                }
                                                else {
                                                    quote::quote! {
                                                        #ident::#variant_ident(error) => error.get_source()
                                                    }
                                                }
                                            },
                                            _ => panic!("ImplGetSourceForEnumWithMethod only work on enums with type_path.segments.len() == 1!")
                                        }
                                    },
                                    _ => panic!("ImplGetSourceForEnumWithMethod only work on enums with Path(type_path)!")
                                }
                            }
                            _ => panic!("ImplGetSourceForEnumWithMethod only work on enums with unnamed.len() == 1!")
                        }
                    }
                }
            });
            let gen = quote::quote! {
                impl tufa_common::traits::get_source::GetSource for #ident {
                    fn get_source(&self) -> String {
                        match self {
                            #(#variants,)*
                        }
                    }
                }
            };
            gen.into()
        }
    }
}
