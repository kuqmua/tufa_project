mod from_log_and_return_error;
mod generate_postgres_transaction;
mod type_variants_from_request_response_generator;

//todo generate for each create update delete body length checked and for path query headers too
//todo how to write filter logic for sqlx rust postgresql types?
//todo decide where to do error log (maybe add in some places)
//todo validate uuid
//todo add regex filter to query parameters - now supports only in body variants
//todo regex filter support only for string-like types postgresql
//todo generate route what will return columns of the table and their rust and postgersql types
//todo - check if fields for filter are unique in the input array
//todo created at and updated at fields
//todo attributes for activation generation crud methods(like generate create, update_one, delete_one)
//todo authorization for returning concrete error or just minimal info(user role)
//todo generate rules and roles
//todo unique(meaning not primary key unique column) and nullable support
//todo add check on max postgresql bind elements
//todo add route name as argument for macro - generation constant and add to generation logic
//todo make sqlx macros instead of just queries?
//todo support for arrays as column values
//todo maybe add unnest sql types?
//todo maybe add unnest to filter parameters if its array ?
//todo swagger ui https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs
//todo test #[derive(generate_postgresql_crud::GeneratePostgresqlCrud)] for few structs in one module - will it work? fix if not will not work
//todo derive utoipa::ToSchema for what? original structs or with serialize deserialize?
//todo need to add utoipa::ToSchema annotation #[schema(value_type = YourToSchemaTraitImplStruct)] for all fields
//todo remove useless derives like useless serde::Serialize and Deserialize
//todo maybe add private generated code in specific mod in case of some compilation error - to show info like "no struct BlaBla found in mod ..." ?
//todo add ident prefix to public functions structs and other
//todo maybe generate compisite type for user defined type https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
//todo read again some interesting thoughts about sql as api https://habr.com/ru/companies/timeweb/articles/798937/
//todo sqlx query! macros not in actual api, but just unused function just for compile time query check
//todo reexport all crates what logic depends on (from crates.io) (use of undeclared crate or module `time`)
//todo write checks on fields one by one with early return. not need to check all and create combinatorial complexity
//todo add transaction isolation level (see postgresql docs)
//todo check on postgresql max length value of type
//todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
//todo update one and delete one must be a transaction

#[proc_macro_attribute]
pub fn create_many_additional_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn create_one_additional_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_one_additional_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_many_additional_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_one_additional_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_many_additional_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_one_additional_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_many_additional_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn common_additional_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_attribute]
pub fn create_many_additional_route_logic(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn create_one_additional_route_logic(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_many_additional_route_logic(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_one_additional_route_logic(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_many_additional_route_logic(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_one_additional_route_logic(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_many_additional_route_logic(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_one_additional_route_logic(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn common_additional_route_logic(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_derive(GeneratePostgresqlCrud)]
pub fn generate_postgresql_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GeneratePostgresqlCrud";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", syn_derive_input.data);
    let ident = &syn_derive_input.ident;
    let ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &ident.to_string()
    );
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let table_name_stringified = pluralizer::pluralize(&ident_snake_case_stringified, 2, false);
    let table_name_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
        &table_name_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    #[derive(Debug, Clone)]
    struct SynFieldWithAdditionalInfo<'a> {
        field: &'a syn::Field,
        field_ident: &'a syn::Ident,
        rust_sqlx_map_to_postgres_type_variant:
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant, //todo maybe not need to add here
        _maybe_generic_token_stream: proc_macro2::TokenStream, //todo maybe not need to add here
        // path_token_stream: proc_macro2::TokenStream,
        original_type_token_stream: proc_macro2::TokenStream,
        inner_type_token_stream: proc_macro2::TokenStream,
        inner_type_with_serialize_deserialize_token_stream: proc_macro2::TokenStream,
        inner_type_with_serialize_deserialize_error_named_token_stream: proc_macro2::TokenStream,
        // where_inner_type_with_serialize_deserialize_handle_stringified: std::string::String,
        where_inner_type_token_stream: proc_macro2::TokenStream,
        where_inner_type_with_serialize_deserialize_token_stream: proc_macro2::TokenStream,
    }   
    impl<'a> std::convert::TryFrom<&'a syn::Field> for SynFieldWithAdditionalInfo<'a> {
        type Error = std::string::String;
        fn try_from(value: &'a syn::Field) -> Result<Self, Self::Error> {
            let name = "SynFieldWithAdditionalInfo from syn::Field error";
            let field_ident = match value.ident.as_ref() {
                Some(value) => value,
                None => {
                    return Err(format!("{name} field ident is none"));
                }
            };
            let (rust_sqlx_map_to_postgres_type_variant, maybe_generic_token_stream) = match &value.ty {
                syn::Type::Path(value) => {
                    if value.path.segments.len() == 2 {
                        let first = match value.path.segments.first() {
                            Some(value) => value,
                            None => {
                                return Err(std::string::String::from("no first value in punctuated"));
                            }
                        };
                        if first.ident != postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE {
                            return Err(format!("{name} field_type is not syn::Type::Path"));
                        }
                        match first.arguments {
                            syn::PathArguments::None => (),
                            syn::PathArguments::AngleBracketed(_) | 
                            syn::PathArguments::Parenthesized(_) => {
                                return Err(format!("{name} value.path().segments[0].arguments != syn::PathArguments::None"));
                            }
                        }
                        let second_element = match value.path.segments.iter().nth(1) {
                            Some(value) => value,
                            None => {
                                return Err(std::string::String::from("no second element"));
                            }
                        };
                        let rust_sqlx_map_to_postgres_type_variant =
                            match <postgresql_crud_common::RustSqlxMapToPostgresTypeVariant as std::str::FromStr>::from_str(
                                &second_element.ident.to_string(),
                            ) {
                                Ok(value) => value,
                                Err(error) => {
                                    return Err(format!("{name} RustSqlxMapToPostgresTypeVariant::try_from failed {error}"));
                                },
                            };
                        let maybe_generic_token_stream = match &second_element.arguments {
                            syn::PathArguments::None => quote::quote! {},
                            syn::PathArguments::AngleBracketed(value) => {
                                quote::quote! {#value} //< test_mod :: Something >
                            }
                            syn::PathArguments::Parenthesized(_) => {
                                return Err(format!("{name} does not support syn::PathArguments::Parenthesized"));
                            }
                        };
                        (
                            rust_sqlx_map_to_postgres_type_variant,
                            maybe_generic_token_stream,
                        )
                    }
                    else {
                        return Err(std::string::String::from("value.path.segments.len() != 2"));
                    }
                }
                syn::Type::Array(_) | 
                syn::Type::BareFn(_) | 
                syn::Type::Group(_) | 
                syn::Type::ImplTrait(_) | 
                syn::Type::Infer(_) | 
                syn::Type::Macro(_) | 
                syn::Type::Never(_) | 
                syn::Type::Paren(_) | 
                syn::Type::Ptr(_) | 
                syn::Type::Reference(_) | 
                syn::Type::Slice(_) | 
                syn::Type::TraitObject(_) | 
                syn::Type::Tuple(_) | 
                syn::Type::Verbatim(_) => {
                    return Err(format!("{name} field_type is not syn::Type::Path"));
                }
                _ => {
                    return Err(format!("{name} field_type is not syn::Type::Path (exhaustive)"));
                },
            };
            // let path_token_stream = {
            //     let value = &rust_sqlx_map_to_postgres_type_variant.get_path_stringified(); //todo generic for json
            //     value.parse::<proc_macro2::TokenStream>()
            //     .unwrap_or_else(|_| panic!("{name} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            // };
            let original_type_token_stream = {
                let value = &rust_sqlx_map_to_postgres_type_variant.get_original_type_stringified(""); //todo generic for json
                match value.parse::<proc_macro2::TokenStream>() {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(format!("{name} {value} {} {error:#?}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                    }
                }
            };
            let inner_type_token_stream = {
                let value = &rust_sqlx_map_to_postgres_type_variant.get_inner_type_stringified(""); //todo generic for json
                match value.parse::<proc_macro2::TokenStream>() {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(format!("{name} {value} {} {error:#?}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                    }
                }
            };
            let inner_type_with_serialize_deserialize_token_stream = {
                let value = &rust_sqlx_map_to_postgres_type_variant.get_inner_type_with_serialize_deserialize_stringified(""); //todo generic for json
                match value.parse::<proc_macro2::TokenStream>() {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(format!("{name} {value} {} {error:#?}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                    }
                }
            };
            let inner_type_with_serialize_deserialize_error_named_token_stream = {
                let value = &rust_sqlx_map_to_postgres_type_variant.get_inner_type_with_serialize_deserialize_error_named_stringified(""); //todo generic for json
                match value.parse::<proc_macro2::TokenStream>() {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(format!("{name} {value} {} {error:#?}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                    }
                }
            };
            // let where_inner_type_with_serialize_deserialize_handle_stringified = rust_sqlx_map_to_postgres_type_variant.get_where_inner_type_with_serialize_deserialize_handle_stringified("");
            let where_inner_type_token_stream = {
                let value = &rust_sqlx_map_to_postgres_type_variant.get_where_inner_type_stringified("");
                match value.parse::<proc_macro2::TokenStream>() {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(format!("{name} {value} {} {error:#?}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                    }
                }
            };
            let where_inner_type_with_serialize_deserialize_token_stream = {
                let value = &rust_sqlx_map_to_postgres_type_variant.get_where_inner_type_with_serialize_deserialize_stringified("");//todo json generics
                match value.parse::<proc_macro2::TokenStream>() {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(format!("{name} {value} {} {error:#?}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                    }
                }
            };
            Ok(Self {
                field: value,
                field_ident,
                rust_sqlx_map_to_postgres_type_variant, //todo maybe not need to add here
                _maybe_generic_token_stream: maybe_generic_token_stream, //todo maybe not need to add here
                // path_token_stream,
                original_type_token_stream,
                inner_type_token_stream,
                inner_type_with_serialize_deserialize_token_stream,
                inner_type_with_serialize_deserialize_error_named_token_stream,
                // where_inner_type_with_serialize_deserialize_handle_stringified,
                where_inner_type_token_stream,
                where_inner_type_with_serialize_deserialize_token_stream,
            })
        }
    }
    let syn_field_with_additional_info_fields_named = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            fields_named.named.iter()
                .map(|element| SynFieldWithAdditionalInfo::try_from(element).unwrap_or_else(|error|panic!("SynFieldWithAdditionalInfo::try_from(element) failed {error}")))
                .collect::<std::vec::Vec<SynFieldWithAdditionalInfo<'_>>>()
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Named");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let primary_key_syn_field_with_additional_info = {
        let mut primary_key_field_option = None;
        for element in &syn_field_with_additional_info_fields_named {
            match &element.field.ty {
                syn::Type::Path(value) => {
                    if value.path.segments.len() == 2 {
                        assert!(value.path.segments.first().expect("no first value in punctuated").ident == postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, "{proc_macro_name_upper_camel_case_ident_stringified} field_type is not syn::Type::Path");
                        match <postgresql_crud_common::RustSqlxMapToPostgresTypeVariant as std::str::FromStr>::from_str(&value.path.segments.iter().nth(1).expect("no second element").ident.to_string()) {
                            Ok(value) => {
                                if postgresql_crud_common::RustSqlxMapToPostgresTypeVariantPrimaryKey::try_from(&value).is_ok() {
                                    match primary_key_field_option {
                                        Some(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} must have one PrimaryKey"),
                                        None => {
                                            primary_key_field_option = Some(element.clone());
                                        },
                                    }
                                }
                            },
                            Err(error) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} RustSqlxMapToPostgresTypeVariant::try_from failed {error}")
                        }
                    }
                    else {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} field_type is not syn::Type::Path")
                    }
                },
                _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} field_type is not syn::Type::Path")
            }
        }
        primary_key_field_option.map_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} no primary_key_field_option"), |value| value)
    };
    let primary_key_field = &primary_key_syn_field_with_additional_info.field;
    let primary_key_field_ident = &primary_key_syn_field_with_additional_info.field_ident;
    let primary_key_rust_sqlx_map_to_postgres_type_variant = &primary_key_syn_field_with_additional_info.rust_sqlx_map_to_postgres_type_variant; 
    let primary_key_original_type_token_stream = &primary_key_syn_field_with_additional_info.original_type_token_stream;
    let primary_key_inner_type_token_stream = &primary_key_syn_field_with_additional_info.inner_type_token_stream;
    let primary_key_inner_type_with_serialize_deserialize_token_stream = &primary_key_syn_field_with_additional_info.inner_type_with_serialize_deserialize_token_stream;
    let primary_key_inner_type_with_serialize_deserialize_error_named_token_stream = &primary_key_syn_field_with_additional_info.inner_type_with_serialize_deserialize_error_named_token_stream;
    fn syn_ident_to_upper_camel_case_stringified(value: &syn::Ident) -> std::string::String {
        proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value.to_string())
    }
    let syn_ident_to_upper_camel_case_token_stream = |value: &syn::Ident| -> proc_macro2::TokenStream {
        let value = syn_ident_to_upper_camel_case_stringified(value);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {proc_macro_name_upper_camel_case_ident_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let primary_key_field_ident_upper_camel_case_token_stream = syn_ident_to_upper_camel_case_token_stream(primary_key_field_ident);
    let primary_key_field_ident_token_stream = quote::quote!{#primary_key_field_ident};
    let std_vec_vec_primary_key_inner_type_token_stream = quote::quote!{std::vec::Vec<#primary_key_inner_type_token_stream>};
    let std_string_string = token_patterns::StdStringString;
    let syn_field_with_additional_info_fields_named_excluding_primary_key = syn_field_with_additional_info_fields_named
        .clone()
        .into_iter()
        .filter(|element| element.field != *primary_key_field)
        .collect::<std::vec::Vec<SynFieldWithAdditionalInfo<'_>>>();
    let fields_named_len = syn_field_with_additional_info_fields_named.len();
    assert!(fields_named_len > 1, "{proc_macro_name_upper_camel_case_ident_stringified} false = fields_named.len() > 1");
    let syn_field_with_additional_info_fields_named_excluding_primary_key_len = syn_field_with_additional_info_fields_named_excluding_primary_key.len();
    let fields_named_from_or_try_from = {
        let mut value = postgresql_crud_common::FromOrTryFrom::From;
        for element in &syn_field_with_additional_info_fields_named {
            let from_or_try_from = element.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize();
            if from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
                value = from_or_try_from;
                break;
            }
        }
        value
    };
    let primary_key_from_or_try_from = primary_key_rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize();
    let fields_named_excluding_primary_key_from_or_try_from = {
        let mut value = postgresql_crud_common::FromOrTryFrom::From;
        for element in &syn_field_with_additional_info_fields_named_excluding_primary_key {
            let from_or_try_from = element.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize();
            if from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
                value = from_or_try_from;
                break;
            }
        }
        value
    };
    let debug_upper_camel_case = naming_constants::DebugUpperCamelCase;
    let error_snake_case = naming_constants::ErrorSnakeCase;
    let eprintln_error_token_stream = proc_macro_common::eprintln_error_token_stream();
    let serde_serialize = token_patterns::SerdeSerialize;
    let serde_deserialize = token_patterns::SerdeDeserialize;
    let derive_debug_serde_serialize_serde_deserialize = token_patterns::DeriveDebugSerdeSerializeSerdeDeserialize;
    let from_str_upper_camel_case = naming_conventions::FromStrUpperCamelCase;
    let from_str_snake_case = naming_conventions::FromStrSnakeCase;
    let sqlx_row = token_patterns::SqlxRow;
    let struct_options_ident_token_stream = {
        let value = format!(
            "{ident}{}",
            naming_constants::OptionsUpperCamelCase
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let struct_options_token_stream = {
        // let serde_skip_serializing_if_value_attribute_token_stream = quote::quote! {#[serde(skip_serializing_if = "Option::is_none")]};//todo maybe its not correct for nullable\option types
        let field_option_primary_key_token_stream = {
            let inner_type_with_serialize_deserialize_token_stream = &primary_key_inner_type_with_serialize_deserialize_token_stream;
            quote::quote! {
                // #serde_skip_serializing_if_value_attribute_token_stream
                pub #primary_key_field_ident: std::option::Option<postgresql_crud::Value<#inner_type_with_serialize_deserialize_token_stream>>
            }
        };
        let fields_options_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
            let field_vis = &element.field.vis;
            let field_ident = &element.field_ident;
            let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
            quote::quote!{
                // #serde_skip_serializing_if_value_attribute_token_stream
                #field_vis #field_ident: std::option::Option<postgresql_crud::Value<#inner_type_with_serialize_deserialize_token_stream>>
            }
        });
        quote::quote! {
            #derive_debug_serde_serialize_serde_deserialize
            pub struct #struct_options_ident_token_stream {
                #field_option_primary_key_token_stream,
                #(#fields_options_excluding_primary_key_token_stream),*
            }
        }
    };
    let from_snake_case = naming_constants::FromSnakeCase;
    let try_from_snake_case = naming_conventions::TryFromSnakeCase;
    let from_ident_for_ident_options_token_stream = {
        let inner_type_with_serialize_deserialize_token_stream = &primary_key_inner_type_with_serialize_deserialize_token_stream;
        let ident_option_variant_primary_key_token_stream = quote::quote! {
            #primary_key_field_ident: Some(
                postgresql_crud::Value {
                    value: #inner_type_with_serialize_deserialize_token_stream::#from_snake_case(
                        value.#primary_key_field_ident.0
                    )
                }
            ),
        };
        let ident_option_variants_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
            let field_ident = &element.field_ident;
            let inner_type_with_serialize_deserialize_token_stream =
                &element.inner_type_with_serialize_deserialize_token_stream;
            quote::quote! {
                #field_ident: Some(
                    postgresql_crud::Value {
                        value: #inner_type_with_serialize_deserialize_token_stream::#from_snake_case(value.#field_ident.0)
                    }
                )
            }
        });
        quote::quote! {
            impl std::convert::From<#ident> for #struct_options_ident_token_stream {
                fn from(value: #ident) -> Self {
                    Self {
                        #ident_option_variant_primary_key_token_stream
                        #(#ident_option_variants_excluding_primary_key_token_stream),*
                    }
                }
            }
        }
    };
    // println!("{from_ident_for_ident_options_token_stream}");
    let code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence = token_patterns::CodeOccurenceSnakeCaseDoubleDotSpaceErrorOccurenceLibCodeOccurenceCodeOccurence;
    let eo_error_occurence_attribute_token_stream = proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence.to_attribute_view_token_stream();
    let ident_column_upper_camel_case_stringified = format!("{ident}{}", naming_constants::ColumnUpperCamelCase);
    let ident_column_upper_camel_case_token_stream = ident_column_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_column_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let column_token_stream = {
        let variants = syn_field_with_additional_info_fields_named.iter().map(|element| {
            let field_ident_stringified = element.field_ident.to_string();
            let serialize_deserialize_ident_token_stream = proc_macro_common::generate_quotes::token_stream(
                &field_ident_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_ident_upper_camel_case_token_stream = {
                let value = convert_case::Casing::to_case(&field_ident_stringified, convert_case::Case::UpperCamel);
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote! {
                #[serde(rename(serialize = #serialize_deserialize_ident_token_stream, deserialize = #serialize_deserialize_ident_token_stream))]
                #field_ident_upper_camel_case_token_stream
            }
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        let display_variants = syn_field_with_additional_info_fields_named.iter().map(|element| {
            let field_ident_stringified = element.field_ident.to_string();
            let field_ident_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
                &field_ident_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_ident_upper_camel_case_token_stream = {
                let value = convert_case::Casing::to_case(&field_ident_stringified, convert_case::Case::UpperCamel);
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote! {
                Self::#field_ident_upper_camel_case_token_stream => write!(formatter, #field_ident_quotes_token_stream)
            }
        })
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            #[derive(
                #debug_upper_camel_case,
                #serde_serialize,
                #serde_deserialize,
                enum_extension_lib::EnumExtension,
                postgresql_crud::EnumIter,
                PartialEq,
                Eq,
                #from_str_snake_case::#from_str_upper_camel_case,
                Clone,
                Copy,
            )]
            pub enum #ident_column_upper_camel_case_token_stream {
                #(#variants),*
            }
            impl std::fmt::Display for #ident_column_upper_camel_case_token_stream {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#display_variants),*
                    }
                }
            }
            //todo maybe reuse naming
            impl error_occurence_lib::ToStdStringString for #ident_column_upper_camel_case_token_stream {
                fn to_std_string_string(&self) -> #std_string_string {
                    format!("{self}")
                }
            }
        }
    };
    // println!("{column_token_stream}");
    let value_snake_case = naming_constants::ValueSnakeCase;
    let element_snake_case = naming_constants::ElementSnakeCase;
    let acc_snake_case = naming_constants::AccSnakeCase;
    let generate_query_vec_column_snake_case_token_stream = quote::quote!{generate_query_vec_column};
    let generate_query_vec_column_token_stream = {
        let variants_token_stream = syn_field_with_additional_info_fields_named.iter().map(|element|{
            let field_ident_upper_camel_case_token_stream = syn_ident_to_upper_camel_case_token_stream(element.field_ident);
            let field_ident_string_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
                &element.field_ident.to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {#ident_column_upper_camel_case_token_stream::#field_ident_upper_camel_case_token_stream => #field_ident_string_quotes_token_stream}
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            fn #generate_query_vec_column_snake_case_token_stream(#value_snake_case: &[#ident_column_upper_camel_case_token_stream]) -> #std_string_string {
                let mut #value_snake_case = #value_snake_case.iter().fold(#std_string_string::#from_snake_case(""), |mut #acc_snake_case, #element_snake_case| {
                    #acc_snake_case += match #element_snake_case {
                        #(#variants_token_stream),*
                    };
                    #acc_snake_case += ",";
                    #acc_snake_case
                });
                let _ = #value_snake_case.pop();
                #value_snake_case
            }
        }
    };
    // println!("{generate_query_vec_column_token_stream}");
    let primary_key_field_ident_variant_initialization_token_stream = {
        let field_code_occurence_new_46d303bf_4267_4eb4_a98d_22193db9d220_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            #primary_key_field_ident_upper_camel_case_token_stream {
                #primary_key_field_ident: #error_snake_case,
                #field_code_occurence_new_46d303bf_4267_4eb4_a98d_22193db9d220_token_stream
            }
        }
    };
    let parameters_snake_case = naming_constants::ParametersSnakeCase;
    let payload_snake_case = naming_constants::PayloadSnakeCase;
    let select_snake_case = naming_constants::SelectSnakeCase;
    let sqlx_error_syn_punctuated_punctuated = proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
        &["sqlx","Error"],
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let postgresql_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_constants::PostgresqlUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::InternalServerError500),
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_constants::PostgresqlSnakeCase,
                sqlx_error_syn_punctuated_punctuated.clone(),
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let ref_std_primitive_str = token_patterns::RefStdPrimitiveStr;
    let error_0_token_stream = token_patterns::Error0;
    let error_1_token_stream = token_patterns::Error1;
    let error_2_token_stream = token_patterns::Error2;
    let error_3_token_stream = token_patterns::Error3;
    //todo find out how to declare lifetime on closures
    //todo refactor as &[&'a SynRust...]
    let generate_self_fields_token_stream = |fields: &[&syn::Field]| -> std::vec::Vec<syn::Ident> {
        fields.iter().map(|field| {
            field.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_constants::FIELD_IDENT_IS_NONE
                )
            }).clone()
        })
        .collect()
    };
    #[derive(Debug, strum_macros::Display)]
    enum GeneratePostgresqlCrudAttribute {
        CreateManyAdditionalErrorVariants,
        CreateOneAdditionalErrorVariants,
        ReadManyAdditionalErrorVariants,
        ReadOneAdditionalErrorVariants,
        UpdateManyAdditionalErrorVariants,
        UpdateOneAdditionalErrorVariants,
        DeleteManyAdditionalErrorVariants,
        DeleteOneAdditionalErrorVariants,
        CommonAdditionalErrorVariants,
        CreateManyAdditionalRouteLogic,
        CreateOneAdditionalRouteLogic,
        ReadManyAdditionalRouteLogic,
        ReadOneAdditionalRouteLogic,
        UpdateManyAdditionalRouteLogic,
        UpdateOneAdditionalRouteLogic,
        DeleteManyAdditionalRouteLogic,
        DeleteOneAdditionalRouteLogic,
        CommonAdditionalRouteLogic,
    }
    impl GeneratePostgresqlCrudAttribute {
        fn generate_path_to_attribute(self) -> std::string::String {
            let value = match self {
                Self::CreateManyAdditionalErrorVariants => naming_conventions::CreateManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::CreateOneAdditionalErrorVariants => naming_conventions::CreateOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::ReadManyAdditionalErrorVariants => naming_conventions::ReadManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::ReadOneAdditionalErrorVariants => naming_conventions::ReadOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::UpdateManyAdditionalErrorVariants => naming_conventions::UpdateManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::UpdateOneAdditionalErrorVariants => naming_conventions::UpdateOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::DeleteManyAdditionalErrorVariants => naming_conventions::DeleteManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::DeleteOneAdditionalErrorVariants => naming_conventions::DeleteOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::CommonAdditionalErrorVariants => naming_conventions::CommonAdditionalErrorVariantsSnakeCase.to_string(),
                Self::CreateManyAdditionalRouteLogic => naming_conventions::CreateManyAdditionalRouteLogicSnakeCase.to_string(),
                Self::CreateOneAdditionalRouteLogic => naming_conventions::CreateOneAdditionalRouteLogicSnakeCase.to_string(),
                Self::ReadManyAdditionalRouteLogic => naming_conventions::ReadManyAdditionalRouteLogicSnakeCase.to_string(),
                Self::ReadOneAdditionalRouteLogic => naming_conventions::ReadOneAdditionalRouteLogicSnakeCase.to_string(),
                Self::UpdateManyAdditionalRouteLogic => naming_conventions::UpdateManyAdditionalRouteLogicSnakeCase.to_string(),
                Self::UpdateOneAdditionalRouteLogic => naming_conventions::UpdateOneAdditionalRouteLogicSnakeCase.to_string(),
                Self::DeleteManyAdditionalRouteLogic => naming_conventions::DeleteManyAdditionalRouteLogicSnakeCase.to_string(),
                Self::DeleteOneAdditionalRouteLogic => naming_conventions::DeleteOneAdditionalRouteLogicSnakeCase.to_string(),
                Self::CommonAdditionalRouteLogic => naming_conventions::CommonAdditionalRouteLogicSnakeCase.to_string(),
            };
            format!("{}::{value}", postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE)
        }
    }
    #[derive(
        Debug,
        proc_macro_assistants::ToUpperCamelCaseStringified,
        proc_macro_assistants::ToSnakeCaseStringified,
    )]
    enum Operation {
        CreateMany,
        CreateOne,
        ReadMany,
        ReadOne,
        UpdateMany,
        UpdateOne,
        DeleteMany,
        DeleteOne,
    }
    impl Operation {
        const OPERATION_PAYLOAD_TRY_FROM_OPERATION_PAYLOAD_WITH_SERIALIZE_DESERIALIZE_STATUS_CODE: proc_macro_helpers::status_code::StatusCode = proc_macro_helpers::status_code::StatusCode::BadRequest400;
        const fn http_method(&self) -> OperationHttpMethod {
            match self {
                Self::CreateMany |
                Self::CreateOne |
                Self::ReadMany |
                Self::ReadOne => OperationHttpMethod::Post,
                Self::UpdateMany |
                Self::UpdateOne => OperationHttpMethod::Patch,
                Self::DeleteMany |
                Self::DeleteOne => OperationHttpMethod::Delete,
            }
        }
        fn std_vec_vec_self_payload_element_token_stream(&self) -> proc_macro2::TokenStream {
            let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(self);
            quote::quote!{std::vec::Vec<#operation_payload_element_upper_camel_case_token_stream>}
        }
        fn std_vec_vec_self_payload_element_with_serialize_deserialize_token_stream(&self) -> proc_macro2::TokenStream {
            let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(self);
            quote::quote!{std::vec::Vec<#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream>}
        }
        fn self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
            &self,
        ) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> {
            let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
            handle.push_value(
                syn::PathSegment {
                    ident: proc_macro2::Ident::new(
                        &format!(
                            "{}{}{}",
                            naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(self),
                            naming_constants::ErrorUpperCamelCase,
                            naming_constants::NamedUpperCamelCase,
                        ),
                        proc_macro2::Span::call_site()
                    ),
                    arguments: syn::PathArguments::None,
                }
            );
            handle
        }
        const fn desirable_status_code(&self) -> proc_macro_helpers::status_code::StatusCode {
            match self {
                Self::CreateMany |
                Self::CreateOne => proc_macro_helpers::status_code::StatusCode::Created201,
                Self::ReadMany |
                Self::ReadOne |
                Self::UpdateMany |
                Self::UpdateOne |
                Self::DeleteMany |
                Self::DeleteOne => proc_macro_helpers::status_code::StatusCode::Ok200,
            }
        }
        fn operation_payload_try_from_operation_payload_with_serialize_deserialize_status_code(&self) -> proc_macro_helpers::status_code::StatusCode {
            Self::OPERATION_PAYLOAD_TRY_FROM_OPERATION_PAYLOAD_WITH_SERIALIZE_DESERIALIZE_STATUS_CODE.clone()
        }
        fn to_additional_error_variants(&self) -> GeneratePostgresqlCrudAttribute {
            match self {
                Self::CreateMany => GeneratePostgresqlCrudAttribute::CreateManyAdditionalErrorVariants,
                Self::CreateOne => GeneratePostgresqlCrudAttribute::CreateOneAdditionalErrorVariants,
                Self::ReadMany => GeneratePostgresqlCrudAttribute::ReadManyAdditionalErrorVariants,
                Self::ReadOne => GeneratePostgresqlCrudAttribute::ReadOneAdditionalErrorVariants,
                Self::UpdateMany => GeneratePostgresqlCrudAttribute::UpdateManyAdditionalErrorVariants,
                Self::UpdateOne => GeneratePostgresqlCrudAttribute::UpdateOneAdditionalErrorVariants,
                Self::DeleteMany => GeneratePostgresqlCrudAttribute::DeleteManyAdditionalErrorVariants,
                Self::DeleteOne => GeneratePostgresqlCrudAttribute::DeleteOneAdditionalErrorVariants,
            }
        }
        fn to_additional_route_logic(&self) -> GeneratePostgresqlCrudAttribute {
            match self {
                Self::CreateMany => GeneratePostgresqlCrudAttribute::CreateManyAdditionalRouteLogic,
                Self::CreateOne => GeneratePostgresqlCrudAttribute::CreateOneAdditionalRouteLogic,
                Self::ReadMany => GeneratePostgresqlCrudAttribute::ReadManyAdditionalRouteLogic,
                Self::ReadOne => GeneratePostgresqlCrudAttribute::ReadOneAdditionalRouteLogic,
                Self::UpdateMany => GeneratePostgresqlCrudAttribute::UpdateManyAdditionalRouteLogic,
                Self::UpdateOne => GeneratePostgresqlCrudAttribute::UpdateOneAdditionalRouteLogic,
                Self::DeleteMany => GeneratePostgresqlCrudAttribute::DeleteManyAdditionalRouteLogic,
                Self::DeleteOne => GeneratePostgresqlCrudAttribute::DeleteOneAdditionalRouteLogic,
            }
        }
        fn generate_error_initialization_eprintln_response_creation_token_stream(
            &self,
            syn_variant_wrapper: &proc_macro_helpers::construct_syn_variant::SynVariantWrapper,
            file: &'static str,
            line: std::primitive::u32,
            column: std::primitive::u32,
            proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
        ) -> proc_macro2::TokenStream {
            let try_operation_route_logic_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedUpperCamelCaseTokenStream::try_self_route_logic_error_named_upper_camel_case_token_stream(self);
            let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(self);
            let from_snake_case = naming_constants::FromSnakeCase;
            let error_snake_case = naming_constants::ErrorSnakeCase;
            let response_snake_case = naming_constants::ResponseSnakeCase;
            let into_response_snake_case = naming_conventions::IntoResponseSnakeCase;
            let eprintln_error_token_stream = proc_macro_common::eprintln_error_token_stream();
            let syn_variant_initialization_token_stream = syn_variant_wrapper.generate_initialization_token_stream(
                &file,
                line,
                column,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let status_code_token_stream = syn_variant_wrapper.get_option_status_code().unwrap().to_axum_http_status_code_token_stream();
            quote::quote! {
                let #error_snake_case = #try_operation_route_logic_error_named_upper_camel_case_token_stream::#syn_variant_initialization_token_stream;
                #eprintln_error_token_stream
                let mut #response_snake_case = axum::response::IntoResponse::#into_response_snake_case(axum::Json(#try_operation_route_logic_response_variants_upper_camel_case_token_stream::#from_snake_case(#error_snake_case)));
                *#response_snake_case.status_mut() = #status_code_token_stream;
                return #response_snake_case;
            }
        }
        fn generate_self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper(
            &self,
            proc_macro_name_upper_camel_case_ident_stringified: &str
        ) -> proc_macro_helpers::construct_syn_variant::SynVariantWrapper {
            proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
                &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(self),
                Some(self.operation_payload_try_from_operation_payload_with_serialize_deserialize_status_code()),
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
                        &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified(self),
                        self.self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated()
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            )
        }
    }
    #[derive(proc_macro_assistants::ToSnakeCaseStringified)]
    enum OperationHttpMethod {
        Post,
        Patch,
        Delete,
    }
    let generate_options_try_from_sqlx_row_token_stream = |
        operation: &Operation
    |{
        let declaration_primary_key_token_stream = {
            let inner_type_with_serialize_deserialize_token_stream = &primary_key_inner_type_with_serialize_deserialize_token_stream;
            quote::quote! {
                let mut #primary_key_field_ident: std::option::Option<postgresql_crud::Value<#inner_type_with_serialize_deserialize_token_stream>> = None;
            }
        };
        let declaration_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
            let field_ident = &element.field_ident;
            let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
            quote::quote! {
                let mut #field_ident: std::option::Option<postgresql_crud::Value<#inner_type_with_serialize_deserialize_token_stream>> = None;
            }
        });
        let assignment_variant_primary_key_token_stream = {
            let primary_key_field_ident_upper_camel_case_token_stream = {
                let value = convert_case::Casing::to_case(&primary_key_field_ident.to_string(), convert_case::Case::UpperCamel);
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let primary_key_field_ident_string_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
                &primary_key_field_ident.to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                &postgresql_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #ident_column_upper_camel_case_token_stream::#primary_key_field_ident_upper_camel_case_token_stream => match sqlx::Row::try_get::<
                    std::option::Option<#primary_key_original_type_token_stream>, 
                    #ref_std_primitive_str
                >(
                    &#value_snake_case, 
                    #primary_key_field_ident_string_quotes_token_stream
                ) {
                    Ok(#value_snake_case) => {
                        #primary_key_field_ident = #value_snake_case.map(|#value_snake_case| {
                            postgresql_crud::Value {
                                #value_snake_case: #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#primary_key_inner_type_token_stream(#value_snake_case))
                            }
                        });
                    },
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                }
            }
        };
        let assignment_variants_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
            let field_ident = &element.field_ident;
            let field_ident_upper_camel_case_token_stream = {
                let value = convert_case::Casing::to_case(&element.field_ident.to_string(), convert_case::Case::UpperCamel);
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let original_type_token_stream = &element.original_type_token_stream;
            let field_ident_string_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
                &element.field_ident.to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
            let inner_type_token_stream = &element.inner_type_token_stream;
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                &postgresql_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #ident_column_upper_camel_case_token_stream::#field_ident_upper_camel_case_token_stream => match sqlx::Row::try_get::<
                    std::option::Option<#original_type_token_stream>, 
                    #ref_std_primitive_str
                >(
                    &#value_snake_case, 
                    #field_ident_string_quotes_token_stream
                ) {
                    Ok(#value_snake_case) => {
                        #field_ident = #value_snake_case.map(|#value_snake_case| {
                            postgresql_crud::Value {
                                #value_snake_case: #inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#inner_type_token_stream(#value_snake_case))
                            }
                        });
                    },
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                }
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        let option_fields_initiation_token_stream = generate_self_fields_token_stream(
            &syn_field_with_additional_info_fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
        );
        quote::quote!{
            #declaration_primary_key_token_stream
            #(#declaration_excluding_primary_key_token_stream)*
            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.#select_snake_case {
                match #element_snake_case {
                    #assignment_variant_primary_key_token_stream,
                    #(#assignment_variants_excluding_primary_key_token_stream),*
                }
            }
            #struct_options_ident_token_stream {
                #(#option_fields_initiation_token_stream),*
            }
        }
    };
    let order_by_upper_camel_case = naming_conventions::OrderByUpperCamelCase;
    let postgresql_crud_order_by_token_stream = quote::quote! {postgresql_crud::#order_by_upper_camel_case};
    let postgresql_crud_order_token_stream = quote::quote! {postgresql_crud::Order};
    let allow_methods_token_stream = {
        let http_method_token_stream = quote::quote!{http::Method};
        quote::quote! {
            pub const ALLOW_METHODS: [#http_method_token_stream;4] = [
                #http_method_token_stream::GET, 
                #http_method_token_stream::POST, 
                #http_method_token_stream::PATCH, 
                #http_method_token_stream::DELETE
            ];//todo new axum version does not support it or something - find out
        }
    };
    let ident_column_read_permission_token_stream = {
        let derive_debug_clone_copy = token_patterns::DeriveDebugCloneCopy;
        let ident_column_read_permission_name_token_stream = {
            let ident_column_read_permission_name = format!(
                "{ident}{}{}{}",
                naming_constants::ColumnUpperCamelCase,
                naming_constants::ReadUpperCamelCase,
                naming_constants::PermissionUpperCamelCase,
            );
            ident_column_read_permission_name.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_column_read_permission_name} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let fields_permission_token_stream = syn_field_with_additional_info_fields_named.iter().map(|element| {
            let field_ident = &element.field_ident;
            quote::quote!{
                #field_ident: std::primitive::bool
            }
        });
        quote::quote! {
            #derive_debug_clone_copy
            pub struct #ident_column_read_permission_name_token_stream {
                #(#fields_permission_token_stream),*
            }
        }
    };
    let reexport_postgresql_sqlx_column_types_token_stream = syn_field_with_additional_info_fields_named.iter().map(|element|{
        let inner_type_token_stream = &element.inner_type_token_stream;
        quote::quote! {pub use #inner_type_token_stream;}
    });
    let field_and_field_with_serialize_deserialize_token_stream = {
        let derive_debug = token_patterns::DeriveDebug;
        let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
        let field_upper_camel_case = naming_constants::FieldUpperCamelCase;
        let field_with_serialize_deserialize_upper_camel_case = naming_conventions::FieldWithSerializeDeserializeUpperCamelCase;
        quote::quote!{
            #derive_debug
            pub struct #field_upper_camel_case<T> {
                pub #value_snake_case: T
            }       
            #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
            pub struct #field_with_serialize_deserialize_upper_camel_case<T> {
                pub #value_snake_case: T
            }
        }
    };
    let query_string_snake_case = naming_conventions::QueryStringSnakeCase;
    let binded_query_snake_case = naming_conventions::BindedQuerySnakeCase;
    let rollback_snake_case = naming_constants::RollbackSnakeCase;
    let query_snake_case = naming_constants::QuerySnakeCase;
    let update_snake_case = naming_constants::UpdateSnakeCase;
    let set_snake_case = naming_constants::SetSnakeCase;
    let from_snake_case = naming_constants::FromSnakeCase;
    let insert_snake_case = naming_constants::InsertSnakeCase;
    let into_snake_case = naming_constants::IntoSnakeCase;
    let values_snake_case = naming_constants::ValuesSnakeCase;
    let delete_snake_case = naming_constants::DeleteSnakeCase;
    let where_snake_case = naming_constants::WhereSnakeCase;
    let use_futures_try_stream_ext_token_stream = quote::quote! {use futures::TryStreamExt};
    let returning_snake_case = naming_constants::ReturningSnakeCase;
    let returning_primary_key_stringified = format!(" {returning_snake_case} {primary_key_field_ident}");
    let returning_primary_key_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
        &returning_primary_key_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let postgresql_crud_snake_case = naming_conventions::PostgresqlCrudSnakeCase;
    let std_string_string_syn_punctuated_punctuated = proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
        &["std","string","String"],
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let checked_add_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::CheckedAddUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::BadRequest400),
        std::vec::Vec::<(
            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute,
            &'static dyn std::fmt::Display,
            syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
        )>::default(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let row_and_rollback_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::RowAndRollbackUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::InternalServerError500),
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_constants::RowSnakeCase,
                sqlx_error_syn_punctuated_punctuated.clone(),
            ),
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &rollback_snake_case,
                sqlx_error_syn_punctuated_punctuated.clone(),
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let primary_key_std_vec_vec_inner_type_syn_punctuated_punctuated = {
        let panic_message = format!("primary key functionality is not implemented for {primary_key_rust_sqlx_map_to_postgres_type_variant} in {proc_macro_name_upper_camel_case_ident_stringified} logic");
        let name = match primary_key_rust_sqlx_map_to_postgres_type_variant {
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonBNotNull |

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB |
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => panic!("{panic_message}"),

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => "StdPrimitiveI64",
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => "SqlxTypesUuidUuid",
        };
        let generate_std_vec_vec_syn_punctuated_punctuated = |parts_vec: &[&str]| -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> {
            let parts_vec_len = parts_vec.len();
            if parts_vec_len >= 1 {
                let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                handle.push_value(
                    syn::PathSegment {
                        ident: proc_macro2::Ident::new("std", proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    }
                );
                handle.push_punct(syn::token::PathSep{
                    spans: [proc_macro2::Span::call_site(),proc_macro2::Span::call_site()],
                });
                handle.push_value(
                    syn::PathSegment {
                        ident: proc_macro2::Ident::new("vec", proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    }
                );
                handle.push_punct(syn::token::PathSep{
                    spans: [proc_macro2::Span::call_site(),proc_macro2::Span::call_site()],
                });
                handle.push_value(
                    syn::PathSegment {
                        ident: proc_macro2::Ident::new("Vec", proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{
                            colon2_token: None,
                            lt_token: syn::token::Lt{
                                spans: [proc_macro2::Span::call_site()],
                            },
                            args: {
                                let mut handle = syn::punctuated::Punctuated::<syn::GenericArgument, syn::token::Comma>::new();
                                handle.push(syn::GenericArgument::Type(syn::Type::Path(syn::TypePath{
                                    qself: None,
                                    path: syn::Path {
                                        leading_colon: None,
                                        segments: {
                                            let parts_vec_len_minus_one = parts_vec_len.checked_sub(1).unwrap();
                                            let mut std_vec_vec_generic_type = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                                            for (index, element) in parts_vec.iter().enumerate() {
                                                std_vec_vec_generic_type.push_value(
                                                    syn::PathSegment {
                                                        ident: proc_macro2::Ident::new(element, proc_macro2::Span::call_site()),
                                                        arguments: syn::PathArguments::None,
                                                    }
                                                );
                                                if index < parts_vec_len_minus_one {
                                                    std_vec_vec_generic_type.push_punct(syn::token::PathSep{
                                                        spans: [proc_macro2::Span::call_site(),proc_macro2::Span::call_site()],
                                                    });
                                                }
                                            }
                                            std_vec_vec_generic_type
                                        },
                                    },
                                })));
                                handle
                            },
                            gt_token: syn::token::Gt {
                                spans: [proc_macro2::Span::call_site()],
                            },
                        }),
                    }
                );
                handle
            }
            else {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} generate_simple_syn_punctuated_punctuated parts_vec_len.len() > 1 == false for {parts_vec:?}")
            }
        };
        generate_std_vec_vec_syn_punctuated_punctuated(&[postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, name])
    };
    let non_existing_primary_keys_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::NonExistingPrimaryKeysUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
                &naming_conventions::NonExistingPrimaryKeysSnakeCase,
                primary_key_std_vec_vec_inner_type_syn_punctuated_punctuated.clone()
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let non_existing_primary_keys_and_rollback_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::NonExistingPrimaryKeysAndRollbackUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
                &naming_conventions::NonExistingPrimaryKeysSnakeCase, 
                primary_key_std_vec_vec_inner_type_syn_punctuated_punctuated.clone()
            ),
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &rollback_snake_case,
                sqlx_error_syn_punctuated_punctuated.clone()
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    //todo maybe first convert to type what can be primary key ?
    let (
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variant_option,
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream_option
    ) = {
        //todo refactor this
        match primary_key_from_or_try_from {
            postgresql_crud_common::FromOrTryFrom::From => (
                None,
                None,
            ),
            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_upper_camel_case = naming_conventions::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClientUpperCamelCase;
                let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_snake_case = naming_conventions::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClientSnakeCase;
                (
                    Some(syn::Variant {
                        attrs: vec![],
                        ident: syn::Ident::new(&operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_upper_camel_case.to_string(), proc_macro2::Span::call_site()),
                        fields: syn::Fields::Named(syn::FieldsNamed {
                            brace_token: syn::token::Brace::default(),
                            named: {
                                let mut punctuated = syn::punctuated::Punctuated::new();
                                punctuated.push_value(syn::Field {
                                    attrs: vec![syn::Attribute {
                                        pound_token: syn::token::Pound {
                                            spans: [proc_macro2::Span::call_site()],
                                        },
                                        style: syn::AttrStyle::Outer,
                                        bracket_token: syn::token::Bracket::default(),
                                        meta: syn::Meta::Path(syn::Path {
                                            leading_colon: None,
                                            segments: {
                                                let mut handle = syn::punctuated::Punctuated::new();
                                                handle.push(syn::PathSegment {
                                                    ident: proc_macro2::Ident::new(
                                                        proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence),
                                                        proc_macro2::Span::call_site(),
                                                    ),
                                                    arguments: syn::PathArguments::None,
                                                });
                                                handle
                                            },
                                        }),
                                    }],
                                    vis: syn::Visibility::Inherited,
                                    mutability: syn::FieldMutability::None,
                                    ident: Some(syn::Ident::new(&operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_snake_case.to_string(), proc_macro2::Span::call_site())),
                                    colon_token: Some(syn::token::Colon {
                                        spans: [proc_macro2::Span::call_site()],
                                    }),
                                    ty: {
                                        let value: syn::Type = syn::parse(primary_key_inner_type_with_serialize_deserialize_error_named_token_stream.clone().into())
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot convert {primary_key_inner_type_with_serialize_deserialize_error_named_token_stream} into syn::Type {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                        value
                                    }
                                });
                                punctuated.push_punct(syn::token::Comma {
                                    spans: [proc_macro2::Span::call_site()],
                                });
                                punctuated.push_value(proc_macro_helpers::code_occurence_syn_field::code_occurence_syn_field(&proc_macro_name_upper_camel_case_ident_stringified));
                                punctuated
                            },
                        }),
                        discriminant: None,
                    }),
                    Some({
                        let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_upper_camel_case = naming_conventions::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClientUpperCamelCase;
                        let field_code_occurence_new_a1c07748_20c3_49eb_85e0_615161d95345_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            #operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_upper_camel_case {
                                #operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_snake_case: #error_snake_case,
                                #field_code_occurence_new_a1c07748_20c3_49eb_85e0_615161d95345_token_stream,
                            }
                        }
                    })
                )
            }
        }
    };
    // let axum_response_into_response = token_patterns::AxumResponseIntoResponse;
    // let axum_extract_rejection_json_rejection = token_patterns::AxumExtractRejectionJsonRejection;
    let sqlx_query_sqlx_postgres_token_stream = quote::quote! {sqlx::query::<sqlx::Postgres>};
    // let axum_extract_state_token_stream = quote::quote! {axum::extract::State};
    // let axum_json_token_stream = quote::quote! {axum::Json};
    //todo remove it after refactor to not support middleware
    // let crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream = quote::quote! {crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor};
    //todo reuse BindQuery path
    let postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream = quote::quote! {#postgresql_crud_snake_case::BindQuery::bind_value_to_query};
    let crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream = quote::quote! {#postgresql_crud_snake_case::BindQuery::try_generate_bind_increments};
    let postgresql_crud_bind_query_bind_query_try_increment_token_stream = quote::quote! {#postgresql_crud_snake_case::BindQuery::try_increment};
    let increment_initialization_token_stream = quote::quote! {let mut increment: u64 = 0;};
    // let http_status_code = token_patterns::HttpStatusCode;
    // let reqwest_header_header_map = token_patterns::ReqwestHeaderHeaderMap;
    // let reqwest_error = token_patterns::ReqwestError;
    // let crate_common_api_request_unexpected_error_response_text_result_token_stream = quote::quote! {crate::common::api_request_unexpected_error::ResponseTextResult};
    // let try_extract_value_snake_case = naming_conventions::TryExtractValueSnakeCase;
    let dot_space = ", ";
    // let pg_temp_stringified = "pg_temp";
    let where_snake_case_qoutes_token_stream = proc_macro_common::generate_quotes::token_stream(
        &where_snake_case.to_string(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let and_snake_case = naming_constants::AndSnakeCase;
    let order_by_snake_case = naming_conventions::OrderBySnakeCase;
    let limit_snake_case = naming_constants::LimitSnakeCase;
    let offset_snake_case = naming_constants::OffsetSnakeCase;
    let in_snake_case = naming_constants::InSnakeCase;
    let unnest_snake_case = naming_constants::UnnestSnakeCase;
    let error_snake_case = naming_constants::ErrorSnakeCase;
    let response_snake_case = naming_constants::ResponseSnakeCase;
    // let primary_keys_snake_case = naming_conventions::PrimaryKeysSnakeCase;
    // let primary_key_snake_case = naming_conventions::PrimaryKeySnakeCase;
    // let into_inner_type_vec_snake_case = naming_conventions::IntoInnerTypeVecSnakeCase;
    // let api_request_unexpected_error_module_path_token_stream = quote::quote! { crate::common::api_request_unexpected_error };
    // let expected_type_upper_camel_case = naming_conventions::ExpectedTypeUpperCamelCase;
    // let expected_type_snake_case = naming_conventions::ExpectedTypeSnakeCase;
    // let unexpected_status_code_upper_camel_case = naming_conventions::UnexpectedStatusCodeUpperCamelCase;
    // let status_code_snake_case = naming_conventions::StatusCodeSnakeCase;
    // let headers_snake_case = naming_constants::HeadersSnakeCase;
    let body_snake_case = naming_constants::BodySnakeCase;
    // let response_text_result_snake_case = naming_conventions::ResponseTextResultSnakeCase;
    // let response_text_snake_case = naming_conventions::ResponseTextSnakeCase;
    let fields_named_idents_comma_token_stream = generate_self_fields_token_stream(&syn_field_with_additional_info_fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>())
        .iter()
        .map(|element| quote::quote! {#element,})
        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    // let select_full_variant_token_stream = {
    //     let value = fields_named.iter().fold(std::string::String::default(), |mut #acc_snake_case, element| {
    //         #acc_snake_case.push_str(&convert_case::Casing::to_case(&element.field_ident.to_string(), convert_case::Case::UpperCamel));
    //         #acc_snake_case
    //     });
    //     value.parse::<proc_macro2::TokenStream>()
    //     .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    // };
    let (
        limit_and_offset_type_token_stream,
        limit_and_offset_type_with_serialize_deserialize_token_stream,
    ) = {
        let supported_sqlx_postgres_type_std_primitive_i64 = postgresql_crud_common::SupportedSqlxPostgresType::StdPrimitiveI64;
        (
            {
                let value = supported_sqlx_postgres_type_std_primitive_i64.get_inner_type_stringified("");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            },
            {
                let value = supported_sqlx_postgres_type_std_primitive_i64.get_inner_type_with_serialize_deserialize_stringified("");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            }
        )
    };
    let fields_idents_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| element.field_ident).collect::<std::vec::Vec<&syn::Ident>>();
    let std_vec_vec_primary_key_inner_type_with_serialize_deserialize_token_stream = quote::quote!{std::vec::Vec::<#primary_key_inner_type_with_serialize_deserialize_token_stream>};
    let std_vec_vec_struct_options_ident_token_stream = quote::quote!{std::vec::Vec::<#struct_options_ident_token_stream>};
    //todo reuse naming
    // let not_unique_column_upper_camel_case = naming_conventions::NotUniqueColumnUpperCamelCase;
    // let not_unique_column_snake_case = naming_conventions::NotUniqueColumnSnakeCase;
    let not_unique_column_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::NotUniqueColumnUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
                &naming_conventions::NotUniqueColumnSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &[&ident_column_upper_camel_case_stringified],
                    &proc_macro_name_upper_camel_case_ident_stringified
                )
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    // let axum_http_status_code_token_stream = quote::quote!{axum::http::StatusCode};
    // let postgresql_crud_get_axum_http_status_code_token_stream = quote::quote!{postgresql_crud::GetAxumHttpStatusCode};
    // let get_axum_http_status_code_snake_case = naming_conventions::GetAxumHttpStatusCodeSnakeCase;
    // let app_state_dyn_postgresql_crud_combination_of_traits_for_postgresql_crud_logic_comma_token_stream = quote::quote!{#app_state_snake_case: &dyn postgresql_crud::CombinationOfTraitsForPostgresqlCrudLogic,};
    // let body_bytes_snake_case = naming_conventions::BodyBytesSnakeCase;
    // let body_bytes_bytes_bytes_token_stream = quote::quote!{#body_bytes_snake_case: bytes::Bytes,};
    // let axum_response_into_response_token_stream = quote::quote!{axum::response::IntoResponse};
    // let axum_response_response_token_stream = quote::quote!{axum::response::Response};
    // let into_response_snake_case = naming_conventions::IntoResponseSnakeCase;
    let serde_json_to_string_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::SerdeJsonToStringUpperCamelCase,
        None,
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_conventions::SerdeJsonToStringSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &["serde_json","Error"],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let failed_to_get_response_text_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::FailedToGetResponseTextUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_conventions::StatusCodeSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &["http","StatusCode"],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            ),
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_constants::HeadersSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &["reqwest","header","HeaderMap"],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            ),
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_constants::ReqwestSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &["reqwest","Error"],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let deserialize_response_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::DeserializeResponseUpperCamelCase,
        None,
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_conventions::StatusCodeSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &["http","StatusCode"],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            ),
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_constants::HeadersSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &["reqwest","header", "HeaderMap"],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            ),
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
                &naming_conventions::ResponseTextSnakeCase,
                std_string_string_syn_punctuated_punctuated.clone(),
            ),
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_constants::SerdeSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &["serde_json","Error"],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let reqwest_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_constants::ReqwestUpperCamelCase,
        None,
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_constants::ReqwestSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &["reqwest","Error"],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let common_http_request_syn_variants = {
        vec![
            serde_json_to_string_syn_variant_wrapper.get_syn_variant().clone(),
            failed_to_get_response_text_syn_variant_wrapper.get_syn_variant().clone(),
            deserialize_response_syn_variant_wrapper.get_syn_variant().clone(),
            reqwest_syn_variant_wrapper.get_syn_variant().clone(),
        ]
    };
    let check_body_size_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::CheckBodySizeUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
                &naming_conventions::CheckBodySizeSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &["route_validators", "check_body_size", &naming_conventions::CheckBodySizeErrorNamedUpperCamelCase.to_string()],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let serde_json_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::SerdeJsonUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_conventions::SerdeJsonSnakeCase,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &["serde_json","Error"],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let bind_query_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::BindQueryUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::InternalServerError500),
        vec![(
            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
            &naming_conventions::BindQuerySnakeCase,
            proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                &[postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, &naming_conventions::TryGenerateBindIncrementsErrorNamedUpperCamelCase.to_string()],
                &proc_macro_name_upper_camel_case_ident_stringified
            ),
        )],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let not_unique_primary_key_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::NotUniquePrimaryKeyUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_conventions::NotUniquePrimaryKeySnakeCase,
                {
                    let mut value = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                    value.push_value(syn::PathSegment {
                        ident: proc_macro2::Ident::new(&postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    });
                    value.push_punct(syn::token::PathSep {
                        spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                    });
                    value.push_value(syn::PathSegment {
                        ident: proc_macro2::Ident::new(&primary_key_rust_sqlx_map_to_postgres_type_variant.get_inner_type_handle_stringified(""),proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    });
                    value
                }
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let no_payload_fields_primary_key_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
        &naming_conventions::NoPayloadFieldsPrimaryKeyUpperCamelCase,
        Some(proc_macro_helpers::status_code::StatusCode::InternalServerError500),
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming_conventions::NoPayloadFieldsPrimaryKeySnakeCase,
                {
                    let mut value = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                    value.push_value(syn::PathSegment {
                        ident: proc_macro2::Ident::new(&postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    });
                    value.push_punct(syn::token::PathSep {
                        spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                    });
                    value.push_value(syn::PathSegment {
                        ident: proc_macro2::Ident::new(
                            &postgresql_crud_common::SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(
                                &postgresql_crud_common::SupportedSqlxPostgresType::from(primary_key_rust_sqlx_map_to_postgres_type_variant)
                            ).to_string(),
                            proc_macro2::Span::call_site()
                        ),
                        arguments: syn::PathArguments::None,
                    });
                    value
                }
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let generate_additional_error_variants = |
        syn_derive_input: &syn::DeriveInput,
        generate_postgresql_crud_attribute: GeneratePostgresqlCrudAttribute,
    | -> std::vec::Vec<syn::Variant> {
        let generate_postgresql_crud_attribute_stringified = generate_postgresql_crud_attribute.to_string();
        let common_additional_error_variants_attribute_token_stream = proc_macro_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(
            &syn_derive_input.attrs,
            &generate_postgresql_crud_attribute.generate_path_to_attribute(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let value: syn::DeriveInput = syn::parse((*common_additional_error_variants_attribute_token_stream).clone().into()).unwrap_or_else(|error| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {}: {error}",
                proc_macro_common::constants::AST_PARSE_FAILED
            )
        });
        let value_ident_stringified = value.ident.to_string();
        assert!(value_ident_stringified == generate_postgresql_crud_attribute_stringified, "{proc_macro_name_upper_camel_case_ident_stringified} {value_ident_stringified} is not equal to {generate_postgresql_crud_attribute_stringified}");
        let variants = if let syn::Data::Enum(data_enum) = value.data {
            data_enum.variants
        }
        else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} value.data is not syn::Data::Enum");
        };
        variants.into_iter().collect()
    };
    let common_additional_error_variants = generate_additional_error_variants(
        &syn_derive_input,
        GeneratePostgresqlCrudAttribute::CommonAdditionalErrorVariants,
    );
    let common_route_syn_variants = {
        let common_additional_error_variants_vec = common_additional_error_variants.iter().collect::<std::vec::Vec<&syn::Variant>>();
        let mut value = std::vec::Vec::with_capacity(common_additional_error_variants_vec.len() + common_additional_error_variants_vec.len());
        value.push(check_body_size_syn_variant_wrapper.get_syn_variant());
        value.push(&postgresql_syn_variant_wrapper.get_syn_variant());
        value.push(&serde_json_syn_variant_wrapper.get_syn_variant());
        // value.push(&bind_query_syn_variant);
        for element in common_additional_error_variants_vec {
            value.push(element);
        }
        value
    };
    let common_additional_route_logic_token_stream = proc_macro_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(
        &syn_derive_input.attrs,
        &GeneratePostgresqlCrudAttribute::CommonAdditionalRouteLogic.generate_path_to_attribute(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let generate_fields_named_excluding_primary_key_token_stream = |function: fn(&SynFieldWithAdditionalInfo<'_>)-> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(function);
        quote::quote! {#(#fields_token_stream),*}
    };
    let pub_fields_idents_std_option_option_std_vec_vec_where_inner_type_token_stream = {
        fn generate_pub_fields_idents_std_option_option_std_vec_vec_where_inner_type_token_stream(element: &SynFieldWithAdditionalInfo<'_>) -> proc_macro2::TokenStream {
            let field_ident = &element.field_ident;
            let where_inner_type_token_stream = &element.where_inner_type_token_stream;
            quote::quote! {
                pub #field_ident: std::option::Option<std::vec::Vec<#where_inner_type_token_stream>>
            }
        }
        generate_fields_named_excluding_primary_key_token_stream(generate_pub_fields_idents_std_option_option_std_vec_vec_where_inner_type_token_stream)
    };
    let fields_idents_std_option_option_std_vec_vec_where_inner_type_with_serialize_deserialize_token_stream = {
        fn generate_fields_idents_std_option_option_std_vec_vec_where_inner_type_with_serialize_deserialize_token_stream(element: &SynFieldWithAdditionalInfo<'_>) -> proc_macro2::TokenStream {
            let field_ident = &element.field_ident;
            let where_inner_type_with_serialize_deserialize_token_stream = &element.where_inner_type_with_serialize_deserialize_token_stream;
            quote::quote! {
                #field_ident: std::option::Option<std::vec::Vec<#where_inner_type_with_serialize_deserialize_token_stream>>
            }
        }
        generate_fields_named_excluding_primary_key_token_stream(generate_fields_idents_std_option_option_std_vec_vec_where_inner_type_with_serialize_deserialize_token_stream)
    };
    let generate_pub_handle_token_stream = |is_pub: bool|match is_pub {
        true => quote::quote! {pub},
        false => proc_macro2::TokenStream::new()
    };
    let generate_primary_key_inner_type_handle_token_stream = |is_original: bool|match is_original {
        true => &primary_key_inner_type_token_stream,
        false => &primary_key_inner_type_with_serialize_deserialize_token_stream,
    };
    let generate_fields_idents_std_option_option_std_vec_vec_where_inner_type_handle_token_stream = |is_pub: bool|match is_pub {
        true => &pub_fields_idents_std_option_option_std_vec_vec_where_inner_type_token_stream,
        false => &fields_idents_std_option_option_std_vec_vec_where_inner_type_with_serialize_deserialize_token_stream,
    };
    let generate_primary_key_field_ident_std_option_option_std_vec_vec_primary_key_inner_type_handle_token_stream = |is_pub: bool|{
        let pub_handle_token_stream = generate_pub_handle_token_stream(is_pub);
        let primary_key_inner_type_handle_token_stream = generate_primary_key_inner_type_handle_token_stream(is_pub);
        quote::quote! {#pub_handle_token_stream #primary_key_field_ident: std::option::Option<std::vec::Vec<#primary_key_inner_type_handle_token_stream>>}
    };
    let generate_pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream = |is_pub: bool|{
        let pub_handle_token_stream = generate_pub_handle_token_stream(is_pub);
        quote::quote! {#pub_handle_token_stream #select_snake_case: std::vec::Vec<#ident_column_upper_camel_case_token_stream>}
    };
    let generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = |is_pub: bool|{
        let pub_handle_token_stream = generate_pub_handle_token_stream(is_pub);
        let primary_key_inner_type_handle_token_stream = generate_primary_key_inner_type_handle_token_stream(is_pub);
        quote::quote! {#pub_handle_token_stream #primary_key_field_ident: #primary_key_inner_type_handle_token_stream}
    };
    let self_init_fields_token_stream = generate_self_fields_token_stream(&syn_field_with_additional_info_fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>());
    let common_http_request_with_possible_operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variants = &{
        let mut value = common_http_request_syn_variants.clone();
        if let Some(variant) = &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variant_option {
            value.push(variant.clone());
        }
        value
    };
    let generate_read_operation_payload_with_serialize_deserialize_initialization_token_stream = |
        operation: &Operation
    | {
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
        quote::quote!{
            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)
        }
    };
    let generate_filter_not_unique_column_route_logic_token_stream = |operation: &Operation|{
        let not_unique_column_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
            &not_unique_column_syn_variant_wrapper,
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            let mut #acc_snake_case = std::vec::Vec::new();
            for #element_snake_case in &#value_snake_case.#select_snake_case {
                if #acc_snake_case.contains(&#element_snake_case) {
                    let #error_0_token_stream = *#element_snake_case;
                    #not_unique_column_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                }
                else {
                    #acc_snake_case.push(#element_snake_case);
                }
            }
        }
    };
    let generate_filter_not_unique_column_http_request_token_stream = |operation: &Operation|{
         let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
         let not_unique_column_syn_variant_wrapper_initialization_token_stream = &not_unique_column_syn_variant_wrapper.generate_initialization_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            let mut #acc_snake_case = std::vec::Vec::new();
            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.#select_snake_case {
                if #acc_snake_case.contains(&#element_snake_case) {
                    let #error_0_token_stream = *#element_snake_case;
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#not_unique_column_syn_variant_wrapper_initialization_token_stream);
                } else {
                    #acc_snake_case.push(#element_snake_case);
                }
            }
        }
    };
    let update_fields_token_stream = {
        let primary_key_field_named_token_stream = quote::quote! {pub #primary_key_field_ident: #primary_key_inner_type_token_stream};
        let fields_named_excluding_primary_key_token_stream = generate_fields_named_excluding_primary_key_token_stream({
            fn generate_option_fields_token_stream(element: &SynFieldWithAdditionalInfo<'_>) -> proc_macro2::TokenStream {
               let field_ident = &element.field_ident;
               let inner_type_token_stream = &element.inner_type_token_stream;
               let field_upper_camel_case = naming_constants::FieldUpperCamelCase;
               quote::quote! {
                   pub #field_ident: std::option::Option<#field_upper_camel_case<#inner_type_token_stream>>
               }
            }
            generate_option_fields_token_stream
        });
        quote::quote! {
            #primary_key_field_named_token_stream,
            #fields_named_excluding_primary_key_token_stream
        }
    };
    let update_fields_with_serialize_deserialize_token_stream = {
        let primary_key_field_named_token_stream = quote::quote! {#primary_key_field_ident: #primary_key_inner_type_with_serialize_deserialize_token_stream};
        let fields_named_excluding_primary_key_token_stream = generate_fields_named_excluding_primary_key_token_stream({
            fn generate_option_field_with_serialize_deserialize_token_stream(element: &SynFieldWithAdditionalInfo<'_>) -> proc_macro2::TokenStream {
                let field_ident = &element.field_ident;
                let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
                let field_with_serialize_deserialize_upper_camel_case = naming_conventions::FieldWithSerializeDeserializeUpperCamelCase;
                quote::quote! {
                    #field_ident: std::option::Option<#field_with_serialize_deserialize_upper_camel_case<#inner_type_with_serialize_deserialize_token_stream>>
                }
            }
            generate_option_field_with_serialize_deserialize_token_stream
        });
        quote::quote! {
            #primary_key_field_named_token_stream,
            #fields_named_excluding_primary_key_token_stream
        }
    };
    let impl_std_convert_from_update_with_serialize_deserialize_for_update_token_stream = {
        let primary_key_field_assignment_token_stream = quote::quote! {
            let #primary_key_field_ident = #primary_key_inner_type_token_stream::#from_snake_case(#value_snake_case.#primary_key_field_ident);
        };
        let fields_assignments_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map({
            fn generate_let_field_ident_value_option_field_from_option_field_with_serialize_deserialize_token_stream(
                element: &SynFieldWithAdditionalInfo<'_>,
            ) -> proc_macro2::TokenStream {
                let field_ident = &element.field_ident;
                let inner_type_token_stream = &element.inner_type_token_stream;
                let from_snake_case = naming_constants::FromSnakeCase;
                let value_snake_case = naming_constants::ValueSnakeCase;
                let field_upper_camel_case = naming_constants::FieldUpperCamelCase;
                quote::quote! {
                    let #field_ident = match #value_snake_case.#field_ident {
                        Some(#value_snake_case) => Some(#field_upper_camel_case {
                            #value_snake_case: #inner_type_token_stream::#from_snake_case(
                                #value_snake_case.#value_snake_case,
                            )
                        }),
                        None => None
                    };
                }
            }
            generate_let_field_ident_value_option_field_from_option_field_with_serialize_deserialize_token_stream
        });
        quote::quote! {
            #primary_key_field_assignment_token_stream
            #(#fields_assignments_excluding_primary_key_token_stream)*
            Self{
                #(#self_init_fields_token_stream),*
            }
        }
    };
    let generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_token_stream = |
        value: &SynFieldWithAdditionalInfo<'_>,
        primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        match value.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
            postgresql_crud_common::FromOrTryFrom::From => proc_macro2::TokenStream::new(),
            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                //whats here match 
                let with_serialize_deserialize_error_named_token_stream = match postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::from(
                    &postgresql_crud_common::SupportedSqlxPostgresType::from(&value.rust_sqlx_map_to_postgres_type_variant)
                ) {
                    postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::SqlxPostgresType(_) => {
                        let value = value.rust_sqlx_map_to_postgres_type_variant.get_inner_type_with_serialize_deserialize_error_named_stringified("");
                        value.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    },
                    postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::OptionSupportedSqlxPostgresType(_) => {
                        let value = value.rust_sqlx_map_to_postgres_type_variant.get_inner_type_with_serialize_deserialize_error_named_without_option_stringified("");
                        value.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    },
                };
                let field_ident_upper_camel_case_token_stream = syn_ident_to_upper_camel_case_token_stream(value.field_ident);
                let eo_error_occurence_attribute_token_stream = proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence.to_attribute_view_token_stream();
                let code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence = token_patterns::CodeOccurenceSnakeCaseDoubleDotSpaceErrorOccurenceLibCodeOccurenceCodeOccurence;
                quote::quote!{
                    #field_ident_upper_camel_case_token_stream {
                        #eo_error_occurence_attribute_token_stream
                        #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: #with_serialize_deserialize_error_named_token_stream,
                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                    },//must use comma here
                }
            }
        }
    };
    let generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream = |
        value: &[SynFieldWithAdditionalInfo<'_>],
    | {
        value.iter().map(|element| {
            let field_ident = element.field_ident;
            generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_token_stream(
                element,
                &quote::quote!{#field_ident},
            )
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>()
    };
    let update_try_from_update_with_serialize_deserialize_error_named_token_stream = {
        let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
            &syn_field_with_additional_info_fields_named,
        );
        quote::quote! {
            #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
        }
    };
    let generate_let_field_ident_value_field_ident_from_or_try_from_token_stream = |
        element: &SynFieldWithAdditionalInfo<'_>,
        field_code_occurence_new_token_stream: &proc_macro2::TokenStream,
    | {
        let field_ident = &element.field_ident;
        let initialization_token_stream = {
            let inner_type_token_stream = &element.inner_type_token_stream;
            let inner_token_stream = quote::quote! {#value_snake_case.#field_ident};
            match element.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
                postgresql_crud_common::FromOrTryFrom::From => quote::quote!{#inner_type_token_stream::#from_snake_case(#inner_token_stream)},
                postgresql_crud_common::FromOrTryFrom::TryFrom => {
                    let field_ident_upper_camel_case_token_stream = syn_ident_to_upper_camel_case_token_stream(field_ident);
                    quote::quote!{
                        match #inner_type_token_stream::#try_from_snake_case(#inner_token_stream) {
                            Ok(#value_snake_case) => #value_snake_case,
                            Err(#error_snake_case) => {
                                return Err(Self::Error::#field_ident_upper_camel_case_token_stream {
                                    #primary_key_field_ident: #error_snake_case,
                                    #field_code_occurence_new_token_stream
                                });
                            }
                        }
                    }
                }
            }
        };
        quote::quote! {
            let #field_ident = #initialization_token_stream;
        }
    };
    let impl_std_convert_try_from_update_with_serialize_deserialize_for_update_token_stream = {
        let field_code_occurence_new_77f303a5_de96_4f73_a274_f2195cb619b1_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let fields_assignments_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
            fn generate_let_field_ident_value_option_field_from_or_try_from_option_field_with_serialize_deserialize_token_stream(
                element: &SynFieldWithAdditionalInfo<'_>,
                proc_macro_name_upper_camel_case_ident_stringified: &str,
                field_code_occurence_new_token_stream: &proc_macro2::TokenStream,
            ) -> proc_macro2::TokenStream {
                    let field_ident = &element.field_ident;
                    let initialization_token_stream = {
                        let inner_type_token_stream = &element.inner_type_token_stream;
                        let from_snake_case = naming_constants::FromSnakeCase;
                        let value_snake_case = naming_constants::ValueSnakeCase;
                        let field_upper_camel_case = naming_constants::FieldUpperCamelCase;
                        match element.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
                            postgresql_crud_common::FromOrTryFrom::From => {
                                quote::quote!{
                                    match #value_snake_case.#field_ident {
                                        Some(#value_snake_case) => Some(#field_upper_camel_case {
                                            #value_snake_case: #inner_type_token_stream::#from_snake_case(
                                                #value_snake_case.#value_snake_case,
                                            )
                                        }),
                                        None => None
                                    }
                                }
                            },
                            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                                let try_from_snake_case = naming_conventions::TryFromSnakeCase;
                                let error_snake_case = naming_constants::ErrorSnakeCase;
                                let field_ident_upper_camel_case_token_stream = {
                                    //todo fix syn_ident_to_upper_camel_case_token_stream error[E0434]: can't capture dynamic environment in a fn item
                                    let value = syn_ident_to_upper_camel_case_stringified(&element.field_ident);
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{value} {proc_macro_name_upper_camel_case_ident_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                let field_ident_snake_case_token_stream = {
                                    //todo its a temporal naming desicion. maybe it can be better
                                    let value = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&element.field_ident.to_string());
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                quote::quote!{
                                    match #value_snake_case.#field_ident {
                                        Some(#value_snake_case) => Some(#field_upper_camel_case {
                                            #value_snake_case: match #inner_type_token_stream::#try_from_snake_case(#value_snake_case.#value_snake_case) {
                                                Ok(#value_snake_case) => #value_snake_case,
                                                Err(#error_snake_case) => {
                                                    return Err(Self::Error::#field_ident_upper_camel_case_token_stream {
                                                        #field_ident_snake_case_token_stream: #error_snake_case,
                                                        #field_code_occurence_new_token_stream
                                                    });
                                                }
                                            }
                                        }),
                                        None => None
                                    }
                                }
                            }
                        }
                    };
                    quote::quote! {
                        let #field_ident = #initialization_token_stream;
                    }
            }
            generate_let_field_ident_value_option_field_from_or_try_from_option_field_with_serialize_deserialize_token_stream(
                element,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &field_code_occurence_new_77f303a5_de96_4f73_a274_f2195cb619b1_token_stream,
            )
        });
        let field_code_occurence_new_814b41f8_3219_4b62_bc0b_02a26d23b262_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let primary_key_let_field_ident_value_field_ident_from_or_try_from_token_stream = generate_let_field_ident_value_field_ident_from_or_try_from_token_stream(
            &primary_key_syn_field_with_additional_info,
            &field_code_occurence_new_814b41f8_3219_4b62_bc0b_02a26d23b262_token_stream,
        );
        quote::quote! {
            #primary_key_let_field_ident_value_field_ident_from_or_try_from_token_stream
            #(#fields_assignments_token_stream)*
            Ok(Self{
                #(#self_init_fields_token_stream),*
            })
        }
    };
    let impl_std_convert_from_update_for_update_with_serialize_deserialize_token_stream = {
        let fields_assignments_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
            fn generate_let_field_ident_value_option_field_with_serialize_deserialize_from_option_field_token_stream(element: &SynFieldWithAdditionalInfo<'_>) -> proc_macro2::TokenStream {
                let field_ident = &element.field_ident;
                let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
                let from_snake_case = naming_constants::FromSnakeCase;
                let value_snake_case = naming_constants::ValueSnakeCase;
                let field_with_serialize_deserialize_upper_camel_case = naming_conventions::FieldWithSerializeDeserializeUpperCamelCase;
                quote::quote! {
                    let #field_ident = match #value_snake_case.#field_ident {
                        Some(#value_snake_case) => Some(#field_with_serialize_deserialize_upper_camel_case {
                            #value_snake_case: #inner_type_with_serialize_deserialize_token_stream::#from_snake_case(
                                #value_snake_case.#value_snake_case,
                            )
                        }),
                        None => None
                    };
                }
            }
            generate_let_field_ident_value_option_field_with_serialize_deserialize_from_option_field_token_stream(element)
        });
        quote::quote! {
            let #primary_key_field_ident = #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#value_snake_case.#primary_key_field_ident);
            #(#fields_assignments_token_stream)*
            Self {
                #(#self_init_fields_token_stream),*
            }
        }
    };
    let generate_filter_no_payload_fields_token_stream = |operation: &Operation, source_token_stream: &proc_macro2::TokenStream|{
        let no_payload_fields_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
            &no_payload_fields_primary_key_syn_variant_wrapper,
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let none_fields_named_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|_|naming_constants::NoneUpperCamelCase);
        let match_fields_named_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
            let field_ident = &element.field_ident;
            quote::quote! {&#source_token_stream.#field_ident}
        });
        quote::quote! {
            if let (#(#none_fields_named_excluding_primary_key_token_stream),*) = (#(#match_fields_named_excluding_primary_key_token_stream),*) {
                let #error_0_token_stream = #source_token_stream.#primary_key_field_ident;
                #no_payload_fields_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
            }
        }
    };
    let executor_snake_case = naming_constants::ExecutorSnakeCase;
    let generate_match_postgres_transaction_rollback_await_token_stream = |
        operation: &Operation,
        postgresql_file: &'static str,
        postgresql_line: std::primitive::u32,
        postgresql_column: std::primitive::u32,
        row_and_rollback_file: &'static str,
        row_and_rollback_line: std::primitive::u32,
        row_and_rollback_column: std::primitive::u32,
    |{
        let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
            &postgresql_syn_variant_wrapper,
            postgresql_file,
            postgresql_line,
            postgresql_column,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let row_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
            &row_and_rollback_syn_variant_wrapper,
            row_and_rollback_file,
            row_and_rollback_line,
            row_and_rollback_column,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            match #executor_snake_case.#rollback_snake_case().await {
                Ok(_) => {
                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
                Err(#error_1_token_stream) => {
                    #row_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
            }
        }
    };
    let rows_snake_case = naming_constants::RowsSnakeCase;
    let generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream = |
        operation: &Operation,
        postgresql_file: &'static str,
        postgresql_line: std::primitive::u32,
        postgresql_column: std::primitive::u32,
        row_and_rollback_file: &'static str,
        row_and_rollback_line: std::primitive::u32,
        row_and_rollback_column: std::primitive::u32,
    |{
        let match_postgres_transaction_rollback_await_token_stream = generate_match_postgres_transaction_rollback_await_token_stream(
            &operation,
            postgresql_file,
            postgresql_line,
            postgresql_column,
            row_and_rollback_file,
            row_and_rollback_line,
            row_and_rollback_column,
        );
        quote::quote!{
            drop(#rows_snake_case);
            #match_postgres_transaction_rollback_await_token_stream
        }
    };
    let expected_primary_keys_snake_case = naming_conventions::ExpectedPrimaryKeysSnakeCase;
    let generate_non_existing_primary_keys_check_token_stream = |
        operation: &Operation,
        expected_primary_keys_token_stream: &dyn quote::ToTokens,
    |{
        let non_existing_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
            &non_existing_primary_keys_syn_variant_wrapper,
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let non_existing_primary_keys_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
            &non_existing_primary_keys_and_rollback_syn_variant_wrapper,
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            let #error_0_token_stream = #expected_primary_keys_token_stream.into_iter().fold(std::vec::Vec::new(),|mut #acc_snake_case, #element_snake_case| {
                if let false = #value_snake_case.contains(&#element_snake_case) {
                    #acc_snake_case.push(#element_snake_case);
                }
                #acc_snake_case
            });
            if let false = #error_0_token_stream.is_empty() {
                match #executor_snake_case.#rollback_snake_case().await {
                    Ok(_) => {
                        #non_existing_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                    Err(#error_1_token_stream) => {
                        #non_existing_primary_keys_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                }
            }
        }
    };
    let not_unique_fields_syn_variants_wrappers = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|
        proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
            &format!(
                "{}{}",
                naming_conventions::NotUniqueUpperCamelCase,
                proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&element.field_ident.to_string()),
            ),
            Some(not_unique_primary_key_syn_variant_wrapper.get_option_status_code().unwrap()),
            vec![
                (
                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                    &format!(
                        "{}_{}",
                        naming_conventions::NotUniqueSnakeCase,
                        &element.field_ident,
                    ),
                    {
                        let mut value = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                        value.push_value(syn::PathSegment {
                            ident: proc_macro2::Ident::new(&postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, proc_macro2::Span::call_site()),
                            arguments: syn::PathArguments::None,
                        });
                        value.push_punct(syn::token::PathSep {
                            spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                        });
                        value.push_value(syn::PathSegment {
                            ident: proc_macro2::Ident::new(
                                &postgresql_crud_common::SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(
                                    &postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)
                                ).to_string(),
                                proc_macro2::Span::call_site()
                            ),
                            arguments: syn::PathArguments::None,
                        });
                        value
                    }
                )
            ],
            &proc_macro_name_upper_camel_case_ident_stringified,
        )
    ).collect::<std::vec::Vec<proc_macro_helpers::construct_syn_variant::SynVariantWrapper>>();
    let wrap_into_value_token_stream = |content_token_stream: &proc_macro2::TokenStream|quote::quote!{
        let #value_snake_case = {
            #content_token_stream
        };
    };
    let generate_fetch_token_stream = |
        value_handle_token_stream: &proc_macro2::TokenStream,
        try_next_error_initialization_token_stream: &proc_macro2::TokenStream
    |{
        wrap_into_value_token_stream(&quote::quote!{
            let mut #rows_snake_case = #binded_query_snake_case.fetch(#executor_snake_case.as_mut());
            let mut #acc_snake_case = std::vec::Vec::new();
            while let Some(#value_snake_case) = match { 
                #use_futures_try_stream_ext_token_stream; 
                #rows_snake_case.try_next() 
            }.await
            {
                Ok(#value_snake_case) => match #value_snake_case {
                    Some(#value_snake_case) => #value_handle_token_stream,
                    None => None,
                },
                Err(#error_0_token_stream) => {
                    #try_next_error_initialization_token_stream
                }
            } 
            {
                #acc_snake_case.push(#value_snake_case);
            }
            #acc_snake_case
        })
        
    };
    let generate_fetch_one_token_stream = |
        value_handle_token_stream: &proc_macro2::TokenStream,
        fetch_one_error_initialization_token_stream: &proc_macro2::TokenStream
    |{
        wrap_into_value_token_stream(&quote::quote!{
            match #binded_query_snake_case.fetch_one(#executor_snake_case.as_mut()).await {
                Ok(#value_snake_case) => {
                    #value_handle_token_stream
                },
                Err(#error_0_token_stream) => {
                    #fetch_one_error_initialization_token_stream
                }
            }
        })
    };
    let generate_sqlx_row_try_get_primary_key_token_stream = |
        ok_token_stream: &proc_macro2::TokenStream,
        err_token_stream: &proc_macro2::TokenStream,
    |{
        let primary_key_field_ident_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
            &primary_key_field_ident.to_string(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            match #sqlx_row::try_get::<#primary_key_original_type_token_stream, #ref_std_primitive_str>(&#value_snake_case, #primary_key_field_ident_quotes_token_stream) {
                Ok(#value_snake_case) => #ok_token_stream,
                Err(#error_0_token_stream) => {
                    #err_token_stream
                }
            }
        }
    };
    let wrap_content_into_postgresql_transaction_begin_commit_value_token_stream = |
        operation: &Operation,
        content_token_stream: &proc_macro2::TokenStream,
    |{
        let postgres_transaction_begin_token_stream = {
            let sqlx_acquire = token_patterns::SqlxAcquire;
            let begin_snake_case = naming_constants::BeginSnakeCase;
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                &postgresql_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                let mut #executor_snake_case = match {
                    use #sqlx_acquire;
                    #executor_snake_case.#begin_snake_case()
                }
                .await
                {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                };
            }
        };
        let postgres_transaction_commit_token_stream = {
            let commit_snake_case = naming_constants::CommitSnakeCase;
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                &postgresql_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote!{
                if let Err(#error_0_token_stream) = #executor_snake_case.#commit_snake_case().await {
                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
            }
        };
        quote::quote! {
            #postgres_transaction_begin_token_stream
            #content_token_stream
            #postgres_transaction_commit_token_stream
            #value_snake_case
        }
    };
    let generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_primary_key_token_stream = |
        value: &[SynFieldWithAdditionalInfo<'_>],
        primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
    | {
        value.iter().map(|element| generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_token_stream(
            element,
            primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
        )).collect::<std::vec::Vec<proc_macro2::TokenStream>>()
    };
    let generate_impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = |
        operation: &Operation,
        content_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
            &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation),
            &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation),
            &content_token_stream,
        )
    };
    let generate_impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_and_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_token_stream = |
        operation: &Operation,
        impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_content_token_stream: &proc_macro2::TokenStream,
        impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_content_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        let impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream = proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
            &naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(operation),
            &naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(operation),
            &impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_content_token_stream,
        );
        let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = generate_impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream(
            &operation,
            &impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_content_token_stream,
        );
        quote::quote! {
            #impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream
            #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream
        }
    };
    let generate_error_occurence_variant_token_stream = |error_variant: &syn::Variant| -> proc_macro2::TokenStream {
        let variant_ident = &error_variant.ident;
        let fields_named = if let syn::Fields::Named(fields_named) = &error_variant.fields {
            fields_named
        }
        else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
        };
        let fields_mapped_into_token_stream = fields_named.named.iter().map(|field|{
            let field_ident = field.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_constants::FIELD_IDENT_IS_NONE
                )
            });
            let error_occurence_attribute = if *field_ident == *naming_conventions::CodeOccurenceSnakeCase.to_string() {
                proc_macro2::TokenStream::new()
            }
            else {
                let mut error_occurence_attribute: Option<proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute> = None;
                for element in &field.attrs {
                    if element.path().segments.len() == 1 {
                        let segment = element.path().segments.first().unwrap_or_else(|| {panic!("{proc_macro_name_upper_camel_case_ident_stringified} element.path().segments.get(0) is None")});
                        if let Ok(value) = {
                            use std::str::FromStr;
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::from_str(&segment.ident.to_string())
                        } {
                            match error_occurence_attribute {
                                Some(value) => panic!(
                                    "{proc_macro_name_upper_camel_case_ident_stringified} duplicated attributes ({}) are not supported", 
                                    proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&value)
                                ),
                                None => {
                                    error_occurence_attribute = Some(value);
                                }
                            }
                        }
                    }
                }
                error_occurence_attribute.map_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {variant_ident} no supported attribute"), |value| value.to_attribute_view_token_stream())
            };
            let field_type = &field.ty;
            quote::quote! {
                #error_occurence_attribute
                #field_ident: #field_type
            }
        });
        quote::quote! {
            #variant_ident {
                #(#fields_mapped_into_token_stream),*
            }
        }
    };
    let desirable_upper_camel_case = naming_constants::DesirableUpperCamelCase;
    let generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = |
        operation: &Operation,
        desirable_type_token_stream: &proc_macro2::TokenStream,
        type_variants_from_request_response_syn_variants: &std::vec::Vec<syn::Variant>,
    | -> proc_macro2::TokenStream {
        let try_operation_route_logic_response_variants_token_stream = {
            let derive_debug_serde_serialize_serde_deserialize = token_patterns::DeriveDebugSerdeSerializeSerdeDeserialize;
            let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(operation);
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(|element|proc_macro_helpers::error_occurence::generate_serialize_deserialize_version_of_named_syn_variant(
                &element,
                &proc_macro_name_upper_camel_case_ident_stringified,
            ));
            quote::quote! {
                #derive_debug_serde_serialize_serde_deserialize
                pub enum #try_operation_route_logic_response_variants_upper_camel_case_token_stream {
                    #desirable_upper_camel_case(#desirable_type_token_stream),
                    #(#variants_token_stream),*
                }
            }
        };
        let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = {
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(|element| {
                let variant_ident = &element.ident;
                let fields_named = if let syn::Fields::Named(fields_named) = &element.fields {
                    fields_named
                }
                else {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
                };
                let fields_mapped_into_token_stream = {
                    let fields_token_stream = fields_named.named.iter().map(|field|{
                        &field.ident
                    });
                    quote::quote! {#(#fields_token_stream),*}
                };
                let try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseTokenStream::try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream(operation);
                quote::quote! {
                    #try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream::#variant_ident {
                        #fields_mapped_into_token_stream
                    } => Self::#variant_ident {
                        #fields_mapped_into_token_stream
                    }
                }
            });
            let try_operation_route_logic_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedUpperCamelCaseTokenStream::try_self_route_logic_error_named_upper_camel_case_token_stream(operation);
            let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(operation);
            let into_serialize_deserialize_version_snake_case = naming_conventions::IntoSerializeDeserializeVersionSnakeCase;
            let value_snake_case = naming_constants::ValueSnakeCase;
            proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                &try_operation_route_logic_error_named_upper_camel_case_token_stream,
                &try_operation_route_logic_response_variants_upper_camel_case_token_stream,
                &quote::quote!{
                    match #value_snake_case.#into_serialize_deserialize_version_snake_case() {
                        #(#variants_token_stream),*
                    }
                }
            )
        };
        let try_operation_route_logic_error_named_token_stream = {
            let try_operation_route_logic_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedUpperCamelCaseTokenStream::try_self_route_logic_error_named_upper_camel_case_token_stream(operation);
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(generate_error_occurence_variant_token_stream);
            quote::quote! {
                #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #try_operation_route_logic_error_named_upper_camel_case_token_stream {
                    #(#variants_token_stream),*
                }
            }
        };
        quote::quote! {
            #try_operation_route_logic_response_variants_token_stream
            #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
            #try_operation_route_logic_error_named_token_stream
        }
    };
    let generate_impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream = |
        operation: &Operation,
        content_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
            &naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(operation),
            &naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(operation),
            &naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(operation),
            &content_token_stream,
        )
    };
    let generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream = |
        operation: &Operation,
        content_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
        let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream =     naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(operation);
        quote::quote! {
            #derive_debug_thiserror_error_occurence
            pub enum #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                #content_token_stream
            }
        }
    };
    let generate_impl_std_convert_try_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream =|
        operation: &Operation,
        content_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
            &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation),
            &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation),
            &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(operation),
            &content_token_stream,
        )
    };
    let generate_impl_std_convert_from_self_payload_element_with_serialize_deserialize_for_self_payload_element_token_stream = |
        operation: &Operation,
        content_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
            &naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(operation),
            &naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(operation),
            &content_token_stream,
        )
    };
    let generate_impl_std_convert_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream = |
        operation: &Operation,
        content_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
            &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation),
            &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation),
            &content_token_stream,
        )
    };
    let generate_parameters_pattern_token_stream = |
        operation: &Operation,
        payload_token_stream: proc_macro2::TokenStream,
        payload_with_serialize_deserialize_token_stream: proc_macro2::TokenStream,
        impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream: proc_macro2::TokenStream,
        impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream: proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        let parameters_token_stream = {
            let operation_parameters_upper_camel_case_token_stream = naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
            let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
            let payload_snake_case = naming_constants::PayloadSnakeCase;
            let derive_debug = token_patterns::DeriveDebug;
            quote::quote! {
                #derive_debug
                pub struct #operation_parameters_upper_camel_case_token_stream {//todo maybe not need additional info, so parameters wrapper potentially can be removed
                    pub #payload_snake_case: #operation_payload_upper_camel_case_token_stream,
                }
            }
        };
        quote::quote! {
            #payload_token_stream
            #payload_with_serialize_deserialize_token_stream
            #impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
            #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream
            #parameters_token_stream
        }
    };
    let generate_payload_and_payload_with_serialize_deserialize_create_many_or_update_many = |
        operation: &Operation,
        fields_token_stream: &proc_macro2::TokenStream,
        fields_with_serialize_deserialize_token_stream: &proc_macro2::TokenStream,
    | -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let payload_token_stream = {
            let operation_payload_element_token_stream = {
                let derive_debug = token_patterns::DeriveDebug;
                let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(operation);
                quote::quote! {
                    #derive_debug
                    pub struct #operation_payload_element_upper_camel_case_token_stream {
                        #fields_token_stream
                    }
                }
            };
            let operation_payload_token_stream = {
                let derive_debug = token_patterns::DeriveDebug;
                let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
                let std_vec_vec_operation_payload_element_token_stream = operation.std_vec_vec_self_payload_element_token_stream();
                quote::quote! {
                    #derive_debug
                    pub struct #operation_payload_upper_camel_case_token_stream(pub #std_vec_vec_operation_payload_element_token_stream);
                }
            };
            quote::quote! {
                #operation_payload_element_token_stream
                #operation_payload_token_stream
            }
        };
        let payload_with_serialize_deserialize_token_stream = {
            let operation_payload_element_with_serialize_deserialize_token_stream = {
                let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
                let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(operation);
                quote::quote! {
                    #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
                    pub struct #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream {
                        #fields_with_serialize_deserialize_token_stream
                    }
                }
            };
            let operation_payload_with_serialize_deserialize_token_stream = {
                let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
                let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
                let std_vec_vec_operation_payload_element_with_serialize_deserialize_token_stream = operation.std_vec_vec_self_payload_element_with_serialize_deserialize_token_stream();
                quote::quote! {
                    #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
                    pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream(pub #std_vec_vec_operation_payload_element_with_serialize_deserialize_token_stream);
                }
            };
            quote::quote! {
                #operation_payload_element_with_serialize_deserialize_token_stream
                #operation_payload_with_serialize_deserialize_token_stream
            }
        };
        (
            payload_token_stream,
            payload_with_serialize_deserialize_token_stream
        )
    };
    let generate_operation_payload_token_stream = |
        operation: &Operation,
        fields_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        let derive_debug = token_patterns::DeriveDebug;
        let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
        quote::quote! {
            #derive_debug
            pub struct #operation_payload_upper_camel_case_token_stream {
                #fields_token_stream
            }
        }
    };
    let generate_payload_with_serialize_deserialize_token_stream = |
        operation: &Operation,
        fields_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
        let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
        quote::quote! {
            #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
            pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                #fields_token_stream
            }
        }
    };
    let generate_type_variants_from_request_response_syn_variants = |
        syn_variants: &std::vec::Vec<&syn::Variant>,
        from_or_try_from: &postgresql_crud_common::FromOrTryFrom,
        self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper: &proc_macro_helpers::construct_syn_variant::SynVariantWrapper,
        operation: &Operation,
        syn_derive_input: &syn::DeriveInput,
    | -> std::vec::Vec<syn::Variant> {
        let mut type_variants_from_request_response_syn_variants = std::vec::Vec::new();
        for element in syn_variants {
            type_variants_from_request_response_syn_variants.push((*element).clone());
        }
        let operation_additional_error_variants = generate_additional_error_variants(
            &syn_derive_input,
            operation.to_additional_error_variants(),
        );
        for element in operation_additional_error_variants {
            type_variants_from_request_response_syn_variants.push(element.clone());
        }
        if *from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
            type_variants_from_request_response_syn_variants.push(self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper.get_syn_variant().clone());
        }
        type_variants_from_request_response_syn_variants
    };
    let generate_try_operation_error_named_token_stream = |
        operation: &Operation,
        common_http_request_syn_variants: &std::vec::Vec<syn::Variant>,
    | -> proc_macro2::TokenStream {
        let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
        let syn_variants = {
            let mut value = std::vec::Vec::with_capacity(common_http_request_syn_variants.len() + 1);
            for element in common_http_request_syn_variants {
                value.push(element.clone());
            }
            value.push({
                let try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified = naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseStringified::try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified(operation);
                proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
                    &try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified,
                    None,
                    vec![
                        (
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                            &naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCaseStringified::try_self_route_logic_error_named_with_serialize_deserialize_snake_case_stringified(operation),
                            proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                                &[&try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified],
                                &proc_macro_name_upper_camel_case_ident_stringified
                            ),
                        )
                    ],
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ).get_syn_variant().clone()
            });
            value
        };
        let variants_token_stream = syn_variants.iter().map(generate_error_occurence_variant_token_stream);
        quote::quote! {
            #derive_debug_thiserror_error_occurence
            pub enum #try_operation_error_named_upper_camel_case_token_stream {
                #(#variants_token_stream),*
            }
        }
    };
    let generate_try_operation_route_logic_token_stream = |
        operation: &Operation,
        syn_derive_input: &syn::DeriveInput,
        common_additional_route_logic_token_stream: &proc_macro2::TokenStream,
        parameters_logic_token_stream: &proc_macro2::TokenStream,
        expected_updated_primary_keys_token_stream: &proc_macro2::TokenStream,
        query_string_token_stream: &proc_macro2::TokenStream,
        binded_query_token_stream: &proc_macro2::TokenStream,
        postgresql_logic_token_stream: &proc_macro2::TokenStream,
        eprintln_error_token_stream: &proc_macro2::TokenStream,
        check_body_size_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream: &proc_macro2::TokenStream,
        postgresql_syn_variant_wrapper_initialization_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        let try_operation_route_logic_snake_case_token_stream = naming_conventions::TrySelfRouteLogicSnakeCaseTokenStream::try_self_route_logic_snake_case_token_stream(operation);
        let request_snake_case = naming_constants::RequestSnakeCase;
        let parameters_snake_case = naming_constants::ParametersSnakeCase;
        let app_state_snake_case = naming_conventions::AppStateSnakeCase;
        let query_string_snake_case = naming_conventions::QueryStringSnakeCase;
        let binded_query_snake_case = naming_conventions::BindedQuerySnakeCase;
        let value_snake_case = naming_constants::ValueSnakeCase;
        let status_code_snake_case = naming_conventions::StatusCodeSnakeCase;
        let error_0_token_stream = token_patterns::Error0;
        let request_parts_preparation_token_stream = {
            quote::quote! {
                let (parts, #body_snake_case) = #request_snake_case.into_parts();
                let headers = parts.headers;
                let body_bytes = match route_validators::check_body_size::check_body_size(#body_snake_case, *#app_state_snake_case.get_maximum_size_of_http_body_in_bytes()).await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        let #status_code_snake_case = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&#error_0_token_stream);
                        #check_body_size_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                    }
                };
            }
        };
        let additional_validators_token_stream = {
            let operation_additional_route_logic_token_stream = proc_macro_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(
                &syn_derive_input.attrs,
                &operation.to_additional_route_logic().generate_path_to_attribute(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #common_additional_route_logic_token_stream
                #operation_additional_route_logic_token_stream
            }
        };
        let acquire_pool_and_connection_token_stream = {
            let try_operation_route_logic_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedUpperCamelCaseTokenStream::try_self_route_logic_error_named_upper_camel_case_token_stream(operation);
            let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(operation);
            let value_snake_case = naming_constants::ValueSnakeCase;
            let error_snake_case = naming_constants::ErrorSnakeCase;
            let from_snake_case = naming_constants::FromSnakeCase;
            let executor_snake_case = naming_constants::ExecutorSnakeCase;
            let pool_connection_snake_case = naming_conventions::PoolConnectionSnakeCase;
            let into_response_snake_case = naming_conventions::IntoResponseSnakeCase;
            quote::quote! {
                let mut #pool_connection_snake_case = match #app_state_snake_case.get_postgres_pool().acquire().await {//todo find out difference between acquire and try_acquire
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        let #error_snake_case = #try_operation_route_logic_error_named_upper_camel_case_token_stream::#postgresql_syn_variant_wrapper_initialization_token_stream;
                        #eprintln_error_token_stream;//todo reuse it as token_stream    
                        let mut res = axum::response::IntoResponse::#into_response_snake_case(axum::Json(#try_operation_route_logic_response_variants_upper_camel_case_token_stream::#from_snake_case(#error_snake_case)));
                        *res.status_mut() = axum::http::StatusCode::CREATED;//todo
                        // *res.headers_mut() = axum::http::HeaderMap::new();
                        return res;
                    }
                };
                let #executor_snake_case = match sqlx::Acquire::acquire(&mut #pool_connection_snake_case).await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        let #error_snake_case = #try_operation_route_logic_error_named_upper_camel_case_token_stream::#postgresql_syn_variant_wrapper_initialization_token_stream;
                        #eprintln_error_token_stream;   
                        let mut res = axum::response::IntoResponse::#into_response_snake_case(axum::Json(#try_operation_route_logic_response_variants_upper_camel_case_token_stream::#from_snake_case(#error_snake_case)));
                        *res.status_mut() = axum::http::StatusCode::CREATED;//todo
                        // *res.headers_mut() = axum::http::HeaderMap::new();
                        return res;
                    }
                };
            }
        };
        let desirable_response_creation_token_stream = {
            let into_response_snake_case = naming_conventions::IntoResponseSnakeCase;
            let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(operation);
            let status_code_token_stream = &operation.desirable_status_code().to_axum_http_status_code_token_stream();
            quote::quote! {
                let mut #response_snake_case = axum::response::IntoResponse::#into_response_snake_case(
                    axum::Json(
                        #try_operation_route_logic_response_variants_upper_camel_case_token_stream::#desirable_upper_camel_case(#value_snake_case)
                    )
                );
                *#response_snake_case.status_mut() = #status_code_token_stream;
                return #response_snake_case;
            }
        };
        quote::quote! {
            // // #swagger_open_api_token_stream
            pub async fn #try_operation_route_logic_snake_case_token_stream(
                #app_state_snake_case: axum::extract::State<
                    crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits,
                >,
                #request_snake_case: axum::extract::Request,
            ) -> axum::response::Response {
                #request_parts_preparation_token_stream
                #additional_validators_token_stream
                #parameters_logic_token_stream
                println!("{:#?}", #parameters_snake_case);
                #expected_updated_primary_keys_token_stream
                let #query_string_snake_case = #query_string_token_stream;
                println!("{}", #query_string_snake_case);
                let #binded_query_snake_case = {
                    #binded_query_token_stream
                };
                #acquire_pool_and_connection_token_stream
                let #value_snake_case = {
                    #postgresql_logic_token_stream
                };
                #desirable_response_creation_token_stream
            }
        }
    };
    let generate_parameters_logic_token_stream = |
        operation: &Operation,
        from_or_try_from: &postgresql_crud_common::FromOrTryFrom,
        serde_json_syn_variant_wrapper: &proc_macro_helpers::construct_syn_variant::SynVariantWrapper,
        self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper: &proc_macro_helpers::construct_syn_variant::SynVariantWrapper,
        operation_payload_with_serialize_deserialize_check_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        let body_bytes_snake_case = naming_conventions::BodyBytesSnakeCase;
        let try_or_try_from_operation_payload_upper_camel_case_token_stream = {
            let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
            match from_or_try_from {
                postgresql_crud_common::FromOrTryFrom::From => quote::quote! {#operation_payload_upper_camel_case_token_stream::#from_snake_case(#value_snake_case)},
                postgresql_crud_common::FromOrTryFrom::TryFrom => {
                    let self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        match #operation_payload_upper_camel_case_token_stream::#try_from_snake_case(#value_snake_case) {
                            Ok(#value_snake_case) => #value_snake_case,
                            Err(#error_0_token_stream) => {
                                #self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                            }
                        }
                    }
                },
            }
        };
        let serde_json_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
            &serde_json_syn_variant_wrapper,
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let operation_parameters_upper_camel_case_token_stream = naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
        quote::quote! {
            let #parameters_snake_case = #operation_parameters_upper_camel_case_token_stream {
                //todo maybe use serde json parsing instead of axum. (coz less info)
                #payload_snake_case: match serde_json::from_slice::<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>(
                    &#body_bytes_snake_case,
                ) {
                    Ok(#value_snake_case) => {
                        let #value_snake_case = #try_or_try_from_operation_payload_upper_camel_case_token_stream;
                        #operation_payload_with_serialize_deserialize_check_token_stream
                        #value_snake_case
                    },
                    Err(#error_0_token_stream) => {
                        #serde_json_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                    }
                },
            };
        }
    };
    let generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = |
        operation: &Operation,
        variants_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream =     naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(operation);
        quote::quote! {
            #derive_debug_thiserror_error_occurence
            pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                #variants_token_stream
            }
        }
    };
    let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
        match element.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
            postgresql_crud_common::FromOrTryFrom::From => proc_macro2::TokenStream::new(),
            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                let where_with_serialize_deserialize_error_named_token_stream = {
                    let value = element.rust_sqlx_map_to_postgres_type_variant.get_where_with_serialize_deserialize_error_named_stringified("");
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let field_ident_upper_camel_case_token_stream = syn_ident_to_upper_camel_case_token_stream(element.field_ident);
                quote::quote!{
                    #field_ident_upper_camel_case_token_stream {
                        #eo_error_occurence_attribute_token_stream
                        #primary_key_field_ident_token_stream: #where_with_serialize_deserialize_error_named_token_stream,
                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                    },//must use comma here
                }
            }
        }
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let generate_let_field_ident_value_option_vec_with_serialize_deserialize_token_stream = |element: &SynFieldWithAdditionalInfo<'_>| -> proc_macro2::TokenStream {
        let field_ident = &element.field_ident;
        let where_inner_type_with_serialize_deserialize_token_stream = &element.where_inner_type_with_serialize_deserialize_token_stream;
        quote::quote! {
            let #field_ident = match #value_snake_case.#field_ident {
                Some(#value_snake_case) => Some(#value_snake_case.into_iter().map(|#element_snake_case|#where_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#element_snake_case)).collect()),
                None => None
            };
        }
    };
    let generate_let_field_ident_value_inner_type_with_serialize_deserialize_from_token_stream = |element: &SynFieldWithAdditionalInfo<'_>| -> proc_macro2::TokenStream {
        let field_ident = &element.field_ident;
        let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
        quote::quote! {
            let #field_ident = #inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#value_snake_case.#field_ident);//todo from or try_from
        }
    };
    let generate_option_vec_where_inner_type_from_or_try_from_option_vec_where_inner_type_with_serialize_deserialize_token_stream = |
        value: &SynFieldWithAdditionalInfo<'_>,
        primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        let field_ident = &value.field_ident;
        let inner_token_stream = quote::quote! {value.#field_ident};
        let where_inner_type_token_stream = &value.where_inner_type_token_stream;
        let initialization_token_stream = match value.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
            postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                match #inner_token_stream {
                    Some(#value_snake_case) => Some(#value_snake_case.into_iter().map(|#element_snake_case|#where_inner_type_token_stream::#from_snake_case(#element_snake_case)).collect()),
                    None => None,
                }
            },
            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                let field_code_occurence_new_68674e53_54cf_4cfe_b532_2e4aecda32c5_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let field_ident_upper_camel_case_token_stream = syn_ident_to_upper_camel_case_token_stream(value.field_ident);
                quote::quote!{
                    match #inner_token_stream {
                        Some(#value_snake_case) => {
                            let mut #acc_snake_case = std::vec::Vec::with_capacity(#value_snake_case.len());
                            for #element_snake_case in #value_snake_case {
                                match #where_inner_type_token_stream::try_from(#element_snake_case) {
                                    Ok(#value_snake_case) => {
                                        #acc_snake_case.push(#value_snake_case);
                                    }
                                    Err(#error_snake_case) => {
                                        return Err(Self::Error::#field_ident_upper_camel_case_token_stream {
                                            #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: #error_snake_case,
                                            #field_code_occurence_new_68674e53_54cf_4cfe_b532_2e4aecda32c5_token_stream,
                                        });
                                    }
                                }
                            }
                            Some(#acc_snake_case)
                        }
                        None => None,
                    }
                }
            }
        };
        quote::quote! {
            let #field_ident = #initialization_token_stream;
        }
    };
    let generate_let_field_ident_value_inner_type_from_token_stream = |element: &SynFieldWithAdditionalInfo<'_>| -> proc_macro2::TokenStream {
        let field_ident = &element.field_ident;
        let inner_type_token_stream = &element.inner_type_token_stream;
        quote::quote! {
            let #field_ident = #inner_type_token_stream::#from_snake_case(#value_snake_case.#field_ident);
        }
    };
    let generate_field_ident_field_type_with_serialize_deserialize_token_stream = |element: &SynFieldWithAdditionalInfo<'_>| -> proc_macro2::TokenStream {
        let field_ident = &element.field_ident;
        let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
        quote::quote! {
            #field_ident: #inner_type_with_serialize_deserialize_token_stream
        }
    };
    let generate_pub_field_ident_field_type_token_stream = |element: &SynFieldWithAdditionalInfo<'_>| -> proc_macro2::TokenStream {
        let field_ident = &element.field_ident;
        let inner_type_token_stream = &element.inner_type_token_stream;
        quote::quote! {
            pub #field_ident: #inner_type_token_stream
        }
    };
    let expected_response_snake_case = naming_conventions::ExpectedResponseSnakeCase;
    let generate_try_operation_token_stream = |
        operation: &Operation,
        table_name_stringified: &std::primitive::str,
        type_variants_from_request_response_syn_variants: &[syn::Variant],
        result_ok_type_token_stream: &proc_macro2::TokenStream,
        payload_check_token_stream: &proc_macro2::TokenStream,
        operation_payload_with_serialize_deserialize_initialization_token_stream: &proc_macro2::TokenStream,
        desirable_from_or_try_from_desirable_with_serialize_deserialize_token_stream: &proc_macro2::TokenStream,
        reqwest_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
        deserialize_response_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
        failed_to_get_response_text_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
        serde_json_to_string_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    | -> proc_macro2::TokenStream {
        let try_operation_snake_case_token_stream = naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(operation);
        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
        let operation_parameters_upper_camel_case_token_stream = naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
        let payload_token_stream = quote::quote! {
            let #payload_snake_case = {
                #payload_check_token_stream
                let #value_snake_case = #operation_payload_with_serialize_deserialize_initialization_token_stream;
                match serde_json::to_string(&#value_snake_case) {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_syn_variant_initialization_token_stream);
                    }
                }
            };
        };
        let url_snake_case = naming_constants::UrlSnakeCase;
        let server_location_snake_case = naming_conventions::ServerLocationSnakeCase;
        let url_token_stream = {
            let url_handle_token_stream = naming_conventions::UrlHandleSelfSnakeCaseTokenStream::url_handle_self_snake_case_token_stream(operation, table_name_stringified);
            quote::quote! {
                let #url_snake_case = format!(
                    #url_handle_token_stream,
                    #server_location_snake_case,
                );
            }
        };
        let future_snake_case = naming_constants::FutureSnakeCase;
        let future_token_stream = {
            let operation_http_method_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(
                &operation.http_method(),
            );
            let reqwest_client_new_token_stream = quote::quote! {reqwest::Client::new()};
            let commit_header_addition_token_stream = quote::quote! {
                .header(
                    &postgresql_crud::CommitSnakeCase.to_string(),//todo remove it
                    git_info::PROJECT_GIT_INFO.commit,
                )
            };
            let application_json_quotes_token_stream =
                proc_macro_common::generate_quotes::token_stream(
                    "application/json",
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
            let content_type_application_json_header_addition_token_stream = quote::quote! {
                .header(reqwest::header::CONTENT_TYPE, #application_json_quotes_token_stream)
            };
            quote::quote! {
                let #future_snake_case = #reqwest_client_new_token_stream
                    .#operation_http_method_snake_case_token_stream(&#url_snake_case)
                    #commit_header_addition_token_stream
                    #content_type_application_json_header_addition_token_stream
                    .#body_snake_case(#payload_snake_case)
                    .send();
            }
        };
        let response_token_stream = quote::quote! {
            let #response_snake_case = match #future_snake_case.await {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_0_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#reqwest_syn_variant_initialization_token_stream);
                }
            };
        };
        let status_code_token_stream = quote::quote! {
            let #error_0_token_stream = #response_snake_case.status();
        };
        let headers_token_stream = quote::quote! {
            let #error_1_token_stream = #response_snake_case.headers().clone();
        };
        let response_text_token_stream = quote::quote! {
            let #error_2_token_stream = match #response_snake_case.text().await {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_2_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#failed_to_get_response_text_syn_variant_initialization_token_stream);
                }
            };
        };
        let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(operation);
        let expected_response_token_stream = quote::quote! {
            let #expected_response_snake_case = match serde_json::from_str::<#try_operation_route_logic_response_variants_upper_camel_case_token_stream>(&#error_2_token_stream) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_3_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#deserialize_response_syn_variant_initialization_token_stream);
                }
            };
        };
        let try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseTokenStream::try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream(operation);
        let try_operation_route_logic_error_named_with_serialize_deserialize_snake_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCaseTokenStream::try_self_route_logic_error_named_with_serialize_deserialize_snake_case_token_stream(operation);
        let try_operation_route_logic_error_named_with_serialize_deserialize_token_stream = {
            let try_operation_route_logic_response_variants_to_try_operation_route_logic_error_named_with_serialize_deserialize = type_variants_from_request_response_syn_variants.iter().map(|element|{
                let variant_ident = &element.ident;
                let fields_idents_token_stream = if let syn::Fields::Named(fields_named) = &element.fields {
                    let fields_idents = fields_named.named.iter().map(|field|&field.ident);
                    quote::quote! {#(#fields_idents),*}
                }
                else {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
                };
                quote::quote! {
                    #try_operation_route_logic_response_variants_upper_camel_case_token_stream::#variant_ident {
                        #fields_idents_token_stream
                    } => #try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream::#variant_ident { #fields_idents_token_stream }
                }
            });
            quote::quote! {
                let #try_operation_route_logic_error_named_with_serialize_deserialize_snake_case_token_stream = match #expected_response_snake_case {
                    #try_operation_route_logic_response_variants_upper_camel_case_token_stream::#desirable_upper_camel_case(#value_snake_case) => {
                        let #value_snake_case = #desirable_from_or_try_from_desirable_with_serialize_deserialize_token_stream;
                        return Ok(#value_snake_case);
                    },
                    #(#try_operation_route_logic_response_variants_to_try_operation_route_logic_error_named_with_serialize_deserialize),*
                };
            }
        };
        let return_error_token_stream = {
            let field_code_occurence_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                Err(#try_operation_error_named_upper_camel_case_token_stream::#try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream {
                    #try_operation_route_logic_error_named_with_serialize_deserialize_snake_case_token_stream,
                    #field_code_occurence_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_token_stream,
                })
            }
        };
        quote::quote! {
            pub async fn #try_operation_snake_case_token_stream(
                #server_location_snake_case: #ref_std_primitive_str,//todo rename as endpoint location
                #parameters_snake_case: #operation_parameters_upper_camel_case_token_stream,
            ) -> Result<#result_ok_type_token_stream, #try_operation_error_named_upper_camel_case_token_stream> {
                #payload_token_stream
                #url_token_stream
                #future_token_stream
                #response_token_stream
                #status_code_token_stream
                #headers_token_stream
                #response_text_token_stream
                #expected_response_token_stream
                #try_operation_route_logic_error_named_with_serialize_deserialize_token_stream
                #return_error_token_stream
            }
        }
    };
    let (create_many_token_stream, create_many_test_token_stream) = {
        let operation = Operation::CreateMany;
        let self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper = operation.generate_self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper(
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let expected_length_snake_case = naming_conventions::ExpectedLengthSnakeCase;
        let got_length_snake_case = naming_conventions::GotLengthSnakeCase;
        let std_primitive_usize_syn_punctuated_punctuated = proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
            &["std", "primitive", "usize"],
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let unexpected_rows_length_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
            &naming_conventions::UnexpectedRowsLengthUpperCamelCase,
            Some(proc_macro_helpers::status_code::StatusCode::InternalServerError500),
            vec![
                (
                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                    &expected_length_snake_case,
                    std_primitive_usize_syn_punctuated_punctuated.clone(),
                ),
                (
                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                    &got_length_snake_case,
                    std_primitive_usize_syn_punctuated_punctuated.clone(),
                )
            ],
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let unexpected_rows_length_and_rollback_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
            &naming_conventions::UnexpectedRowsLengthAndRollbackUpperCamelCase,
            Some(proc_macro_helpers::status_code::StatusCode::InternalServerError500),
            vec![
                (
                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                    &expected_length_snake_case,
                    std_primitive_usize_syn_punctuated_punctuated.clone(),
                ),
                (
                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                    &got_length_snake_case,
                    std_primitive_usize_syn_punctuated_punctuated.clone(),
                ),
                //todo reuse vec elements
                (
                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                    &rollback_snake_case,
                    sqlx_error_syn_punctuated_punctuated.clone(),
                ),
            ],
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 1);
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value.push(&checked_add_syn_variant_wrapper.get_syn_variant());
                value.push(&row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(&unexpected_rows_length_syn_variant_wrapper.get_syn_variant());
                value.push(&unexpected_rows_length_and_rollback_syn_variant_wrapper.get_syn_variant());
                value
            },
            &fields_named_excluding_primary_key_from_or_try_from,
            &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
            &operation,
            &syn_derive_input,
        );
        let parameters_token_stream = {
            let (
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream
            ) = {
                generate_payload_and_payload_with_serialize_deserialize_create_many_or_update_many(
                    &operation,
                    &generate_fields_named_excluding_primary_key_token_stream(generate_pub_field_ident_field_type_token_stream),
                    &generate_fields_named_excluding_primary_key_token_stream(generate_field_ident_field_type_with_serialize_deserialize_token_stream),
                )
            };
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = match &fields_named_excluding_primary_key_from_or_try_from {
                postgresql_crud_common::FromOrTryFrom::From => {
                    let impl_std_convert_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream = generate_impl_std_convert_from_self_payload_element_with_serialize_deserialize_for_self_payload_element_token_stream(
                        &operation,
                        &{
                            let fields_assignment_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(generate_let_field_ident_value_inner_type_from_token_stream);
                            quote::quote! {
                                #(#fields_assignment_excluding_primary_key_token_stream)*
                                Self {
                                    #(#fields_idents_excluding_primary_key_token_stream),*
                                }
                            }
                        },
                    );
                    let impl_std_convert_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                        &operation,
                        &{
                            let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation);
                            quote::quote! {
                                let mut elements = std::vec::Vec::with_capacity(#value_snake_case.0.len());
                                for #element_snake_case in #value_snake_case.0 {//todo refactor
                                    elements.push(#operation_payload_element_upper_camel_case_token_stream::#from_snake_case(#element_snake_case));
                                }
                                Self(elements)
                            }
                        },
                    );
                    quote::quote! {
                        #impl_std_convert_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream
                        #impl_std_convert_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                    }
                },
                postgresql_crud_common::FromOrTryFrom::TryFrom => {
                    let impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream = {
                        let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream(
                            &operation,
                            &{
                                let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_primary_key_token_stream(
                                    &syn_field_with_additional_info_fields_named_excluding_primary_key,
                                    &primary_key_field_ident_token_stream,
                                );
                                quote::quote! {
                                    #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                                }
                            },
                        );
                        let impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream = generate_impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream(
                            &operation,
                            &{
                                let fields_assignment_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter()
                                    .map(|element| {
                                        let field_code_occurence_new_ac56a8a4_9fc6_430b_b511_a779d7e07be4_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                            file!(),
                                            line!(),
                                            column!(),
                                            &proc_macro_name_upper_camel_case_ident_stringified,
                                        );
                                        generate_let_field_ident_value_field_ident_from_or_try_from_token_stream(
                                            element,
                                            &field_code_occurence_new_ac56a8a4_9fc6_430b_b511_a779d7e07be4_token_stream,
                                        )
                                    });
                                //todo it can be std::convert::From if all #(#fields_assignment_excluding_primary_key_token_stream)* are impl from
                                quote::quote! {
                                    #(#fields_assignment_excluding_primary_key_token_stream)*
                                    Ok(Self {
                                        #(#fields_idents_excluding_primary_key_token_stream),*
                                    })
                                }
                            },
                        );
                        quote::quote! {
                            #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream
                            #impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream
                        }
                    };
                    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                        let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                            &operation,
                            &{
                                let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream =  naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                                quote::quote!{
                                    #primary_key_field_ident_upper_camel_case_token_stream {
                                        #eo_error_occurence_attribute_token_stream
                                        #primary_key_field_ident: #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream,
                                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                                    },
                                }
                            },
                        );
                        let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_try_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                            &operation,
                            &{
                                let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation);
                                quote::quote! {
                                    let mut elements = std::vec::Vec::with_capacity(#value_snake_case.0.len());
                                    for #element_snake_case in #value_snake_case.0 {
                                        match #operation_payload_element_upper_camel_case_token_stream::#try_from_snake_case(#element_snake_case) {
                                            Ok(#value_snake_case) => {
                                                elements.push(#value_snake_case);
                                            },
                                            Err(#error_snake_case) => {
                                                return Err(Self::Error::#primary_key_field_ident_variant_initialization_token_stream);
                                            }
                                        }
                                    }
                                    Ok(Self(elements))
                                }
                            },
                        );
                        quote::quote! {
                            #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                            #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                        }
                    };
                    quote::quote! {
                        #impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream
                        #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                    }
                }
            };
            // println!("{impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = generate_impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_and_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_token_stream(
                &operation,
                &{
                    let fields_assignment_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(generate_let_field_ident_value_inner_type_with_serialize_deserialize_from_token_stream);
                    quote::quote!{
                        #(#fields_assignment_excluding_primary_key_token_stream)*
                        Self{
                            #(#fields_idents_excluding_primary_key_token_stream),*
                        }
                    }
                },
                &{
                    let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{
                        Self(
                            #value_snake_case.0.into_iter()
                            .map(|#element_snake_case|#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#element_snake_case))
                            .collect()
                        )
                    }
                },
            );
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            generate_parameters_pattern_token_stream(
                &operation,
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream,
                impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream,
                impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream,
            )
        };
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &std_vec_vec_primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            // println!("{try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream}");
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                    &operation,
                    &fields_named_excluding_primary_key_from_or_try_from,
                    &serde_json_syn_variant_wrapper,
                    &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
                    &proc_macro2::TokenStream::new(),
                );
                // println!("{parameters_logic_token_stream}");
                let query_string_token_stream = {
                    let column_names = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, element)| {
                        let field_ident = &element.field_ident;
                        let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                        if incremented_index == syn_field_with_additional_info_fields_named_excluding_primary_key_len {
                            acc.push_str(&format!("{field_ident}"));
                        }
                        else {
                            acc.push_str(&format!("{field_ident}{dot_space}"));
                        }
                        acc
                    });
                    let column_increments_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                            &checked_add_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote!{
                            match postgresql_crud::BindQuery::try_increment(
                                &#element_snake_case.#field_ident,
                                &mut increment,
                            ) {
                                Ok(_) => {
                                    #value_snake_case.push_str(&format!("${},", increment));
                                }
                                Err(_) => {//todo try_increment has own error. is it must be used? or no?
                                    #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                    });
                    let query_token_stream = {
                        let value = format!(
                            "\"{insert_snake_case} {into_snake_case} {table_name_stringified} ({column_names}) {values_snake_case} {{values}} {returning_primary_key_stringified}\""
                        );
                        value.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote!{
                        {
                            #increment_initialization_token_stream
                            let mut values = #std_string_string::default();
                            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                let mut #value_snake_case = #std_string_string::default();
                                #(#column_increments_token_stream)*
                                let _ = #value_snake_case.pop();
                                values.push_str(&format!("({value}),"));
                            }
                            let _ = values.pop();
                            format!(#query_token_stream)
                        }
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let query_string_snake_case = naming_conventions::QueryStringSnakeCase;
                    let query_bind_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
                        let field_ident =  &element.field_ident;
                        quote::quote! {
                            #query_snake_case = #query_snake_case.bind(#element_snake_case.#field_ident.0);
                        }
                    });
                    quote::quote! {
                        let mut #query_snake_case = sqlx::query::<sqlx::Postgres>(&#query_string_snake_case);
                        for #element_snake_case in #parameters_snake_case.#payload_snake_case.0 {
                            #(#query_bind_token_stream)*
                        }
                        #query_snake_case
                    }
                };
                // println!("{binded_query_token_stream}");
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &{
                        let fetch_token_stream = generate_fetch_token_stream(
                            &generate_sqlx_row_try_get_primary_key_token_stream(
                                &quote::quote!{Some(#primary_key_inner_type_token_stream(#value_snake_case))},
                                &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(
                                    &operation,
                                    file!(),
                                    line!(),
                                    column!(),
                                    file!(),
                                    line!(),
                                    column!(),
                                )
                            ),
                            &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(
                                &operation,
                                file!(),
                                line!(),
                                column!(),
                                file!(),
                                line!(),
                                column!(),
                            )
                        );
                        let unexpected_rows_length_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                            &unexpected_rows_length_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let unexpected_rows_length_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                            &unexpected_rows_length_and_rollback_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote!{
                            #fetch_token_stream
                            {
                                let #error_1_token_stream = #value_snake_case.len();
                                if #error_0_token_stream != #error_1_token_stream {
                                    match #executor_snake_case.#rollback_snake_case().await {
                                        Ok(_) => {
                                            #unexpected_rows_length_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        }
                                        Err(#error_2_token_stream) => {
                                            #unexpected_rows_length_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        }
                                    }
                                }
                            }
                            let #value_snake_case = #value_snake_case.into_iter().map(
                                |#element_snake_case|#primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#element_snake_case)
                            ).collect();
                        }
                    },
                );
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &table_name_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &syn_derive_input,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &{
                        quote::quote! {
                            let #error_0_token_stream = #parameters_snake_case.#payload_snake_case.0.len();
                        }
                    },
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &check_body_size_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                    &postgresql_syn_variant_wrapper.generate_initialization_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_with_possible_operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variants,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_inner_type_token_stream,
                &proc_macro2::TokenStream::new(),
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream_option {
                    None => quote::quote!{
                        #value_snake_case
                        .into_iter()
                        .map(|#element_snake_case| #primary_key_inner_type_token_stream::#from_snake_case(#element_snake_case))
                        .collect()
                    },
                    Some(syn_variant_initialization_token_stream) => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            {
                                let mut values = std::vec::Vec::new();
                                for #element_snake_case in #value_snake_case {
                                    match #primary_key_inner_type_token_stream::#try_from_snake_case(#element_snake_case) {
                                        Ok(#value_snake_case) => {
                                            values.push(#value_snake_case);
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(
                                                #try_operation_error_named_upper_camel_case_token_stream::#syn_variant_initialization_token_stream
                                            );
                                        }
                                    }
                                }
                                values
                            }
                        }
                    }
                },
                &reqwest_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &deserialize_response_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &failed_to_get_response_text_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &serde_json_to_string_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
            );
            // let try_operation_test_token_stream = {
            //     let element_fields_initialization_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         let field_type = &element.field.ty;
            //         quote::quote!{
            //             #field_ident: #field_type::default()
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         let #primary_keys_token_stream = match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream(vec![
            //                     #operation_payload_element_upper_camel_case_token_stream{
            //                         #(#element_fields_initialization_token_stream),*
            //                     }
            //                 ])
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => {
            //                 println!("{value:#?}");
            //                 value
            //             },
            //             Err(#error_snake_case) => panic!(
            //                 "{}",
            //                 #error_snake_case
            //             ),//todo remove panic maybe?
            //         };
            //     };
            //     naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #try_operation_token_stream
                },
                quote::quote! {},
                // try_operation_test_token_stream,
            )
        };
        // println!("{try_operation_token_stream}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_route_logic_token_stream
                #try_operation_token_stream
            },
            // try_operation_test_token_stream,
            quote::quote! {}
        )
    };
    let (create_one_token_stream, create_one_test_token_stream) = {
        let operation = Operation::CreateOne;
        let self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper = operation.generate_self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper(
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 1);
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value.push(&row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value
            },
            &fields_named_excluding_primary_key_from_or_try_from,
            &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
            &operation,
            &syn_derive_input,
        );
        let parameters_token_stream = {
            let payload_token_stream = generate_operation_payload_token_stream(
                &operation,
                &generate_fields_named_excluding_primary_key_token_stream(generate_pub_field_ident_field_type_token_stream),
            );
            let payload_with_serialize_deserialize_token_stream = generate_payload_with_serialize_deserialize_token_stream(
                &operation,
                &generate_fields_named_excluding_primary_key_token_stream(generate_field_ident_field_type_with_serialize_deserialize_token_stream),
            );
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = match fields_named_excluding_primary_key_from_or_try_from {
                postgresql_crud_common::FromOrTryFrom::From => generate_impl_std_convert_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                    &operation,
                    &{
                        let fields_assignment_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter()
                        .map(generate_let_field_ident_value_inner_type_from_token_stream);
                        quote::quote! {
                            #(#fields_assignment_excluding_primary_key_token_stream)*
                            Self {
                                #(#fields_idents_excluding_primary_key_token_stream),*
                            }
                        }
                    },
                ),
                postgresql_crud_common::FromOrTryFrom::TryFrom => {
                    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                        &operation,
                        &{
                            let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_primary_key_token_stream(
                                &syn_field_with_additional_info_fields_named_excluding_primary_key,
                                &primary_key_field_ident_token_stream,
                            );
                            quote::quote! {#(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*}
                        },
                    );
                    // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_try_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                        &operation,
                        &{
                            let field_code_occurence_new_3763990f_5c49_47d0_a774_5ef584cd1236_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                file!(),
                                line!(),
                                column!(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            let fields_assignment_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter()
                            .map(|element| generate_let_field_ident_value_field_ident_from_or_try_from_token_stream(
                                element,
                                &field_code_occurence_new_3763990f_5c49_47d0_a774_5ef584cd1236_token_stream,
                            ));
                            quote::quote! {
                                #(#fields_assignment_excluding_primary_key_token_stream)*
                                Ok(Self {
                                    #(#fields_idents_excluding_primary_key_token_stream),*
                                })
                            }
                        },
                    );
                    // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
                    quote::quote! {
                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                        #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                    }
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = generate_impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream(
                &operation,
                &{
                    let fields_assignment_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(generate_let_field_ident_value_inner_type_with_serialize_deserialize_from_token_stream);
                    quote::quote! {
                        #(#fields_assignment_excluding_primary_key_token_stream)*
                        Self{
                            #(#fields_idents_excluding_primary_key_token_stream),*
                        }
                    }
                },
            );
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            generate_parameters_pattern_token_stream(
                &operation,
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream,
                impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream,
                impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream,
            )
        };
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                    &operation,
                    &fields_named_excluding_primary_key_from_or_try_from,
                    &serde_json_syn_variant_wrapper,
                    &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
                    &proc_macro2::TokenStream::new(),
                );
                let query_string_token_stream = {
                    let (column_names, column_increments) = {
                        syn_field_with_additional_info_fields_named_excluding_primary_key.iter().enumerate().fold((
                            std::string::String::default(),
                            std::string::String::default()
                        ), |mut acc, (index, element)| {
                            let field_ident = &element.field_ident;
                            let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                            if incremented_index == syn_field_with_additional_info_fields_named_excluding_primary_key_len {
                                acc.0.push_str(&format!("{field_ident}"));
                                acc.1.push_str(&format!("${incremented_index}"));
                            }
                            else {
                                acc.0.push_str(&format!("{field_ident}{dot_space}"));
                                acc.1.push_str(&format!("${incremented_index}{dot_space}"));
                            }
                            acc
                        })
                    };
                    proc_macro_common::generate_quotes::token_stream(
                        &format!("{insert_snake_case} {into_snake_case} {table_name_stringified}({column_names}) {values_snake_case} ({column_increments}){returning_primary_key_stringified}"),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    )
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        quote::quote!{
                            query = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case.#payload_snake_case.#field_ident, query);
                        }
                    });
                    quote::quote! {
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        query
                    }
                };
                // println!("{binded_query_token_stream}");
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &{
                        let fetch_one_token_stream = generate_fetch_one_token_stream(
                            &generate_sqlx_row_try_get_primary_key_token_stream(
                                &quote::quote!{#primary_key_inner_type_token_stream(#value_snake_case)},
                                &generate_match_postgres_transaction_rollback_await_token_stream(
                                    &operation,
                                    file!(),
                                    line!(),
                                    column!(),
                                    file!(),
                                    line!(),
                                    column!(),
                                ),
                            ),
                            &generate_match_postgres_transaction_rollback_await_token_stream(
                                &operation,
                                file!(),
                                line!(),
                                column!(),
                                file!(),
                                line!(),
                                column!(),
                            )
                        );
                        quote::quote!{
                            #fetch_one_token_stream
                            let #value_snake_case = #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#value_snake_case);
                        }
                    },
                );
                // // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                // //     &table_name_stringified,
                // //     &unique_status_codes,
                // //     &application_json_quotes_token_stream,
                // //     &table_name_quotes_token_stream,
                // //     &operation_payload_upper_camel_case_token_stream,
                // //     &operation,
                // // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &syn_derive_input,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &check_body_size_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                    &postgresql_syn_variant_wrapper.generate_initialization_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_with_possible_operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variants,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &primary_key_inner_type_token_stream,
                &proc_macro2::TokenStream::new(),
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream_option {
                    None => quote::quote!{#primary_key_inner_type_token_stream::#from_snake_case(#value_snake_case)},
                    Some(syn_variant_initialization_token_stream) => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            match #primary_key_inner_type_token_stream::#try_from_snake_case(#value_snake_case) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_snake_case) => {
                                    return Err(
                                        #try_operation_error_named_upper_camel_case_token_stream::#syn_variant_initialization_token_stream
                                    );
                                }
                            }
                        }
                    }
                },
                &reqwest_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &deserialize_response_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &failed_to_get_response_text_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &serde_json_to_string_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
            );
            // let try_operation_test_token_stream = {
            //     let element_fields_initialization_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         let field_type = &element.field.ty;
            //         quote::quote!{
            //             #field_ident: #field_type::default()
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         let #primary_key_token_stream = match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
            //                     #(#element_fields_initialization_token_stream),*
            //                 }
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => {
            //                 println!("{value:#?}");
            //                 value
            //             },
            //             Err(#error_snake_case) => panic!(
            //                 "{}",
            //                 #error_snake_case
            //             )
            //         };
            //     };
            //     naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #try_operation_token_stream
                },
                quote::quote! {}
                // try_operation_test_token_stream,
            )
        };
        // println!("{try_operation_token_stream}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_route_logic_token_stream
                #try_operation_token_stream
            },
            // try_operation_test_token_stream,
            quote::quote! {}
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &create_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    //todo add additional filters
    let (read_many_token_stream, read_many_test_token_stream) = {
        let operation = Operation::ReadMany;
        let self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper = operation.generate_self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper(
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 2);
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value.push(&checked_add_syn_variant_wrapper.get_syn_variant());
                value.push(&bind_query_syn_variant_wrapper.get_syn_variant());
                value.push(&not_unique_primary_key_syn_variant_wrapper.get_syn_variant());
                value.push(&not_unique_column_syn_variant_wrapper.get_syn_variant());
                not_unique_fields_syn_variants_wrappers.iter().for_each(|element|{
                    value.push(element.get_syn_variant());
                });
                value
            },
            &fields_named_from_or_try_from,
            &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
            &operation,
            &syn_derive_input,
        );
        let parameters_token_stream = {
            let (
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream
            ) = {
                let generate_fields_token_stream = |is_pub: bool| -> proc_macro2::TokenStream {
                    let pub_handle_token_stream = generate_pub_handle_token_stream(is_pub);
                    let primary_key_field_ident_std_option_option_std_vec_vec_primary_key_inner_type_handle_token_stream = generate_primary_key_field_ident_std_option_option_std_vec_vec_primary_key_inner_type_handle_token_stream(is_pub);
                    let fields_idents_std_option_option_std_vec_vec_where_inner_type_handle_token_stream = generate_fields_idents_std_option_option_std_vec_vec_where_inner_type_handle_token_stream(is_pub);
                    let pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream = generate_pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream(is_pub);
                    let limit_and_offset_type_handle_token_stream = match is_pub {
                        true => &limit_and_offset_type_token_stream,
                        false => &limit_and_offset_type_with_serialize_deserialize_token_stream,
                    };
                    quote::quote! {
                        #primary_key_field_ident_std_option_option_std_vec_vec_primary_key_inner_type_handle_token_stream,
                        #fields_idents_std_option_option_std_vec_vec_where_inner_type_handle_token_stream,
                        #pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream,
                        #pub_handle_token_stream #order_by_snake_case: #postgresql_crud_order_by_token_stream<#ident_column_upper_camel_case_token_stream>,
                        #pub_handle_token_stream #limit_snake_case: #limit_and_offset_type_handle_token_stream,
                        #pub_handle_token_stream #offset_snake_case: #limit_and_offset_type_handle_token_stream,
                    }
                };
                let payload_token_stream = generate_operation_payload_token_stream(
                    &operation,
                    &generate_fields_token_stream(true)
                );
                // println!("{payload_token_stream}");
                let payload_with_serialize_deserialize_token_stream = generate_payload_with_serialize_deserialize_token_stream(
                    &operation,
                    &generate_fields_token_stream(false)
                );
                // println!("{payload_with_serialize_deserialize_token_stream}");
                (
                    payload_token_stream,
                    payload_with_serialize_deserialize_token_stream
                )
            };
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = match fields_named_from_or_try_from {
                postgresql_crud_common::FromOrTryFrom::From => generate_impl_std_convert_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                    &operation,
                    &{
                        let primary_key_field_assignment_token_stream = {
                            let primary_key_initialization_token_stream = match primary_key_from_or_try_from {
                                postgresql_crud_common::FromOrTryFrom::From => quote::quote! {
                                    Some(#value_snake_case.into_iter().map(|#element_snake_case|#primary_key_inner_type_token_stream::#from_snake_case(
                                        #element_snake_case
                                    )).collect())
                                },
                                postgresql_crud_common::FromOrTryFrom::TryFrom => quote::quote! {
                                    Some({
                                        let mut #acc_snake_case = std::vec::Vec::new();
                                        for #element_snake_case in #value_snake_case {
                                            match #primary_key_inner_type_token_stream::#try_from_snake_case(#element_snake_case) {
                                                Ok(#value_snake_case) => {
                                                    #acc_snake_case.push(#value_snake_case);
                                                },
                                                Err(#error_snake_case) => {
                                                    return Err(Self::Error::#primary_key_field_ident_variant_initialization_token_stream);
                                                }
                                            }
                                        }
                                        #acc_snake_case
                                    })
                                }
                            };
                            quote::quote! {
                                let #primary_key_field_ident = match #value_snake_case.#primary_key_field_ident {
                                    Some(#value_snake_case) => #primary_key_initialization_token_stream,
                                    None => None
                                };
                            }
                        };
                        let fields_assignment_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter()
                            .map(|element| generate_option_vec_where_inner_type_from_or_try_from_option_vec_where_inner_type_with_serialize_deserialize_token_stream(
                                element,
                                &primary_key_field_ident_token_stream,
                            ));
                        quote::quote! {
                            #primary_key_field_assignment_token_stream
                            #(#fields_assignment_excluding_primary_key_token_stream)*
                            let #select_snake_case = #value_snake_case.#select_snake_case;
                            let #order_by_snake_case = #value_snake_case.#order_by_snake_case;
                            let #limit_snake_case = #limit_and_offset_type_token_stream::#from_snake_case(#value_snake_case.#limit_snake_case);
                            let #offset_snake_case = #limit_and_offset_type_token_stream::#from_snake_case(#value_snake_case.#offset_snake_case);
                            Self {
                                #(#fields_named_idents_comma_token_stream)*
                                #select_snake_case,
                                #order_by_snake_case,
                                #limit_snake_case,
                                #offset_snake_case,
                            }
                        }
                    },
                ),
                postgresql_crud_common::FromOrTryFrom::TryFrom => {
                    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                        &operation,
                        &{
                            let primary_key_variant_token_stream = match primary_key_from_or_try_from {
                                postgresql_crud_common::FromOrTryFrom::From => proc_macro2::TokenStream::new(),
                                postgresql_crud_common::FromOrTryFrom::TryFrom => quote::quote!{
                                    #primary_key_field_ident_upper_camel_case_token_stream {
                                        #eo_error_occurence_attribute_token_stream
                                        #primary_key_field_ident: #primary_key_inner_type_with_serialize_deserialize_error_named_token_stream,
                                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                                    },
                                }
                            };
                            quote::quote! {
                                #primary_key_variant_token_stream
                                #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                            }
                        },
                    );
                    // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_try_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                        &operation,
                        &{
                            let primary_key_field_assignment_token_stream = {
                                let primary_key_initialization_token_stream = match primary_key_from_or_try_from {
                                    postgresql_crud_common::FromOrTryFrom::From => quote::quote! {
                                        Some(#value_snake_case.into_iter().map(|#element_snake_case|#primary_key_inner_type_token_stream::#from_snake_case(
                                            #element_snake_case
                                        )).collect())
                                    },
                                    postgresql_crud_common::FromOrTryFrom::TryFrom => quote::quote! {
                                        Some({
                                            let mut #acc_snake_case = std::vec::Vec::new();
                                            for #element_snake_case in #value_snake_case {
                                                match #primary_key_inner_type_token_stream::#try_from_snake_case(#element_snake_case) {
                                                    Ok(#value_snake_case) => {
                                                        #acc_snake_case.push(#value_snake_case);
                                                    },
                                                    Err(#error_snake_case) => {
                                                        return Err(Self::Error::#primary_key_field_ident_variant_initialization_token_stream);
                                                    }
                                                }
                                            }
                                            #acc_snake_case
                                        })
                                    }
                                };
                                quote::quote! {
                                    let #primary_key_field_ident = match #value_snake_case.#primary_key_field_ident {
                                        Some(#value_snake_case) => #primary_key_initialization_token_stream,
                                        None => None
                                    };
                                }
                            };
                            let fields_assignment_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter()
                                .map(|element| generate_option_vec_where_inner_type_from_or_try_from_option_vec_where_inner_type_with_serialize_deserialize_token_stream(
                                    element,
                                    &primary_key_field_ident_token_stream,
                                ));
                            quote::quote! {
                                #primary_key_field_assignment_token_stream
                                #(#fields_assignment_excluding_primary_key_token_stream)*
                                let #select_snake_case = #value_snake_case.#select_snake_case;
                                let #order_by_snake_case = #value_snake_case.#order_by_snake_case;
                                let #limit_snake_case = #limit_and_offset_type_token_stream::#from_snake_case(#value_snake_case.#limit_snake_case);
                                let #offset_snake_case = #limit_and_offset_type_token_stream::#from_snake_case(#value_snake_case.#offset_snake_case);
                                Ok(Self {
                                    #(#fields_named_idents_comma_token_stream)*
                                    #select_snake_case,
                                    #order_by_snake_case,
                                    #limit_snake_case,
                                    #offset_snake_case,
                                })
                            }
                        },
                    );
                    quote::quote! {
                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                        #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                    }
                }
            };
            // println!("{impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = generate_impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream(
                &operation,
                &{
                    let primary_key_field_assignment_token_stream = quote::quote! {
                        let #primary_key_field_ident = match #value_snake_case.#primary_key_field_ident {
                            Some(#value_snake_case) => Some(
                                #value_snake_case
                                    .into_iter()
                                    .map(|#element_snake_case| #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#element_snake_case))
                                    .collect::<std::vec::Vec<#primary_key_inner_type_with_serialize_deserialize_token_stream>>(),
                            ),
                            None => None,
                        };
                    };
                    let fields_assignment_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(generate_let_field_ident_value_option_vec_with_serialize_deserialize_token_stream);
                    quote::quote! {
                        #primary_key_field_assignment_token_stream
                        #(#fields_assignment_excluding_primary_key_token_stream)*
                        let #select_snake_case = #value_snake_case.#select_snake_case;
                        let #order_by_snake_case = #value_snake_case.#order_by_snake_case;
                        let #limit_snake_case = #limit_and_offset_type_with_serialize_deserialize_token_stream::#from_snake_case(#value_snake_case.#limit_snake_case);//todo reuse
                        let #offset_snake_case = #limit_and_offset_type_with_serialize_deserialize_token_stream::#from_snake_case(#value_snake_case.#offset_snake_case);//todo reuse
                        Self{
                            #(#fields_named_idents_comma_token_stream)*
                            #select_snake_case,
                            #order_by_snake_case,
                            #limit_snake_case,
                            #offset_snake_case,
                        }
                    }
                },
            );
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            generate_parameters_pattern_token_stream(
                &operation,
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream,
                impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream,
                impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream,
            )
        };
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &std_vec_vec_struct_options_ident_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &fields_named_from_or_try_from,
                   &serde_json_syn_variant_wrapper,
                   &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
                   &{
                        let filter_not_unique_fields_token_stream = {
                            let filter_not_unique_primary_key_token_stream = {
                                let not_unique_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                                    &not_unique_primary_key_syn_variant_wrapper,
                                    file!(),
                                    line!(),
                                    column!(),
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                quote::quote! {
                                    if let Some(#primary_key_field_ident) = &#value_snake_case.#primary_key_field_ident {
                                        let mut #acc_snake_case = std::vec::Vec::new();
                                        for #element_snake_case in #primary_key_field_ident {
                                            if !#acc_snake_case.contains(&#element_snake_case) {
                                                #acc_snake_case.push(&#element_snake_case);
                                            }
                                            else {
                                                let #error_0_token_stream = *#element_snake_case;
                                                #not_unique_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                                            }
                                        }
                                    }
                                }
                            };
                            let filter_not_unique_fields_named_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
                                let element_field_ident = &element.field_ident;
                                let field_ident_token_stream = {
                                    let value = format!(
                                        "{}::{}",
                                        postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE,
                                        postgresql_crud_common::SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(
                                            &postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)
                                        ),
                                    );
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                //todo remove syn_variant_wrapper creation duplication
                                let not_unique_fields_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
                                    &format!(
                                        "{}{}",
                                        naming_conventions::NotUniqueUpperCamelCase,
                                        proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&element.field_ident.to_string()),
                                    ),
                                    Some(not_unique_primary_key_syn_variant_wrapper.get_option_status_code().unwrap()),
                                    vec![
                                        (
                                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                                            &format!(
                                                "{}_{}",
                                                naming_conventions::NotUniqueSnakeCase,
                                                &element.field_ident,
                                            ),
                                            {
                                                let mut value = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                                                value.push_value(syn::PathSegment {
                                                    ident: proc_macro2::Ident::new(&postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, proc_macro2::Span::call_site()),
                                                    arguments: syn::PathArguments::None,
                                                });
                                                value.push_punct(syn::token::PathSep {
                                                    spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                                                });
                                                value.push_value(syn::PathSegment {
                                                    ident: proc_macro2::Ident::new(
                                                        &postgresql_crud_common::SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(
                                                            &postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)
                                                        ).to_string(),
                                                        proc_macro2::Span::call_site()
                                                    ),
                                                    arguments: syn::PathArguments::None,
                                                });
                                                value
                                            }
                                        )
                                    ],
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                let not_unique_fields_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                                    &not_unique_fields_syn_variant_wrapper,
                                    file!(),
                                    line!(),
                                    column!(),
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                quote::quote!{
                                    if let Some(#value_snake_case) = &#value_snake_case.#element_field_ident {
                                        let mut #acc_snake_case = std::vec::Vec::new();
                                        for #element_snake_case in #value_snake_case {
                                            if let Some(#value_snake_case) = &#element_snake_case.#value_snake_case.0 {
                                                if !#acc_snake_case.contains(&#value_snake_case) {
                                                    #acc_snake_case.push(&#value_snake_case);
                                                }
                                                else {
                                                    let #error_0_token_stream = #field_ident_token_stream(#value_snake_case.clone());
                                                    #not_unique_fields_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                            quote::quote! {
                                #filter_not_unique_primary_key_token_stream
                                #(#filter_not_unique_fields_named_excluding_primary_key_token_stream)*
                            }
                        };
                        let filter_not_unique_column_token_stream = generate_filter_not_unique_column_route_logic_token_stream(&operation);
                        quote::quote!{
                            #filter_not_unique_fields_token_stream
                            #filter_not_unique_column_token_stream
                        }
                   },
                );
                let query_string_token_stream = {
                    let additional_parameters_primary_key_modification_token_stream = {
                        let prefix_false_handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!(" {and_snake_case}"),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!("{{}} {primary_key_field_ident} {in_snake_case} ({select_snake_case} {unnest_snake_case}(${{}}))"),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let checked_add_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                            &checked_add_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            if let Some(_) = &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                                let prefix = match additional_parameters.is_empty() {
                                    true => #where_snake_case_qoutes_token_stream,
                                    false => #prefix_false_handle_token_stream,
                                };
                                match increment.checked_add(1) {
                                    Some(#value_snake_case) => {
                                        increment = #value_snake_case;
                                    },
                                    None => {
                                        #checked_add_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                                    },
                                }
                                additional_parameters.push_str(&format!(
                                    #handle_token_stream,
                                    prefix,
                                    increment
                                ));
                            }
                        }
                    };
                    let additional_parameters_modification_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        let handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!("{field_ident} ~ {{value}} "),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let prefix_false_handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!(" {and_snake_case}"),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        // let field_handle_token_stream = {
                        //     let field_handle_stringified = format!("{field_ident}_handle");
                        //     field_handle_stringified.parse::<proc_macro2::TokenStream>()
                        //     .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        // };
                        let bind_query_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                            &bind_query_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote!{
                            if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                let prefix = match additional_parameters.is_empty() {
                                    true => #where_snake_case_qoutes_token_stream,
                                    false => #prefix_false_handle_token_stream,
                                };
                                let bind_increments = {
                                    let mut bind_increments = #std_string_string::default();
                                    for (index, #element_snake_case) in #value_snake_case.iter().enumerate() {
                                        match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(
                                            #element_snake_case,
                                            &mut increment
                                        ) {
                                            Ok(#value_snake_case) => {
                                                let handle = format!(#handle_token_stream);
                                                match index == 0 {
                                                    true => {
                                                        bind_increments.push_str(&handle);
                                                    },
                                                    false => {
                                                        bind_increments.push_str(&format!("{} {handle}", #element_snake_case.conjuctive_operator));
                                                    },
                                                }
                                            },
                                            Err(#error_0_token_stream) => {
                                                #bind_query_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                                            },
                                        }
                                    }
                                    if let false = bind_increments.is_empty() {
                                        let _ = bind_increments.pop();
                                    }
                                    bind_increments
                                };
                                additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
                            }
                        }
                    });
                    let handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                        &format!("{select_snake_case} {{}} {from_snake_case} {table_name_stringified} {{}}"),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let order_snake_case = naming_constants::OrderSnakeCase;
                    let by_snake_case = naming_constants::BySnakeCase;
                    let additional_parameters_order_by_handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                        &format!("{{}}{order_snake_case} {by_snake_case} {{}} {{}}"),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let additional_parameters_limit_handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                        &format!("{{}}{limit_snake_case} {{}}"),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let additional_parameters_offset_handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                        &format!("{{}}{offset_snake_case} {{}}"),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let bind_query_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream_e6e820dd_ec74_4bc5_b482_2aa9cd6b3793 = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &bind_query_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let bind_query_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream_6d384a1b_d37a_4fd3_9ed3_c160afbb74fc = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &bind_query_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        format!(
                            #handle_token_stream,
                            #generate_query_vec_column_snake_case_token_stream(
                                &#parameters_snake_case.#payload_snake_case.#select_snake_case
                            ),
                            {
                                #increment_initialization_token_stream
                                let mut additional_parameters = #std_string_string::default();
                                #additional_parameters_primary_key_modification_token_stream
                                #(#additional_parameters_modification_token_stream)*
                                {
                                    let prefix = match additional_parameters.is_empty() {
                                        true => "",
                                        false => " ",
                                    };
                                    let #value_snake_case = &#parameters_snake_case.#payload_snake_case.#order_by_snake_case;
                                    let order_stringified = match &#value_snake_case.order {
                                        Some(order) => order.to_string(),
                                        None => #postgresql_crud_order_token_stream::default().to_string(),
                                    };
                                    additional_parameters.push_str(&format!(
                                        #additional_parameters_order_by_handle_token_stream,
                                        prefix,
                                        #value_snake_case.column,
                                        order_stringified
                                    ));
                                }
                                {
                                    let prefix = match additional_parameters.is_empty() {
                                        true => "",
                                        false => " ",
                                    };
                                    let #value_snake_case = match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(
                                        &#parameters_snake_case.#payload_snake_case.#limit_snake_case,
                                        &mut increment
                                    ) {
                                        Ok(#value_snake_case) => #value_snake_case,
                                        Err(#error_0_token_stream) => {
                                            #bind_query_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream_e6e820dd_ec74_4bc5_b482_2aa9cd6b3793
                                        },
                                    };
                                    additional_parameters.push_str(&format!(
                                        #additional_parameters_limit_handle_token_stream,
                                        prefix,
                                        #value_snake_case
                                    ));
                                }
                                {
                                    let prefix = match additional_parameters.is_empty() {
                                        true => "",
                                        false => " ",
                                    };
                                    let #value_snake_case = match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(
                                        &#parameters_snake_case.#payload_snake_case.#offset_snake_case,
                                        &mut increment
                                    ) {
                                        Ok(#value_snake_case) => #value_snake_case,
                                        Err(#error_0_token_stream) => {
                                            #bind_query_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream_6d384a1b_d37a_4fd3_9ed3_c160afbb74fc
                                        },
                                    };
                                    additional_parameters.push_str(&format!(
                                        #additional_parameters_offset_handle_token_stream,
                                        prefix,
                                        #value_snake_case
                                    ));
                                }
                                additional_parameters
                            }
                        )
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_primary_key_modification_token_stream = quote::quote! {
                        if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                            #query_snake_case = #query_snake_case.bind(#value_snake_case.into_iter().map(|#element_snake_case|#element_snake_case.into_inner().clone()).collect::<std::vec::Vec<#primary_key_original_type_token_stream>>());
                        }
                    };
                    let binded_query_modifications_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        quote::quote!{
                            if let Some(values) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                for #value_snake_case in values {
                                    #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(
                                        #value_snake_case, #query_snake_case,
                                    );
                                }
                            }
                        }
                    });
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_primary_key_modification_token_stream
                        #(#binded_query_modifications_token_stream)*
                        #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case.#payload_snake_case.#limit_snake_case,
                            #query_snake_case,
                        );
                        #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case.#payload_snake_case.#offset_snake_case,
                            #query_snake_case,
                        );
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = {
                    let fetch_token_stream = generate_fetch_token_stream(
                        &{
                            let options_try_from_sqlx_row_token_stream = generate_options_try_from_sqlx_row_token_stream(&operation);
                            quote::quote!{Some({#options_try_from_sqlx_row_token_stream})}
                        },
                        &operation.generate_error_initialization_eprintln_response_creation_token_stream(
                            &postgresql_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        )
                    );
                    quote::quote! {
                        #fetch_token_stream
                        #value_snake_case
                    }
                };
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &table_name_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &syn_derive_input,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &check_body_size_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                    &postgresql_syn_variant_wrapper.generate_initialization_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &{
                    let mut value = common_http_request_syn_variants.clone();
                    value.push(not_unique_primary_key_syn_variant_wrapper.get_syn_variant().clone());
                    not_unique_fields_syn_variants_wrappers.iter().for_each(|element|{
                        value.push(element.get_syn_variant().clone());
                    });
                    value.push(not_unique_column_syn_variant_wrapper.get_syn_variant().clone());
                    value
                },
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_struct_options_ident_token_stream,
                &{
                    let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                    let filter_not_unique_fields_token_stream = {
                        let filter_not_unique_primary_key_token_stream = {
                            let not_unique_primary_key_syn_variant_initialization_token_stream = not_unique_primary_key_syn_variant_wrapper.generate_initialization_token_stream(
                                file!(),
                                line!(),
                                column!(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            quote::quote!{
                                if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                                    let mut #acc_snake_case = std::vec::Vec::new();
                                    for #element_snake_case in #value_snake_case {
                                        if !#acc_snake_case.contains(&#element_snake_case) {
                                            #acc_snake_case.push(&#element_snake_case);
                                        } else {
                                            let #error_0_token_stream = *#element_snake_case;
                                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#not_unique_primary_key_syn_variant_initialization_token_stream);
                                        }
                                    }
                                }
                            }
                        };
                        let filter_not_unique_fields_named_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
                            let element_field_ident = &element.field_ident;
                            let not_unique_fields_syn_variant_initialization_token_stream = {
                                let not_unique_field_ident_upper_camel_case_token_stream = {
                                    let value = format!(
                                        "{}{}",
                                        naming_conventions::NotUniqueUpperCamelCase,
                                        proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&element_field_ident.to_string()),
                                    );
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                let not_unique_field_ident_snake_case_token_stream = {
                                    let value = format!(
                                        "{}_{}",
                                        naming_conventions::NotUniqueSnakeCase,
                                        &element_field_ident,
                                    );
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                let field_code_occurence_new_eb1a9553_449e_4767_9e5c_c1856b77bd4e_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                    file!(),
                                    line!(),
                                    column!(),
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                let field_ident_token_stream = {
                                    let value = format!(
                                        "{}::{}",
                                        postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE,
                                        postgresql_crud_common::SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(
                                            &postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)
                                        ),
                                    );
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                quote::quote! {
                                    #not_unique_field_ident_upper_camel_case_token_stream {
                                        #not_unique_field_ident_snake_case_token_stream: #field_ident_token_stream(#value_snake_case.clone()),
                                        #field_code_occurence_new_eb1a9553_449e_4767_9e5c_c1856b77bd4e_token_stream,
                                    }
                                }
                            };
                            quote::quote!{
                                if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#element_field_ident {
                                    let mut #acc_snake_case = std::vec::Vec::new();
                                    for #element_snake_case in #value_snake_case {
                                        if let Some(#value_snake_case) = &#element_snake_case.#value_snake_case.0 {
                                            if !#acc_snake_case.contains(&#value_snake_case) {
                                                #acc_snake_case.push(&#value_snake_case);
                                            }
                                            else {
                                                return Err(#try_operation_error_named_upper_camel_case_token_stream::#not_unique_fields_syn_variant_initialization_token_stream);
                                            }
                                        }
                                    }
                                }
                            }
                        });
                        quote::quote! {
                            #filter_not_unique_primary_key_token_stream
                            #(#filter_not_unique_fields_named_excluding_primary_key_token_stream)*
                        }
                    };
                    let filter_not_unique_column_token_stream = generate_filter_not_unique_column_http_request_token_stream(&operation);
                    quote::quote!{
                        #filter_not_unique_fields_token_stream
                        #filter_not_unique_column_token_stream
                    }
                },
                &generate_read_operation_payload_with_serialize_deserialize_initialization_token_stream(&operation),
                &match fields_named_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                        #value_snake_case
                        .into_iter()
                        .map(|#element_snake_case| #struct_options_ident_token_stream::#from_snake_case(#element_snake_case))
                        .collect()
                    },
                    postgresql_crud_common::FromOrTryFrom::TryFrom => quote::quote!{
                        #value_snake_case
                        .into_iter()
                        .fold(std::vec::Vec::new(), |mut #acc_snake_case, #element_snake_case| {
                            #acc_snake_case.push(#element_snake_case);
                            #acc_snake_case
                        })
                    }
                },
                &reqwest_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &deserialize_response_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &failed_to_get_response_text_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &serde_json_to_string_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
            );
            // println!("{try_operation_token_stream}");
            // // let try_operation_test_token_stream = {
            // //     let element_fields_initialization_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            // //         let field_ident = &element.field_ident;
            // //         let field_type = &element.field.ty;
            // //         quote::quote!{
            // //             #field_ident: #field_type::default()
            // //         }
            // //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            // //     let test_content_token_stream = quote::quote! {
            // //         let #primary_key_token_stream = match #try_operation_snake_case_token_stream(
            // //             #reference_api_location_test_token_stream,
            // //             #operation_parameters_upper_camel_case_token_stream {
            // //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
            // //                     #(#element_fields_initialization_token_stream),*
            // //                 }
            // //             },
            // //         )
            // //         .await
            // //         {
            // //             Ok(value) => {
            // //                 println!("{value:#?}");
            // //                 value
            // //             },
            // //             Err(#error_snake_case) => panic!(
            // //                 "{}",
            // //                 #error_snake_case
            // //             )
            // //         };
            // //     };
            // //     naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // // };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #try_operation_token_stream
                },
                quote::quote! {}
                // try_operation_test_token_stream,
            )
        };
        // println!("{try_operation_token_stream}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_route_logic_token_stream
                #try_operation_token_stream
            },
            // try_operation_test_token_stream,
            quote::quote! {}
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &read_many_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (
        read_one_token_stream,
        read_one_http_request_test_expect_success_token_stream,
        read_one_http_request_test_expect_fail_token_stream,
    ) = {
        let operation = Operation::ReadOne;
        let self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper = operation.generate_self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper(
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = common_route_syn_variants.clone();
                value.push(&not_unique_column_syn_variant_wrapper.get_syn_variant());
                value
            },
            &primary_key_from_or_try_from,
            &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
            &operation,
            &syn_derive_input,
        );
        let parameters_token_stream = {
            let (
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream
            ) = {
                let generate_fields_token_stream = |is_pub: bool| -> proc_macro2::TokenStream {
                    let pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream(is_pub);
                    let pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream = generate_pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream(is_pub);
                    quote::quote! {
                        #pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream,
                        #pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream,
                    }
                };
                let payload_token_stream = generate_operation_payload_token_stream(
                    &operation,
                    &generate_fields_token_stream(true)
                );
                // println!("{payload_token_stream}");
                let payload_with_serialize_deserialize_token_stream = generate_payload_with_serialize_deserialize_token_stream(
                    &operation,
                    &generate_fields_token_stream(false)
                );
                // println!("{payload_with_serialize_deserialize_token_stream}");
                (
                    payload_token_stream,
                    payload_with_serialize_deserialize_token_stream
                )
            };
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let primary_key_let_field_ident_value_field_ident_from_token_stream = quote::quote! {
                    let #primary_key_field_ident = #primary_key_inner_type_token_stream::#from_snake_case(#value_snake_case.#primary_key_field_ident);
                };
                match primary_key_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => generate_impl_std_convert_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                        &operation,
                        &quote::quote! {
                            #primary_key_let_field_ident_value_field_ident_from_token_stream
                            let #select_snake_case = #value_snake_case.#select_snake_case;
                            Self {
                                #primary_key_field_ident,
                                #select_snake_case
                            }
                        },
                    ),
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                            &operation,
                            &{
                                let with_serialize_deserialize_error_named_token_stream = {
                                    let value = primary_key_rust_sqlx_map_to_postgres_type_variant.get_inner_type_with_serialize_deserialize_error_named_stringified("");
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                quote::quote!{
                                    #primary_key_field_ident_upper_camel_case_token_stream {
                                        #eo_error_occurence_attribute_token_stream
                                        #primary_key_field_ident: #with_serialize_deserialize_error_named_token_stream,
                                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                                    },
                                }
                            },
                        );
                        // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                        let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_try_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                            &operation,
                            &{
                                let primary_key_let_field_ident_value_field_ident_from_or_try_from_token_stream = match &primary_key_from_or_try_from {
                                    postgresql_crud_common::FromOrTryFrom::From => primary_key_let_field_ident_value_field_ident_from_token_stream,
                                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                                        let field_code_occurence_new_3b778bbe_aec5_4ebe_af05_11074800c690_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                            file!(),
                                            line!(),
                                            column!(),
                                            &proc_macro_name_upper_camel_case_ident_stringified,
                                        );
                                        generate_let_field_ident_value_field_ident_from_or_try_from_token_stream(
                                            &primary_key_syn_field_with_additional_info,
                                            &field_code_occurence_new_3b778bbe_aec5_4ebe_af05_11074800c690_token_stream,
                                        )
                                    },
                                };
                                quote::quote! {
                                    #primary_key_let_field_ident_value_field_ident_from_or_try_from_token_stream
                                    let #select_snake_case = #value_snake_case.#select_snake_case;
                                    Ok(Self {
                                        #primary_key_field_ident,
                                        #select_snake_case
                                    })
                                }
                            },
                        );
                        // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
                        quote::quote! {
                            #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                            #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                        }
                    }
                }
            };
            // println!("{impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = 
            generate_impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream(
                &operation,
                &quote::quote! {
                    let #primary_key_field_ident = #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#value_snake_case.#primary_key_field_ident);
                    let #select_snake_case = #value_snake_case.#select_snake_case;
                    Self {
                        #primary_key_field_ident,
                        #select_snake_case,
                    }
                },
            );
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            generate_parameters_pattern_token_stream(
                &operation,
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream,
                impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream,
                impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream,
            )
        };
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &struct_options_ident_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                    &operation,
                    &primary_key_from_or_try_from,
                    &serde_json_syn_variant_wrapper,
                    &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
                    &{
                         let filter_not_unique_column_token_stream = generate_filter_not_unique_column_route_logic_token_stream(&operation);
                         quote::quote!{
                             #filter_not_unique_column_token_stream
                         }
                    },
                );
                let query_string_token_stream = {
                    let query_token_stream = proc_macro_common::generate_quotes::token_stream(
                        &format!("{select_snake_case} {{}} {from_snake_case} {table_name_stringified} {where_snake_case} {primary_key_field_ident} = $1"),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        format!(
                            #query_token_stream,
                            #generate_query_vec_column_snake_case_token_stream(&#parameters_snake_case.#payload_snake_case.#select_snake_case),
                        )
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = quote::quote! {
                        let #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case.#payload_snake_case.#primary_key_field_ident, #query_snake_case);
                    };
                    quote::quote! {
                        let #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_modifications_token_stream
                        #query_snake_case
                    }
                };
                // println!("{binded_query_token_stream}");
                let postgresql_logic_token_stream = {
                    let fetch_one_token_stream = generate_fetch_one_token_stream(
                        &generate_options_try_from_sqlx_row_token_stream(&operation),
                        &operation.generate_error_initialization_eprintln_response_creation_token_stream(
                            &postgresql_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        )
                    );
                    quote::quote!{
                        #fetch_one_token_stream
                        #value_snake_case
                    }
                };
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &table_name_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &syn_derive_input,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &check_body_size_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                    &postgresql_syn_variant_wrapper.generate_initialization_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &{
                    let mut value = common_http_request_syn_variants.clone();
                    value.push(not_unique_column_syn_variant_wrapper.get_syn_variant().clone());
                    value
                },
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &struct_options_ident_token_stream,
                &{
                    let filter_not_unique_column_token_stream = generate_filter_not_unique_column_http_request_token_stream(&operation);
                    quote::quote!{
                        #filter_not_unique_column_token_stream
                    }
                },
                &generate_read_operation_payload_with_serialize_deserialize_initialization_token_stream(&operation),
                &match fields_named_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{#struct_options_ident_token_stream::#from_snake_case(#value_snake_case)},
                    postgresql_crud_common::FromOrTryFrom::TryFrom => quote::quote!{#value_snake_case}
                },
                &reqwest_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &deserialize_response_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &failed_to_get_response_text_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &serde_json_to_string_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
            );
            // let try_operation_test_token_stream = {
            //     let element_fields_initialization_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         let field_type = &element.field.ty;
            //         quote::quote!{
            //             #field_ident: #field_type::default()
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         let #primary_key_token_stream = match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
            //                     #(#element_fields_initialization_token_stream),*
            //                 }
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => {
            //                 println!("{value:#?}");
            //                 value
            //             },
            //             Err(#error_snake_case) => panic!(
            //                 "{}",
            //                 #error_snake_case
            //             )
            //         };
            //     };
            //     naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #try_operation_token_stream
                },
                quote::quote! {}
                // try_operation_test_token_stream,
            )
        };
        // println!("{try_operation_token_stream}");
        //     let http_request_test_expect_success_token_stream = {
        //         let test_content_token_stream = quote::quote! {
        //             match #try_operation_snake_case_token_stream(
        //                 #reference_api_location_test_token_stream,
        //                 #operation_parameters_upper_camel_case_token_stream {
        //                     #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
        //                         #primary_key_field_ident: #primary_key_token_stream.clone(),//todo
        //                         #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream::#select_full_variant_token_stream
        //                     }
        //                 },
        //             )
        //             .await
        //             {
        //                 Ok(value) => println!("{value:#?}"),
        //                 Err(#error_snake_case) => panic!("{}", #error_snake_case)
        //             };
        //         };
        //         naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
        //     };
        //     let http_request_test_expect_fail_token_stream = {
        //         let test_content_token_stream = quote::quote! {
        //             match #try_operation_snake_case_token_stream(
        //                 #reference_api_location_test_token_stream,
        //                 #operation_parameters_upper_camel_case_token_stream {
        //                     #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
        //                         #primary_key_field_ident: #primary_key_token_stream.clone(),//todo
        //                         #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream::#select_full_variant_token_stream
        //                     }
        //                 },
        //             )
        //             .await
        //             {
        //                 Ok(value) => panic!("{value:#?}"),
        //                 Err(#error_snake_case) => println!("{}", #error_snake_case)
        //             };
        //         };
        //         naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
        //     };
        //     (
        //         quote::quote! {
        //             #try_operation_error_named_token_stream
        //             #try_operation_token_stream
        //         },
        //         http_request_test_expect_success_token_stream,
        //         http_request_test_expect_fail_token_stream,
        //     )
        // println!("{try_operation_token_stream}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_route_logic_token_stream
                #try_operation_token_stream
            },
            quote::quote! {},
            // http_request_test_expect_success_token_stream,
            quote::quote! {},
            // http_request_test_expect_fail_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &read_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (update_many_token_stream, update_many_test_token_stream) = {
        let operation = Operation::UpdateMany;
        let self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper = operation.generate_self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper(
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len());
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value.push(&row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(&non_existing_primary_keys_syn_variant_wrapper.get_syn_variant());
                value.push(&non_existing_primary_keys_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(&not_unique_primary_key_syn_variant_wrapper.get_syn_variant());
                value.push(&bind_query_syn_variant_wrapper.get_syn_variant());
                value.push(&no_payload_fields_primary_key_syn_variant_wrapper.get_syn_variant());
                value
            },
            &fields_named_from_or_try_from,
            &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
            &operation,
            &syn_derive_input,
        );
        let parameters_token_stream = {
            let (
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream
            ) = generate_payload_and_payload_with_serialize_deserialize_create_many_or_update_many(
                &operation,
                &update_fields_token_stream,
                &update_fields_with_serialize_deserialize_token_stream,
            );
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let impl_std_convert_from_or_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream = match fields_named_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => generate_impl_std_convert_from_self_payload_element_with_serialize_deserialize_for_self_payload_element_token_stream(
                        &operation,
                        &impl_std_convert_from_update_with_serialize_deserialize_for_update_token_stream
                    ),
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream(
                            &operation,
                            &update_try_from_update_with_serialize_deserialize_error_named_token_stream
                        );
                        // println!("{operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream}");
                        let impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream = generate_impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream(
                            &operation,
                            &impl_std_convert_try_from_update_with_serialize_deserialize_for_update_token_stream
                        );
                        // println!("{impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream}");
                        quote::quote! {
                            #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream
                            #impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream
                        }
                    }
                };
                let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = match fields_named_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => generate_impl_std_convert_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                        &operation,
                        &{
                            let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation);
                            quote::quote! {
                                Self (
                                    #value_snake_case.0.into_iter()//todo rewrite as try_from
                                    .map(|#element_snake_case|#operation_payload_element_upper_camel_case_token_stream::#from_snake_case(#element_snake_case))
                                    .collect()
                                )
                            }
                        },
                    ),
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = {
                            let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                                &operation,
                                &{
                                    let primary_key_variant_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_token_stream(
                                        &primary_key_syn_field_with_additional_info,
                                        &primary_key_field_ident_token_stream,
                                    );
                                    let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                                        &syn_field_with_additional_info_fields_named_excluding_primary_key,
                                    );
                                    quote::quote! {
                                        #primary_key_variant_token_stream
                                        #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                                    }
                                }
                            );
                            let impl_std_convert_from_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream_for_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                                &naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation),
                                &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation),
                                &{
                                    let generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = |
                                        value: &SynFieldWithAdditionalInfo<'_>,
                                        primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
                                    | -> proc_macro2::TokenStream {
                                        let code_occurence_snake_case = naming_conventions::CodeOccurenceSnakeCase;
                                        match value.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
                                            postgresql_crud_common::FromOrTryFrom::From => proc_macro2::TokenStream::new(),
                                            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                                                let field_ident_upper_camel_case_token_stream = syn_ident_to_upper_camel_case_token_stream(value.field_ident);
                                                let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                                                quote::quote!{
                                                    #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream::#field_ident_upper_camel_case_token_stream {
                                                        #primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                                        #code_occurence_snake_case,
                                                    } => Self::#field_ident_upper_camel_case_token_stream {
                                                        #primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                                        #code_occurence_snake_case,
                                                    },
                                                }
                                            }
                                        }
                                    };
                                    let primary_key_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream =     generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
                                        &primary_key_syn_field_with_additional_info,
                                        &primary_key_field_ident_token_stream,
                                    );
                                    let fields_named_excluding_primary_key_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
                                        let field_ident_token_stream = {
                                            let field_ident = &element.field_ident;
                                            quote::quote!{#field_ident}
                                        };
                                        generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
                                            element,
                                            &field_ident_token_stream,
                                        )
                                    });
                                    quote::quote! {
                                        match #value_snake_case {
                                            #primary_key_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream
                                            #(#fields_named_excluding_primary_key_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream)*
                                        }
                                    }
                                },
                            );
                            quote::quote! {
                                #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                                #impl_std_convert_from_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream_for_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                            }
                        };
                        let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_upper_camel_case_token_stream = generate_impl_std_convert_try_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                            &operation,
                            &{
                                let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation);
                                let std_vec_vec_operation_payload_element_token_stream = operation.std_vec_vec_self_payload_element_token_stream();
                                let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                                quote::quote! {
                                    match #value_snake_case.0.into_iter()//todo rewrite as try_from
                                        .map(|#element_snake_case|#operation_payload_element_upper_camel_case_token_stream::#try_from_snake_case(#element_snake_case))
                                        .collect::<Result<
                                            #std_vec_vec_operation_payload_element_token_stream,
                                            #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream
                                        >>()
                                    {
                                        Ok(#value_snake_case) => Ok(Self(#value_snake_case)),
                                        Err(#error_snake_case) => Err(Self::Error::#from_snake_case(#error_snake_case)),
                                    }
                                }
                            },
                        );
                        quote::quote! {
                            #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                            #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_upper_camel_case_token_stream
                        }
                    }
                };
                // println!("{impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
                quote::quote! {
                    #impl_std_convert_from_or_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream
                    #impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = generate_impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_and_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_token_stream(
                &operation,
                &impl_std_convert_from_update_for_update_with_serialize_deserialize_token_stream,
                &{
                    let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote! {
                        Self(
                            #value_snake_case.0.into_iter()
                            .map(|#element_snake_case|#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#element_snake_case))
                            .collect()
                        )
                    }
                },
            );
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            generate_parameters_pattern_token_stream(
                &operation,
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream,
                impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream,
                impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream,
            )
        };
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &std_vec_vec_primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            // println!("{try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream}");
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &fields_named_from_or_try_from,
                   &serde_json_syn_variant_wrapper,
                   &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
                   &{
                        let filter_not_unique_primary_key_token_stream = {
                            let not_unique_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                                &not_unique_primary_key_syn_variant_wrapper,
                                file!(),
                                line!(),
                                column!(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            quote::quote! {
                                {
                                    let mut #acc_snake_case = std::vec::Vec::new();
                                    for #element_snake_case in &#value_snake_case.0 {
                                        let #error_0_token_stream = #element_snake_case.#primary_key_field_ident;
                                        if !#acc_snake_case.contains(&#error_0_token_stream) {
                                            #acc_snake_case.push(#error_0_token_stream.clone());
                                        } else {
                                            #not_unique_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                                        }
                                    }
                                }

                            }
                        };
                        let filter_no_payload_fields_token_stream = {
                            let filter_no_payload_fields_element_token_stream = generate_filter_no_payload_fields_token_stream(&operation, &quote::quote! {#element_snake_case});
                            quote::quote! {
                                for #element_snake_case in &#value_snake_case.0 {
                                    #filter_no_payload_fields_element_token_stream
                                }
                            }
                        };
                        quote::quote!{
                            #filter_not_unique_primary_key_token_stream
                            #filter_no_payload_fields_token_stream
                        }
                   },
                );
                let expected_primary_keys_token_stream = quote::quote! {
                    let #expected_primary_keys_snake_case = #parameters_snake_case
                        .#payload_snake_case
                        .0
                        .iter()
                        .map(|#element_snake_case| #element_snake_case.#primary_key_field_ident.clone()) //todo - maybe its not a good idea to remove .clone here coz in macro dont know what type
                        .collect::<#std_vec_vec_primary_key_inner_type_token_stream>();
                };
                let query_string_token_stream = {
                    let query_start_token_stream = proc_macro_common::generate_quotes::token_stream(
                        &format!("{update_snake_case} {table_name_stringified} {set_snake_case} "),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let query_snake_case = naming_constants::QuerySnakeCase;
                    let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &bind_query_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let fields_named_excluding_primary_key_update_assignment_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        let is_field_ident_update_exists_token_stream = {
                            let is_snake_case = naming_constants::IsSnakeCase;
                            let value = format!("{is_snake_case}_{}_update_exist", &field_ident);
                            value.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let case_snake_case = naming_constants::CaseSnakeCase;
                        let field_ident_equals_case_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!("{field_ident} = {case_snake_case} "),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let else_snake_case = naming_constants::ElseSnakeCase;
                        let end_snake_case = naming_constants::EndSnakeCase;
                        let else_field_ident_end_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!("{else_snake_case} {field_ident} {end_snake_case},"),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let when_primary_key_field_ident_equals_then_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!(
                                "{} {primary_key_field_ident} = {{}} {} {{}} ",
                                naming_constants::WhenSnakeCase,
                                naming_constants::ThenSnakeCase
                            ),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote!{
                            {
                                let mut #is_field_ident_update_exists_token_stream = false;
                                for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                    if #element_snake_case.#field_ident.is_some() {
                                        #is_field_ident_update_exists_token_stream = true;
                                        break;
                                    }
                                }
                                if #is_field_ident_update_exists_token_stream {
                                    let mut #acc_snake_case = #std_string_string::#from_snake_case(#field_ident_equals_case_token_stream);
                                    for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                        if let Some(#value_snake_case) = &#element_snake_case.#field_ident {
                                            #acc_snake_case.push_str(&format!(
                                                #when_primary_key_field_ident_equals_then_token_stream,
                                                match postgresql_crud::BindQuery::try_generate_bind_increments(&#element_snake_case.#primary_key_field_ident, &mut increment) {
                                                    Ok(#value_snake_case) => #value_snake_case,
                                                    Err(#error_0_token_stream) => {
                                                        #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                    }
                                                },
                                                match postgresql_crud::BindQuery::try_generate_bind_increments(&#value_snake_case.#value_snake_case, &mut increment) {
                                                    Ok(#value_snake_case) => #value_snake_case,
                                                    Err(#error_0_token_stream) => {
                                                        #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                    }
                                                }
                                            ));
                                        }
                                    }
                                    #query_snake_case.push_str(&format!("{}{}",
                                        #acc_snake_case,
                                        #else_field_ident_end_token_stream
                                    ));
                                }
                            }
                        }
                    });
                    let where_primary_key_field_ident_in_primary_keys_returning_primary_key_field_ident_token_stream = {
                        let where_primary_key_field_ident_in_primary_keys_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!(
                                " {} {primary_key_field_ident} {} ({{}}) {returning_snake_case} {primary_key_field_ident};",
                                naming_constants::WhereSnakeCase,
                                naming_constants::InSnakeCase,
                            ),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote!{
                            #query_snake_case.push_str(&format!(
                                #where_primary_key_field_ident_in_primary_keys_quotes_token_stream,
                                {
                                    let mut #acc_snake_case = #std_string_string::default();
                                    for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                        match postgresql_crud::BindQuery::try_generate_bind_increments(&#element_snake_case.#primary_key_field_ident, &mut increment) {
                                            Ok(#value_snake_case) => {
                                                #acc_snake_case.push_str(&format!("{value},"));
                                            },
                                            Err(#error_0_token_stream) => {
                                                #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                            }
                                        }
                                    }
                                    let _ = #acc_snake_case.pop();
                                    #acc_snake_case
                                }
                            ));
                        }
                    };
                    quote::quote!{
                        {
                            let mut #query_snake_case = #std_string_string::#from_snake_case(#query_start_token_stream);
                            let mut increment: u64 = 0;
                            #(#fields_named_excluding_primary_key_update_assignment_token_stream)*
                            let _ = #query_snake_case.pop();
                            #where_primary_key_field_ident_in_primary_keys_returning_primary_key_field_ident_token_stream
                            #query_snake_case
                        }
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    //todo remove () if in fields named only one element
                    // let column_vecs_token_stream = fields_named.iter().map(|element|{
                    //     let field_ident_underscore_vec_stringified = format!("{}_{}", &element.field_ident, naming_constants::VecSnakeCase);
                    //     field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                    //     .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    // });
                    // let column_vecs_with_capacity_token_stream = fields_named.iter().map(|_|quote::quote!{std::vec::Vec::with_capacity(#current_vec_len_snake_case)});
                    // let columns_acc_push_elements_token_stream = fields_named.iter().enumerate().map(|(index, element)|{
                    //     let field_ident = &element.field_ident;
                    //     let index_token_stream = {
                    //         let index_stringified = format!("{index}");
                    //         index_stringified.parse::<proc_macro2::TokenStream>()
                    //         .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    //     };
                    //     quote::quote!{#acc_snake_case.#index_token_stream.push(#element_snake_case.#field_ident);}
                    // });
                    // let column_query_bind_primary_key_vec_token_stream = {
                    //     let field_ident_underscore_vec_token_stream = {
                    //         let field_ident_underscore_vec_stringified = format!(
                    //             "{primary_key_field_ident}_{}", naming_constants::VecSnakeCase
                    //         );
                    //         field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                    //         .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    //     };
                    //     quote::quote! {
                    //         #query_snake_case = #query_snake_case.bind(
                    //             #field_ident_underscore_vec_token_stream
                    //             .into_iter()
                    //             .map(|#element_snake_case| #element_snake_case.into_inner())
                    //             .collect::<std::vec::Vec<#primary_key_original_type_token_stream>>()
                    //         );
                    //     }
                    // };
                    // let column_query_bind_vecs_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                    //     let field_ident_underscore_vec_token_stream = {
                    //         let field_ident_underscore_vec_stringified = {
                    //             let field_ident = &element.field_ident;
                    //             format!("{field_ident}_{}", naming_constants::VecSnakeCase)
                    //         };
                    //         field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                    //         .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    //     };
                    //     let original_type_token_stream = &element.original_type_token_stream;
                    //     quote::quote!{
                    //         //todo this is wrong!!! if OptionField value in None then simpli ignore it, not need to push in vec
                    //         #query_snake_case = #query_snake_case.bind(
                    //             #field_ident_underscore_vec_token_stream
                    //                 .into_iter()
                    //                 .map(|#element_snake_case| match #element_snake_case.#value_snake_case {
                    //                     Some(#value_snake_case) => #value_snake_case.into_inner(),
                    //                     None => None
                    //                 })
                    //                 .collect::<std::vec::Vec<#original_type_token_stream>>(),
                    //         );
                    //     }
                    // });
                    let fields_named_excluding_primary_key_update_assignment_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        let is_field_ident_update_exists_token_stream = {
                            let is_snake_case = naming_constants::IsSnakeCase;
                            let value = format!("{is_snake_case}_{}_update_exist", &field_ident);
                            value.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        // let else_field_ident_end_token_stream = proc_macro_common::generate_quotes::token_stream(
                        //     &format!("{else_snake_case} {field_ident} {end_snake_case},"),
                        //     &proc_macro_name_upper_camel_case_ident_stringified,
                        // );
                        // let when_primary_key_field_ident_equals_then_token_stream = proc_macro_common::generate_quotes::token_stream(
                        //     &format!(
                        //         "{} {primary_key_field_ident} = {{}} {} {{}} ",
                        //         naming_constants::WhenSnakeCase,
                        //         naming_constants::ThenSnakeCase
                        //     ),
                        //     &proc_macro_name_upper_camel_case_ident_stringified,
                        // );
                        quote::quote!{
                            {
                                let mut #is_field_ident_update_exists_token_stream = false;
                                for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                    if #element_snake_case.#field_ident.is_some() {
                                        #is_field_ident_update_exists_token_stream = true;
                                        break;
                                    }
                                }
                                if #is_field_ident_update_exists_token_stream {  
                                    for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                        if let Some(#value_snake_case) = &#element_snake_case.#field_ident {
                                            #query_snake_case = #query_snake_case.bind(#element_snake_case.#primary_key_field_ident.into_inner());
                                            #query_snake_case = #query_snake_case.bind(#value_snake_case.#value_snake_case.0);
                                        }
                                    }
                                }
                            }
                        }
                    });
                    let primary_key_update_assignment_token_stream = quote::quote! {
                        {
                            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                #query_snake_case = #query_snake_case.bind(#element_snake_case.#primary_key_field_ident.into_inner());
                            }
                        }
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#fields_named_excluding_primary_key_update_assignment_token_stream)*
                        #primary_key_update_assignment_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &{
                        let fetch_token_stream = generate_fetch_token_stream(
                            &generate_sqlx_row_try_get_primary_key_token_stream(
                                &quote::quote!{Some(#primary_key_inner_type_token_stream(#value_snake_case))},
                                &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(
                                    &operation,
                                    file!(),
                                    line!(),
                                    column!(),
                                    file!(),
                                    line!(),
                                    column!(),
                                )
                            ),
                            &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(
                                &operation,
                                file!(),
                                line!(),
                                column!(),
                                file!(),
                                line!(),
                                column!(),
                            )
                        );
                        let non_existing_primary_keys_check_token_stream = generate_non_existing_primary_keys_check_token_stream(
                            &operation,
                            &expected_primary_keys_snake_case,
                        );
                        quote::quote!{
                            #fetch_token_stream
                            {
                                #non_existing_primary_keys_check_token_stream
                            }
                            let #value_snake_case = #value_snake_case.into_iter().map(
                                |#element_snake_case|#primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#element_snake_case)
                            ).collect();
                        }
                    },
                );
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &table_name_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &syn_derive_input,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &expected_primary_keys_token_stream,
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &check_body_size_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                    &postgresql_syn_variant_wrapper.generate_initialization_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!(" {try_operation_route_logic_token_stream}");
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &{
                    let mut value = common_http_request_with_possible_operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variants.clone();
                    value.push(not_unique_primary_key_syn_variant_wrapper.get_syn_variant().clone());
                    value
                },
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_inner_type_token_stream,
                &{
                    let filter_not_unique_primary_key_token_stream = {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        let not_unique_primary_key_syn_variant_initialization_token_stream = not_unique_primary_key_syn_variant_wrapper.generate_initialization_token_stream(
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote!{
                            let mut #acc_snake_case = std::vec::Vec::new();
                            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                let #error_0_token_stream = #element_snake_case.#primary_key_field_ident;
                                if !#acc_snake_case.contains(&&#element_snake_case.#primary_key_field_ident) {
                                    #acc_snake_case.push(&#element_snake_case.#primary_key_field_ident);
                                } else {
                                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#not_unique_primary_key_syn_variant_initialization_token_stream);
                                }
                            }
                        }
                    };
                    quote::quote!{
                        #filter_not_unique_primary_key_token_stream
                    }
                },
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream_option {
                    None => quote::quote!{
                        #value_snake_case
                        .into_iter()
                        .map(|#element_snake_case| #primary_key_inner_type_token_stream::#from_snake_case(#element_snake_case))
                        .collect()
                    },
                    Some(syn_variant_initialization_token_stream) => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            {
                                let mut values = std::vec::Vec::new();
                                for #element_snake_case in #value_snake_case {
                                    match #primary_key_inner_type_token_stream::#try_from_snake_case(#element_snake_case) {
                                        Ok(#value_snake_case) => {
                                            values.push(#value_snake_case);
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(
                                                #try_operation_error_named_upper_camel_case_token_stream::#syn_variant_initialization_token_stream
                                            );
                                        }
                                    }
                                }
                                values
                            }
                        }
                    }
                },
                &reqwest_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &deserialize_response_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &failed_to_get_response_text_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &serde_json_to_string_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
            );
            // println!("{try_operation_token_stream}");
            // let try_operation_test_token_stream = {
            //     let fields_initialization_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         let field_type = &element.field.ty;
            //         quote::quote!{
            //             #field_ident: #field_type::default()
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         match #try_operation_snake_case_token_stream(
            //             &api_location,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream (
            //                     #primary_keys_token_stream.clone().into_iter().map(|element| {
            //                         #operation_payload_element_upper_camel_case_token_stream {
            //                             #primary_key_field_ident: element,
            //                             #(#fields_initialization_excluding_primary_key_token_stream),*//todo make sure name and color both are not None(make it option<value>, not just a value)
            //                         }
            //                     }).collect()
            //                 )
            //             }
            //         )
            //         .await
            //         {
            //             Ok(value) => println!("{value:#?}"),
            //             Err(#error_snake_case) => panic!("{}", #error_snake_case)
            //         }
            //     };
            //     naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #try_operation_token_stream
                },
                quote::quote! {}
                // try_operation_test_token_stream,
            )
        };
        // println!("{try_operation_token_stream}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_route_logic_token_stream
                #try_operation_token_stream
            },
            quote::quote! {}
            // try_operation_test_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &update_many_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    // //todo WHY ITS RETURN SUCCESS EVEN IF ROW DOES NOT EXISTS?
    let (update_one_token_stream, update_one_test_token_stream) = {
        let operation = Operation::UpdateOne;
        let self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper = operation.generate_self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper(
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 1);
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value.push(&bind_query_syn_variant_wrapper.get_syn_variant());
                value.push(&no_payload_fields_primary_key_syn_variant_wrapper.get_syn_variant());
                value.push(&row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value
            },
            &fields_named_from_or_try_from,
            &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
            &operation,
            &syn_derive_input,
        );
        let parameters_token_stream = {
            let payload_token_stream = generate_operation_payload_token_stream(
                &operation,
                &update_fields_token_stream,
            );
            let payload_with_serialize_deserialize_token_stream = generate_payload_with_serialize_deserialize_token_stream(
                &operation,
                &update_fields_with_serialize_deserialize_token_stream,
            );
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = match fields_named_from_or_try_from {
                postgresql_crud_common::FromOrTryFrom::From => generate_impl_std_convert_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                    &operation,
                    &impl_std_convert_from_update_with_serialize_deserialize_for_update_token_stream
                ),
                postgresql_crud_common::FromOrTryFrom::TryFrom => {
                    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                        &operation,
                        &update_try_from_update_with_serialize_deserialize_error_named_token_stream
                    );
                    // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_try_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                        &operation,
                        &impl_std_convert_try_from_update_with_serialize_deserialize_for_update_token_stream
                    );
                    // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
                    quote::quote! {
                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                        #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                    }
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = generate_impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream(
                &operation,
                &impl_std_convert_from_update_for_update_with_serialize_deserialize_token_stream
            );
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            generate_parameters_pattern_token_stream(
                &operation,
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream,
                impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream,
                impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream,
            )
        };
        //  println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &fields_named_from_or_try_from,
                   &serde_json_syn_variant_wrapper,
                   &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
                   &{
                        let filter_no_payload_fields_token_stream = generate_filter_no_payload_fields_token_stream(&operation, &quote::quote! {#value_snake_case});
                        quote::quote!{
                            #filter_no_payload_fields_token_stream
                        }
                   },
                );
                let query_string_token_stream = {
                    let query_start_token_stream = proc_macro_common::generate_quotes::token_stream(
                        &format!("{update_snake_case} {table_name_stringified} {set_snake_case} "),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &bind_query_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let additional_parameters_modification_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        // let handle_token_stream = {
                        //     let possible_dot_space = if (
                        //         index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE))
                        //     ) == fields_named_excluding_primary_key_len {
                        //         ""
                        //     }
                        //     else {
                        //         dot_space
                        //     };
                        //     proc_macro_common::generate_quotes::token_stream(
                        //         &format!("{field_ident} = ${{increment}}{possible_dot_space}"),
                        //         &proc_macro_name_upper_camel_case_ident_stringified,
                        //     )
                        // };
                        let field_ident_equals_dollar_increment_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!("{field_ident} = ${{}},"),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote!{
                            if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                match #postgresql_crud_bind_query_bind_query_try_increment_token_stream(
                                    &#value_snake_case.#value_snake_case,
                                    &mut increment,
                                ) {
                                    Ok(_) => {
                                        #query_snake_case.push_str(&format!(#field_ident_equals_dollar_increment_token_stream, increment));
                                    }
                                    Err(#error_0_token_stream) => {
                                        #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                        }
                    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                    let additional_parameters_primary_key_modification_token_stream = {
                        let query_part_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!(" {where_snake_case} {primary_key_field_ident} = ${{increment}}"),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            match #postgresql_crud_bind_query_bind_query_try_increment_token_stream(&#parameters_snake_case.#payload_snake_case.#primary_key_field_ident, &mut increment) {
                                Ok(_) => {
                                    #query_snake_case.push_str(&format!(#query_part_token_stream));
                                },
                                Err(#error_0_token_stream) => {
                                    #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                },
                            }
                        }
                    };
                    quote::quote! {
                        {
                            #increment_initialization_token_stream
                            let mut #query_snake_case = #std_string_string::#from_snake_case(#query_start_token_stream);
                            #(#additional_parameters_modification_token_stream)*
                            let _ = #query_snake_case.pop();
                            #additional_parameters_primary_key_modification_token_stream
                            #query_snake_case.push_str(&format!(#returning_primary_key_quotes_token_stream));
                            #query_snake_case
                        }
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        quote::quote!{
                            if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                #query_snake_case = postgresql_crud::BindQuery::bind_value_to_query(
                                    #value_snake_case.#value_snake_case,
                                    #query_snake_case,
                                );
                            }
                        }
                    });
                    let binded_query_primary_key_modification_token_stream = quote::quote! {
                        #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            #query_snake_case,
                        );
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        #binded_query_primary_key_modification_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &{
                        let fetch_one_token_stream = generate_fetch_one_token_stream(
                            &generate_sqlx_row_try_get_primary_key_token_stream(
                                &quote::quote!{#primary_key_inner_type_token_stream(#value_snake_case)},
                                &generate_match_postgres_transaction_rollback_await_token_stream(
                                    &operation,
                                    file!(),
                                    line!(),
                                    column!(),
                                    file!(),
                                    line!(),
                                    column!(),
                                )
                            ),
                            &generate_match_postgres_transaction_rollback_await_token_stream(
                               &operation,
                               file!(),
                               line!(),
                               column!(),
                               file!(),
                               line!(),
                               column!(),
                            )
                        );
                        quote::quote!{
                            #fetch_one_token_stream
                            let #value_snake_case = #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#value_snake_case);
                        }
                    },
                );
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &table_name_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &syn_derive_input,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &check_body_size_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                    &postgresql_syn_variant_wrapper.generate_initialization_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!(" {try_operation_route_logic_token_stream}");
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_with_possible_operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variants,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &primary_key_inner_type_token_stream,
                &proc_macro2::TokenStream::new(),
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream_option {
                    None => quote::quote!{#primary_key_inner_type_token_stream::#from_snake_case(#value_snake_case)},
                    Some(syn_variant_initialization_token_stream) => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            match #primary_key_inner_type_token_stream::#try_from_snake_case(#value_snake_case) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_snake_case) => {
                                    return Err(
                                        #try_operation_error_named_upper_camel_case_token_stream::#syn_variant_initialization_token_stream
                                    );
                                }
                            }
                        }
                    }
                },
                &reqwest_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &deserialize_response_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &failed_to_get_response_text_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &serde_json_to_string_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
            );
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #try_operation_token_stream
                },
                quote::quote! {}
                // try_operation_test_token_stream,
            )
        };
        // println!("{try_operation_token_stream}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_route_logic_token_stream
                #try_operation_token_stream
            },
            // try_operation_test_token_stream,
            quote::quote! {},
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &update_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (delete_many_token_stream, delete_many_test_token_stream) = {
        let operation = Operation::DeleteMany;
        let self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper = operation.generate_self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper(
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        // let not_unique_primary_key_syn_variant_initialization_token_stream = generate_not_unique_primary_key_syn_variant_initialization_token_stream(&quote::quote!{#element_snake_case.clone()});
        let no_payload_fields_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
            &naming_conventions::NoPayloadFieldsUpperCamelCase,
            Some(proc_macro_helpers::status_code::StatusCode::BadRequest400),
            std::vec::Vec::<(
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute,
                &'static dyn std::fmt::Display,
                syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
            )>::default(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let no_primary_keys_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
            &naming_conventions::NoPrimaryKeysUpperCamelCase,
            Some(proc_macro_helpers::status_code::StatusCode::BadRequest400),
            std::vec::Vec::<(
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute,
                &'static dyn std::fmt::Display,
                syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
            )>::default(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // let not_unique_field_vec_syn_variants = fields_named_excluding_primary_key
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 1);
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value.push(&bind_query_syn_variant_wrapper.get_syn_variant());
                value.push(&no_payload_fields_syn_variant_wrapper.get_syn_variant());
                value.push(&no_primary_keys_syn_variant_wrapper.get_syn_variant());
                value.push(&row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(&non_existing_primary_keys_syn_variant_wrapper.get_syn_variant());
                value.push(&non_existing_primary_keys_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(&not_unique_primary_key_syn_variant_wrapper.get_syn_variant());
                not_unique_fields_syn_variants_wrappers.iter().for_each(|element|{
                    value.push(element.get_syn_variant());
                });
                value
            },
            &fields_named_from_or_try_from,
            &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
            &operation,
            &syn_derive_input,
        );
        let parameters_token_stream = {
            let (
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream
            ) = {
                let generate_fields_token_stream = |is_pub: bool| -> proc_macro2::TokenStream {
                    let primary_key_field_ident_std_option_option_std_vec_vec_primary_key_inner_type_handle_token_stream = generate_primary_key_field_ident_std_option_option_std_vec_vec_primary_key_inner_type_handle_token_stream(is_pub);
                    let fields_idents_std_option_option_std_vec_vec_where_inner_type_handle_token_stream = generate_fields_idents_std_option_option_std_vec_vec_where_inner_type_handle_token_stream(is_pub);
                    quote::quote! {
                        #primary_key_field_ident_std_option_option_std_vec_vec_primary_key_inner_type_handle_token_stream,
                        #fields_idents_std_option_option_std_vec_vec_where_inner_type_handle_token_stream
                    }
                };
                let payload_token_stream = generate_operation_payload_token_stream(
                    &operation,
                    &generate_fields_token_stream(true),
                );
                // println!("{payload_token_stream}");
                let payload_with_serialize_deserialize_token_stream = generate_payload_with_serialize_deserialize_token_stream(
                    &operation,
                    &generate_fields_token_stream(false),
                );
                // println!("{payload_with_serialize_deserialize_token_stream}");
                (
                    payload_token_stream,
                    payload_with_serialize_deserialize_token_stream
                )
            };
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = match fields_named_from_or_try_from {
                postgresql_crud_common::FromOrTryFrom::From => generate_impl_std_convert_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                    &operation,
                    &{
                        let fields_assignments_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
                            let field_ident = &element.field_ident;
                            let where_inner_type_token_stream = &element.where_inner_type_token_stream;
                            quote::quote! {
                                let #field_ident = match #value_snake_case.#field_ident {
                                    Some(#value_snake_case) => Some(#value_snake_case.into_iter().map(|#element_snake_case|#where_inner_type_token_stream::#from_snake_case(#element_snake_case)).collect()),
                                    None => None,
                                };
                            }
                        });
                        //todo maybe use where with conjuctive operator
                        let primary_key_let_field_ident_value_field_ident_from_token_stream = quote::quote! {
                            let #primary_key_field_ident = match #value_snake_case.#primary_key_field_ident {
                                Some(#value_snake_case) => Some(#value_snake_case.into_iter().map(|#element_snake_case|#primary_key_inner_type_token_stream::#from_snake_case(#element_snake_case)).collect()),
                                None => None,
                            };
                        };
                        quote::quote! {
                            #primary_key_let_field_ident_value_field_ident_from_token_stream
                            #(#fields_assignments_token_stream)*
                            Self{
                                #(#self_init_fields_token_stream),*
                            }
                        }
                    },
                ),
                postgresql_crud_common::FromOrTryFrom::TryFrom => {
                    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                        &operation,
                        &{
                            let primary_key_variant_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_token_stream(
                                &primary_key_syn_field_with_additional_info,
                                &primary_key_field_ident_token_stream,
                            );
                            quote::quote! {
                                #primary_key_variant_token_stream
                                #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                            }
                        },
                    );
                    // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_try_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                        &operation,
                        &{
                            let primary_key_let_field_ident_value_field_ident_try_from_token_stream = {
                                let inner_token_stream = quote::quote! {#value_snake_case.#primary_key_field_ident};
                                let initialization_token_stream = match primary_key_rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
                                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                                        match #inner_token_stream {
                                            Some(#value_snake_case) => Some(#value_snake_case.into_iter().map(|#element_snake_case|#primary_key_inner_type_token_stream::#from_snake_case(#element_snake_case)).collect()),
                                            None => None,
                                        }
                                    },
                                    postgresql_crud_common::FromOrTryFrom::TryFrom => quote::quote!{
                                        match #inner_token_stream {
                                            Some(#value_snake_case) => {
                                                let mut values = std::vec::Vec::with_capacity(#value_snake_case.len());
                                                for #element_snake_case in #value_snake_case {
                                                    match #primary_key_inner_type_token_stream::#try_from_snake_case(#element_snake_case) {
                                                        Ok(#value_snake_case) => {
                                                            values.push(#value_snake_case);
                                                        }
                                                        Err(#error_snake_case) => {
                                                            return Err(Self::Error::#primary_key_field_ident_variant_initialization_token_stream);
                                                        }
                                                    }
                                                }
                                                Some(values)
                                            }
                                            None => None,
                                        }
                                    }
                                };
                                quote::quote! {
                                    let #primary_key_field_ident = #initialization_token_stream;
                                }
                            };
                            let fields_assignments_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter()
                                .map(|element| generate_option_vec_where_inner_type_from_or_try_from_option_vec_where_inner_type_with_serialize_deserialize_token_stream(
                                    element,
                                    &primary_key_field_ident_token_stream,
                                ));
                            quote::quote! {
                                #primary_key_let_field_ident_value_field_ident_try_from_token_stream
                                #(#fields_assignments_token_stream)*
                                Ok(Self {
                                    #(#self_init_fields_token_stream),*
                                })
                            }
                        },
                    );
                    quote::quote! {
                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                        #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                    }
                },
            };
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = generate_impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream(
                &operation,
                &{
                    let fields_assignments_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(generate_let_field_ident_value_option_vec_with_serialize_deserialize_token_stream);
                    quote::quote! {
                        let #primary_key_field_ident = match #value_snake_case.#primary_key_field_ident {
                            Some(#value_snake_case) => Some(#value_snake_case.into_iter()
                                .map(|#element_snake_case|#primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#element_snake_case))
                                .collect::<std::vec::Vec<#primary_key_inner_type_with_serialize_deserialize_token_stream>>()),
                            None => None,
                        };
                        #(#fields_assignments_token_stream)*
                        Self{
                            #(#self_init_fields_token_stream),*
                        }
                    }
                },
            );
            generate_parameters_pattern_token_stream(
                &operation,
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream,
                impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream,
                impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream,
            )
        };
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &std_vec_vec_primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                    &operation,
                    &fields_named_from_or_try_from,
                    &serde_json_syn_variant_wrapper,
                    &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
                    &{
                        let filter_no_payload_fields_token_stream = {
                            let none_fields_named_token_stream = syn_field_with_additional_info_fields_named.iter().map(|_|{
                                let none_upper_camel_case = naming_constants::NoneUpperCamelCase;
                                quote::quote!{#none_upper_camel_case}
                            });
                            let value_fields_named_token_stream = syn_field_with_additional_info_fields_named.iter().map(|element|{
                                let field_ident = &element.field_ident;
                                quote::quote!{&#value_snake_case.#field_ident}
                            });
                            let no_payload_fields_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                                &no_payload_fields_syn_variant_wrapper,
                                file!(),
                                line!(),
                                column!(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            quote::quote!{
                                if let (#(#none_fields_named_token_stream),*) = (#(#value_fields_named_token_stream),*) {
                                    #no_payload_fields_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        };
                        let filter_no_primary_keys_token_stream = {
                            let no_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                                &no_primary_keys_syn_variant_wrapper,
                                file!(),
                                line!(),
                                column!(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            quote::quote!{
                                if let Some(#value_snake_case) = &#value_snake_case.#primary_key_field_ident {
                                    if #value_snake_case.is_empty() {
                                        #no_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                        };
                        let filter_not_unique_fields_token_stream = {
                            let filter_not_unique_primary_key_token_stream = {
                                let not_unique_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                                    &not_unique_primary_key_syn_variant_wrapper,
                                    file!(),
                                    line!(),
                                    column!(),
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                quote::quote! {
                                    if let Some(#primary_key_field_ident) = &#value_snake_case.#primary_key_field_ident {
                                        let mut #acc_snake_case = std::vec::Vec::new();
                                        for #element_snake_case in #primary_key_field_ident {
                                            if !#acc_snake_case.contains(&#element_snake_case) {
                                                #acc_snake_case.push(&#element_snake_case);
                                            }
                                            else {
                                                let #error_0_token_stream = *#element_snake_case;
                                                #not_unique_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                                            }
                                        }
                                    }
                                }
                            };
                            let filter_not_unique_fields_named_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
                                let element_field_ident = &element.field_ident;
                                let field_ident_token_stream = {
                                    let value = format!(
                                        "{}::{}",
                                        postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE,
                                        postgresql_crud_common::SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(
                                            &postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)
                                        ),
                                    );
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                //todo remove syn_variant_wrapper creation duplication
                                let not_unique_fields_syn_variant_wrapper = proc_macro_helpers::construct_syn_variant::SynVariantWrapper::new(
                                    &format!(
                                        "{}{}",
                                        naming_conventions::NotUniqueUpperCamelCase,
                                        proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&element.field_ident.to_string()),
                                    ),
                                    Some(not_unique_primary_key_syn_variant_wrapper.get_option_status_code().unwrap()),
                                    vec![
                                        (
                                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                                            &format!(
                                                "{}_{}",
                                                naming_conventions::NotUniqueSnakeCase,
                                                &element.field_ident,
                                            ),
                                            {
                                                let mut value = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                                                value.push_value(syn::PathSegment {
                                                    ident: proc_macro2::Ident::new(&postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, proc_macro2::Span::call_site()),
                                                    arguments: syn::PathArguments::None,
                                                });
                                                value.push_punct(syn::token::PathSep {
                                                    spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                                                });
                                                value.push_value(syn::PathSegment {
                                                    ident: proc_macro2::Ident::new(
                                                        &postgresql_crud_common::SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(
                                                            &postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)
                                                        ).to_string(),
                                                        proc_macro2::Span::call_site()
                                                    ),
                                                    arguments: syn::PathArguments::None,
                                                });
                                                value
                                            }
                                        )
                                    ],
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                let not_unique_fields_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                                    &not_unique_fields_syn_variant_wrapper,
                                    file!(),
                                    line!(),
                                    column!(),
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                quote::quote!{
                                    if let Some(#value_snake_case) = &#value_snake_case.#element_field_ident {
                                        let mut #acc_snake_case = std::vec::Vec::new();
                                        for #element_snake_case in #value_snake_case {
                                            if let Some(#value_snake_case) = &#element_snake_case.#value_snake_case.0 {
                                                if !#acc_snake_case.contains(&#value_snake_case) {
                                                    #acc_snake_case.push(&#value_snake_case);
                                                }
                                                else {
                                                    let #error_0_token_stream = #field_ident_token_stream(#value_snake_case.clone());
                                                    #not_unique_fields_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                            quote::quote! {
                                #filter_not_unique_primary_key_token_stream
                                #(#filter_not_unique_fields_named_excluding_primary_key_token_stream)*
                            }
                        };
                        quote::quote!{
                            #filter_no_payload_fields_token_stream
                            #filter_no_primary_keys_token_stream
                            #filter_not_unique_fields_token_stream
                        }
                    },
                );
                let expected_primary_keys_token_stream = quote::quote! {
                    let #expected_primary_keys_snake_case = #parameters_snake_case.#payload_snake_case.#primary_key_field_ident.clone();
                };
                let query_string_token_stream = {
                    let additional_parameters_modification_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        let handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!("{field_ident} = ${{increment}}"),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                            &bind_query_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote!{
                            if let Some(value) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                for element in value {
                                    match #postgresql_crud_bind_query_bind_query_try_increment_token_stream(
                                        element,
                                        &mut increment
                                    ) {
                                        Ok(_) => {
                                            let handle = format!(#handle_token_stream);
                                            match additional_parameters.is_empty() {
                                                true => {
                                                    additional_parameters.push_str(&handle);
                                                },
                                                false => {
                                                    additional_parameters.push_str(&format!(" AND {handle}"));//todo
                                                },
                                            }
                                        },
                                        Err(#error_0_token_stream) => {
                                            #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        },
                                    }
                                }
                            }
                        }
                    });
                    let additional_parameters_primary_key_modification_token_stream = {
                        let handle_token_stream = {
                            let handle_stringified = format!(
                                "\" {primary_key_field_ident} {in_snake_case} ({{}})\""
                            );
                            handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let additional_parameters_and_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!(" {and_snake_case}"),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = operation.generate_error_initialization_eprintln_response_creation_token_stream(
                            &bind_query_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            if let Some(#primary_key_field_ident) = &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                                if let false = additional_parameters.is_empty() {
                                    additional_parameters.push_str(#additional_parameters_and_token_stream);
                                }
                                additional_parameters.push_str(&format!(
                                    #handle_token_stream,
                                    {
                                        let mut additional_parameters = #std_string_string::default();
                                        for #element_snake_case in #primary_key_field_ident {
                                            match #postgresql_crud_bind_query_bind_query_try_increment_token_stream(
                                                #element_snake_case,
                                                &mut increment,
                                            ) {
                                                Ok(_) => {
                                                    additional_parameters.push_str(&format!("${increment},"));
                                                }
                                                Err(#error_0_token_stream) => {
                                                    #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                }
                                            }
                                        }
                                        let _ = additional_parameters.pop();
                                        additional_parameters
                                    }
                                ));
                            }
                        }
                    };
                    let handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                        &format!("{delete_snake_case} {from_snake_case} {table_name_stringified} {where_snake_case} {{}}{returning_primary_key_stringified}"),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        format!(
                            #handle_token_stream,
                            {
                                #increment_initialization_token_stream
                                let mut additional_parameters = #std_string_string::default();
                                #(#additional_parameters_modification_token_stream)*
                                #additional_parameters_primary_key_modification_token_stream
                                additional_parameters
                            }
                        )
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        quote::quote!{
                            if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                for #element_snake_case in #value_snake_case {
                                    #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(#element_snake_case, #query_snake_case);
                                }
                            }
                        }
                    });
                    let binded_query_primary_key_modifications_token_stream = quote::quote! {
                        if let Some(#primary_key_field_ident) = #parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                            for #element_snake_case in #primary_key_field_ident {
                                #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(#element_snake_case, #query_snake_case);
                            }
                        }
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        #binded_query_primary_key_modifications_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &{
                        let fetch_token_stream = generate_fetch_token_stream(
                            &generate_sqlx_row_try_get_primary_key_token_stream(
                                &quote::quote!{Some(#primary_key_inner_type_token_stream(#value_snake_case))},
                                &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(
                                    &operation,
                                    file!(),
                                    line!(),
                                    column!(),
                                    file!(),
                                    line!(),
                                    column!(),
                                )
                            ),
                            &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(
                                &operation,
                                file!(),
                                line!(),
                                column!(),
                                file!(),
                                line!(),
                                column!(),
                            )
                        );
                        let non_existing_primary_keys_check_token_stream = generate_non_existing_primary_keys_check_token_stream(
                            &operation,
                            &error_snake_case,
                        );
                        quote::quote!{
                            #fetch_token_stream
                            {
                                if let Some(#error_snake_case) = #expected_primary_keys_snake_case {
                                    #non_existing_primary_keys_check_token_stream
                                }
                            }
                            let #value_snake_case = #value_snake_case.into_iter().map(
                                |#element_snake_case|#primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#element_snake_case)
                            ).collect();
                        }
                    },
                );
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &table_name_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &syn_derive_input,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &expected_primary_keys_token_stream,
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &check_body_size_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                    &postgresql_syn_variant_wrapper.generate_initialization_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_with_possible_operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variants,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_inner_type_token_stream,
                &proc_macro2::TokenStream::new(),
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream_option {
                    None => quote::quote!{
                        #value_snake_case
                        .into_iter()
                        .map(|#element_snake_case| #primary_key_inner_type_token_stream::#from_snake_case(#element_snake_case))
                        .collect()
                    },
                    Some(syn_variant_initialization_token_stream) => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            {
                                let mut values = std::vec::Vec::new();
                                for #element_snake_case in #value_snake_case {
                                    match #primary_key_inner_type_token_stream::#try_from_snake_case(#element_snake_case) {
                                        Ok(#value_snake_case) => {
                                            values.push(#value_snake_case);
                                        },
                                        Err(#error_snake_case) => {
                                            return Err(
                                                #try_operation_error_named_upper_camel_case_token_stream::#syn_variant_initialization_token_stream
                                            );
                                        }
                                    }
                                }
                                values
                            }
                        }
                    }
                },
                &reqwest_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &deserialize_response_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &failed_to_get_response_text_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &serde_json_to_string_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
            );
            // println!("{try_operation_token_stream}");
            // let try_operation_test_token_stream = {
            //     let fields_initialization_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         quote::quote!{
            //             //todo or and support where filter
            //             #field_ident: None
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             //todo - builder pattern?
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
            //                     #primary_key_field_ident: Some(
            //                         #primary_keys_token_stream.clone()
            //                     ),
            //                     #(#fields_initialization_excluding_primary_key_token_stream),*
            //                 }
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => println!("{value:#?}"),
            //             // let vec_cat_id: Vec<
            //             //     super::DogId,
            //             // > = value
            //             //     .into_iter()
            //             //     .filter_map(|value| match value.id {
            //             //         Some(id) => Some(
            //             //             super::DogId {
            //             //                 id,
            //             //             },
            //             //         ),
            //             //         None => None,
            //             //     })
            //             //     .collect();
            //             // println!("{vec_cat_id:#?}");
            //             Err(#error_snake_case) => panic!("{}", #error_snake_case)
            //         }
            //     };
            //     naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #try_operation_token_stream
                },
                quote::quote! {}
                // try_operation_test_token_stream,
            )
        };
        // println!("{try_operation_token_stream}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_route_logic_token_stream
                #try_operation_token_stream
            },
            quote::quote! {}
            // try_operation_test_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &delete_many_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (delete_one_token_stream, delete_one_test_token_stream) = {
        let operation = Operation::DeleteOne;
        let self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper = operation.generate_self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper(
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 1);
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value.push(&row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value
            },
            &primary_key_from_or_try_from,
            &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
            &operation,
            &syn_derive_input,
        );
        let parameters_token_stream = {
            let (
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream
            ) = {
                let generate_fields_token_stream = |is_pub: bool| -> proc_macro2::TokenStream {
                    let pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream(is_pub);
                    quote::quote! {
                        #pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream
                    }
                };
                let payload_token_stream = generate_operation_payload_token_stream(
                    &operation,
                    &generate_fields_token_stream(true)
                );
                // println!("{payload_token_stream}");
                let payload_with_serialize_deserialize_token_stream = generate_payload_with_serialize_deserialize_token_stream(
                    &operation,
                    &generate_fields_token_stream(false)
                );
                // println!("{payload_with_serialize_deserialize_token_stream}");
                (
                    payload_token_stream,
                    payload_with_serialize_deserialize_token_stream
                )
            };
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = match &primary_key_from_or_try_from {
                postgresql_crud_common::FromOrTryFrom::From => generate_impl_std_convert_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                    &operation,
                    &quote::quote! {
                        Self{ #primary_key_field_ident: #primary_key_inner_type_token_stream::#from_snake_case(value.#primary_key_field_ident) }
                    },
                ),
                postgresql_crud_common::FromOrTryFrom::TryFrom => {
                    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                        &operation,
                        &{
                            let primary_key_variant_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_token_stream(
                                &primary_key_syn_field_with_additional_info,
                                &primary_key_field_ident_token_stream,
                            );
                            quote::quote! {
                                #primary_key_variant_token_stream
                            }
                        },
                    );
                    // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_try_from_self_payload_with_serialize_deserialize_for_self_payload_token_stream(
                        &operation,
                        &{
                            let logic_token_stream =  {
                                let inner_token_stream = quote::quote! {#value_snake_case.#primary_key_field_ident};
                                match primary_key_rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
                                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                                        Ok(Self{ #primary_key_field_ident: #primary_key_inner_type_token_stream::#from_snake_case(#inner_token_stream) })
                                    },
                                    postgresql_crud_common::FromOrTryFrom::TryFrom => quote::quote!{
                                        match #primary_key_inner_type_token_stream::#try_from_snake_case(#inner_token_stream) {
                                            Ok(#value_snake_case) => Ok(Self{ #primary_key_field_ident: #value_snake_case }),
                                            Err(#error_snake_case) => Err(Self::Error::#primary_key_field_ident_variant_initialization_token_stream)
                                        }
                                    }
                                }
                            };
                            quote::quote! {
                                #logic_token_stream
                            }
                        },
                    );
                    quote::quote! {
                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                        #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                    }
                }
            };
            // println!("{impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = generate_impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream(
                &operation,
                &quote::quote! {
                    let #primary_key_field_ident = #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(value.#primary_key_field_ident);
                    Self{
                        #primary_key_field_ident
                    }
                }
            );
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            generate_parameters_pattern_token_stream(
                &operation,
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream,
                impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream,
                impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream,
            )
        };
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &primary_key_from_or_try_from,
                   &serde_json_syn_variant_wrapper,
                   &self_payload_try_from_self_payload_with_serialize_deserialize_syn_variant_wrapper,
                   &proc_macro2::TokenStream::new(),
                );
                let query_string_token_stream = {
                    let additional_parameters_primary_key_modification_token_stream = {
                        let query_part_token_stream = proc_macro_common::generate_quotes::token_stream(
                            &format!(" {primary_key_field_ident} = $1"),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            #query_snake_case.push_str(&format!(#query_part_token_stream));
                        }
                    };
                    let handle_token_stream = proc_macro_common::generate_quotes::token_stream(
                        &format!("{delete_snake_case} {from_snake_case} {table_name_stringified} {where_snake_case}"),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        {
                            let mut #query_snake_case = format!(#handle_token_stream);
                            #additional_parameters_primary_key_modification_token_stream
                            #query_snake_case.push_str(&format!(#returning_primary_key_quotes_token_stream));
                            #query_snake_case
                        }
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = quote::quote! {
                        #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case.#payload_snake_case.#primary_key_field_ident, 
                            #query_snake_case
                        );
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_modifications_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &{
                        let fetch_one_token_stream = generate_fetch_one_token_stream(
                            &generate_sqlx_row_try_get_primary_key_token_stream(
                                &quote::quote!{#primary_key_inner_type_token_stream(#value_snake_case)},
                                &generate_match_postgres_transaction_rollback_await_token_stream(
                                    &operation,
                                    file!(),
                                    line!(),
                                    column!(),
                                    file!(),
                                    line!(),
                                    column!(),
                                )
                            ),
                            &generate_match_postgres_transaction_rollback_await_token_stream(
                                &operation,
                                file!(),
                                line!(),
                                column!(),
                                file!(),
                                line!(),
                                column!(),
                            )
                        );
                        quote::quote!{
                            #fetch_one_token_stream
                            let #value_snake_case = #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#value_snake_case);
                        }
                    },
                );
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &table_name_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &syn_derive_input,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &operation.generate_error_initialization_eprintln_response_creation_token_stream(
                        &check_body_size_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                    &postgresql_syn_variant_wrapper.generate_initialization_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_with_possible_operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variants,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &primary_key_inner_type_token_stream,
                &proc_macro2::TokenStream::new(),
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream_option {
                    None => quote::quote!{#primary_key_inner_type_token_stream::#from_snake_case(#value_snake_case)},
                    Some(syn_variant_initialization_token_stream) => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            match #primary_key_inner_type_token_stream::#try_from_snake_case(#value_snake_case) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_snake_case) => {
                                    return Err(
                                        #try_operation_error_named_upper_camel_case_token_stream::#syn_variant_initialization_token_stream
                                    );
                                }
                            }
                        }
                    }
                },
                &reqwest_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &deserialize_response_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &failed_to_get_response_text_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                &serde_json_to_string_syn_variant_wrapper.generate_initialization_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
            );
            // let try_operation_test_token_stream = {
            //     let test_content_token_stream = quote::quote! {
            //         match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
            //                     #primary_key_field_ident: #primary_key_token_stream.clone()
            //                 }
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => println!("{value:#?}"),
            //             Err(#error_snake_case) => panic!("{}", #error_snake_case)
            //         }
            //     };
            //     naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #try_operation_token_stream
                },
                quote::quote! {}
                // try_operation_test_token_stream,
            )
        };
        // println!("{try_operation_token_stream}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_route_logic_token_stream
                #try_operation_token_stream
            },
            quote::quote! {}
            // try_operation_test_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &delete_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    // let emulate_crud_api_usage_test_token_stream = {
    //     let ident_emulate_crud_api_usage_test_snake_case_token_stream = {
    //         let ident_emulate_crud_api_usage_test_snake_case_stringified =
    //             format!("{ident_snake_case_stringified}_emulate_crud_api_usage_test");
    //         ident_emulate_crud_api_usage_test_snake_case_stringified.parse::<proc_macro2::TokenStream>()
    //         .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_emulate_crud_api_usage_test_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    //     };
    //     quote::quote! {
    //         #[test]
    //         fn #ident_emulate_crud_api_usage_test_snake_case_token_stream() {
    //             async fn find_out_if_it_works() {
    //                 let api_location = #std_string_string_token_stream::#from_snake_case("http://127.0.0.1:8080");//todo port from env or config maybe?
    //                 let limit = 1000;
    //                 let offset = 0;
    //                 #create_many_test_token_stream
    //                 #read_many_test_token_stream
    //                 #update_many_test_token_stream
    //                 #read_many_test_token_stream
    //                 #delete_many_test_token_stream
    //                 #read_many_test_token_stream
    //                 #create_one_test_token_stream
    //                 #read_one_test_token_stream
    //                 #update_one_test_token_stream
    //                 #read_one_test_token_stream
    //                 #delete_one_test_token_stream
    //                 #read_one_test_token_stream
    //             }
    //             match tokio::runtime::Builder::new_multi_thread()
    //                 .worker_threads(num_cpus::get())
    //                 .enable_all()
    //                 .build()
    //             {
    //                 Err(#error_snake_case) => {
    //                     panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {:#?}", #error_snake_case)
    //                 }
    //                 Ok(runtime) => {
    //                     runtime.block_on(find_out_if_it_works());
    //                 }
    //             }
    //         }
    //     }
    // };
    // println!("{emulate_crud_api_usage_test_token_stream}");
    let common_token_stream = quote::quote! {
        pub const TABLE_NAME: #ref_std_primitive_str = #table_name_quotes_token_stream;
        #struct_options_token_stream
        #from_ident_for_ident_options_token_stream
        // // #(#structs_variants_token_stream)*
        // // #(#impl_std_convert_try_from_ident_options_for_struct_variants_token_stream)*
        #column_token_stream
        #generate_query_vec_column_token_stream
        #allow_methods_token_stream
        #ident_column_read_permission_token_stream
        #(#reexport_postgresql_sqlx_column_types_token_stream)*
        #field_and_field_with_serialize_deserialize_token_stream
        // #[cfg(test)]
        // mod test_try_create_many {
            // #emulate_crud_api_usage_test_token_stream
        // }
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &common_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    //comment out coz its impossible to correctly generate tokens for debug purposes
    // let _mod_name_snake_case_token_stream = {
    //     let value = format!("{proc_macro_name_snake_case}_{table_name_stringified}");
    //     value.parse::<proc_macro2::TokenStream>()
    //     .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    // };
    //todo pub and private impl quote group
    let generated = quote::quote! {
        //comment out coz its impossible to correctly generate tokens
        // pub mod #mod_name_snake_case_token_stream {/
            #common_token_stream

            #create_many_token_stream
            #create_one_token_stream
            #read_many_token_stream
            #read_one_token_stream
            #update_many_token_stream
            #update_one_token_stream
            #delete_many_token_stream
            #delete_one_token_stream
        // }
    };
    // if ident == "" {
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &generated,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    // }
    generated.into()
}

// fn generate_swagger_open_api_token_stream(
//     table_name_stringified: &str,
//     unique_status_codes: &[proc_macro_helpers::status_code::StatusCode],
//     application_json_quotes_token_stream: &proc_macro2::TokenStream,
//     table_name_quotes_token_stream: &proc_macro2::TokenStream,
//     content_type_token_stream: &proc_macro2::TokenStream,
//     operation: &Operation,
// ) -> proc_macro2::TokenStream {
//     let swagger_url_path_quotes_token_stream = naming_conventions::SwaggerUrlPathSelfQuotesTokenStream::swagger_url_path_self_quotes_token_stream(operation, table_name_stringified);
//     let content_type_snake_case_token_stream = quote::quote! {content_type};
//     let description_snake_case_token_stream = quote::quote! {description};
//     let responses_token_stream = unique_status_codes.iter().map(|element|{
//         let status_token_stream = element.to_status_code_token_stream();
//         let description_token_stream = element.to_status_code_description_token_stream();
//         let body_token_stream = operation.try_self_response_variants_status_code_token_stream(element);
//         quote::quote!{
//             (
//                 status = #status_token_stream,
//                 #description_snake_case_token_stream = #description_token_stream,
//                 body = #body_token_stream,
//                 #content_type_snake_case_token_stream = #application_json_quotes_token_stream
//             )
//         }
//     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
//     let method_snake_case_token_stream =
//         proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(
//             &operation.http_method(),
//         );
//     let request_body_token_stream = {
//         let request_body_description_token_stream = {
//             let value = proc_macro_common::generate_quotes::stringified(&format!(
//                 "{table_name_stringified} {} {}",
//                 proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(operation),
//                 naming_constants::PayloadSnakeCase
//             ));
//             value.parse::<proc_macro2::TokenStream>()
//             .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//         };
//         quote::quote! {
//             request_body(
//                 content = #content_type_token_stream,
//                 #description_snake_case_token_stream = #request_body_description_token_stream,
//                 #content_type_snake_case_token_stream = #application_json_quotes_token_stream
//             )
//         }
//     };
//     let path_snake_case_token_stream =
//         naming_constants::PathSnakeCase;
//     quote::quote! {
//         #[utoipa::path(
//             #method_snake_case_token_stream,
//             #path_snake_case_token_stream = #swagger_url_path_quotes_token_stream,
//             operation_id = #swagger_url_path_quotes_token_stream,
//             tag = #table_name_quotes_token_stream,
//             #request_body_token_stream,
//             responses(
//                 #(#responses_token_stream),*
//             ),
//         )]
//     }
// }

// fn generate_async_test_wrapper_token_stream(
//     operation_name_snake_case_stringified: &str,
//     test_inner_content_token_stream: &proc_macro2::TokenStream,
//     proc_macro_name_upper_camel_case_ident_stringified: &str
// ) -> proc_macro2::TokenStream {
//     fn generate_operation_test_snake_case_token_stream(
//         operation_name_snake_case_stringified: &str,
//         proc_macro_name_upper_camel_case_ident_stringified: &str,
//     ) -> proc_macro2::TokenStream {
//         let value = format!("{operation_name_snake_case_stringified}_test");
//         value.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
//     let operation_test_snake_case_token_stream = generate_operation_test_snake_case_token_stream(
//         &operation_name_snake_case_stringified,
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     quote::quote!{
//         #[test]
//         fn #operation_test_snake_case_token_stream() {
//             match tokio::runtime::Builder::new_multi_thread()
//                 .worker_threads(num_cpus::get())
//                 .enable_all()
//                 .build()
//             {
//                 Err(#error_snake_case) => {
//                     panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {:#?}", #error_snake_case)
//                 }
//                 Ok(runtime) => {
//                     async fn test() {
//                         #test_inner_content_token_stream
//                     }
//                     runtime.block_on(test());
//                 }
//             }
//         }
//     }
// }

//todo test asc desc
// enum Order {
//     Asc,
//     Desc,
// }
// impl Order {
//     fn to_token_stream(&self) -> proc_macro2::TokenStream {
//         match self {
//             Self::Asc => quote::quote! {Asc},
//             Self::Desc => quote::quote! {Desc},
//         }
//     }
// }