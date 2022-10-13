#[proc_macro_derive(ProviderKindFromConfigTrait)]
pub fn derive_provider_kind_from_config(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ProviderKindFromConfigTrait syn::parse(input) failed"); //if need to print ast use syn = { version = "1.0.75", features = ["extra-traits"]} instead of syn="1.0.75"
    let ident: &syn::Ident = &ast.ident;
    let data: syn::Data = ast.data;
    let function_vec_idents: Vec<(syn::Ident, syn::ReturnType)>;
    let trait_handle = quote::quote! {
        pub trait ProviderKindFromConfigTrait {
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_mongo_initialization_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_mongo_write_error_logs_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_mongo_cleaning_warning_logs_db_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_mongo_cleaning_warning_logs_db_collections_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_mongo_link_parts_randomize_order_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_postgres_initialization_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_write_error_logs_in_local_folder_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_cleaning_warning_logs_directory_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn check_link(&self) -> &'static str;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_dbs_initialization_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_prints_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_warning_high_prints_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_warning_low_prints_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_success_prints_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_partial_success_prints_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_error_prints_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_time_measurement_prints_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_info_prints_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn is_links_limit_enabled(&self) -> bool;
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn links_limit(&self) -> usize;
        }
    };
    let token_stream: proc_macro::TokenStream = trait_handle
        .to_string()
        .parse()
        .expect("cannot parse file into proc_macro::TokenStream");
    let trait_ast: syn::ItemTrait =
        syn::parse(token_stream).expect("cannot parse token_stream from file into syn::ItemTrait");
    let trait_name = trait_ast.ident;
    function_vec_idents = trait_ast
        .items
        .iter()
        .filter_map(|trait_item| match trait_item {
            syn::TraitItem::Method(trait_item_method) => Some((
                trait_item_method.sig.ident.clone(),
                trait_item_method.sig.output.clone(),
            )),
            _ => None,
        })
        .collect();
    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = data {
        variants
    } else {
        panic!("not a valid data type for this proc macro");
    };
    let mut function_quote_vec_ident = Vec::with_capacity(function_vec_idents.len());
    for (function_name_ident, output) in function_vec_idents {
        let mut is_str = false;
        if let syn::ReturnType::Type(_, box_type) = &output {
            if let syn::Type::Reference(type_reference) = &**box_type {
                if let syn::Type::Path(reference_type_path) = &*type_reference.elem {
                    for i in &reference_type_path.path.segments {
                        if i.ident == "str" {
                            is_str = true;
                        }
                    }
                }
            }
        }
        let variants_for_quote = variants.iter().map(|variant| {
            use convert_case::Casing;
            let variant_name = &variant.ident;
            let config_field_name = syn::Ident::new(
                &format!(
                    "{}_{}",
                    function_name_ident
                        .to_string()
                        .to_case(convert_case::Case::Snake)
                        .to_lowercase(),
                    variant_name
                        .to_string()
                        .to_case(convert_case::Case::Snake)
                        .to_lowercase()
                ),
                variant_name.span(),
            );
            if is_str {
                quote::quote! {
                    #ident::#variant_name => &crate::lazy_static::config::CONFIG.#config_field_name
                }
            } else {
                quote::quote! {
                        #ident::#variant_name => crate::lazy_static::config::CONFIG.#config_field_name
                }
            }
        });
        function_quote_vec_ident.push(quote::quote! {
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn #function_name_ident(&self) #output {
                match self {
                   #(#variants_for_quote,)*
                }
            }
        });
    }
    let generated = quote::quote! {
        #trait_handle
        impl #trait_name for #ident {
            #(#function_quote_vec_ident)*
        }
    };
    generated.into()
}
