#[proc_macro_derive(ErrorDisplay)]
pub fn derive_error_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("ErrorDisplay syn::parse(input) failed");
    let ident = &ast.ident;
    let gen = match ast.data {
        syn::Data::Struct(_) => quote::quote! {
            if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled {
                write!(
                    f,
                    "{}\n{}\n{}",
                    self.where_was.file_line_column(),//_with_readable_time
                    self.where_was.github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data),//_with_readable_time
                    self.source
                )
            } else if CONFIG.is_show_source_place_enabled {
                write!(
                    f,
                    "{}\n{}",
                    self.where_was.file_line_column(),//_with_readable_time
                    self.source
                )
            } else if CONFIG.is_show_github_source_place_enabled {
                write!(
                    f,
                    "{}\n{}",
                    self.where_was.github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data),//_with_readable_time
                    self.source
                )
            } else {
                write!(f, "{}", self.source)
            }
        },
        syn::Data::Enum(data_enum) => {
            let generated = data_enum.variants.into_iter().map(|v| {
                let variant = v.ident;
                quote::quote! {
                    #ident::#variant { source, where_was } => {
                        if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled
                        {
                            write!(
                                f,
                                "{}\n{}\n{}",
                                where_was.file_line_column(),//_with_readable_time
                                where_was.github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data),//_with_readable_time
                                source
                            )
                        } else if CONFIG.is_show_source_place_enabled {
                            write!(
                                f,
                                "{}\n{}",
                                where_was.file_line_column(),//_with_readable_time
                                source
                            )
                        } else if CONFIG.is_show_github_source_place_enabled {
                            write!(
                                f,
                                "{}\n{}",
                                where_was.github_file_line_column(&crate::lazy_static::git_info::GIT_INFO.data),//_with_readable_time
                                source
                            )
                        } else {
                            write!(f, "{}", source)
                        }
                    }
                }
            });
            quote::quote! {
                match self {
                    #(#generated)*
                }
            }
        }
        _ => panic!("ErrorDisplay only works on enum and struct"),
    };
    quote::quote! {
        impl fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                #gen
            }
        }
    }
    .into()
}
