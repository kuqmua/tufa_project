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
                        
                        // let one = fields_named.clone();
                        // let two = fields_named.clone();
                        // let mut scopes = one.named.iter().map(|_| String::from("{}\n")).fold(
                        //     String::from(""),
                        //     |mut acc, elem| {
                        //         acc.push_str(&elem);
                        //         acc
                        //     },
                        // );
                        // if !scopes.is_empty() {
                        //     scopes.pop();
                        // }
                        // let fields_idents = two.named.iter().map(|field| {
                        //     let field_ident = field
                        //         .ident
                        //         .clone()
                        //         .expect("some of named fields doesnt have ident");
                        //     quote::quote! { #field_ident }
                        // });
                        // let fields_idents_map = fields_named.named.iter().map(|field| {
                        //     let field_ident = field
                        //         .ident
                        //         .clone()
                        //         .expect("some of named fields doesnt have ident");
                        //     quote::quote! { #field_ident.get_source() }
                        // });
                        quote::quote! {
                            // #ident::#variant_ident{
                            //     #(#fields_idents,)*
                            // } => {
                            //     format!(
                            //         #scopes,
                            //         #(#fields_idents_map,)*
                            //     )
                            // }
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
                                                                .map(|error| format!("{} ,", error.get_source()))
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
                                                        #ident::#variant_ident(error) => format!("{}", error.get_source())
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
