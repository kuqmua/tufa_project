mod check_for_none;
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
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", ast.data);
    let ident = &ast.ident;
    let ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
        &ident.to_string()
    );
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let table_name_stringified = pluralizer::pluralize(&ident_snake_case_stringified, 2, false);
    let table_name_quotes_token_stream =
        proc_macro_common::generate_quotes::token_stream(
            &table_name_stringified,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
    let ref_std_primitive_str = token_patterns::RefStdPrimitiveStr;
    let table_name_declaration_token_stream = quote::quote! {pub const TABLE_NAME: #ref_std_primitive_str = #table_name_quotes_token_stream;};
    let fields_named = if let syn::Data::Struct(data_struct) = &ast.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            fields_named.named
                .iter()
                .map(|element| SynFieldWithAdditionalInfo::try_from(element).unwrap_or_else(|error|panic!("SynFieldWithAdditionalInfo::try_from(element) failed {error}")))
                .collect::<std::vec::Vec<SynFieldWithAdditionalInfo<'_>>>()
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Named");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let primary_key_syn_field = {
        let primary_key_attr_name = "generate_postgresql_crud_primary_key";
        let mut primary_key_field_option = None;
        for element in &fields_named {
            match &element.field.ty {
                syn::Type::Path(value) => {
                    if value.path.segments.len() == 2 {
                        assert!(value.path.segments.first().expect("no first value in punctuated").ident == postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, "{proc_macro_name_upper_camel_case_ident_stringified} field_type is not syn::Type::Path");
                        match value.path.segments.first().expect("no first value in punctuated").arguments {
                            syn::PathArguments::None => (),
                            syn::PathArguments::AngleBracketed(_) | 
                            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} value.path().segments[0].arguments != syn::PathArguments::None")
                        }
                        let _maybe_generic_token_stream = match &value.path.segments.iter().nth(1).expect("no second element").arguments {
                            syn::PathArguments::None => quote::quote!{},
                            syn::PathArguments::AngleBracketed(value) => {
                                quote::quote!{#value}//< test_mod :: Something >
                            },
                            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} does not support syn::PathArguments::Parenthesized"),
                        };
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
                syn::Type::Verbatim(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} field_type is not syn::Type::Path"),
                _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} field_type is not syn::Type::Path (exhaustive)")
            }
        }
        primary_key_field_option.map_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} no {primary_key_attr_name} attribute"), |value| value)
    };
    let primary_key_field = &primary_key_syn_field.field;
    let primary_key_field_ident = &primary_key_syn_field.field_ident;
    let primary_key_rust_sqlx_map_to_postgres_type_variant = &primary_key_syn_field.rust_sqlx_map_to_postgres_type_variant; 
    // let primary_key_maybe_generic_token_stream = &primary_key_syn_field.maybe_generic_token_stream; 
    // let primary_key_path_token_stream = &primary_key_syn_field.path_token_stream;
    let primary_key_original_type_token_stream = &primary_key_syn_field.original_type_token_stream;
    let primary_key_inner_type_token_stream = &primary_key_syn_field.inner_type_token_stream;
    let primary_key_inner_type_with_serialize_deserialize_token_stream = &primary_key_syn_field.inner_type_with_serialize_deserialize_token_stream;
    let primary_key_inner_type_with_serialize_deserialize_error_named_token_stream = &primary_key_syn_field.inner_type_with_serialize_deserialize_error_named_token_stream;
    // let primary_key_where_inner_type_with_serialize_deserialize_handle_stringified = &primary_key_syn_field.where_inner_type_with_serialize_deserialize_handle_stringified;
    // let primary_key_where_inner_type_token_stream = &primary_key_syn_field.where_inner_type_token_stream;
    // let primary_key_where_inner_type_with_serialize_deserialize_token_stream = &primary_key_syn_field.where_inner_type_with_serialize_deserialize_token_stream;
    let primary_key_supported_sqlx_postgres_type_snake_case_token_stream = {
        let value = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &postgresql_crud_common::SupportedSqlxPostgresType::from(primary_key_rust_sqlx_map_to_postgres_type_variant)
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let primary_key_field_ident_upper_camel_case_token_stream = {
        //todo its a temporal naming desicion. maybe it can be better
        let value = syn_ident_to_upper_camel_case_stringified(primary_key_field_ident);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let primary_key_field_ident_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
        &primary_key_field_ident.to_string(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let std_string_string = token_patterns::StdStringString;
    let fields_named_excluding_primary_key = fields_named
        .clone()
        .into_iter()
        .filter(|element| element.field != *primary_key_field)
        .collect::<std::vec::Vec<SynFieldWithAdditionalInfo<'_>>>();
    let fields_named_len = fields_named.len();
    assert!(fields_named_len > 1, "{proc_macro_name_upper_camel_case_ident_stringified} false = fields_named.len() > 1");
    let field_named_len_token_stream = {
        let value = fields_named_len.to_string();
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let fields_named_excluding_primary_key_len = fields_named_excluding_primary_key.len();
    let is_fields_named_excluding_primary_key_len_equals_one = fields_named_excluding_primary_key_len == 1;
    let fields_named_from_or_try_from = {
        let mut value = postgresql_crud_common::FromOrTryFrom::From;
        for element in &fields_named {
            let from_or_try_from = element.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize();
            if from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
                value = from_or_try_from;
                break;
            }
        }
        value
    };
    let primary_key_from_or_try_from = primary_key_syn_field.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize();
    let fields_named_excluding_primary_key_from_or_try_from = {
        let mut value = postgresql_crud_common::FromOrTryFrom::From;
        for element in &fields_named_excluding_primary_key {
            let from_or_try_from = element.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize();
            if from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
                value = from_or_try_from;
                break;
            }
        }
        value
    };
    let debug_upper_camel_case = naming_constants::DebugUpperCamelCase;
    let thiserror_error = token_patterns::ThiserrorError;
    let error_occurence_error_occurence = token_patterns::ErrorOccurenceLibErrorOccurence;
    let error_snake_case = naming_constants::ErrorSnakeCase;
    let app_state_snake_case = naming_conventions::AppStateSnakeCase;
    let eprintln_error_token_stream = {
        let error_snake_case_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
            &format!("{{{error_snake_case}}}"),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{eprintln!(#error_snake_case_quotes_token_stream);}
    };
    let utoipa_to_schema = token_patterns::UtoipaToSchema;
    let serde_serialize = token_patterns::SerdeSerialize;
    let serde_deserialize = token_patterns::SerdeDeserialize;
    let derive_debug = token_patterns::DeriveDebug;
    let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
    let derive_debug_utoipa_to_schema = token_patterns::DeriveDebugUtoipaToSchema;
    let derive_debug_serde_serialize_serde_deserialize = token_patterns::DeriveDebugSerdeSerializeSerdeDeserialize;
    let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
    let from_str_upper_camel_case = naming_conventions::FromStrUpperCamelCase;
    let from_str_snake_case = naming_conventions::FromStrSnakeCase;
    let sqlx_row = token_patterns::SqlxRow;
    let std_primitive_str_sqlx_column_index = token_patterns::StdPrimitiveStrSqlxColumnIndex;
    let sqlx_decode_decode_database = token_patterns::SqlxDecodeDecodeDatabase;
    let sqlx_types_type_database = token_patterns::SqlxTypesTypeDatabase;
    let error_named_upper_camel_case = naming_conventions::ErrorNamedUpperCamelCase;
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
            let inner_type_with_serialize_deserialize_token_stream = &primary_key_syn_field.inner_type_with_serialize_deserialize_token_stream;
            quote::quote! {
                // #serde_skip_serializing_if_value_attribute_token_stream
                pub #primary_key_field_ident: std::option::Option<postgresql_crud::Value<#inner_type_with_serialize_deserialize_token_stream>>
            }
        };
        let fields_options_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element| {
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
        let inner_type_with_serialize_deserialize_token_stream =
            &primary_key_syn_field
                .inner_type_with_serialize_deserialize_token_stream;
        let ident_option_variant_primary_key_token_stream = quote::quote! {
            #primary_key_field_ident: Some(
                postgresql_crud::Value {
                    value: #inner_type_with_serialize_deserialize_token_stream::#from_snake_case(
                        value.#primary_key_field_ident.0
                    )
                }
            ),
        };
        let ident_option_variants_excluding_primary_key_token_stream =
            fields_named_excluding_primary_key
                .iter()
                .map(|element| {
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
    let code_occurence_upper_camel_case = naming_conventions::CodeOccurenceUpperCamelCase;
    let code_occurence_snake_case = naming_conventions::CodeOccurenceSnakeCase;
    let error_occurence_lib_code_occurence_code_occurence = token_patterns::ErrorOccurenceLibCodeOccurenceCodeOccurence;
    let code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence = token_patterns::CodeOccurenceSnakeCaseDoubleDotSpaceErrorOccurenceLibCodeOccurenceCodeOccurence;
    let eo_error_occurence_attribute_token_stream = proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence.to_attribute_view_token_stream();
    let eo_to_std_string_string_serialize_deserialize_token_stream = proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize.to_attribute_view_token_stream();
    let eo_to_std_string_string_token_stream = proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString.to_attribute_view_token_stream();
    let eo_vec_error_occurence_token_stream = proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecErrorOccurence.to_attribute_view_token_stream();
    let ident_column_upper_camel_case_stringified = format!("{ident}{}", naming_constants::ColumnUpperCamelCase);
    let ident_column_upper_camel_case_token_stream = ident_column_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_column_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let column_token_stream = {
        let variants = fields_named
            .iter()
            .map(|element| {
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
        let display_variants = fields_named
            .iter()
            .map(|element| {
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
                fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    let generate_query_vec_column_snake_case_token_stream = quote::quote!{generate_query_vec_column};
    let generate_query_vec_column_token_stream = {
        let variants_token_stream = fields_named.iter().map(|element|{
            let field_ident_upper_camel_case_token_stream = {
                //todo its a temporal naming desicion. maybe it can be better
                let value = syn_ident_to_upper_camel_case_stringified(element.field_ident);
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_ident_string_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
                &element.field_ident.to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {#ident_column_upper_camel_case_token_stream::#field_ident_upper_camel_case_token_stream => #field_ident_string_quotes_token_stream}
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            fn #generate_query_vec_column_snake_case_token_stream(value: &[#ident_column_upper_camel_case_token_stream]) -> #std_string_string {
                let mut value = value.iter().fold(#std_string_string::from(""), |mut acc, element| {
                    acc += match element {
                        #(#variants_token_stream),*
                    };
                    acc += ",";
                    acc
                });
                value.pop();
                value
            }
        }
    };
    // println!("{generate_query_vec_column_token_stream}");
    let column_select_upper_camel_case = naming_conventions::ColumnSelectUpperCamelCase;
    let ident_column_select_upper_camel_case_token_stream = {
        let ident_column_select_upper_camel_case_stringified =
            format!("{ident}{column_select_upper_camel_case}");
        ident_column_select_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_column_select_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let options_try_from_sqlx_row_snake_case = naming_conventions::OptionsTryFromSqlxRowSnakeCase;
    let wrapper_vec_column_upper_camel_case = naming_conventions::WrapperVecColumnUpperCamelCase;
    let wrapper_vec_column_snake_case = naming_conventions::WrapperVecColumnSnakeCase;
    let options_try_from_sqlx_row_token_stream = {
        let declaration_primary_key_token_stream = {
            let inner_type_with_serialize_deserialize_token_stream =
                &primary_key_syn_field
                    .inner_type_with_serialize_deserialize_token_stream;
            quote::quote! {
                let mut #primary_key_field_ident: std::option::Option<
                    postgresql_crud::Value<#inner_type_with_serialize_deserialize_token_stream>
                > = None;
            }
        };
        let declaration_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            let field_ident = &element.field_ident;
            let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
            quote::quote! {
                let mut #field_ident: std::option::Option<
                    postgresql_crud::Value<#inner_type_with_serialize_deserialize_token_stream>,
                > = None;
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
            quote::quote! {
                #ident_column_upper_camel_case_token_stream::#primary_key_field_ident_upper_camel_case_token_stream => {
                    #primary_key_field_ident = {
                        let value: std::option::Option<#primary_key_original_type_token_stream> = row.try_get(#primary_key_field_ident_string_quotes_token_stream)?;
                        value.map(|value|postgresql_crud::Value{ value: #primary_key_inner_type_with_serialize_deserialize_token_stream::from(#primary_key_inner_type_token_stream(value))})
                    };
                }
            }
        };
        let assignment_variants_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
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
            quote::quote! {
                #ident_column_upper_camel_case_token_stream::#field_ident_upper_camel_case_token_stream => {
                    #field_ident = {
                        let value: std::option::Option<#original_type_token_stream> = row.try_get(#field_ident_string_quotes_token_stream)?;
                        value.map(|value|postgresql_crud::Value{ value: #inner_type_with_serialize_deserialize_token_stream::from(#inner_type_token_stream(value))})
                    };
                }
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        let option_fields_initiation_token_stream = generate_self_fields_token_stream(
            &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let sqlx_decode_decode_and_sqlx_types_type_primary_key_token_stream = {
            quote::quote! {
                std::option::Option<#primary_key_original_type_token_stream>: #sqlx_decode_decode_database,
                std::option::Option<#primary_key_original_type_token_stream>: #sqlx_types_type_database,
            }
        };
        let sqlx_decode_decode_and_sqlx_types_type_with_excluded_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element| {
            let original_type_token_stream = &element.original_type_token_stream;
            quote::quote!{
                std::option::Option<#original_type_token_stream>: #sqlx_decode_decode_database,
                std::option::Option<#original_type_token_stream>: #sqlx_types_type_database,
            }
        });
        quote::quote! {
            #[derive(Debug)]
            struct #wrapper_vec_column_upper_camel_case(std::vec::Vec<#ident_column_upper_camel_case_token_stream>);
            impl #wrapper_vec_column_upper_camel_case {
                fn #options_try_from_sqlx_row_snake_case<'a, R: #sqlx_row>(
                    &self, 
                    row: &'a R
                ) -> sqlx::Result<#struct_options_ident_token_stream>
                where
                    #std_primitive_str_sqlx_column_index
                    #sqlx_decode_decode_and_sqlx_types_type_primary_key_token_stream
                    #(#sqlx_decode_decode_and_sqlx_types_type_with_excluded_primary_key_token_stream)*
                {
                    #declaration_primary_key_token_stream
                    #(#declaration_excluding_primary_key_token_stream)*
                    //assuming all enum variants are unique
                    for element in &self.0 {
                        match element {
                            #assignment_variant_primary_key_token_stream,
                            #(#assignment_variants_excluding_primary_key_token_stream),*
                        }
                    }
                    //todo maybe it must be not DogOptions but DogOptionWithPrimaryKeyNotNone
                    Ok(#struct_options_ident_token_stream {
                        #(#option_fields_initiation_token_stream),*
                    })
                }
            }
        }
    };
    // println!("{options_try_from_sqlx_row_token_stream}");
    //todo reuse path
    let primary_key_try_from_sqlx_row_snake_case = naming_conventions::PrimaryKeyTryFromSqlxRowSnakeCase;
    let primary_key_try_get_sqlx_row_token_stream = {
        let primary_key_str_token_stream = proc_macro_common::generate_quotes::token_stream(
            &primary_key_field_ident.to_string(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let row_snake_case = naming_constants::RowSnakeCase;
        let primary_key_name = naming_conventions::PrimaryKeySnakeCase;
        quote::quote! {
            fn #primary_key_try_from_sqlx_row_snake_case<'a, R: #sqlx_row>(#row_snake_case: &'a R) -> sqlx::Result<#primary_key_inner_type_token_stream>
            where
                #std_primitive_str_sqlx_column_index
                #primary_key_original_type_token_stream: #sqlx_decode_decode_database,
                #primary_key_original_type_token_stream: #sqlx_types_type_database,
            {
                let #primary_key_name: #primary_key_original_type_token_stream = #row_snake_case.try_get(#primary_key_str_token_stream)?;
                Ok(#primary_key_inner_type_token_stream(#primary_key_name))
            }
        }
    };
    // println!("{primary_key_try_get_sqlx_row_token_stream}");
    let order_by_upper_camel_case = naming_conventions::OrderByUpperCamelCase;
    let postgresql_crud_order_by_token_stream = quote::quote! {postgresql_crud::#order_by_upper_camel_case};
    let postgresql_crud_order_token_stream = quote::quote! {postgresql_crud::Order};
    // let limit_snake_case = naming_constants::LimitSnakeCase;
    // let offset_snake_case = naming_constants::OffsetSnakeCase;
    // let order_snake_case = naming_constants::OrderSnakeCase;
    // let column_snake_case = naming_constants::ColumnSnakeCase;
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
        let fields_permission_token_stream = fields_named.iter().map(|element| {
            let field_ident = &element.field_ident;
            quote::quote!{
                #field_ident: std::primitive::bool
            }
        });
        quote::quote! {
            pub struct #ident_column_read_permission_name_token_stream {
                #(#fields_permission_token_stream),*
            }
        }
    };
    let reexport_postgresql_sqlx_column_types_token_stream = fields_named.iter().map(|element|{
        let inner_type_token_stream = &element.inner_type_token_stream;
        quote::quote! {pub use #inner_type_token_stream;}
    });
    let reference_api_location_test_token_stream = quote::quote! {&api_location};
    let extraction_result_snake_case = naming_conventions::ExtractionResultSnakeCase;
    let parameters_snake_case = naming_constants::ParametersSnakeCase;
    let payload_upper_camel_case = naming_constants::PayloadUpperCamelCase;
    let payload_snake_case = naming_constants::PayloadSnakeCase;
    let payload_snake_case = naming_constants::PayloadSnakeCase;
    let query_upper_camel_case = naming_constants::QueryUpperCamelCase;
    let query_snake_case = naming_constants::QuerySnakeCase;
    let rollback_upper_camel_case = naming_constants::RollbackUpperCamelCase;
    let rollback_snake_case = naming_constants::RollbackSnakeCase;
    let rollback_snake_case = naming_constants::RollbackSnakeCase;
    let pg_connection_snake_case = naming_conventions::PgConnectionSnakeCase;
    let query_string_snake_case = naming_conventions::QueryStringSnakeCase;
    let binded_query_snake_case = naming_conventions::BindedQuerySnakeCase;
    let current_vec_len_snake_case = naming_conventions::CurrentVecLenSnakeCase;
    let desirable_upper_camel_case = naming_constants::DesirableUpperCamelCase;
    let select_snake_case = naming_constants::SelectSnakeCase;
    let limit_snake_case = naming_constants::LimitSnakeCase;
    let offset_snake_case = naming_constants::OffsetSnakeCase;
    let rollback_snake_case = naming_constants::RollbackSnakeCase;
    let element_snake_case = naming_constants::ElementSnakeCase;
    let acc_snake_case = naming_constants::AccSnakeCase;
    let query_snake_case = naming_constants::QuerySnakeCase;
    let update_snake_case = naming_constants::UpdateSnakeCase;
    let as_snake_case = naming_constants::AsSnakeCase;
    let set_snake_case = naming_constants::SetSnakeCase;
    let from_snake_case = naming_constants::FromSnakeCase;
    let insert_snake_case = naming_constants::InsertSnakeCase;
    let into_snake_case = naming_constants::IntoSnakeCase;
    let values_snake_case = naming_constants::ValuesSnakeCase;
    let delete_snake_case = naming_constants::DeleteSnakeCase;
    let where_snake_case = naming_constants::WhereSnakeCase;
    let payload_extraction_result_snake_case = naming_conventions::ExtractionResultSnakeCase;
    let use_futures_try_stream_ext_token_stream = quote::quote! {use futures::TryStreamExt};
    let serde_json_to_string_token_stream = quote::quote! {serde_json::to_string};
    // let payload_element_upper_camel_case_stringified = format!("{payload_upper_camel_case_stringified}Element");
    let returning_snake_case = naming_constants::ReturningSnakeCase;
    let returning_primary_key_stringified = format!(" {returning_snake_case} {primary_key_field_ident}");
    let returning_primary_key_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
        &returning_primary_key_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let postgresql_crud_snake_case = naming_conventions::PostgresqlCrudSnakeCase;
    let app_state_path = quote::quote! {#postgresql_crud_snake_case::app_state::DynArcGetConfigGetPostgresPoolSendSync}; //todo path
    let serde_json_error_token_stream = quote::quote! {serde_json::Error};
    let std_string_string_syn_punctuated_punctuated = proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
        &["std","string","String"],
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let sqlx_error_syn_punctuated_punctuated = proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
        &["sqlx","Error"],
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let primary_key_std_vec_vec_inner_type_syn_punctuated_punctuated = {
        let panic_message = format!("primary key functionality is not implemented for {primary_key_rust_sqlx_map_to_postgres_type_variant} in {proc_macro_name_upper_camel_case_ident_stringified} logic");
        match primary_key_rust_sqlx_map_to_postgres_type_variant {
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

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => generate_std_vec_vec_syn_punctuated_punctuated(
                &[postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, "StdPrimitiveI64"],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => generate_std_vec_vec_syn_punctuated_punctuated(
                &[postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, "SqlxTypesUuidUuid"],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
        }
    };
    let into_serialize_deserialize_version_snake_case = naming_conventions::IntoSerializeDeserializeVersionSnakeCase;
    let (
        checked_add_syn_variant, 
        checked_add_syn_variant_initialization_token_stream,
        checked_add_syn_variant_status_code
    ) = {
        let checked_add_upper_camel_case= naming_conventions::CheckedAddUpperCamelCase;
        let checked_add_snake_case = naming_conventions::CheckedAddSnakeCase;
        let checked_add_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::BadRequest400;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                checked_add_syn_variant_status_code.clone(),
                &checked_add_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
                        &checked_add_snake_case,
                        std_string_string_syn_punctuated_punctuated.clone()
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let checked_is_none_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
                    &format!(
                        "{checked_add_snake_case} {} {}",
                        naming_constants::IsSnakeCase,
                        naming_constants::NoneUpperCamelCase
                    ),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let field_code_occurence_new_9afdf71d_e375_455f_87a3_a16947625a7a_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #checked_add_upper_camel_case { //todo remove it? refactor it?
                        #checked_add_snake_case: #std_string_string::#from_snake_case(#checked_is_none_quotes_token_stream),
                        #field_code_occurence_new_9afdf71d_e375_455f_87a3_a16947625a7a_token_stream,
                    }
                }
            },
            checked_add_syn_variant_status_code
        )
    };
    let (
        query_and_rollback_failed_syn_variant,
        query_and_rollback_failed_syn_variant_initialization_token_stream,
        query_and_rollback_failed_syn_variant_status_code
     ) = {
        let query_and_rollback_failed_upper_camel_case = naming_conventions::QueryAndRollbackFailedUpperCamelCase;
        let query_and_rollback_failed_snake_case = naming_conventions::QueryAndRollbackFailedSnakeCase;
        let query_and_rollback_failed_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::InternalServerError500;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                query_and_rollback_failed_syn_variant_status_code.clone(),
                &query_and_rollback_failed_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &query_snake_case.to_string(),
                        sqlx_error_syn_punctuated_punctuated.clone(),
                    ),
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &rollback_snake_case.to_string(),
                        sqlx_error_syn_punctuated_punctuated.clone(),
                    ),
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_254f2939_bca7_4b8a_b737_cd9bbbbdd5df_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let rollback_error_snake_case = naming_conventions::RollbackErrorSnakeCase;
                quote::quote! {
                    #query_and_rollback_failed_upper_camel_case {
                        #query_snake_case: #error_snake_case,
                        #rollback_snake_case: #rollback_error_snake_case,
                        #field_code_occurence_new_254f2939_bca7_4b8a_b737_cd9bbbbdd5df_token_stream,
                    }
                }
            },
            query_and_rollback_failed_syn_variant_status_code
        )
    };
    let (
        primary_key_from_row_and_failed_rollback_syn_variant,
        primary_key_from_row_and_failed_rollback_syn_variant_initialization_token_stream,
        primary_key_from_row_and_failed_rollback_syn_variant_status_code
     ) = {
        let primary_key_from_row_and_failed_rollback_upper_camel_case = naming_conventions::PrimaryKeyFromRowAndFailedRollbackUpperCamelCase;
        let primary_key_from_row_and_failed_rollback_snake_case = naming_conventions::PrimaryKeyFromRowAndFailedRollbackSnakeCase;
        let primary_key_from_row_snake_case = naming_conventions::PrimaryKeyFromRowSnakeCase;
        let primary_key_from_row_and_failed_rollback_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::InternalServerError500;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                primary_key_from_row_and_failed_rollback_syn_variant_status_code.clone(),
                &primary_key_from_row_and_failed_rollback_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &primary_key_from_row_snake_case.to_string(),
                        sqlx_error_syn_punctuated_punctuated.clone(),
                    ),
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &rollback_snake_case.to_string(),
                        sqlx_error_syn_punctuated_punctuated.clone(),
                    ),
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let rollback_error_snake_case = naming_conventions::RollbackErrorSnakeCase;
                let field_code_occurence_new_494adabc_50aa_4d57_acc8_4a0444df7d28_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #primary_key_from_row_and_failed_rollback_upper_camel_case {
                        #primary_key_from_row_snake_case: #error_snake_case,
                        #rollback_snake_case: #rollback_error_snake_case,
                        #field_code_occurence_new_494adabc_50aa_4d57_acc8_4a0444df7d28_token_stream,
                    }
                }
            },
            primary_key_from_row_and_failed_rollback_syn_variant_status_code
        )
    };
    let (
        non_existing_primary_keys_syn_variant, 
        non_existing_primary_keys_syn_variant_initialization_token_stream,
        non_existing_primary_keys_syn_variant_status_code
    ) = {
        let non_existing_primary_keys_upper_camel_case = naming_conventions::NonExistingPrimaryKeysUpperCamelCase;
        let non_existing_primary_keys_snake_case = naming_conventions::NonExistingPrimaryKeysSnakeCase;
        let non_existing_primary_keys_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::BadRequest400;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                non_existing_primary_keys_syn_variant_status_code.clone(),
                &non_existing_primary_keys_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,//todo display with serialize deserialize
                        &non_existing_primary_keys_snake_case,
                        primary_key_std_vec_vec_inner_type_syn_punctuated_punctuated.clone()
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_4853d33a_b7e0_45df_8024_98ba66d26973_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #non_existing_primary_keys_upper_camel_case {
                        #non_existing_primary_keys_snake_case,
                        #field_code_occurence_new_4853d33a_b7e0_45df_8024_98ba66d26973_token_stream,
                    }
                }
            },
            non_existing_primary_keys_syn_variant_status_code
        )
    };
    let (
        non_existing_primary_keys_and_failed_rollback_syn_variant,
        non_existing_primary_keys_and_failed_rollback_syn_variant_initialization_token_stream,
        non_existing_primary_keys_and_failed_rollback_syn_variant_status_code,
     ) = {
        let non_existing_primary_keys_and_failed_rollback_upper_camel_case = naming_conventions::NonExistingPrimaryKeysAndFailedRollbackUpperCamelCase;
        let non_existing_primary_keys_snake_case = naming_conventions::NonExistingPrimaryKeysSnakeCase;
        let non_existing_primary_keys_and_failed_rollback_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::BadRequest400;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                non_existing_primary_keys_and_failed_rollback_syn_variant_status_code.clone(),
                &non_existing_primary_keys_and_failed_rollback_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,//todo display with serialize deserialize
                        &non_existing_primary_keys_snake_case.to_string(), 
                        primary_key_std_vec_vec_inner_type_syn_punctuated_punctuated.clone()
                    ),
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &rollback_snake_case.to_string(),
                        sqlx_error_syn_punctuated_punctuated.clone()
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_5e07939c_0aa6_4f48_9f1f_5d3866c651ab_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #non_existing_primary_keys_and_failed_rollback_upper_camel_case {
                        #non_existing_primary_keys_snake_case,
                        #rollback_snake_case: #error_snake_case,
                        #field_code_occurence_new_5e07939c_0aa6_4f48_9f1f_5d3866c651ab_token_stream,
                    }
                }
            },
            non_existing_primary_keys_and_failed_rollback_syn_variant_status_code
        )
    };
    let (
        commit_failed_syn_variant,
        commit_failed_syn_variant_initialization_token_stream,
        commit_failed_syn_variant_status_code,
     ) = {
        let commit_failed_upper_camel_case = naming_conventions::CommitFailedUpperCamelCase;
        let commit_failed_snake_case = naming_conventions::CommitFailedSnakeCase;
        let commit_failed_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::InternalServerError500;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                commit_failed_syn_variant_status_code.clone(),
                &commit_failed_upper_camel_case,
                vec![(
                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                    &commit_failed_snake_case,
                    sqlx_error_syn_punctuated_punctuated.clone(),
                )],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_52fad21a_c2cd_40f2_85af_dfec05be9d22_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #commit_failed_upper_camel_case {
                        #commit_failed_snake_case: #error_snake_case,
                        #field_code_occurence_new_52fad21a_c2cd_40f2_85af_dfec05be9d22_token_stream,
                    }
                }
            },
            commit_failed_syn_variant_status_code
        )
    };
    let (
        not_unique_primary_keys_syn_variant,
        not_unique_primary_keys_variant_initialization_token_stream,
        not_unique_primary_keys_status_code
     ) = {
        let not_unique_primary_keys_upper_camel_case = naming_conventions::NotUniquePrimaryKeysUpperCamelCase;
        let not_unique_primary_keys_snake_case = naming_conventions::NotUniquePrimaryKeysSnakeCase;
        let not_unique_primary_keys_status_code = proc_macro_helpers::status_code::StatusCode::InternalServerError500;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                not_unique_primary_keys_status_code.clone(),
                &not_unique_primary_keys_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
                        &not_unique_primary_keys_snake_case,
                        primary_key_std_vec_vec_inner_type_syn_punctuated_punctuated
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_0a70da64_9e15_4760_9656_14961b286f36_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #not_unique_primary_keys_upper_camel_case {
                        #not_unique_primary_keys_snake_case,
                        #field_code_occurence_new_0a70da64_9e15_4760_9656_14961b286f36_token_stream,
                    }
                }
            },
            not_unique_primary_keys_status_code
        )
    };
    //todo maybe instead primary key put upper camel case RustSqlxMapToPostgresTypeVariant variant
    let (
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant,
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_initialization_token_stream,
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_status_code,
    ) = {
        let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_upper_camel_case = naming_conventions::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServerUpperCamelCase;
        let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_snake_case = naming_conventions::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServerSnakeCase;
        let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::InternalServerError500;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_status_code.clone(), //todo - is it right status code for this case?
                &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_upper_camel_case,
                vec![(
                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                    &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_snake_case,
                    sqlx_error_syn_punctuated_punctuated.clone(),
                )],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_3567ece5_74c9_4b48_a46c_8230cd728182_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_upper_camel_case {
                        #operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_snake_case: #error_snake_case,
                        #field_code_occurence_new_3567ece5_74c9_4b48_a46c_8230cd728182_token_stream,
                    }
                }
            },
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_status_code
        )
    };
    let (
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many_syn_variant,
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many_initialization_token_stream
     ) = {
        let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many_upper_camel_case = naming_conventions::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClientManyUpperCamelCase;
        let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many_snake_case = naming_conventions::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClientManySnakeCase;
        (
            syn::Variant {
                attrs: vec![],
                ident: syn::Ident::new(&operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many_upper_camel_case.to_string(), proc_macro2::Span::call_site()),
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
                                                proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecErrorOccurence),
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
                            ident: Some(syn::Ident::new(&operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many_snake_case.to_string(), proc_macro2::Span::call_site())),
                            colon_token: Some(syn::token::Colon {
                                spans: [proc_macro2::Span::call_site()],
                            }),
                            ty: syn::Type::Path(syn::TypePath {
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    segments: {
                                        let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                                        handle.push_value(syn::PathSegment {
                                            ident: proc_macro2::Ident::new("std", proc_macro2::Span::call_site()),
                                            arguments: syn::PathArguments::None,
                                        });
                                        handle.push_punct(syn::token::PathSep {
                                            spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                                        });
                                        handle.push_value(syn::PathSegment {
                                            ident: proc_macro2::Ident::new("vec", proc_macro2::Span::call_site()),
                                            arguments: syn::PathArguments::None,
                                        });
                                        handle.push_punct(syn::token::PathSep {
                                            spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                                        });
                                        handle.push_value(syn::PathSegment {
                                            ident: proc_macro2::Ident::new("Vec", proc_macro2::Span::call_site()),
                                            arguments: syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{
                                                colon2_token: None,
                                                lt_token: syn::token::Lt::default(),
                                                args: {
                                                    let mut handle_arguments_type = syn::punctuated::Punctuated::<syn::GenericArgument, syn::token::Comma>::new();
                                                    handle_arguments_type.push_value(syn::GenericArgument::Type(
                                                        syn::Type::Path(syn::TypePath{
                                                            qself: None,
                                                            path: syn::Path {
                                                                leading_colon: None,
                                                                segments: {
                                                                    let mut handle_vec_type = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                                                                    handle_vec_type.push_value(syn::PathSegment {
                                                                        ident: proc_macro2::Ident::new(
                                                                            &naming_conventions::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClientErrorUnnamedUpperCamelCase.to_string(),
                                                                            proc_macro2::Span::call_site()
                                                                        ),
                                                                        arguments: syn::PathArguments::None,
                                                                    });
                                                                    handle_vec_type
                                                                }
                                                            },
                                                        }),
                                                    ));
                                                    handle_arguments_type
                                                },
                                                gt_token: syn::token::Gt::default(),
                                            }),
                                        });
                                        handle
                                    }
                                },
                            }),
                        });
                        punctuated
                    },
                }),
                discriminant: None,
            },
            {
                let field_code_occurence_new_bb9fbcd9_7cea_42e2_b7d8_bc42710bf35e_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many_upper_camel_case {
                        #operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many_snake_case: vec_errors,
                        #field_code_occurence_new_bb9fbcd9_7cea_42e2_b7d8_bc42710bf35e_token_stream
                    }
                }
            }
        )
     };
    //todo maybe first convert to type what can be primary key ?
    let (
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variant,
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream
     ) = {
        let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_snake_case = naming_conventions::OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClientSnakeCase;
        (
            syn::Variant {
                attrs: vec![],
                ident: syn::Ident::new(&operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_snake_case.to_string(), proc_macro2::Span::call_site()),
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
                            ty: primary_key_syn_field.field.ty.clone(),
                        });
                        punctuated
                    },
                }),
                discriminant: None,
            },
            {
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
            }
        )
     };
    let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_declaration_handle_token_stream = match primary_key_from_or_try_from {
        postgresql_crud_common::FromOrTryFrom::From => proc_macro2::TokenStream::new(),
        postgresql_crud_common::FromOrTryFrom::TryFrom => generate_error_occurence_variant_token_stream(
            &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_syn_variant,
            &proc_macro_name_upper_camel_case_ident_stringified,
        ),
    };
    let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many_declaration_hadle_token_stream = match primary_key_from_or_try_from {
        postgresql_crud_common::FromOrTryFrom::From => proc_macro2::TokenStream::new(),
        postgresql_crud_common::FromOrTryFrom::TryFrom => generate_error_occurence_variant_token_stream(
            &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many_syn_variant,
            &proc_macro_name_upper_camel_case_ident_stringified,
        ),
    };
    let (
        not_unique_field_vec_syn_variants_with_initialization_and_status_code,
        generate_not_unique_field_vec_syn_variant_error_initialization_eprintln_response_creation_token_stream,
    ) = {
        const NOT_UNIQUE_FIELD_VEC_SYN_VARIANTS_STATUS_CODE: proc_macro_helpers::status_code::StatusCode = proc_macro_helpers::status_code::StatusCode::BadRequest400;
        let generate_not_unique_field_vec_syn_variant_error_initialization_eprintln_response_creation_token_stream = |
            operation: &Operation,
            field_ident: &proc_macro2::Ident,
        |{
            let not_unique_field_vec_syn_variant_initialization_token_stream = {
                let not_unique_field_vec_upper_camel_case_stringified = generate_not_unique_field_vec_upper_camel_stringified(field_ident);
                let not_unique_field_vec_upper_camel_case_token_stream = {
                    not_unique_field_vec_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_unique_field_vec_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let not_unique_field_vec_snake_case_stringified = generate_not_unique_field_vec_snake_case_stringified(field_ident);
                let not_unique_field_vec_snake_case_token_stream = {
                    not_unique_field_vec_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_unique_field_vec_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let field_code_occurence_new_4300d2b0_b9ce_47ac_a33c_a143254e1652_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #not_unique_field_vec_upper_camel_case_token_stream {
                        #not_unique_field_vec_snake_case_token_stream,
                        #field_code_occurence_new_4300d2b0_b9ce_47ac_a33c_a143254e1652_token_stream,
                    }
                }
            };
            generate_error_initialization_eprintln_response_creation_token_stream(
                &operation,
                &not_unique_field_vec_syn_variant_initialization_token_stream,
                &quote::quote! {#from_snake_case(#error_snake_case)},
                &NOT_UNIQUE_FIELD_VEC_SYN_VARIANTS_STATUS_CODE.to_axum_http_status_code_token_stream(),
                &eprintln_error_token_stream,
            )
        };
        (
            fields_named.iter().fold(std::vec::Vec::with_capacity(fields_named_len.checked_sub(1).unwrap()), |mut acc, element| {
                let field_ident = &element.field_ident;
                let not_unique_field_vec_upper_camel_case_stringified = generate_not_unique_field_vec_upper_camel_stringified(field_ident);
                let not_unique_field_vec_snake_case_stringified = generate_not_unique_field_vec_snake_case_stringified(field_ident);
                let where_inner_type_with_serialize_deserialize_handle_stringified = &element.where_inner_type_with_serialize_deserialize_handle_stringified;
                acc.push(proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                    NOT_UNIQUE_FIELD_VEC_SYN_VARIANTS_STATUS_CODE.clone(),
                    &not_unique_field_vec_upper_camel_case_stringified,
                    vec![
                        (
                            proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize,
                            &not_unique_field_vec_snake_case_stringified,
                            generate_std_vec_vec_syn_punctuated_punctuated(
                                //todo reuse
                                &[postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, where_inner_type_with_serialize_deserialize_handle_stringified],
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            )//todo its dont work with json generic
                        )
                    ],
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ));
                acc
            }),
            generate_not_unique_field_vec_syn_variant_error_initialization_eprintln_response_creation_token_stream
        )
    };
    let (
        no_payload_fields_syn_variant,
        no_payload_fields_syn_variant_initialization_token_stream,
    ) = {
        let no_payload_fields_upper_camel_case = naming_conventions::NoPayloadFieldsUpperCamelCase;
        let no_payload_fields_snake_case = naming_conventions::NoPayloadFieldsSnakeCase;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                proc_macro_helpers::status_code::StatusCode::BadRequest400,
                &no_payload_fields_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
                        &no_payload_fields_snake_case,
                        std_string_string_syn_punctuated_punctuated.clone()
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let no_payload_fields_quotes_token_stream = proc_macro_common::generate_quotes::token_stream(
                    &format!(
                        "{} {} {}",
                        &naming_constants::NoSnakeCase,
                        &naming_constants::PayloadSnakeCase,
                        &naming_constants::FieldsSnakeCase
                    ),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let field_code_occurence_new_23fdf468_0468_4c5c_8670_08f6f747e417_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #no_payload_fields_upper_camel_case {
                        #no_payload_fields_snake_case: #std_string_string::#from_snake_case(#no_payload_fields_quotes_token_stream),
                        #field_code_occurence_new_23fdf468_0468_4c5c_8670_08f6f747e417_token_stream
                    }
                }
            }
        )
    };
    let (
        no_payload_parameters_syn_variant,
        no_payload_parameters_syn_variant_initialization_token_stream,
     ) = {
        let no_payload_parameters_upper_camel_case = naming_conventions::NoPayloadParametersUpperCamelCase;
        let no_payload_parameters_snake_case = naming_conventions::NoPayloadParametersSnakeCase;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                proc_macro_helpers::status_code::StatusCode::BadRequest400,
                &no_payload_parameters_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
                        &no_payload_parameters_snake_case,
                        std_string_string_syn_punctuated_punctuated.clone()
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_0b135c41_f41a_4117_8b43_2ad519ad6386_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #no_payload_parameters_upper_camel_case {
                        #no_payload_parameters_snake_case: #error_snake_case,
                        #field_code_occurence_new_0b135c41_f41a_4117_8b43_2ad519ad6386_token_stream
                    }
                }
            }
        )
    };
    let axum_response_into_response = token_patterns::AxumResponseIntoResponse;
    let axum_extract_rejection_json_rejection = token_patterns::AxumExtractRejectionJsonRejection;
    let sqlx_query_sqlx_postgres_token_stream = quote::quote! {sqlx::query::<sqlx::Postgres>};
    let axum_extract_state_token_stream = quote::quote! {axum::extract::State};
    let axum_json_token_stream = quote::quote! {axum::Json};
    //todo remove it after refactor to not support middleware
    let crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream = quote::quote! {crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor};
    //todo reuse BindQuery path
    let crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream = quote::quote! {#postgresql_crud_snake_case::BindQuery::bind_value_to_query};
    let crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream = quote::quote! {#postgresql_crud_snake_case::BindQuery::try_generate_bind_increments};
    let crate_server_postgres_bind_query_bind_query_try_increment_token_stream = quote::quote! {#postgresql_crud_snake_case::BindQuery::try_increment};
    let increment_initialization_token_stream = quote::quote! {let mut increment: u64 = 0;};
    let http_status_code = token_patterns::HttpStatusCode;
    let reqwest_header_header_map = token_patterns::ReqwestHeaderHeaderMap;
    let reqwest_error = token_patterns::ReqwestError;
    // let crate_common_api_request_unexpected_error_response_text_result_token_stream = quote::quote! {crate::common::api_request_unexpected_error::ResponseTextResult};
    let try_extract_value_snake_case = naming_conventions::TryExtractValueSnakeCase;
    let dot_space = ", ";
    // let pg_temp_stringified = "pg_temp";
    let where_snake_case_qoutes_token_stream = proc_macro_common::generate_quotes::token_stream(
        &where_snake_case.to_string(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let and_snake_case = naming_constants::AndSnakeCase;
    let select_snake_case = naming_constants::SelectSnakeCase;
    let order_by_snake_case = naming_conventions::OrderBySnakeCase;
    let limit_snake_case = naming_constants::LimitSnakeCase;
    let offset_snake_case = naming_constants::OffsetSnakeCase;
    let in_snake_case = naming_constants::InSnakeCase;
    let unnest_snake_case = naming_constants::UnnestSnakeCase;
    let value_snake_case = naming_constants::ValueSnakeCase;
    let error_snake_case = naming_constants::ErrorSnakeCase;
    let response_snake_case = naming_constants::ResponseSnakeCase;
    let primary_keys_snake_case = naming_conventions::PrimaryKeysSnakeCase;
    let primary_key_snake_case = naming_conventions::PrimaryKeySnakeCase;
    let into_inner_type_vec_snake_case = naming_conventions::IntoInnerTypeVecSnakeCase;
    let api_request_unexpected_error_module_path_token_stream = quote::quote! { crate::common::api_request_unexpected_error };
    let expected_type_upper_camel_case = naming_conventions::ExpectedTypeUpperCamelCase;
    let expected_type_snake_case = naming_conventions::ExpectedTypeSnakeCase;
    let unexpected_status_code_upper_camel_case = naming_conventions::UnexpectedStatusCodeUpperCamelCase;
    let status_code_snake_case = naming_conventions::StatusCodeSnakeCase;
    let headers_snake_case = naming_constants::HeadersSnakeCase;
    let body_snake_case = naming_constants::BodySnakeCase;
    let response_text_result_snake_case = naming_conventions::ResponseTextResultSnakeCase;
    let response_text_snake_case = naming_conventions::ResponseTextSnakeCase;
    let fields_named_idents_comma_token_stream = generate_self_fields_token_stream(
        &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    )
    .iter()
    .map(|element| quote::quote! {#element,})
    .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let select_full_variant_token_stream = {
        let value = fields_named.iter().fold(std::string::String::default(), |mut acc, element| {
            acc.push_str(&convert_case::Casing::to_case(&element.field_ident.to_string(), convert_case::Case::UpperCamel));
            acc
        });
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
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
    let fields_idents_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element| element.field_ident).collect::<std::vec::Vec<&syn::Ident>>();
    let std_vec_vec_primary_key_inner_type_with_serialize_deserialize_token_stream = quote::quote!{std::vec::Vec::<#primary_key_inner_type_with_serialize_deserialize_token_stream>};
    let std_vec_vec_struct_options_ident_token_stream = quote::quote!{std::vec::Vec::<#struct_options_ident_token_stream>};
    //todo reuse naming
    let not_unique_column_upper_camel_case = naming_conventions::NotUniqueColumnUpperCamelCase;
    let not_unique_column_snake_case = naming_conventions::NotUniqueColumnSnakeCase;
    let (
        not_unique_column_syn_variant,
        not_unique_column_syn_variant_initialization_token_stream,
        not_unique_column_syn_variant_status_code,
     ) = {
        let not_unique_column_upper_camel_case = naming_conventions::NotUniqueColumnUpperCamelCase;
        let not_unique_column_snake_case = naming_conventions::NotUniqueColumnSnakeCase;
        let not_unique_column_status_code = proc_macro_helpers::status_code::StatusCode::BadRequest400;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                not_unique_column_status_code.clone(),
                &not_unique_column_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
                        &not_unique_column_snake_case,
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &[&ident_column_upper_camel_case_stringified],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        )
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_ce1f22e2_2619_43b3_9d5e_8fbe72845431_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #not_unique_column_upper_camel_case {
                        #not_unique_column_snake_case: #element_snake_case,
                        #field_code_occurence_new_ce1f22e2_2619_43b3_9d5e_8fbe72845431_token_stream
                    }
                }
            },
            not_unique_column_status_code
        )
    };
    let axum_http_status_code_token_stream = quote::quote!{axum::http::StatusCode};
    let postgresql_crud_get_axum_http_status_code_token_stream = quote::quote!{postgresql_crud::GetAxumHttpStatusCode};
    let get_axum_http_status_code_snake_case = naming_conventions::GetAxumHttpStatusCodeSnakeCase;
    let app_state_dyn_postgresql_crud_combination_of_traits_for_postgresql_crud_logic_comma_token_stream = quote::quote!{#app_state_snake_case: &dyn postgresql_crud::CombinationOfTraitsForPostgresqlCrudLogic,};
    let body_bytes_snake_case = naming_conventions::BodyBytesSnakeCase;
    let body_bytes_bytes_bytes_token_stream = quote::quote!{#body_bytes_snake_case: bytes::Bytes,};
    let axum_response_into_response_token_stream = quote::quote!{axum::response::IntoResponse};
    let axum_response_response_token_stream = quote::quote!{axum::response::Response};
    let into_response_snake_case = naming_conventions::IntoResponseSnakeCase;
    let (
        serde_json_to_string_syn_variant,
        serde_json_to_string_syn_variant_initialization_token_stream
     ) = {
        let serde_json_to_string_upper_camel_case = naming_conventions::SerdeJsonToStringUpperCamelCase;
        let serde_json_to_string_snake_case = naming_conventions::SerdeJsonToStringSnakeCase;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant(
                &serde_json_to_string_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &serde_json_to_string_snake_case,
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &["serde_json","Error"],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_27b49c75_24b2_4480_ac4d_62a1f75f5646_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #serde_json_to_string_upper_camel_case {
                        #serde_json_to_string_snake_case: #error_snake_case,
                        #field_code_occurence_new_27b49c75_24b2_4480_ac4d_62a1f75f5646_token_stream,
                    }
                }
            }
        )
    };
    let (
        failed_to_get_response_text_syn_variant,
        failed_to_get_response_text_syn_variant_initialization_token_stream,
    ) = {
        let failed_to_get_response_text_upper_camel_case = naming_conventions::FailedToGetResponseTextUpperCamelCase;
        let status_code_snake_case = naming_conventions::StatusCodeSnakeCase;
        let headers_snake_case = naming_constants::HeadersSnakeCase;
        let reqwest_snake_case = naming_constants::ReqwestSnakeCase;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                proc_macro_helpers::status_code::StatusCode::BadRequest400,
                &failed_to_get_response_text_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &status_code_snake_case.to_string(),
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &["http","StatusCode"],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    ),
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &headers_snake_case.to_string(),
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &["reqwest","header","HeaderMap"],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    ),
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &reqwest_snake_case.to_string(),
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &["reqwest","Error"],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_4528b9ed_5b9d_486b_af78_345e1b9d95cc_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #failed_to_get_response_text_upper_camel_case {
                        #status_code_snake_case,
                        #headers_snake_case,
                        #reqwest_snake_case: #error_snake_case,
                        #field_code_occurence_new_4528b9ed_5b9d_486b_af78_345e1b9d95cc_token_stream
                    }
                }
            }
        )
    };
    let (
        deserialize_response_syn_variant,
        deserialize_response_syn_variant_initialization_token_stream
    ) = {
        let deserialize_response_upper_camel_case = naming_conventions::DeserializeResponseUpperCamelCase;
        let status_code_snake_case = naming_conventions::StatusCodeSnakeCase;
        let headers_snake_case = naming_constants::HeadersSnakeCase;
        let response_text_snake_case = naming_conventions::ResponseTextSnakeCase;
        let serde_snake_case = naming_constants::SerdeSnakeCase;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant(
                &deserialize_response_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &status_code_snake_case.to_string(),
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &["http","StatusCode"],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    ),
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &headers_snake_case.to_string(),
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &["reqwest","header", "HeaderMap"],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    ),
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
                        &response_text_snake_case.to_string(),
                        std_string_string_syn_punctuated_punctuated.clone(),
                    ),
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &serde_snake_case.to_string(),
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &["serde_json","Error"],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_e04ccb06_5ef1_4e32_82c7_a0aa1e48e5ad_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #deserialize_response_upper_camel_case {
                        #status_code_snake_case,
                        #headers_snake_case,
                        #response_text_snake_case,
                        #serde_snake_case: #error_snake_case,
                        #field_code_occurence_new_e04ccb06_5ef1_4e32_82c7_a0aa1e48e5ad_token_stream,
                    }
                }
            }
        )
    };
    let (
        reqwest_syn_variant,
        reqwest_syn_variant_initialization_token_stream,
    ) = {
        let reqwest_upper_camel_case = naming_constants::ReqwestUpperCamelCase;
        let reqwest_snake_case = naming_constants::ReqwestSnakeCase;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant(
                &reqwest_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &reqwest_snake_case,
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &["reqwest","Error"],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_1f50ca3d_e584_404d_bd9d_22368ce623ab_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #reqwest_upper_camel_case {
                        #reqwest_snake_case: #error_snake_case,
                        #field_code_occurence_new_1f50ca3d_e584_404d_bd9d_22368ce623ab_token_stream,
                    }
                }
            }
        )
    };
    let common_http_request_syn_variants = {
        vec![
            serde_json_to_string_syn_variant,
            failed_to_get_response_text_syn_variant,
            deserialize_response_syn_variant,
            reqwest_syn_variant,
        ]
    };
    let (check_body_size_syn_variant, check_body_size_syn_variant_initialization_token_stream) = {
        let check_body_size_upper_camel_case = naming_conventions::CheckBodySizeUpperCamelCase;
        let check_body_size_snake_case = naming_conventions::CheckBodySizeSnakeCase;
        let field_code_occurence_new_1b7ff451_4836_40a6_a477_00c30fa61894_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                proc_macro_helpers::status_code::StatusCode::BadRequest400,
                &check_body_size_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
                        &check_body_size_snake_case,
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &["route_validators", "check_body_size", &naming_conventions::CheckBodySizeErrorNamedUpperCamelCase.to_string()],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            quote::quote! {
                #check_body_size_upper_camel_case {
                    #check_body_size_snake_case: #error_snake_case,
                    #field_code_occurence_new_1b7ff451_4836_40a6_a477_00c30fa61894_token_stream,
                }
            }
        )
    };
    let (postgresql_syn_variant, postgresql_syn_variant_initialization_token_stream, postgresql_syn_variant_status_code) = {
        let postgresql_upper_camel_case = naming_constants::PostgresqlUpperCamelCase;
        let postgresql_snake_case = naming_constants::PostgresqlSnakeCase;
        let postgresql_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::InternalServerError500;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                postgresql_syn_variant_status_code.clone(),
                &postgresql_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &postgresql_snake_case,
                        sqlx_error_syn_punctuated_punctuated,
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_d63fbe34_868a_43f7_b3ae_1fe88f0b9665_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #postgresql_upper_camel_case {
                        #postgresql_snake_case: error,//todo reuse error_snake_case_token_stream
                        #field_code_occurence_new_d63fbe34_868a_43f7_b3ae_1fe88f0b9665_token_stream,
                    }
                }
            },
            postgresql_syn_variant_status_code
        )
    };
    let (
        json_syn_variant,
        json_syn_variant_initialization_token_stream,
        json_syn_variant_status_code
    ) = {
        let json_upper_camel_case = naming_constants::JsonUpperCamelCase;
        let json_snake_case = naming_constants::JsonSnakeCase;
        let json_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::BadRequest400;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                json_syn_variant_status_code.clone(),
                &json_upper_camel_case,
                vec![
                    (
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &json_snake_case,
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &["axum","extract","rejection","JsonRejection"],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    )
                ],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_699a3261_4228_40a9_944c_b68ec050288b_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #json_upper_camel_case {
                        #json_snake_case: #error_snake_case,
                        #field_code_occurence_new_699a3261_4228_40a9_944c_b68ec050288b_token_stream,
                    }
                }
            },
            json_syn_variant_status_code
        )
    };
    let (
        bind_query_syn_variant, 
        bind_query_syn_variant_initialization_token_stream,
        bind_query_syn_variant_status_code
    ) = {
        let bind_query_upper_camel_case = naming_conventions::BindQueryUpperCamelCase;
        let bind_query_snake_case = naming_conventions::BindQuerySnakeCase;
        let bind_query_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::InternalServerError500;
        (
            proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                bind_query_syn_variant_status_code.clone(),
                &bind_query_upper_camel_case,
                vec![(
                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
                    &bind_query_snake_case,
                    proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                        &[postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE, &naming_conventions::TryGenerateBindIncrementsErrorNamedUpperCamelCase.to_string()],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                )],
                &proc_macro_name_upper_camel_case_ident_stringified,
            ),
            {
                let field_code_occurence_new_d61d7616_3336_43be_aaa8_2144ff2d2158_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    #bind_query_upper_camel_case {
                        #bind_query_snake_case: #error_snake_case,
                        #field_code_occurence_new_d61d7616_3336_43be_aaa8_2144ff2d2158_token_stream
                    }
                }
            },
            bind_query_syn_variant_status_code
        )
    };
    let common_additional_error_variants = generate_additional_error_variants(
        &ast,
        GeneratePostgresqlCrudAttribute::CommonAdditionalErrorVariants,
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let common_route_syn_variants = {
        let common_additional_error_variants_vec = common_additional_error_variants.iter().collect::<std::vec::Vec<&syn::Variant>>();
        let mut value = std::vec::Vec::with_capacity(common_additional_error_variants_vec.len() + common_additional_error_variants_vec.len());
        value.push(&check_body_size_syn_variant);
        value.push(&postgresql_syn_variant);
        value.push(&json_syn_variant);
        // value.push(&bind_query_syn_variant);
        for element in common_additional_error_variants_vec {
            value.push(element);
        }
        value
    };
    let checked_add_and_bind_query_syn_variants = vec![&checked_add_syn_variant, &bind_query_syn_variant];
    let common_additional_route_logic_token_stream = proc_macro_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(
        &ast.attrs,
        &GeneratePostgresqlCrudAttribute::CommonAdditionalRouteLogic.generate_path_to_attribute(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let generate_fields_named_token_stream = |function: fn(&SynFieldWithAdditionalInfo<'_>)-> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_token_stream = fields_named.iter().map(function);
        quote::quote! {#(#fields_token_stream),*}
    };
    let generate_fields_named_excluding_primary_key_token_stream = |function: fn(&SynFieldWithAdditionalInfo<'_>)-> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_token_stream = fields_named_excluding_primary_key.iter().map(function);
        quote::quote! {#(#fields_token_stream),*}
    };
    let generate_payload_and_payload_with_serialize_deserialize_one = |operation: &Operation, exclude_primary_key: bool| -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let (
            payload_fields_token_stream,
            payload_fields_with_serialize_deserialize
        ) = match exclude_primary_key {
            true => (
                &generate_fields_named_excluding_primary_key_token_stream(generate_pub_field_ident_field_type_token_stream),
                &generate_fields_named_excluding_primary_key_token_stream(generate_field_ident_field_type_with_serialize_deserialize_token_stream),
            ),
            false => (
                &generate_fields_named_token_stream(generate_pub_field_ident_field_type_token_stream),
                &generate_fields_named_token_stream(generate_field_ident_field_type_with_serialize_deserialize_token_stream),
            )
        };
        let payload_token_stream = generate_operation_payload_token_stream(
            &operation,
            &payload_fields_token_stream,
        );
        let payload_with_serialize_deserialize_token_stream = generate_payload_with_serialize_deserialize_token_stream(
            &operation,
            &payload_fields_with_serialize_deserialize,
        );
        (
            payload_token_stream,
            payload_with_serialize_deserialize_token_stream
        )
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
    let (create_many_token_stream, create_many_test_token_stream) = {
        let operation = Operation::CreateMany;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &common_route_syn_variants,
            &fields_named_excluding_primary_key_from_or_try_from,
            &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant,
            &operation,
            &ast,
            &proc_macro_name_upper_camel_case_ident_stringified,
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
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream(
                &operation,
                &{
                    let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream =  naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                    quote::quote!{
                        #primary_key_field_ident_upper_camel_case_token_stream {
                            #eo_error_occurence_attribute_token_stream
                            #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream,
                            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                        },
                    }
                },
                &{
                    let field_code_occurence_new_46d303bf_4267_4eb4_a98d_22193db9d220_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        #primary_key_field_ident_upper_camel_case_token_stream {
                            #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: #error_snake_case,
                            #field_code_occurence_new_46d303bf_4267_4eb4_a98d_22193db9d220_token_stream
                        }
                    }
                },
                &fields_named_excluding_primary_key,
                &fields_idents_excluding_primary_key_token_stream,
                &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                &fields_named_excluding_primary_key_from_or_try_from,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            // println!("{impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = generate_impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream(
                &operation,
                &fields_named_excluding_primary_key,
                &fields_idents_excluding_primary_key_token_stream,
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
            let try_operation_route_logic_response_variants_token_stream = generate_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &std_vec_vec_primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = generate_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_error_named_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &fields_named_excluding_primary_key_from_or_try_from,
                   &json_syn_variant_initialization_token_stream,
                   &json_syn_variant_status_code,
                   &eprintln_error_token_stream,
                   &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let query_string_token_stream = {
                    let column_names = fields_named_excluding_primary_key.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, element)| {
                        let field_ident = &element.field_ident;
                        let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                        if incremented_index == fields_named_excluding_primary_key_len {
                            acc.push_str(&format!("{field_ident}"));
                        }
                        else {
                            acc.push_str(&format!("{field_ident}{dot_space}"));
                        }
                        acc
                    });
                    let column_increments = {
                        let mut column_increments = fields_named_excluding_primary_key.iter()
                            .enumerate().fold(std::string::String::default(), |mut acc, (index, _)| {
                                acc.push_str(&format!("${}, ", index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE))));
                                acc
                            });
                        let _: std::option::Option<std::primitive::char> = column_increments.pop();
                        let _: std::option::Option<std::primitive::char> = column_increments.pop();
                        column_increments
                    };
                    let query_stringified = format!(
                        "\"{insert_snake_case} {into_snake_case} {table_name_stringified} ({column_names}) {select_snake_case} {column_names} {from_snake_case} {unnest_snake_case}({column_increments}) {as_snake_case} a({column_names}){returning_primary_key_stringified}\""
                    );
                    query_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let column_vecs_token_stream = {
                        let column_vecs_handle_token_stream = {
                            let value = fields_named_excluding_primary_key.iter().map(|element| {
                                let field_ident_underscore_vec_stringified = {
                                    let field_ident = &element.field_ident;
                                    format!("{field_ident}_{}", naming_constants::VecSnakeCase)
                                };
                                field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}",proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            });
                            quote::quote!{#(#value),*}
                        };
                        if is_fields_named_excluding_primary_key_len_equals_one {
                            column_vecs_handle_token_stream
                        }
                        else {
                            quote::quote!{(#column_vecs_handle_token_stream)}
                        }
                    };
                    let column_vecs_with_capacity_token_stream = {
                        let column_vecs_with_capacity_handle_token_stream = {
                            let value = fields_named_excluding_primary_key.iter().map(|_|quote::quote!{std::vec::Vec::with_capacity(#current_vec_len_snake_case)});
                            quote::quote!{#(#value),*}
                        };
                        if is_fields_named_excluding_primary_key_len_equals_one {
                            column_vecs_with_capacity_handle_token_stream
                        }
                        else {
                            quote::quote!{(#column_vecs_with_capacity_handle_token_stream)}
                        }
                    };
                    let columns_acc_push_elements_token_stream = fields_named_excluding_primary_key.iter().enumerate().map(|(index, element)|{
                        let field_ident = &element.field_ident;
                        let index_token_stream = if is_fields_named_excluding_primary_key_len_equals_one {
                            proc_macro2::TokenStream::new()
                        }
                        else {
                            let index_stringified = index.to_string();
                            let value_token_stream = index_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index_stringified} {}",proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            quote::quote!{.#value_token_stream}
                        };
                        //space need to to concat token stream correctly
                        quote::quote!{#acc_snake_case #index_token_stream.push(#element_snake_case.#field_ident);}
                    });
                    let column_query_bind_vecs_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident_underscore_vec_token_stream = {
                            let field_ident_underscore_vec_stringified = {
                                let field_ident = &element.field_ident;
                                format!("{field_ident}_{}", naming_constants::VecSnakeCase)
                            };
                            field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}",proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let inner_type_token_stream = &element.inner_type_token_stream;
                        quote::quote!{
                            #query_snake_case = #query_snake_case.bind(
                                //todo add ::<T> for serde json <T> case. for others just empty token stream
                                #inner_type_token_stream::#into_inner_type_vec_snake_case(#field_ident_underscore_vec_token_stream)
                            );
                        }
                    });
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        let #current_vec_len_snake_case = #parameters_snake_case.#payload_snake_case.0.len();
                        let #column_vecs_token_stream = #parameters_snake_case.#payload_snake_case.0.into_iter().fold(#column_vecs_with_capacity_token_stream,
                        |mut #acc_snake_case, #element_snake_case| {
                            #(#columns_acc_push_elements_token_stream)*
                            #acc_snake_case
                        });
                        #(#column_query_bind_vecs_token_stream)*
                        #query_snake_case
                    }
                };
                // println!("{binded_query_token_stream}");
                let postgresql_logic_token_stream = {
                    let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &postgresql_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &postgresql_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    quote::quote! {
                        let mut rows = #binded_query_snake_case.fetch(#pg_connection_snake_case.as_mut());
                        let mut vec_values = std::vec::Vec::new();
                        while let Some(value) = {
                            match {
                                use futures::TryStreamExt;
                                rows.try_next()
                            }
                            .await
                            {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_snake_case) => {
                                    #error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        } {
                            match #sqlx_row::try_get::<#primary_key_original_type_token_stream, #ref_std_primitive_str>(&value, #primary_key_field_ident_quotes_token_stream) {
                                Ok(#value_snake_case) => {
                                    vec_values.push(
                                        #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(
                                            #primary_key_inner_type_token_stream(#value_snake_case)
                                        ),
                                    );
                                }
                                Err(#error_snake_case) => {
                                    #error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                        vec_values
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
                generate_try_operation_route_logic_snake_case_token_stream(
                    &operation,
                    &ast,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &check_body_size_syn_variant_initialization_token_stream,
                    &postgresql_syn_variant_initialization_token_stream,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_token_stream
                #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
                #try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &quote::quote!{std::vec::Vec<#primary_key_inner_type_token_stream>},
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match primary_key_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                        #value_snake_case
                        .into_iter()
                        .map(|#element_snake_case| #primary_key_inner_type_token_stream::#from_snake_case(#element_snake_case))
                        .collect()
                    },
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            {
                                let mut values = std::vec::Vec::new();
                                for #element_snake_case in #value_snake_case {
                                    match #primary_key_inner_type_token_stream::#try_from_snake_case(#element_snake_case) {
                                        Ok(#value_snake_case) => {
                                            values.push(#value_snake_case);
                                        },
                                        Err(#error_snake_case) => Err(
                                            #try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream
                                        )
                                    }
                                }
                                values
                            }
                        }
                    }
                },
                &proc_macro_name_upper_camel_case_ident_stringified,
                &reqwest_syn_variant_initialization_token_stream,
                &deserialize_response_syn_variant_initialization_token_stream,
                &failed_to_get_response_text_syn_variant_initialization_token_stream,
                &serde_json_to_string_syn_variant_initialization_token_stream,
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
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &create_many_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (create_one_token_stream, create_one_test_token_stream) = {
        let operation = Operation::CreateOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &common_route_syn_variants,
            &fields_named_excluding_primary_key_from_or_try_from,
            &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant,
            &operation,
            &ast,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let (
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream
            ) = generate_payload_and_payload_with_serialize_deserialize_one(&operation, true);
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = match fields_named_excluding_primary_key_from_or_try_from {
                postgresql_crud_common::FromOrTryFrom::From => proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                    &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                    &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                    &{
                        let fields_assignment_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter()
                        .map(|element| {
                            let field_ident = &element.field_ident;
                            let inner_type_token_stream = &element.inner_type_token_stream;
                            quote::quote!{
                                let #field_ident = #inner_type_token_stream::#from_snake_case(value.#field_ident);
                            }
                        });
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
                            let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                                &fields_named_excluding_primary_key,
                                &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                            );
                            quote::quote! {#(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*}
                        },
                    );
                    // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = 
                    proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
                        &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation),
                        &{
                            let field_code_occurence_new_3763990f_5c49_47d0_a774_5ef584cd1236_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                file!(),
                                line!(),
                                column!(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            let fields_assignment_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter()
                            .map(|element| generate_let_field_ident_value_field_ident_try_from_token_stream(
                                element,
                                &proc_macro_name_upper_camel_case_ident_stringified,
                                &field_code_occurence_new_3763990f_5c49_47d0_a774_5ef584cd1236_token_stream,
                                &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
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
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                &{
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter()
                        .map(|element|generate_let_field_ident_value_inner_type_with_serialize_deserialize_from_token_stream(element));
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
            let try_operation_route_logic_response_variants_token_stream = generate_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = generate_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_error_named_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &fields_named_excluding_primary_key_from_or_try_from,
                   &json_syn_variant_initialization_token_stream,
                   &json_syn_variant_status_code,
                   &eprintln_error_token_stream,
                   &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let query_string_token_stream = {
                    let (column_names, column_increments) = {
                        fields_named_excluding_primary_key.iter().enumerate().fold((
                            std::string::String::default(),
                            std::string::String::default()
                        ), |mut acc, (index, element)| {
                            let field_ident = &element.field_ident;
                            let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                            if incremented_index == fields_named_excluding_primary_key_len {
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
                    let query_stringified = format!("\"{insert_snake_case} {into_snake_case} {table_name_stringified}({column_names}) {values_snake_case} ({column_increments}){returning_primary_key_stringified}\"");
                    query_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        quote::quote!{
                            query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case.#payload_snake_case.#field_ident, query);
                        }
                    });
                    quote::quote! {
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        query
                    }
                };
                // println!("{binded_query_token_stream}");
                let postgresql_logic_token_stream = {
                    let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &postgresql_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &postgresql_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    quote::quote! {
                        match #binded_query_snake_case.fetch_one(#pg_connection_snake_case.as_mut()).await {
                            Ok(#value_snake_case) => match #sqlx_row::try_get::<#primary_key_original_type_token_stream, &str>(&#value_snake_case, #primary_key_field_ident_quotes_token_stream) {
                                Ok(#value_snake_case) => #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(
                                    #primary_key_inner_type_token_stream(#value_snake_case)
                                ),
                                Err(#error_snake_case) => {
                                    #error_initialization_eprintln_response_creation_token_stream
                                }
                            },
                            Err(#error_snake_case) => {
                                #error_initialization_eprintln_response_creation_token_stream
                            }
                        }
                    }
                };
                // // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                // //     &table_name_stringified,
                // //     &unique_status_codes,
                // //     &application_json_quotes_token_stream,
                // //     &table_name_quotes_token_stream,
                // //     &operation_payload_upper_camel_case_token_stream,
                // //     &operation,
                // // );
                generate_try_operation_route_logic_snake_case_token_stream(
                    &operation,
                    &ast,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &check_body_size_syn_variant_initialization_token_stream,
                    &postgresql_syn_variant_initialization_token_stream,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_token_stream
                #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
                #try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &primary_key_inner_type_token_stream,
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match primary_key_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{#primary_key_inner_type_token_stream::#from_snake_case(#value_snake_case)},
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            match #primary_key_inner_type_token_stream::#try_from_snake_case(#value_snake_case) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_snake_case) => Err(
                                    #try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream
                                )
                            }
                        }
                    }
                },
                &proc_macro_name_upper_camel_case_ident_stringified,
                &reqwest_syn_variant_initialization_token_stream,
                &deserialize_response_syn_variant_initialization_token_stream,
                &failed_to_get_response_text_syn_variant_initialization_token_stream,
                &serde_json_to_string_syn_variant_initialization_token_stream,
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
    let (read_many_token_stream, read_many_test_token_stream) = {
        let operation = Operation::ReadMany;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
            let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + checked_add_and_bind_query_syn_variants.len());
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                checked_add_and_bind_query_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value
            },
            &fields_named_excluding_primary_key_from_or_try_from,
            &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant,
            &operation,
            &ast,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // let type_variants_from_request_response_syn_variants = {
        //     let full_additional_http_status_codes_error_variants =
        //         generate_full_additional_http_status_codes_error_variants(
        //             common_middlewares_error_syn_variants.iter().collect(),
        //             additional_http_status_codes_error_variants.iter().collect(),
        //         );
        //     let type_variants_from_request_response_syn_variants_partial = {
        //         let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
        //             common_error_syn_variants.len().checked_add(not_unique_field_vec_syn_variants.len().checked_add(4).unwrap()).unwrap()
        //         );
        //         for element in &common_error_syn_variants {
        //             type_variants_from_request_response.push(element);
        //         }
        //         for element in &not_unique_field_vec_syn_variants {
        //             type_variants_from_request_response.push(element);
        //         }
        //         type_variants_from_request_response.push(&not_unique_primary_keys_syn_variant);
        //         type_variants_from_request_response.push(&bind_query_syn_variant);
        //         if fields_named_from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
        //             type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
        //         }
        //         type_variants_from_request_response
        //     };
        //     generate_type_variants_from_request_response_syn_variants(
        //         &type_variants_from_request_response_syn_variants_partial,
        //         &full_additional_http_status_codes_error_variants,
        //     )
        // };
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
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let order_by_snake_case = naming_conventions::OrderBySnakeCase;
                let common_initialization_token_stream = quote::quote!{
                    let #select_snake_case = #value_snake_case.#select_snake_case;
                    let #order_by_snake_case = #value_snake_case.#order_by_snake_case;
                    let #limit_snake_case = #limit_and_offset_type_token_stream::#from_snake_case(#value_snake_case.#limit_snake_case);
                    let #offset_snake_case = #limit_and_offset_type_token_stream::#from_snake_case(#value_snake_case.#offset_snake_case);
                };
                let common_assignment_token_stream = quote::quote!{
                    #select_snake_case,
                    #order_by_snake_case,
                    #limit_snake_case,
                    #offset_snake_case,
                };
                match fields_named_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                        &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                        &{
                            let primary_key_field_assignment_token_stream = {
                                quote::quote! {
                                    let #primary_key_field_ident = match #value_snake_case.#primary_key_field_ident {
                                        Some(#value_snake_case) => Some(
                                            #value_snake_case.into_iter()
                                            .map(|#element_snake_case|#primary_key_inner_type_token_stream::#from_snake_case(
                                                #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#element_snake_case)
                                            ))//todo refactor
                                            .collect()
                                        ),
                                        None => None
                                    };
                                }
                            };
                            let fields_assignment_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter()
                                .map(|element| {
                                    let field_ident = &element.field_ident;
                                    let where_inner_type_token_stream = &element.where_inner_type_token_stream;
                                    quote::quote!{
                                        let #field_ident = match #value_snake_case.#field_ident {
                                            Some(#value_snake_case) => Some(#value_snake_case.into_iter().map(|#element_snake_case|#where_inner_type_token_stream::#from_snake_case(#element_snake_case)).collect()),
                                            None => None,
                                        };
                                    }
                                });
                            quote::quote! {
                                #primary_key_field_assignment_token_stream
                                #(#fields_assignment_excluding_primary_key_token_stream)*
                                #common_initialization_token_stream
                                Self {
                                    #(#fields_named_idents_comma_token_stream)*
                                    #common_assignment_token_stream
                                }
                            }
                        }
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
                                            #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: #primary_key_inner_type_with_serialize_deserialize_error_named_token_stream,
                                            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                                        },
                                    }
                                };
                                let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_where_inner_type_from_or_try_from_where_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                                    &fields_named_excluding_primary_key,
                                    &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                                    &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                    &eo_error_occurence_attribute_token_stream,
                                );
                                quote::quote! {
                                    #primary_key_variant_token_stream
                                    #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                                }
                            },
                        );
                        // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                        let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = 
                        proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
                            &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                            &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                            &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation),
                            &{
                                let primary_key_field_assignment_token_stream = {
                                    let primary_key_initialization_token_stream = match primary_key_from_or_try_from {
                                        postgresql_crud_common::FromOrTryFrom::From => quote::quote! {
                                            Some(
                                                value.into_iter()
                                                .map(|element|#primary_key_inner_type_token_stream::#from_snake_case(
                                                    #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(element)
                                                ))
                                                .collect::<std::vec::Vec<#primary_key_inner_type_token_stream>>()
                                            )
                                        },
                                        postgresql_crud_common::FromOrTryFrom::TryFrom => {
                                            let field_code_occurence_new_ed55593d_d353_440c_8145_c1c712bc5ace_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                                file!(),
                                                line!(),
                                                column!(),
                                                &proc_macro_name_upper_camel_case_ident_stringified,
                                            );
                                            quote::quote! {
                                                match value.into_iter()
                                                    .map(|element|#primary_key_inner_type_token_stream::try_from(
                                                        #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(element)
                                                    ))
                                                    .collect::<Result<
                                                        std::vec::Vec<#primary_key_inner_type_token_stream>,
                                                        #primary_key_inner_type_with_serialize_deserialize_error_named_token_stream
                                                    >>()
                                                {
                                                    Ok(value) => Some(value),
                                                    Err(#error_snake_case) => {
                                                        return Err(Self::Error::#primary_key_field_ident_upper_camel_case_token_stream {
                                                            #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: #error_snake_case,
                                                            #field_code_occurence_new_ed55593d_d353_440c_8145_c1c712bc5ace_token_stream,
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    quote::quote! {
                                        let #primary_key_field_ident = match value.#primary_key_field_ident {
                                            Some(value) => #primary_key_initialization_token_stream,
                                            None => None
                                        };
                                    }
                                };
                                let fields_assignment_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter()
                                    .map(|element| generate_option_vec_where_inner_type_from_or_try_from_option_vec_where_inner_type_with_serialize_deserialize_token_stream(
                                        element,
                                        &proc_macro_name_upper_camel_case_ident_stringified,
                                        &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                    ));
                                quote::quote! {
                                    #primary_key_field_assignment_token_stream
                                    #(#fields_assignment_excluding_primary_key_token_stream)*
                                    #common_initialization_token_stream
                                    Ok(Self {
                                        #(#fields_named_idents_comma_token_stream)*
                                        #common_assignment_token_stream
                                    })
                                }
                            },
                        );
                        quote::quote! {
                            #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                            #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                        }
                    }
                }
            };
            // println!("{impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                &{
                    let primary_key_field_assignment_token_stream = {
                        quote::quote! {
                            let #primary_key_field_ident = match value.#primary_key_field_ident {
                                Some(value) => Some(
                                    value
                                        .into_iter()
                                        .map(|element| #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(element))
                                        .collect::<std::vec::Vec<#primary_key_inner_type_with_serialize_deserialize_token_stream>>(),
                                ),
                                None => None,
                            };
                        }
                    };
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter()
                        .map(|element| generate_let_field_ident_value_option_vec_with_serialize_deserialize_token_stream(element));
                    let order_by_snake_case = naming_conventions::OrderBySnakeCase;
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
            let try_operation_route_logic_response_variants_token_stream = generate_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &std_vec_vec_struct_options_ident_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = generate_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_error_named_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &fields_named_excluding_primary_key_from_or_try_from,
                   &json_syn_variant_initialization_token_stream,
                   &json_syn_variant_status_code,
                   &eprintln_error_token_stream,
                   &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let filter_unique_parameters_token_stream = {
                    let filter_unique_parameters_primary_key_token_stream = {
                        let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                            &operation,
                            &not_unique_primary_keys_variant_initialization_token_stream,
                            &quote::quote! {#from_snake_case(#error_snake_case)},
                            &not_unique_primary_keys_status_code.to_axum_http_status_code_token_stream(),
                            &eprintln_error_token_stream,
                        );
                        quote::quote! {
                            if let Some(#primary_key_field_ident) = &mut #parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                                let #value_snake_case = {
                                    let mut #value_snake_case = vec![];
                                    #primary_key_field_ident.sort_unstable();
                                    #primary_key_field_ident.dedup_by(|a, b| match a == b {
                                        true => {
                                            value.push(std::mem::take(a));
                                            true
                                        }
                                        false => false,
                                    });
                                    #value_snake_case
                                };
                                if !#value_snake_case.is_empty() {
                                    #error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                    };
                    let filter_unique_parameters_other_columns_token_stream = fields_named_excluding_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let field_handle_token_stream = {
                            let field_handle_stringified = format!("{field_ident}_handle");
                            field_handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let not_unique_field_vec_snake_case_token_stream = {
                            let not_unique_field_vec_snake_case_stringified = generate_not_unique_field_vec_snake_case_stringified(field_ident);
                            not_unique_field_vec_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_unique_field_vec_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let not_unique_field_vec_vec_upper_camel_token_stream = {
                            let not_unique_field_vec_upper_camel_stringified = generate_not_unique_field_vec_upper_camel_stringified(field_ident);
                            not_unique_field_vec_upper_camel_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_unique_field_vec_upper_camel_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let field_code_occurence_new_eb1a9553_449e_4767_9e5c_c1856b77bd4e_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let where_inner_type_with_serialize_deserialize_token_stream = &element.where_inner_type_with_serialize_deserialize_token_stream;
                        let error_initialization_eprintln_response_creation_token_stream = generate_not_unique_field_vec_syn_variant_error_initialization_eprintln_response_creation_token_stream(
                            &operation,
                            &field_ident,
                        );
                        quote::quote!{
                            let #field_handle_token_stream = match #parameters_snake_case.#payload_snake_case.#field_ident {
                                Some(#value_snake_case) => {
                                    let is_unique = {
                                        let mut vec = std::vec::Vec::with_capacity(#value_snake_case.len());
                                        let mut is_unique = true;
                                        for #element_snake_case in &#value_snake_case {
                                            match vec.contains(&#element_snake_case) {
                                                true => {
                                                    is_unique = false;
                                                    break;
                                                }
                                                false => {
                                                    vec.push(#element_snake_case);
                                                }
                                            }
                                        }
                                        is_unique
                                    };
                                    match is_unique {
                                        true => Some(#value_snake_case),
                                        false => {
                                            let #not_unique_field_vec_snake_case_token_stream = {
                                                let mut vec = std::vec::Vec::with_capacity(#value_snake_case.len());
                                                let mut #not_unique_field_vec_snake_case_token_stream = std::vec::Vec::with_capacity(#value_snake_case.len());
                                                for element in #value_snake_case {
                                                    match vec.contains(&#element_snake_case) {
                                                        true => {
                                                            #not_unique_field_vec_snake_case_token_stream.push(#element_snake_case);
                                                        }
                                                        false => {
                                                            vec.push(#element_snake_case);
                                                        }
                                                    }
                                                }
                                                #not_unique_field_vec_snake_case_token_stream.into_iter().map(|#element_snake_case|
                                                    #where_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#element_snake_case)
                                                ).collect()
                                            };
                                            #error_initialization_eprintln_response_creation_token_stream
                                        }
                                    }
                                }
                                None => None,
                            };
                        }
                    });
                    quote::quote! {
                        #filter_unique_parameters_primary_key_token_stream
                        #(#filter_unique_parameters_other_columns_token_stream)*
                    }
                };
                let query_string_token_stream = {
                    let additional_parameters_primary_key_modification_token_stream = {
                        let prefix_false_handle_token_stream = {
                            let prefix_false_handle_stringified = format!("\" {and_snake_case}\"");
                            prefix_false_handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {prefix_false_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let handle_token_stream = {
                            let handle_stringified = format!("\"{{}} {primary_key_field_ident} {in_snake_case} ({select_snake_case} {unnest_snake_case}(${{}}))\"");
                            handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                            &operation,
                            &checked_add_syn_variant_initialization_token_stream,
                            &quote::quote! {#from_snake_case(#error_snake_case)},
                            &checked_add_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                            &eprintln_error_token_stream,
                        );
                        quote::quote! {
                            if let Some(value) = &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                                let prefix = match additional_parameters.is_empty() {
                                    true => #where_snake_case_qoutes_token_stream,
                                    false => #prefix_false_handle_token_stream,
                                };
                                match increment.checked_add(1) {
                                    Some(value) => {
                                        increment = value;
                                    },
                                    None => {
                                        #error_initialization_eprintln_response_creation_token_stream
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
                    let additional_parameters_modification_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        let handle_token_stream = {
                            let handle_stringified = format!("\"{field_ident} ~ {{value}} \"");
                            handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let prefix_false_handle_token_stream = {
                            let prefix_false_handle_stringified = format!("\" {and_snake_case}\"");
                            prefix_false_handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {prefix_false_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let field_handle_token_stream = {
                            let field_handle_stringified = format!("{field_ident}_handle");
                            field_handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                            &operation,
                            &bind_query_syn_variant_initialization_token_stream,
                            &quote::quote! {#from_snake_case(#error_snake_case)},
                            &bind_query_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                            &eprintln_error_token_stream,
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
                                            Err(#error_snake_case) => {
                                                #error_initialization_eprintln_response_creation_token_stream
                                            },
                                        }
                                    }
                                    if let false = bind_increments.is_empty() {
                                        bind_increments.pop();
                                    }
                                    bind_increments
                                };
                                additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
                            }
                        }
                    });
                    let handle_token_stream = {
                        let handle_stringified = format!("\"{select_snake_case} {{}} {from_snake_case} {table_name_stringified} {{}}\"");
                        handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let additional_parameters_order_by_handle_token_stream = {
                        let additional_parameters_order_by_handle_stringified =
                            format!("\"{{}}{order_by_snake_case} {{}} {{}}\"");
                        additional_parameters_order_by_handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_order_by_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let additional_parameters_limit_handle_token_stream = {
                        let additional_parameters_limit_handle_stringified =
                            format!("\"{{}}{limit_snake_case} {{}}\"");
                        additional_parameters_limit_handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_limit_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let additional_parameters_offset_handle_token_stream = {
                        let additional_parameters_offset_handle_stringified =
                            format!("\"{{}}{offset_snake_case} {{}}\"");
                        additional_parameters_offset_handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_offset_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &bind_query_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &bind_query_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
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
                                        Err(#error_snake_case) => {
                                            #error_initialization_eprintln_response_creation_token_stream
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
                                        Err(#error_snake_case) => {
                                            #error_initialization_eprintln_response_creation_token_stream
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
                    let binded_query_modifications_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        quote::quote!{
                            if let Some(values) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                for #value_snake_case in values {
                                    #query_snake_case = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
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
                        #query_snake_case = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case.#payload_snake_case.#limit_snake_case,
                            #query_snake_case,
                        );
                        #query_snake_case = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case.#payload_snake_case.#offset_snake_case,
                            #query_snake_case,
                        );
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = {
                    let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &postgresql_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &postgresql_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    quote::quote! {
                        let mut rows = #binded_query_snake_case.fetch(#pg_connection_snake_case.as_mut());
                        let mut vec_values = std::vec::Vec::new();
                        let #wrapper_vec_column_snake_case = #wrapper_vec_column_upper_camel_case(#parameters_snake_case.#payload_snake_case.#select_snake_case);
                        while let Some(row) = {
                            match {
                                #use_futures_try_stream_ext_token_stream;
                                rows.try_next()
                            }
                            .await
                            {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_snake_case) => {
                                    #error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        } {
                            match #wrapper_vec_column_snake_case.#options_try_from_sqlx_row_snake_case(&row) {
                                Ok(#value_snake_case) => {
                                    vec_values.push(#value_snake_case);
                                }
                                Err(#error_snake_case) => {
                                    #error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                        vec_values
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
                generate_try_operation_route_logic_snake_case_token_stream(
                    &operation,
                    &ast,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &check_body_size_syn_variant_initialization_token_stream,
                    &postgresql_syn_variant_initialization_token_stream,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_token_stream
                #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
                #try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_struct_options_ident_token_stream,
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match fields_named_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                        #value_snake_case
                        .into_iter()
                        .map(|#element_snake_case| #struct_options_ident_token_stream::#from_snake_case(#element_snake_case))
                        .collect()
                    },
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            {
                                let mut values = std::vec::Vec::new();
                                for #element_snake_case in #value_snake_case {
                                    values.push(#element_snake_case);
                                }
                                values
                            }
                        }
                    }
                },
                &proc_macro_name_upper_camel_case_ident_stringified,
                &reqwest_syn_variant_initialization_token_stream,
                &deserialize_response_syn_variant_initialization_token_stream,
                &failed_to_get_response_text_syn_variant_initialization_token_stream,
                &serde_json_to_string_syn_variant_initialization_token_stream,
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
        // let (
        //     operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant,
        //     operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant_initialization_token_stream,
        //     operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant_status_code
        // ) = {
        //     let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        //     let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream = naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(&operation);
        //     let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::BadRequest400;
        //     (
        //         proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
        //             operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant_status_code.clone(),
        //             &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
        //             vec![(
        //                 proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
        //                 &operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream,
        //                 proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
        //                     &[&naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_stringified(&operation)],
        //                     &proc_macro_name_upper_camel_case_ident_stringified
        //                 ),
        //             )],
        //             &proc_macro_name_upper_camel_case_ident_stringified,
        //         ),
        //         quote::quote! {},
        //         operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant_status_code
        //     )
        // };
        let (
            operation_payload_with_serialize_deserialize_try_from_operation_payload_syn_variant,
            operation_payload_with_serialize_deserialize_try_from_operation_payload_syn_variant_initialization_token_stream,
            operation_payload_with_serialize_deserialize_try_from_operation_payload_syn_variant_status_code
        ) = {
            let operation_payload_with_serialize_deserialize_try_from_operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeTryFromSelfPayloadUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_try_from_self_payload_upper_camel_case_token_stream(&operation);
            let operation_payload_with_serialize_deserialize_try_from_operation_payload_snake_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeTryFromSelfPayloadSnakeCaseTokenStream::self_payload_with_serialize_deserialize_try_from_self_payload_snake_case_token_stream(&operation);
            let operation_payload_with_serialize_deserialize_try_from_operation_payload_syn_variant_status_code = proc_macro_helpers::status_code::StatusCode::BadRequest400;
            (
                proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
                    operation_payload_with_serialize_deserialize_try_from_operation_payload_syn_variant_status_code.clone(),
                    &operation_payload_with_serialize_deserialize_try_from_operation_payload_upper_camel_case_token_stream,
                    vec![(
                        proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
                        &operation_payload_with_serialize_deserialize_try_from_operation_payload_snake_case_token_stream,
                        proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                            &[&naming_conventions::SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseStringified::self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_stringified(&operation)],
                            &proc_macro_name_upper_camel_case_ident_stringified
                        ),
                    )],
                    &proc_macro_name_upper_camel_case_ident_stringified,
                ),
                {
                    let field_code_occurence_new_0f1b116c_31a3_4bbd_bf7d_6890030cb363_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        #operation_payload_with_serialize_deserialize_try_from_operation_payload_upper_camel_case_token_stream {
                            #operation_payload_with_serialize_deserialize_try_from_operation_payload_snake_case_token_stream: #error_snake_case,
                            #field_code_occurence_new_0f1b116c_31a3_4bbd_bf7d_6890030cb363_token_stream
                        }
                    }
                },
                operation_payload_with_serialize_deserialize_try_from_operation_payload_syn_variant_status_code
            )
        };
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len());
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value
            },
            &postgresql_crud_common::FromOrTryFrom::TryFrom,//fields_named_excluding_primary_key_from_or_try_from
            &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant,
            &operation,
            &ast,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // let type_variants_from_request_response_syn_variants = {
        //     let full_additional_http_status_codes_error_variants =
        //         generate_full_additional_http_status_codes_error_variants(
        //             common_middlewares_error_syn_variants.iter().collect(),
        //             additional_http_status_codes_error_variants.iter().collect(),
        //         );
        //     let type_variants_from_request_response_syn_variants_partial = {
        //         let mut type_variants_from_request_response =
        //             std::vec::Vec::with_capacity(common_error_syn_variants.len().checked_add(1).unwrap());
        //         for element in &common_error_syn_variants {
        //             type_variants_from_request_response.push(element);
        //         }
        //         // if let postgresql_crud_common::FromOrTryFrom::TryFrom = primary_key_from_or_try_from {
        //             type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
        //         // }
        //         type_variants_from_request_response
        //     };
        //     generate_type_variants_from_request_response_syn_variants(
        //         &type_variants_from_request_response_syn_variants_partial,
        //         &full_additional_http_status_codes_error_variants,
        //     )
        // };
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
            let select_assignment_token_stream = {
                quote::quote! {
                    let #select_snake_case = {
                        let mut vec = std::vec::Vec::with_capacity(#field_named_len_token_stream);
                        for #element_snake_case in #value_snake_case.#select_snake_case {
                            if vec.contains(&#element_snake_case) {
                                return Err(Self::Error::#not_unique_column_syn_variant_initialization_token_stream);
                            }
                            else {
                                vec.push(#element_snake_case);
                            }
                        }
                        vec
                    };
                }
            };
            let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                    &operation,
                    &{
                        let primary_key_variant_token_stream = match &primary_key_from_or_try_from {
                            postgresql_crud_common::FromOrTryFrom::From => proc_macro2::TokenStream::new(),
                            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                                let with_serialize_deserialize_error_named_token_stream = {
                                    let value = primary_key_syn_field.rust_sqlx_map_to_postgres_type_variant.get_inner_type_with_serialize_deserialize_error_named_stringified("");
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                let primary_key_field_ident_upper_camel_case_token_stream = {
                                    //todo its a temporal naming desicion. maybe it can be better
                                    let value = syn_ident_to_upper_camel_case_stringified(primary_key_syn_field.field_ident);
                                    value.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                };
                                quote::quote!{
                                    #primary_key_field_ident_upper_camel_case_token_stream {//todo reuse
                                        #eo_error_occurence_attribute_token_stream
                                        #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: #with_serialize_deserialize_error_named_token_stream,
                                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                                    },//must use comma here
                                }
                            }
                        };
                        let not_unique_column_syn_variant_token_stream = generate_error_occurence_variant_token_stream(
                            &not_unique_column_syn_variant,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            #primary_key_variant_token_stream
                            #not_unique_column_syn_variant_token_stream,
                        }
                    },
                );
                // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = 
                proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
                    &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                    &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                    &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation),
                    &{
                        let primary_key_let_field_ident_value_field_ident_from_or_try_from_token_stream = match &primary_key_from_or_try_from {
                            postgresql_crud_common::FromOrTryFrom::From => quote::quote! {
                                let #primary_key_field_ident = #primary_key_inner_type_token_stream::#from_snake_case(value.#primary_key_field_ident);
                            },
                            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                                let field_code_occurence_new_3b778bbe_aec5_4ebe_af05_11074800c690_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                    file!(),
                                    line!(),
                                    column!(),
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                generate_let_field_ident_value_field_ident_try_from_token_stream(
                                    &primary_key_syn_field,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                    &field_code_occurence_new_3b778bbe_aec5_4ebe_af05_11074800c690_token_stream,
                                    &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                )
                            },
                        };
                        quote::quote! {
                            #primary_key_let_field_ident_value_field_ident_from_or_try_from_token_stream
                            #select_assignment_token_stream
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
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_try_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let operation_payload_with_serialize_deserialize_try_from_operation_payload_error_named_token_stream = generate_operation_payload_with_serialize_deserialize_try_from_operation_payload_error_named_token_stream(
                    &operation,
                    &{
                        let not_unique_column_syn_variant_token_stream = generate_error_occurence_variant_token_stream(
                            &not_unique_column_syn_variant,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            #not_unique_column_syn_variant_token_stream
                        }
                    }
                );
                // println!("{operation_payload_with_serialize_deserialize_try_from_operation_payload_error_named_token_stream}");
                let impl_std_convert_try_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
                    &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                    &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                    &naming_conventions::SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_token_stream(&operation),
                    &quote::quote! {
                        let #primary_key_field_ident = #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#value_snake_case.#primary_key_field_ident);
                        #select_assignment_token_stream
                        Ok(Self {
                            #primary_key_field_ident,
                            #select_snake_case,
                        })
                    },
                );
                quote::quote! {
                    #operation_payload_with_serialize_deserialize_try_from_operation_payload_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream
                }
            };
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            generate_parameters_pattern_token_stream(
                &operation,
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream,
                impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream,
                impl_std_convert_try_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream
            )
        };
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_token_stream = generate_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &struct_options_ident_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = generate_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_error_named_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &postgresql_crud_common::FromOrTryFrom::TryFrom,
                   &json_syn_variant_initialization_token_stream,
                   &json_syn_variant_status_code,
                   &eprintln_error_token_stream,
                   &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let query_string_token_stream = {
                    let query_token_stream = {
                        let query_stringified = format!("\"{select_snake_case} {{}} {from_snake_case} {table_name_stringified} {where_snake_case} {primary_key_field_ident} = $1\"");
                        query_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
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
                        let #query_snake_case = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case.#payload_snake_case.#primary_key_field_ident, #query_snake_case);
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_modifications_token_stream
                        #query_snake_case
                    }
                };
                // println!("{binded_query_token_stream}");
                let postgresql_logic_token_stream = {
                    let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &postgresql_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &postgresql_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    quote::quote!{
                        match #binded_query_snake_case.fetch_one(#pg_connection_snake_case.as_mut()).await {
                            Ok(row) => match #wrapper_vec_column_upper_camel_case(#parameters_snake_case.#payload_snake_case.#select_snake_case).#options_try_from_sqlx_row_snake_case(&row) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_snake_case) => {
                                    #error_initialization_eprintln_response_creation_token_stream
                                },
                            },
                            Err(#error_snake_case) => {
                                #error_initialization_eprintln_response_creation_token_stream
                            },
                        }
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
                generate_try_operation_route_logic_snake_case_token_stream(
                    &operation,
                    &ast,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &check_body_size_syn_variant_initialization_token_stream,
                    &postgresql_syn_variant_initialization_token_stream,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_token_stream
                #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
                #try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &{
                    let mut value = common_http_request_syn_variants.clone();
                    value.push(operation_payload_with_serialize_deserialize_try_from_operation_payload_syn_variant);
                    value
                },
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &struct_options_ident_token_stream,
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                    quote::quote!{
                        match #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#try_from_snake_case(
                            #parameters_snake_case.#payload_snake_case
                        ) {
                            Ok(#value_snake_case) => #value_snake_case,
                            Err(#error_snake_case) => {
                                return Err(#try_operation_error_named_upper_camel_case_token_stream::#operation_payload_with_serialize_deserialize_try_from_operation_payload_syn_variant_initialization_token_stream);
                            }
                        }
                    }
                },
                &match fields_named_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                        #struct_options_ident_token_stream::#from_snake_case(#value_snake_case)
                    },
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            {
                                #value_snake_case
                            }
                        }
                    }
                },
                &proc_macro_name_upper_camel_case_ident_stringified,
                &reqwest_syn_variant_initialization_token_stream,
                &deserialize_response_syn_variant_initialization_token_stream,
                &failed_to_get_response_text_syn_variant_initialization_token_stream,
                &serde_json_to_string_syn_variant_initialization_token_stream,
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
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len());
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                // value.push(postgresql_syn_variant);
                value.push(&query_and_rollback_failed_syn_variant);
                value.push(&primary_key_from_row_and_failed_rollback_syn_variant);
                value.push(&non_existing_primary_keys_syn_variant);
                value.push(&non_existing_primary_keys_and_failed_rollback_syn_variant);
                value.push(&commit_failed_syn_variant);
                //
                value
            },
            &fields_named_excluding_primary_key_from_or_try_from,
            &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant,
            &operation,
            &ast,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let (
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream
            ) = generate_payload_and_payload_with_serialize_deserialize_create_many_or_update_many(
                &operation,
                &generate_fields_named_token_stream(generate_pub_field_ident_field_type_token_stream),
                &generate_fields_named_token_stream(generate_field_ident_field_type_with_serialize_deserialize_token_stream),
            );
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let impl_std_convert_from_or_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream = match fields_named_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                        &naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation),
                        &{
                            let primary_key_let_field_ident_value_field_ident_from_token_stream = generate_let_field_ident_value_inner_type_from_token_stream(&primary_key_syn_field);
                            let fields_assignments_token_stream = fields_named_excluding_primary_key.iter().map(generate_let_field_ident_value_inner_type_from_token_stream);
                            let self_init_fields_token_stream = generate_self_fields_token_stream(
                                &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
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
                        let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream(
                            &operation,
                            &fields_named_excluding_primary_key,
                            &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                        );
                        // println!("{operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream}");
                        let impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream = 
                        proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
                            &naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                            &naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation),
                            &naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation),
                            &{
                                let field_code_occurence_new_77f303a5_de96_4f73_a274_f2195cb619b1_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                    file!(),
                                    line!(),
                                    column!(),
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                let fields_assignments_token_stream = fields_named_excluding_primary_key.iter()
                                    .map(|element| generate_let_field_ident_value_field_ident_try_from_token_stream(
                                        element,
                                        &proc_macro_name_upper_camel_case_ident_stringified,
                                        &field_code_occurence_new_77f303a5_de96_4f73_a274_f2195cb619b1_token_stream,
                                        &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                    ));
                                let self_init_fields_token_stream = generate_self_fields_token_stream(
                                    &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                let field_code_occurence_new_814b41f8_3219_4b62_bc0b_02a26d23b262_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                    file!(),
                                    line!(),
                                    column!(),
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                );
                                let primary_key_let_field_ident_value_field_ident_try_from_token_stream = generate_let_field_ident_value_field_ident_try_from_token_stream(
                                    &primary_key_syn_field,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                    &field_code_occurence_new_814b41f8_3219_4b62_bc0b_02a26d23b262_token_stream,
                                    &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                );
                                quote::quote! {
                                    #primary_key_let_field_ident_value_field_ident_try_from_token_stream
                                    #(#fields_assignments_token_stream)*
                                    Ok(Self{
                                        #(#self_init_fields_token_stream),*
                                    })
                                }
                            },
                        );
                        // println!("{impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream}");
                        quote::quote! {
                            #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream
                            #impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream
                        }
                    }
                };
                let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = match fields_named_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                        &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                        &{
                            let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation);
                            quote::quote! {
                                Self (
                                    value.0.into_iter()//todo rewrite as try_from
                                    .map(|element|#operation_payload_element_upper_camel_case_token_stream::#from_snake_case(element))
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
                                        &primary_key_syn_field,
                                        &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                    );
                                    let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                                        &fields_named_excluding_primary_key,
                                        &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
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
                                    let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                                    let primary_key_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream =     generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
                                        &primary_key_syn_field,
                                        &operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream,
                                        &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                    );
                                    let fields_named_excluding_primary_key_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = fields_named_excluding_primary_key.iter().map(|element|
                                        generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
                                            element,
                                            &operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream,
                                            &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                        )
                                    );
                                    quote::quote! {
                                        match value {
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
                        let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_upper_camel_case_token_stream = proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
                            &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                            &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                            &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation),
                            &{
                                let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation);
                                let std_vec_vec_operation_payload_element_token_stream = operation.std_vec_vec_self_payload_element_token_stream();
                                let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                                quote::quote! {
                                    match value.0.into_iter()//todo rewrite as try_from
                                        .map(|element|#operation_payload_element_upper_camel_case_token_stream::try_from(element))
                                        .collect::<Result<
                                            #std_vec_vec_operation_payload_element_token_stream,
                                            #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream
                                        >>()
                                    {
                                        Ok(value) => Ok(Self(value)),
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
                quote::quote! {
                    #impl_std_convert_from_or_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream
                    #impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream = proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                    &naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation),
                    &naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                    &{
                        let fields_assignments_token_stream = fields_named_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_inner_type_with_serialize_deserialize_from_token_stream(element));
                        let self_init_fields_token_stream = generate_self_fields_token_stream(
                            &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            let #primary_key_field_ident = #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(value.#primary_key_field_ident);
                            #(#fields_assignments_token_stream)*
                            Self {
                                #(#self_init_fields_token_stream),*
                            }
                        }
                    },
                );
                // println!("{impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream}");
                let impl_std_convert_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                    &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                    &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
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
                // println!("{impl_std_convert_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream}");
                quote::quote! {
                    #impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream
                    #impl_std_convert_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream
                }
            };
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
        // let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
        //     proc_macro_helpers::status_code::StatusCode::BadRequest400,
        //     &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
        //     vec![
        //         (
        //             proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
        //             &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
        //             naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        //                 &operation
        //             )
        //         )
        //     ],
        //     &proc_macro_name_upper_camel_case_ident_stringified,
        // );
        // let type_variants_from_request_response_syn_variants = {
        //     let full_additional_http_status_codes_error_variants =
        //         generate_full_additional_http_status_codes_error_variants(
        //             common_middlewares_error_syn_variants.iter().collect(),
        //             additional_http_status_codes_error_variants.iter().collect(),
        //         );
        //     let type_variants_from_request_response_syn_variants_partial = {
        //         let mut type_variants_from_request_response =
        //             std::vec::Vec::with_capacity(common_error_syn_variants.len().checked_add(10).unwrap());
        //         for element in &common_error_syn_variants {
        //             type_variants_from_request_response.push(element);
        //         }
        //         type_variants_from_request_response.push(&not_unique_primary_keys_syn_variant);
        //         type_variants_from_request_response.push(&bind_query_syn_variant);
        //         type_variants_from_request_response.push(&checked_add_syn_variant);
        //         type_variants_from_request_response.push(&no_payload_fields_syn_variant);
        //         type_variants_from_request_response.push(&commit_failed_syn_variant);
        //         type_variants_from_request_response.push(&non_existing_primary_keys_syn_variant);
        //         type_variants_from_request_response
        //             .push(&primary_key_from_row_and_failed_rollback_syn_variant);
        //         type_variants_from_request_response
        //             .push(&non_existing_primary_keys_and_failed_rollback_syn_variant);
        //         type_variants_from_request_response.push(&query_and_rollback_failed_syn_variant);
        //         if fields_named_from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
        //             type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
        //         }
        //         type_variants_from_request_response
        //     };
        //     generate_type_variants_from_request_response_syn_variants(
        //         &type_variants_from_request_response_syn_variants_partial,
        //         &full_additional_http_status_codes_error_variants,
        //     )
        // };
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_token_stream = generate_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &std_vec_vec_primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = generate_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_error_named_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &fields_named_excluding_primary_key_from_or_try_from,
                   &json_syn_variant_initialization_token_stream,
                   &json_syn_variant_status_code,
                   &eprintln_error_token_stream,
                   &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let expected_updated_primary_keys_name_token_stream = quote::quote! {expected_updated_primary_keys};
                let expected_updated_primary_keys_token_stream = {
                   quote::quote! {
                       let #expected_updated_primary_keys_name_token_stream = #parameters_snake_case
                       .#payload_snake_case
                       .0
                       .iter()
                       .map(|#element_snake_case| #element_snake_case.#primary_key_field_ident.clone()) //todo - maybe its not a good idea to remove .clone here coz in macro dont know what type
                       .collect::<std::vec::Vec<#primary_key_inner_type_token_stream>>();
                   }
                };
                let query_string_token_stream = {
                    let column_names = fields_named.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, element)| {
                        let possible_dot_space = if (
                            index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE))
                        ) == fields_named_len {
                            ""
                        }
                        else {
                            dot_space
                        };
                        acc.push_str(&format!("{}{possible_dot_space}", &element.field_ident));
                        acc
                    });
                    let column_increments = fields_named.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, _)| {
                        let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                        let possible_dot_space = if (incremented_index) == fields_named_len {
                            ""
                        }
                        else {
                            dot_space
                        };
                        acc.push_str(&format!("${incremented_index}{possible_dot_space}"));
                        acc
                    });
                    let declarations = fields_named_excluding_primary_key.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, element)| {
                        let field_ident = &element.field_ident;
                        let possible_dot_space = if (
                            index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE))
                        ) == fields_named_excluding_primary_key_len {
                            ""
                        }
                        else {
                            dot_space
                        };
                        acc.push_str(&format!("{field_ident} = data.{field_ident}{possible_dot_space}"));
                        acc
                    });
                    let query_stringified = format!("\"{update_snake_case} {table_name_stringified} {as_snake_case} t {set_snake_case} {declarations} {from_snake_case} ({select_snake_case} * {from_snake_case} {unnest_snake_case}({column_increments})) as data({column_names}) where t.{primary_key_field_ident} = data.{primary_key_field_ident} {returning_snake_case} data.{primary_key_field_ident}\"");
                    query_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    //todo remove () if in fields named only one element
                    let column_vecs_token_stream = fields_named.iter().map(|element|{
                        let field_ident_underscore_vec_stringified = format!("{}_{}", &element.field_ident, naming_constants::VecSnakeCase);
                        field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    });
                    let column_vecs_with_capacity_token_stream = fields_named.iter().map(|_|quote::quote!{std::vec::Vec::with_capacity(#current_vec_len_snake_case)});
                    let columns_acc_push_elements_token_stream = fields_named.iter().enumerate().map(|(index, element)|{
                        let field_ident = &element.field_ident;
                        let index_token_stream = {
                            let index_stringified = format!("{index}");
                            index_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{#acc_snake_case.#index_token_stream.push(#element_snake_case.#field_ident);}
                    });
                    let column_query_bind_primary_key_vec_token_stream = {
                        let field_ident_underscore_vec_token_stream = {
                            let field_ident_underscore_vec_stringified = format!(
                                "{primary_key_field_ident}_{}", naming_constants::VecSnakeCase
                            );
                            field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote! {
                            #query_snake_case = #query_snake_case.bind(
                                #field_ident_underscore_vec_token_stream
                                .into_iter()
                                .map(|#element_snake_case| #element_snake_case.into_inner())
                                .collect::<std::vec::Vec<#primary_key_original_type_token_stream>>()
                            );
                        }
                    };
                    let column_query_bind_vecs_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident_underscore_vec_token_stream = {
                            let field_ident_underscore_vec_stringified = {
                                let field_ident = &element.field_ident;
                                format!("{field_ident}_{}", naming_constants::VecSnakeCase)
                            };
                            field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let original_type_token_stream = &element.original_type_token_stream;
                        quote::quote!{
                            #query_snake_case = #query_snake_case.bind(
                                #field_ident_underscore_vec_token_stream
                                    .into_iter()
                                    .map(|element| element.into_inner())
                                    .collect::<std::vec::Vec<#original_type_token_stream>>(),
                            );
                        }
                    });
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        let #current_vec_len_snake_case = #parameters_snake_case.#payload_snake_case.0.len();
                        let (
                            #(#column_vecs_token_stream),*
                        ) = #parameters_snake_case.#payload_snake_case.0.into_iter().fold((
                            #(#column_vecs_with_capacity_token_stream),*
                        ), |mut #acc_snake_case, #element_snake_case| {
                            #(#columns_acc_push_elements_token_stream)*
                            #acc_snake_case
                        });
                        #column_query_bind_primary_key_vec_token_stream
                        #(#column_query_bind_vecs_token_stream)*
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = {
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &postgresql_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &postgresql_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    let query_and_rollback_failed_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &query_and_rollback_failed_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &query_and_rollback_failed_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    let primary_key_from_row_and_failed_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &primary_key_from_row_and_failed_rollback_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &primary_key_from_row_and_failed_rollback_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    let non_existing_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &non_existing_primary_keys_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &non_existing_primary_keys_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    let non_existing_primary_keys_and_failed_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &non_existing_primary_keys_and_failed_rollback_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &non_existing_primary_keys_and_failed_rollback_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    let commit_failed_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &commit_failed_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &commit_failed_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    let postgres_transaction_token_stream = quote::quote! {postgres_transaction};
                    let primary_key_vec_name_token_stream = quote::quote! {primary_key_vec};
                    let sqlx_acquire = token_patterns::SqlxAcquire;
                    let begin_snake_case = naming_constants::BeginSnakeCase;
                    let binded_query_snake_case = naming_conventions::BindedQuerySnakeCase;
                    let rollback_error_snake_case = naming_conventions::RollbackErrorSnakeCase;
                    let non_existing_primary_keys_snake_case = naming_conventions::NonExistingPrimaryKeysSnakeCase;
                    let commit_snake_case = naming_constants::CommitSnakeCase;
                    quote::quote! {
                        let mut #postgres_transaction_token_stream = match {
                            use #sqlx_acquire;
                            #pg_connection_snake_case.#begin_snake_case()
                        }
                        .await
                        {
                            Ok(#value_snake_case) => #value_snake_case,
                            Err(#error_snake_case) => {
                                #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            }
                        };
                        let results_vec = {
                            let mut results_vec = std::vec::Vec::with_capacity(#expected_updated_primary_keys_name_token_stream.len());
                            let mut option_error: Option<sqlx::Error> = None;
                            {
                                let mut rows = #binded_query_snake_case.fetch(#postgres_transaction_token_stream.as_mut());
                                while let (Some(Some(row)), None) = (
                                    match {
                                        #use_futures_try_stream_ext_token_stream;
                                        rows.try_next()
                                    }
                                    .await
                                    {
                                        Ok(#value_snake_case) => Some(#value_snake_case),
                                        Err(#error_snake_case) => {
                                            option_error = Some(#error_snake_case);
                                            None
                                        }
                                    },
                                    &option_error,
                                ) {
                                    results_vec.push(row);
                                }
                            }
                            if let Some(#error_snake_case) = option_error {
                                match #postgres_transaction_token_stream.#rollback_snake_case().await {
                                    Ok(_) => {
                                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                    Err(#rollback_error_snake_case) => {
                                        //todo  BIG QUESTION - WHAT TO DO IF ROLLBACK FAILED? INFINITE LOOP TRYING TO ROLLBACK?
                                        #query_and_rollback_failed_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                            results_vec
                        };
                        let #primary_key_vec_name_token_stream = {
                            let mut #primary_key_vec_name_token_stream = std::vec::Vec::with_capacity(#expected_updated_primary_keys_name_token_stream.len());
                            for #element_snake_case in results_vec {
                                match #primary_key_try_from_sqlx_row_snake_case(&#element_snake_case) {
                                    Ok(primary_key) => {
                                        #primary_key_vec_name_token_stream.push(primary_key);
                                    }
                                    Err(#error_snake_case) => match #postgres_transaction_token_stream.#rollback_snake_case().await {
                                        Ok(_) => {
                                            #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        }
                                        Err(#rollback_error_snake_case) => {
                                            #primary_key_from_row_and_failed_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        }
                                    },
                                }
                            }
                            #primary_key_vec_name_token_stream
                        };
                        {
                            let #non_existing_primary_keys_snake_case = {
                                let len = #expected_updated_primary_keys_name_token_stream.len();
                                #expected_updated_primary_keys_name_token_stream.into_iter().fold(std::vec::Vec::with_capacity(len), |mut acc, #element_snake_case| {
                                    if let false = #primary_key_vec_name_token_stream.contains(&#element_snake_case) {
                                        acc.push(#element_snake_case);
                                    }
                                    acc
                                })
                            };
                            if let false = #non_existing_primary_keys_snake_case.is_empty() {
                                match #postgres_transaction_token_stream.#rollback_snake_case().await {
                                    Ok(_) => {
                                        #non_existing_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                    Err(#error_snake_case) => {
                                        #non_existing_primary_keys_and_failed_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                        }
                        match #postgres_transaction_token_stream.#commit_snake_case().await {
                            Ok(_) => #primary_key_vec_name_token_stream.into_iter().map(
                                |#element_snake_case|#primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#element_snake_case)
                            ).collect(),
                            Err(#error_snake_case) => {
                                #commit_failed_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            }
                        }
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
                generate_try_operation_route_logic_snake_case_token_stream(
                    &operation,
                    &ast,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &expected_updated_primary_keys_token_stream,
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &check_body_size_syn_variant_initialization_token_stream,
                    &postgresql_syn_variant_initialization_token_stream,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_token_stream
                #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
                #try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &quote::quote!{std::vec::Vec<#primary_key_inner_type_token_stream>},
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match primary_key_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                        #value_snake_case
                        .into_iter()
                        .map(|#element_snake_case| #primary_key_inner_type_token_stream::#from_snake_case(#element_snake_case))
                        .collect()
                    },
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            {
                                let mut values = std::vec::Vec::new();
                                for #element_snake_case in #value_snake_case {
                                    match #primary_key_inner_type_token_stream::#try_from_snake_case(#element_snake_case) {
                                        Ok(#value_snake_case) => {
                                            values.push(#value_snake_case);
                                        },
                                        Err(#error_snake_case) => Err(
                                            #try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream
                                        )
                                    }
                                }
                                values
                            }
                        }
                    }
                },
                &proc_macro_name_upper_camel_case_ident_stringified,
                &reqwest_syn_variant_initialization_token_stream,
                &deserialize_response_syn_variant_initialization_token_stream,
                &failed_to_get_response_text_syn_variant_initialization_token_stream,
                &serde_json_to_string_syn_variant_initialization_token_stream,
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
        // let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
        //     proc_macro_helpers::status_code::StatusCode::BadRequest400,
        //     &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
        //     vec![
        //         (
        //             proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
        //             &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
        //             naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        //                 &operation
        //             )
        //         )
        //     ],
        //     &proc_macro_name_upper_camel_case_ident_stringified,
        // );
        // let type_variants_from_request_response_syn_variants = {
        //     let full_additional_http_status_codes_error_variants =
        //         generate_full_additional_http_status_codes_error_variants(
        //             common_middlewares_error_syn_variants.iter().collect(),
        //             additional_http_status_codes_error_variants.iter().collect(),
        //         );
        //     let type_variants_from_request_response_syn_variants_partial = {
        //         let mut type_variants_from_request_response =
        //             std::vec::Vec::with_capacity(common_error_syn_variants.len().checked_add(4).unwrap());
        //         for element in &common_error_syn_variants {
        //             type_variants_from_request_response.push(element);
        //         }
        //         type_variants_from_request_response.push(&bind_query_syn_variant);
        //         type_variants_from_request_response.push(&no_payload_fields_syn_variant);
        //         if fields_named_from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
        //             type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
        //         }
        //         type_variants_from_request_response.push(&operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant);
        //         type_variants_from_request_response
        //     };
        //     generate_type_variants_from_request_response_syn_variants(
        //         &type_variants_from_request_response_syn_variants_partial,
        //         &full_additional_http_status_codes_error_variants,
        //     )
        // };
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 1);
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value.push(&bind_query_syn_variant);
                value
            },
            &fields_named_excluding_primary_key_from_or_try_from,
            &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant,
            &operation,
            &ast,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let (
                payload_token_stream,
                payload_with_serialize_deserialize_token_stream
            ) = generate_payload_and_payload_with_serialize_deserialize_one(&operation, false);
            let impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = match fields_named_from_or_try_from {
                postgresql_crud_common::FromOrTryFrom::From => proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                    &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                    &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                    &{
                        let primary_key_let_field_ident_value_field_ident_from_token_stream = generate_let_field_ident_value_inner_type_from_token_stream(&primary_key_syn_field);
                        let fields_assignment_token_stream = fields_named.iter()
                        .map(|element| {
                            let field_ident = &element.field_ident;
                            let inner_type_token_stream = &element.inner_type_token_stream;
                            quote::quote!{
                                let #field_ident = #inner_type_token_stream::#from_snake_case(value.#field_ident);
                            }
                        });
                        let self_init_fields_token_stream = generate_self_fields_token_stream(
                            &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            #primary_key_let_field_ident_value_field_ident_from_token_stream
                            #(#fields_assignment_token_stream)*
                            Self {
                                #(#self_init_fields_token_stream),*
                            }
                        }
                    },
                ),
                postgresql_crud_common::FromOrTryFrom::TryFrom => {
                    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                        &operation,
                        &{
                            let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                                &fields_named,
                                &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                            );
                            quote::quote! {#(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*}
                        },
                    );
                    // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = 
                    proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
                        &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation),
                        &{
                            let field_code_occurence_new_3763990f_5c49_47d0_a774_5ef584cd1236_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                file!(),
                                line!(),
                                column!(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            let fields_assignment_token_stream = fields_named.iter()
                            .map(|element| generate_let_field_ident_value_field_ident_try_from_token_stream(
                                element,
                                &proc_macro_name_upper_camel_case_ident_stringified,
                                &field_code_occurence_new_3763990f_5c49_47d0_a774_5ef584cd1236_token_stream,
                                &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                            ));
                            let self_init_fields_token_stream = generate_self_fields_token_stream(
                                &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            quote::quote! {
                                #(#fields_assignment_token_stream)*
                                Ok(Self {
                                    #(#self_init_fields_token_stream),*
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
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                &{
                    let fields_assignment_token_stream = fields_named.iter()
                        .map(|element|generate_let_field_ident_value_inner_type_with_serialize_deserialize_from_token_stream(element));
                    let self_init_fields_token_stream = generate_self_fields_token_stream(
                        &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        #(#fields_assignment_token_stream)*
                        Self{
                            #(#self_init_fields_token_stream),*
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
            let try_operation_route_logic_response_variants_token_stream = generate_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = generate_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_error_named_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &fields_named_from_or_try_from,
                   &json_syn_variant_initialization_token_stream,
                   &json_syn_variant_status_code,
                   &eprintln_error_token_stream,
                   &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let query_string_token_stream = {
                    let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &bind_query_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &bind_query_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    let additional_parameters_modification_token_stream = {
                        fields_named_excluding_primary_key.iter().enumerate().map(|(index, element)| {
                            let field_ident = &element.field_ident;
                            let handle_token_stream = {
                                let possible_dot_space = if (
                                    index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE))
                                ) == fields_named_excluding_primary_key_len {
                                    ""
                                }
                                else {
                                    dot_space
                                };
                                let handle_stringified = format!("\"{field_ident} = ${{increment}}{possible_dot_space}\"");
                                handle_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            quote::quote!{
                                if let Some(_) = &#parameters_snake_case.#payload_snake_case.#field_ident.0 {
                                    match #crate_server_postgres_bind_query_bind_query_try_increment_token_stream(&#parameters_snake_case.#payload_snake_case.#field_ident, &mut increment) {
                                        Ok(_) => {
                                            #query_snake_case.push_str(&format!(#handle_token_stream));//add dot_space for all elements except last
                                        },
                                        Err(#error_snake_case) => {
                                            #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        },
                                    }
                                }
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>()
                    };
                    let additional_parameters_primary_key_modification_token_stream = {
                        let query_part_token_stream = {
                            let query_part_stringified = format!("\" {where_snake_case} {primary_key_field_ident} = ${{increment}}\""); //todo where
                            query_part_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_part_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote! {
                            match #crate_server_postgres_bind_query_bind_query_try_increment_token_stream(&#parameters_snake_case.#payload_snake_case.#primary_key_field_ident, &mut increment) {
                                Ok(_) => {
                                    #query_snake_case.push_str(&format!(#query_part_token_stream));
                                },
                                Err(#error_snake_case) => {
                                    #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                },
                            }
                        }
                    };
                    let handle_token_stream = {
                        let handle_stringified = format!("\"{update_snake_case} {table_name_stringified} {set_snake_case} \""); //todo where
                        handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote! {
                        {
                            #increment_initialization_token_stream
                            let mut #query_snake_case = #std_string_string::#from_snake_case(#handle_token_stream);
                            #(#additional_parameters_modification_token_stream)*
                            #additional_parameters_primary_key_modification_token_stream
                            #query_snake_case.push_str(&format!(#returning_primary_key_quotes_token_stream));
                            #query_snake_case
                        }
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        quote::quote!{
                            if let Some(_) = #parameters_snake_case.#payload_snake_case.#field_ident.0 {
                                #query_snake_case = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
                                    #parameters_snake_case.#payload_snake_case.#field_ident,
                                    #query_snake_case,
                                );
                            }
                        }
                    });
                    let binded_query_primary_key_modification_token_stream = quote::quote! {
                        #query_snake_case = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
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
                let postgresql_logic_token_stream = {
                    let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &postgresql_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &postgresql_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    quote::quote! {
                        match #binded_query_snake_case
                            .fetch_one(#pg_connection_snake_case.as_mut())
                            .await
                        {
                            Ok(#value_snake_case) => match #sqlx_row::try_get::<#primary_key_original_type_token_stream, &std::primitive::str>(&#value_snake_case, #primary_key_field_ident_quotes_token_stream) {
                                Ok(#value_snake_case) => #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(
                                    #primary_key_inner_type_token_stream(#value_snake_case)
                                ),
                                Err(#error_snake_case) => {
                                    #operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            },
                            Err(#error_snake_case) => {
                                #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            }
                        }
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
                generate_try_operation_route_logic_snake_case_token_stream(
                    &operation,
                    &ast,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &check_body_size_syn_variant_initialization_token_stream,
                    &postgresql_syn_variant_initialization_token_stream,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_token_stream
                #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
                #try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!(" {try_operation_route_logic_token_stream}");
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &primary_key_inner_type_token_stream,
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match primary_key_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{#primary_key_inner_type_token_stream::#from_snake_case(#value_snake_case)},
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            match #primary_key_inner_type_token_stream::#try_from_snake_case(#value_snake_case) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_snake_case) => Err(
                                    #try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream
                                )
                            }
                        }
                    }
                },
                &proc_macro_name_upper_camel_case_ident_stringified,
                &reqwest_syn_variant_initialization_token_stream,
                &deserialize_response_syn_variant_initialization_token_stream,
                &failed_to_get_response_text_syn_variant_initialization_token_stream,
                &serde_json_to_string_syn_variant_initialization_token_stream,
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
        // let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
        //     proc_macro_helpers::status_code::StatusCode::BadRequest400,
        //     &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
        //     vec![
        //         (
        //             proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
        //             &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
        //             naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        //                 &operation
        //             )
        //         )
        //     ],
        //     &proc_macro_name_upper_camel_case_ident_stringified,
        // );
        // let type_variants_from_request_response_syn_variants = {
        //     let full_additional_http_status_codes_error_variants =
        //         generate_full_additional_http_status_codes_error_variants(
        //             common_middlewares_error_syn_variants.iter().collect(),
        //             additional_http_status_codes_error_variants.iter().collect(),
        //         );
        //     let type_variants_from_request_response_syn_variants_partial = {
        //         let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
        //             common_error_syn_variants.len().checked_add(not_unique_field_vec_syn_variants.len().checked_add(12).unwrap()).unwrap(),
        //         );
        //         for element in &common_error_syn_variants {
        //             type_variants_from_request_response.push(element);
        //         }
        //         for element in &not_unique_field_vec_syn_variants {
        //             type_variants_from_request_response.push(element);
        //         }
        //         type_variants_from_request_response.push(&not_unique_primary_keys_syn_variant);
        //         type_variants_from_request_response.push(&bind_query_syn_variant);
        //         type_variants_from_request_response.push(&no_payload_fields_syn_variant);
        //         type_variants_from_request_response.push(&no_payload_parameters_syn_variant);
        //         type_variants_from_request_response.push(&non_existing_primary_keys_syn_variant);
        //         type_variants_from_request_response
        //             .push(&non_existing_primary_keys_and_failed_rollback_syn_variant);
        //         type_variants_from_request_response
        //             .push(&primary_key_from_row_and_failed_rollback_syn_variant);
        //         type_variants_from_request_response.push(&commit_failed_syn_variant);
        //         type_variants_from_request_response.push(&query_and_rollback_failed_syn_variant);
        //         if fields_named_from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
        //             type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
        //         }
        //         type_variants_from_request_response.push(&operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant);
        //         type_variants_from_request_response
        //     };
        //     generate_type_variants_from_request_response_syn_variants(
        //         &type_variants_from_request_response_syn_variants_partial,
        //         &full_additional_http_status_codes_error_variants,
        //     )
        // };
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 1);
                common_route_syn_variants.iter().for_each(|element|{
                    value.push(*element);
                });
                value.push(&bind_query_syn_variant);
                value
            },
            &postgresql_crud_common::FromOrTryFrom::TryFrom,//fields_named_excluding_primary_key_from_or_try_from
            &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant,
            &operation,
            &ast,
            &proc_macro_name_upper_camel_case_ident_stringified,
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
                postgresql_crud_common::FromOrTryFrom::From => proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                    &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                    &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                    &{
                        let fields_assignments_token_stream = fields_named_excluding_primary_key.iter().map(|element| {
                            let field_ident = &element.field_ident;
                            let where_inner_type_token_stream = &element.where_inner_type_token_stream;
                            quote::quote! {
                                let #field_ident = match value.#field_ident {
                                    Some(value) => Some(value.into_iter().map(|element|#where_inner_type_token_stream::#from_snake_case(element)).collect()),
                                    None => None,
                                };
                            }
                        });
                        let self_init_fields_token_stream = generate_self_fields_token_stream(
                            &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        //todo maybe use where with conjuctive operator
                        let primary_key_let_field_ident_value_field_ident_from_token_stream = quote::quote! {
                            let #primary_key_field_ident = match value.#primary_key_field_ident {
                                Some(value) => Some(value.into_iter().map(|element|#primary_key_inner_type_token_stream::#from_snake_case(element)).collect()),
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
                                &primary_key_syn_field,
                                &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                            );
                            let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_where_inner_type_from_or_try_from_where_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                                &fields_named_excluding_primary_key,
                                &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                                &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                &eo_error_occurence_attribute_token_stream,
                            );
                            quote::quote! {
                                #primary_key_variant_token_stream
                                #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                            }
                        },
                    );
                    // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
                        &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation),
                        &{
                            let primary_key_let_field_ident_value_field_ident_try_from_token_stream = {
                                let inner_token_stream = quote::quote! {value.#primary_key_field_ident};
                                let initialization_token_stream = match primary_key_rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
                                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                                        match #inner_token_stream {
                                            Some(value) => Some(value.into_iter().map(|element|#primary_key_inner_type_token_stream::#from_snake_case(element)).collect()),
                                            None => None,
                                        }
                                    },
                                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                                        let field_code_occurence_new_5215e422_9693_4c9e_937e_759f477a20c7_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                            file!(),
                                            line!(),
                                            column!(),
                                            &proc_macro_name_upper_camel_case_ident_stringified,
                                        );
                                        quote::quote!{
                                            match #inner_token_stream {
                                                Some(value) => {
                                                    let mut values = std::vec::Vec::with_capacity(value.len());
                                                    for element in value {
                                                        match #primary_key_inner_type_token_stream::try_from(element) {
                                                            Ok(value) => {
                                                                values.push(value);
                                                            }
                                                            Err(error) => {
                                                                return Err(Self::Error::#primary_key_field_ident_upper_camel_case_token_stream {
                                                                    #primary_key_supported_sqlx_postgres_type_snake_case_token_stream; error,
                                                                    #field_code_occurence_new_5215e422_9693_4c9e_937e_759f477a20c7_token_stream,
                                                                });
                                                            }
                                                        }
                                                    }
                                                    Some(values)
                                                }
                                                None => None,
                                            }
                                        }
                                    }
                                };
                                quote::quote! {
                                    let #primary_key_field_ident = #initialization_token_stream;
                                }
                            };
                            let fields_assignments_token_stream = fields_named_excluding_primary_key.iter()
                                .map(|element| generate_option_vec_where_inner_type_from_or_try_from_option_vec_where_inner_type_with_serialize_deserialize_token_stream(
                                    element,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                    &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                                ));
                            let self_init_fields_token_stream = generate_self_fields_token_stream(
                                &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
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
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                &{
                    let fields_assignments_token_stream = fields_named_excluding_primary_key
                        .iter()
                        .map(|element| generate_let_field_ident_value_option_vec_with_serialize_deserialize_token_stream(element));
                    let self_init_fields_token_stream = generate_self_fields_token_stream(
                        &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        let #primary_key_field_ident = match value.#primary_key_field_ident {
                            Some(value) => Some(value.into_iter()
                                .map(|element|#primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(element))
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
            let try_operation_route_logic_response_variants_token_stream = generate_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &std_vec_vec_primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = generate_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_error_named_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_token_stream = {
            //     let parameters_match_token_stream = fields_named.iter().map(|element| {
            //         let field_ident = &element.field_ident;
            //         quote::quote!{
            //             &#parameters_snake_case.#payload_snake_case_token_stream.#field_ident
            //         }
            //     });
            //     let parameters_match_primary_key_some_other_none_token_stream = fields_named.iter().map(|element| {
            //         let field_ident = &element.field_ident;
            //         if field_ident == primary_key_field_ident {
            //             quote::quote!{Some(#primary_key_field_ident)}
            //         }
            //         else {
            //             quote::quote!{None}
            //         }
            //     });
                let parameters_logic_token_stream = {
                    // let check_for_none_token_stream = crate::check_for_none::check_for_none(
                    //     &fields_named.iter().map(|element|element.field).collect::<std::vec::Vec<&syn::Field>>(),
                    //     primary_key_field,
                    //     &proc_macro_name_upper_camel_case_ident_stringified,
                    //     dot_space,
                    //     &try_operation_response_variants_token_stream,
                    //     false,
                    // );
                    let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                        &operation,
                        &fields_named_from_or_try_from,
                        &json_syn_variant_initialization_token_stream,
                        &json_syn_variant_status_code,
                        &eprintln_error_token_stream,
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        // #check_for_none_token_stream
                        #parameters_logic_token_stream
                    }
                };
                let query_string_token_stream = {
                    let additional_parameters_modification_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        let handle_token_stream = {
                            let handle_stringified = format!("\"{field_ident} = ${{increment}}\"");
                            handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                            &operation,
                            &bind_query_syn_variant_initialization_token_stream,
                            &quote::quote! {#from_snake_case(#error_snake_case)},
                            &bind_query_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                            &eprintln_error_token_stream,
                        );
                        quote::quote!{
                            if let Some(value) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                for element in value {
                                    match #crate_server_postgres_bind_query_bind_query_try_increment_token_stream(
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
                                        Err(#error_snake_case) => {
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
                        let additional_parameters_and_token_stream = {
                            let additional_parameters_and_stringified =
                                format!("\" {and_snake_case}\"");
                            additional_parameters_and_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_and_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                            &operation,
                            &bind_query_syn_variant_initialization_token_stream,
                            &quote::quote! {#from_snake_case(#error_snake_case)},
                            &bind_query_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                            &eprintln_error_token_stream,
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
                                        for element in #primary_key_field_ident {
                                            match #crate_server_postgres_bind_query_bind_query_try_increment_token_stream(
                                                element,
                                                &mut increment,
                                            ) {
                                                Ok(_) => {
                                                    additional_parameters.push_str(&format!("${increment},"));
                                                }
                                                Err(#error_snake_case) => {
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
                    let handle_token_stream = {
                        let handle_stringified = format!("\"{delete_snake_case} {from_snake_case} {table_name_stringified} {where_snake_case} {{}}{returning_primary_key_stringified}\"");
                        handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
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
                    let binded_query_modifications_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        quote::quote!{
                            if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                for #element_snake_case in #value_snake_case {
                                    #query_snake_case = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(#element_snake_case, #query_snake_case);
                                }
                            }
                        }
                    });
                    let binded_query_primary_key_modifications_token_stream = quote::quote! {
                        if let Some(#primary_key_field_ident) = #parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                            for #element_snake_case in #primary_key_field_ident {
                                #query_snake_case = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(#element_snake_case, #query_snake_case);
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
                let postgresql_logic_token_stream = {
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &postgresql_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &postgresql_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    // let filter_unique_parameters_token_stream = {
                    //     let filter_unique_parameters_primary_key_token_stream = quote::quote! {
                    //         if let Some(#primary_key_field_ident) = &#parameters_snake_case.#payload_snake_case_token_stream.#primary_key_field_ident {
                    //             let #not_unique_primary_keys_name_token_stream = {
                    //                 let mut vec = std::vec::Vec::with_capacity(#primary_key_field_ident.len());
                    //                 let mut #not_unique_primary_keys_name_token_stream = std::vec::Vec::with_capacity(#primary_key_field_ident.len());
                    //                 for element in #primary_key_field_ident {
                    //                     let handle = element;
                    //                     match vec.contains(&handle) {
                    //                         true => {
                    //                             #not_unique_primary_keys_name_token_stream.push(element.clone());
                    //                         },
                    //                         false => {
                    //                             vec.push(element);
                    //                         }
                    //                     }
                    //                 }
                    //                 #not_unique_primary_keys_name_token_stream
                    //             };
                    //             if let false = #not_unique_primary_keys_name_token_stream.is_empty() {
                    //                 let #error_snake_case = #try_operation_upper_camel_case_token_stream::#not_unique_primary_keys_variant_initialization_token_stream;
                    //                 #eprintln_error_token_stream
                    //                 return #try_operation_response_variants_token_stream::#from_snake_case(#error_snake_case);
                    //             }
                    //         }
                    //     };
                    //     let filter_unique_parameters_other_columns_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                    //         let field_ident = &element.field_ident;
                    //         let field_handle_token_stream = {
                    //             let field_handle_stringified = format!("{field_ident}_handle");
                    //             field_handle_stringified.parse::<proc_macro2::TokenStream>()
                    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    //         };
                    //         let not_unique_field_vec_snake_case_token_stream = {
                    //             let not_unique_field_vec_snake_case_stringified = generate_not_unique_field_vec_snake_case_stringified(field_ident);
                    //             not_unique_field_vec_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_unique_field_vec_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    //         };
                    //         let not_unique_field_vec_vec_upper_camel_token_stream = {
                    //             let not_unique_field_vec_upper_camel_stringified = generate_not_unique_field_vec_upper_camel_stringified(field_ident);
                    //             not_unique_field_vec_upper_camel_stringified.parse::<proc_macro2::TokenStream>()
                    //             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_unique_field_vec_upper_camel_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    //         };
                    //         let field_code_occurence_new_a4cd6c7d_3d82_4ee7_84f0_ca63ddb894e1_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    //             file!(),
                    //             line!(),
                    //             column!(),
                    //             &proc_macro_name_upper_camel_case_ident_stringified,
                    //         );
                    //         let where_inner_type_with_serialize_deserialize_token_stream = &element.where_inner_type_with_serialize_deserialize_token_stream;
                    //         quote::quote!{
                    //             let #field_handle_token_stream = match #parameters_snake_case.#payload_snake_case_token_stream.#field_ident {
                    //                 Some(value) => {
                    //                     let is_unique = {
                    //                         let mut vec = std::vec::Vec::with_capacity(value.len());
                    //                         let mut is_unique = true;
                    //                         for element in &value {
                    //                             match vec.contains(&element) {
                    //                                 true => {
                    //                                     is_unique = false;
                    //                                     break;
                    //                                 }
                    //                                 false => {
                    //                                     vec.push(element);
                    //                                 }
                    //                             }
                    //                         }
                    //                         is_unique
                    //                     };
                    //                     match is_unique {
                    //                         true => Some(value),
                    //                         false => {
                    //                             let #not_unique_field_vec_snake_case_token_stream = {
                    //                                 let mut vec = std::vec::Vec::with_capacity(value.len());
                    //                                 let mut #not_unique_field_vec_snake_case_token_stream = std::vec::Vec::with_capacity(value.len());
                    //                                 for element in value {
                    //                                     match vec.contains(&element) {
                    //                                         true => {
                    //                                             #not_unique_field_vec_snake_case_token_stream.push(element);
                    //                                         }
                    //                                         false => {
                    //                                             vec.push(element);
                    //                                         }
                    //                                     }
                    //                                 }
                    //                                 #not_unique_field_vec_snake_case_token_stream
                    //                                     .into_iter()
                    //                                     .map(|element|#where_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(element))
                    //                                     .collect()
                    //                             };
                    //                             let #error_snake_case = #try_operation_upper_camel_case_token_stream::#not_unique_field_vec_vec_upper_camel_token_stream {
                    //                                 #not_unique_field_vec_snake_case_token_stream,
                    //                                 #field_code_occurence_new_a4cd6c7d_3d82_4ee7_84f0_ca63ddb894e1_token_stream,
                    //                             };
                    //                             #eprintln_error_token_stream
                    //                             return #try_operation_response_variants_token_stream::#from_snake_case(#error_snake_case);
                    //                         }
                    //                     }
                    //                 },
                    //                 None => None
                    //             };
                    //         }
                    //     });
                    //     quote::quote! {
                    //         #filter_unique_parameters_primary_key_token_stream
                    //         #(#filter_unique_parameters_other_columns_token_stream)*
                    //     }
                    // };
                    let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    quote::quote! {
                        // #filter_unique_parameters_token_stream
                        // let #query_string_name_token_stream = {
                        //     #query_string_token_stream
                        // };
                        // println!("{}", #query_string_name_token_stream);
                        // let #binded_query_name_token_stream = {
                        //     #binded_query_token_stream
                        // };
                        // #acquire_pool_and_connection_token_stream
                        let mut rows = #binded_query_snake_case.fetch(#pg_connection_snake_case.as_mut());
                        let mut vec_values = std::vec::Vec::new();
                        while let Some(row) = {
                            match {
                                use futures::TryStreamExt;
                                rows.try_next()
                            }
                            .await
                            {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_snake_case) => {
                                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        } {
                            match #sqlx_row::try_get::<#primary_key_original_type_token_stream, &std::primitive::str>(&row, #primary_key_field_ident_quotes_token_stream) {
                                Ok(#value_snake_case) => {
                                    vec_values.push(
                                        #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(
                                            #primary_key_inner_type_token_stream(#value_snake_case)
                                        ),
                                    );
                                }
                                Err(#error_snake_case) => {
                                    #operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                        vec_values
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
                generate_try_operation_route_logic_snake_case_token_stream(
                    &operation,
                    &ast,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
            //         let expected_updated_primary_keys_token_stream = quote::quote! {
            //             #primary_key_field_ident
            //             .iter()
            //             .map(|element| element.clone()) //todo - maybe its not a good idea to remove .clone here coz in macro dont know what type
            //             .collect::<std::vec::Vec<#primary_key_inner_type_token_stream>>()
            //         };
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &check_body_size_syn_variant_initialization_token_stream,
                    &postgresql_syn_variant_initialization_token_stream,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_token_stream
                #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
                #try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &quote::quote!{std::vec::Vec<#primary_key_inner_type_token_stream>},
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match primary_key_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                        #value_snake_case
                        .into_iter()
                        .map(|#element_snake_case| #primary_key_inner_type_token_stream::#from_snake_case(#element_snake_case))
                        .collect()
                    },
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            {
                                let mut values = std::vec::Vec::new();
                                for #element_snake_case in #value_snake_case {
                                    match #primary_key_inner_type_token_stream::#try_from_snake_case(#element_snake_case) {
                                        Ok(#value_snake_case) => {
                                            values.push(#value_snake_case);
                                        },
                                        Err(#error_snake_case) => Err(
                                            #try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream
                                        )
                                    }
                                }
                                values
                            }
                        }
                    }
                },
                &proc_macro_name_upper_camel_case_ident_stringified,
                &reqwest_syn_variant_initialization_token_stream,
                &deserialize_response_syn_variant_initialization_token_stream,
                &failed_to_get_response_text_syn_variant_initialization_token_stream,
                &serde_json_to_string_syn_variant_initialization_token_stream,
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
        // let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
        //     proc_macro_helpers::status_code::StatusCode::BadRequest400,
        //     &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
        //     vec![
        //         (
        //             proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
        //             &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
        //             naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
        //                 &operation
        //             )
        //         )
        //     ],
        //     &proc_macro_name_upper_camel_case_ident_stringified,
        // );
        // let type_variants_from_request_response_syn_variants = {
        //     let full_additional_http_status_codes_error_variants =
        //         generate_full_additional_http_status_codes_error_variants(
        //             common_middlewares_error_syn_variants.iter().collect(),
        //             additional_http_status_codes_error_variants.iter().collect(),
        //         );
        //     let type_variants_from_request_response_syn_variants_partial = {
        //         let mut type_variants_from_request_response =
        //             std::vec::Vec::with_capacity(common_error_syn_variants.len().checked_add(2).unwrap());
        //         for element in &common_error_syn_variants {
        //             type_variants_from_request_response.push(element);
        //         }
        //         //todo why no bind query error here?
        //         if primary_key_from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
        //             type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
        //         }
        //         type_variants_from_request_response.push(&operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant);
        //         type_variants_from_request_response
        //     };
        //     generate_type_variants_from_request_response_syn_variants(
        //         &type_variants_from_request_response_syn_variants_partial,
        //         &full_additional_http_status_codes_error_variants,
        //     )
        // };
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &common_route_syn_variants,
            &postgresql_crud_common::FromOrTryFrom::From,//fields_named_excluding_primary_key_from_or_try_from
            &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant,
            &operation,
            &ast,
            &proc_macro_name_upper_camel_case_ident_stringified,
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
                postgresql_crud_common::FromOrTryFrom::From => proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                    &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                    &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                    &quote::quote! {
                        Self{ #primary_key_field_ident: #primary_key_inner_type_token_stream::#from_snake_case(value.#primary_key_field_ident) }
                    },
                ),
                postgresql_crud_common::FromOrTryFrom::TryFrom => {
                    // let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = naming_conventions::PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
                        &operation,
                        &{
                            let primary_key_variant_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_token_stream(
                                &primary_key_syn_field,
                                &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
                            );
                            quote::quote! {
                                #primary_key_variant_token_stream
                            }
                        },
                    );
                    // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
                        &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                        &naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation),
                        &{
                            let logic_token_stream =  {
                                let inner_token_stream = quote::quote! {value.#primary_key_field_ident};
                                match primary_key_rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
                                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
                                        Ok(Self{ #primary_key_field_ident: #primary_key_inner_type_token_stream::#from_snake_case(#inner_token_stream) })
                                    },
                                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                                        let try_from_snake_case_token_stream = naming_conventions::TryFromSnakeCase;
                                        let field_code_occurence_new_66343753_b4dc_4b64_b7a6_3f206033a0b1_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                            file!(),
                                            line!(),
                                            column!(),
                                            &proc_macro_name_upper_camel_case_ident_stringified,
                                        );
                                        quote::quote!{
                                            match #primary_key_inner_type_token_stream::#try_from_snake_case_token_stream(#inner_token_stream) {
                                                Ok(value) => Ok(Self{ #primary_key_field_ident: value }),
                                                Err(#error_snake_case) => Err(Self::Error::#primary_key_field_ident_upper_camel_case_token_stream {
                                                    #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: #error_snake_case,
                                                    #field_code_occurence_new_66343753_b4dc_4b64_b7a6_3f206033a0b1_token_stream
                                                })
                                            }
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
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                &naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation),
                &naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation),
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
            let try_operation_route_logic_response_variants_token_stream = generate_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &primary_key_inner_type_with_serialize_deserialize_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = generate_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_error_named_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                   &operation,
                   &primary_key_from_or_try_from,
                   &json_syn_variant_initialization_token_stream,
                   &json_syn_variant_status_code,
                   &eprintln_error_token_stream,
                   &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let query_string_token_stream = {
                    let additional_parameters_primary_key_modification_token_stream = {
                        let query_part_token_stream = {
                            let query_part_stringified = format!("\" {primary_key_field_ident} = $1\""); //todo where
                            query_part_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_part_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote! {
                            #query_snake_case.push_str(&format!(#query_part_token_stream));
                        }
                    };
                    let handle_token_stream = {
                        let handle_stringified = format!("\"{delete_snake_case} {from_snake_case} {table_name_stringified} {where_snake_case}\""); //todo where
                        handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
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
                        #query_snake_case = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
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
                let postgresql_logic_token_stream = {
                    let operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &postgresql_syn_variant_initialization_token_stream,
                        &quote::quote! {#from_snake_case(#error_snake_case)},
                        &postgresql_syn_variant_status_code.to_axum_http_status_code_token_stream(),
                        &eprintln_error_token_stream,
                    );
                    quote::quote! {
                        match #binded_query_snake_case
                            .fetch_one(#pg_connection_snake_case.as_mut())
                            .await
                        {
                            Ok(#value_snake_case) => match #sqlx_row::try_get::<#primary_key_original_type_token_stream, &std::primitive::str>(&#value_snake_case, #primary_key_field_ident_quotes_token_stream) {
                                Ok(#value_snake_case) => #primary_key_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(
                                    #primary_key_inner_type_token_stream(#value_snake_case)
                                ),
                                Err(#error_snake_case) => {
                                    #operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            },
                            Err(#error_snake_case) => {
                                #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            }
                        }
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
                generate_try_operation_route_logic_snake_case_token_stream(
                    &operation,
                    &ast,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                    &eprintln_error_token_stream,
                    &check_body_size_syn_variant_initialization_token_stream,
                    &postgresql_syn_variant_initialization_token_stream,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_token_stream
                #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
                #try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        let (try_operation_token_stream, try_operation_test_token_stream) = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(
                &operation,
                &common_http_request_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &table_name_stringified,
                &type_variants_from_request_response_syn_variants,
                &primary_key_inner_type_token_stream,
                &{
                    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
                    quote::quote!{#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
                },
                &match primary_key_from_or_try_from {
                    postgresql_crud_common::FromOrTryFrom::From => quote::quote!{#primary_key_inner_type_token_stream::#from_snake_case(#value_snake_case)},
                    postgresql_crud_common::FromOrTryFrom::TryFrom => {
                        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                        quote::quote!{
                            match #primary_key_inner_type_token_stream::#try_from_snake_case(#value_snake_case) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_snake_case) => Err(
                                    #try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_one_initialization_token_stream
                                )
                            }
                        }
                    }
                },
                &proc_macro_name_upper_camel_case_ident_stringified,
                &reqwest_syn_variant_initialization_token_stream,
                &deserialize_response_syn_variant_initialization_token_stream,
                &failed_to_get_response_text_syn_variant_initialization_token_stream,
                &serde_json_to_string_syn_variant_initialization_token_stream,
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
        #table_name_declaration_token_stream
        #struct_options_token_stream
        #from_ident_for_ident_options_token_stream
        // // #(#structs_variants_token_stream)*
        // // #(#impl_std_convert_try_from_ident_options_for_struct_variants_token_stream)*
        #column_token_stream
        #generate_query_vec_column_token_stream
        #options_try_from_sqlx_row_token_stream
        #primary_key_try_get_sqlx_row_token_stream
        #allow_methods_token_stream
        #ident_column_read_permission_token_stream
        #(#reexport_postgresql_sqlx_column_types_token_stream)*
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

fn generate_std_vec_vec_syn_punctuated_punctuated(
    parts_vec: &[&str],
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> {
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
}

// fn generate_common_middlewares_error_syn_variants_from_impls(
//     common_middlewares_error_syn_variants: std::vec::Vec<&(
//         syn::Ident,
//         proc_macro2::TokenStream,
//         std::vec::Vec<syn::Variant>,
//     )>,
//     additional_http_status_codes_error_variants: std::vec::Vec<&(
//         syn::Ident,
//         proc_macro2::TokenStream,
//         std::vec::Vec<syn::Variant>,
//     )>,
//     operation: &Operation,
//     proc_macro_name_upper_camel_case_ident_stringified: &str,
// ) -> proc_macro2::TokenStream {
//     let error_syn_variants = generate_full_additional_http_status_codes_error_variants(
//         common_middlewares_error_syn_variants,
//         additional_http_status_codes_error_variants,
//     );
//     let try_operation_upper_camel_case_token_stream = naming_conventions::TrySelfUpperCamelCaseTokenStream::try_self_upper_camel_case_token_stream(operation);
//     let value = error_syn_variants.iter().map(|element|{
//         let element_path_ident_token_stream = {
//             let element_path_ident_stringified = format!("{}{}", element.1, element.0);
//             element_path_ident_stringified.parse::<proc_macro2::TokenStream>()
//             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {element_path_ident_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//         };
//         let variants = element.2.iter().map(|element|{
//             let element_ident = &element.ident;
//             let fields_named = if let syn::Fields::Named(fields_named) = &element.fields {
//                 &fields_named.named
//             } else {
//                 panic!("{proc_macro_name_upper_camel_case_ident_stringified} {element_ident} supports only syn::Fields::Named");
//             };
//             let fields_token_stream = generate_self_fields_token_stream(
//                 &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
//                 proc_macro_name_upper_camel_case_ident_stringified,
//             );
//             quote::quote! {
//                 #element_path_ident_token_stream::#element_ident {
//                     #(#fields_token_stream),*
//                 } => Self::#element_ident {
//                     #(#fields_token_stream),*
//                 }
//             }
//         }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
//         quote::quote! {
//             impl std::convert::From<#element_path_ident_token_stream> for #try_operation_upper_camel_case_token_stream {
//                 fn from(value: #element_path_ident_token_stream) -> Self {
//                     match value {
//                         #(#variants),*
//                     }
//                 }
//             }
//         }
//     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
//     quote::quote! {#(#value)*}
// }

fn generate_not_unique_field_vec_upper_camel_stringified(
    field_ident: &syn::Ident,
) -> std::string::String {
    format!(
        "{}{}{}{}",
        naming_constants::NotUpperCamelCase,
        naming_constants::UniqueUpperCamelCase,
        convert_case::Casing::to_case(
            &field_ident
            .to_string(),
            convert_case::Case::UpperCamel
        ),
        naming_constants::VecUpperCamelCase,
    )
}

fn generate_not_unique_field_vec_snake_case_stringified(
    field_ident: &syn::Ident,
) -> std::string::String {
    format!(
        "{}_{}_{field_ident}_{}",
        naming_constants::NotSnakeCase,
        naming_constants::UniqueSnakeCase,
        naming_constants::VecSnakeCase,
    )
}

fn generate_self_fields_token_stream<'a>(//todo refactor as &[&'a SynRust...]
    fields: &[&'a syn::Field],
    proc_macro_name_upper_camel_case_ident_stringified: &'a str,
) -> std::vec::Vec<&'a syn::Ident> {
    fields
        .iter()
        .map(|field| {
            field.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_constants::FIELD_IDENT_IS_NONE
                )
            })
        })
        .collect()
}

//todo maybe add another parameter for from or try from for operation_payload_with_serialize_deserialize_upper_camel_case_token_stream
    //             &quote::quote! {
    //                 //todo maybe for all cases use this? = remove this parameter and write it inside generate_try_operation_token_stream
    //                 let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(
    //                     &#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(#parameters_snake_case.#payload_snake_case_token_stream)
    //                 ) {
    //                     Ok(value) => value,
    //                     Err(#error_snake_case) => {
    //                         return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_syn_variant_initialization_token_stream);
    //                     }
    //                 };
    //             },
//
    //             &{
    //                 let field_code_occurence_new_40fe4f4a_ae46_496d_8c69_cdb3f41e2755_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
    //                     file!(),
    //                     line!(),
    //                     column!(),
    //                     &proc_macro_name_upper_camel_case_ident_stringified,
    //                 );
    //                 quote::quote! {
    //                     let #payload_snake_case_token_stream = match #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::#try_from_snake_case_token_stream(
    //                         #parameters_snake_case.#payload_snake_case_token_stream
    //                     ) {
    //                         Ok(value) => match #serde_json_to_string_token_stream(&value) {
    //                             Ok(value) => value,
    //                             Err(#error_snake_case) => {
    //                                 return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_syn_variant_initialization_token_stream);
    //                             }
    //                         },
    //                         Err(#error_snake_case) => {
    //                             return Err(#try_operation_error_named_upper_camel_case_token_stream::#operation_payload_with_serialize_deserialize_try_from_operation_payload_upper_camel_case_token_stream {
    //                                 #operation_payload_with_serialize_deserialize_try_from_operation_payload_snake_case_token_stream: #error_snake_case,
    //                                 #field_code_occurence_new_40fe4f4a_ae46_496d_8c69_cdb3f41e2755_token_stream
    //                             });
    //                         }
    //                     };
    //                 }
    //             },
fn generate_try_operation_token_stream(
    operation: &Operation,
    table_name_stringified: &std::primitive::str,
    type_variants_from_request_response_syn_variants: &[syn::Variant],
    result_ok_type_token_stream: &proc_macro2::TokenStream,
    operation_payload_with_serialize_deserialize_initialization_token_stream: &proc_macro2::TokenStream,
    desirable_from_or_try_from_desirable_with_serialize_deserialize_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
    reqwest_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    deserialize_response_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    failed_to_get_response_text_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    serde_json_to_string_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let str_ref_token_stream = token_patterns::RefStdPrimitiveStr;
    let parameters_snake_case = naming_constants::ParametersSnakeCase;
    let try_operation_snake_case_token_stream = naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(operation);
    let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
    let operation_parameters_upper_camel_case_token_stream = naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
    let payload_snake_case = naming_constants::PayloadSnakeCase;
    let payload_token_stream = {
        let parameters_snake_case = naming_constants::ParametersSnakeCase;
        let payload_snake_case = naming_constants::PayloadSnakeCase;
        let value_snake_case = naming_constants::ValueSnakeCase;
        let error_snake_case = naming_constants::ErrorSnakeCase;
        let from_snake_case = naming_constants::FromSnakeCase;
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
        let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
        quote::quote! {
            let #payload_snake_case = {
                let #value_snake_case = #operation_payload_with_serialize_deserialize_initialization_token_stream;
                match serde_json::to_string(&#value_snake_case) {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_snake_case) => {
                        return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_syn_variant_initialization_token_stream);
                    }
                }
            };
        }
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
        let body_snake_case = naming_constants::BodySnakeCase;
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
    let response_snake_case = naming_constants::ResponseSnakeCase;
    let response_token_stream = {
        let value_snake_case = naming_constants::ValueSnakeCase;
        let error_snake_case = naming_constants::ErrorSnakeCase;
        quote::quote! {
            let #response_snake_case = match #future_snake_case.await {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_snake_case) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#reqwest_syn_variant_initialization_token_stream);
                }
            };
        }
    };
    let status_code_snake_case = naming_conventions::StatusCodeSnakeCase;
    let status_code_token_stream = {
        quote::quote! {
            let #status_code_snake_case = #response_snake_case.status();
        }
    };
    let headers_snake_case = naming_constants::HeadersSnakeCase;
    let headers_token_stream = {
        quote::quote! {
            let #headers_snake_case = #response_snake_case.headers().clone();
        }
    };
    let response_text_snake_case = naming_conventions::ResponseTextSnakeCase;
    let response_text_token_stream = {
        let value_snake_case = naming_constants::ValueSnakeCase;
        let error_snake_case = naming_constants::ErrorSnakeCase;
        quote::quote! {
            let #response_text_snake_case = match #response_snake_case.text().await {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_snake_case) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#failed_to_get_response_text_syn_variant_initialization_token_stream);
                }
            };
        }
    };
    let expected_response_snake_case = naming_conventions::ExpectedResponseSnakeCase;
    let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(operation);
    let expected_response_token_stream = {
        let value_snake_case = naming_constants::ValueSnakeCase;
        let error_snake_case = naming_constants::ErrorSnakeCase;
        quote::quote! {
            let #expected_response_snake_case = match serde_json::from_str::<#try_operation_route_logic_response_variants_upper_camel_case_token_stream>(&#response_text_snake_case) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_snake_case) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#deserialize_response_syn_variant_initialization_token_stream);
                }
            };
        }
    };
    let try_operation_route_logic_error_named_with_serialize_deserialize_token_stream = {
        let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(operation);
        let try_operation_route_logic_response_variants_to_try_operation_route_logic_error_named_with_serialize_deserialize = type_variants_from_request_response_syn_variants.iter().map(|element|{
            let variant_ident = &element.ident;
            let fields_idents_token_stream = if let syn::Fields::Named(fields_named) = &element.fields {
                let fields_idents = fields_named.named.iter().map(|field|&field.ident);
                quote::quote! {#(#fields_idents),*}
            }
            else {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
            };
            let try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseTokenStream::try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream(operation);
            quote::quote! {
                #try_operation_route_logic_response_variants_upper_camel_case_token_stream::#variant_ident {
                    #fields_idents_token_stream
                } => #try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream::#variant_ident { #fields_idents_token_stream }
            }
        });
        let desirable_upper_camel_case = naming_constants::DesirableUpperCamelCase;
        let expected_response_snake_case = naming_conventions::ExpectedResponseSnakeCase;
        let value_snake_case = naming_constants::ValueSnakeCase;
        let element_snake_case = naming_constants::ElementSnakeCase;
        let from_snake_case = naming_constants::FromSnakeCase;
        let try_operation_route_logic_error_named_with_serialize_deserialize_snake_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCaseTokenStream::try_self_route_logic_error_named_with_serialize_deserialize_snake_case_token_stream(operation);
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
        let try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseTokenStream::try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_token_stream(operation);
        let try_operation_route_logic_error_named_with_serialize_deserialize_snake_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCaseTokenStream::try_self_route_logic_error_named_with_serialize_deserialize_snake_case_token_stream(operation);
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
            #server_location_snake_case: #str_ref_token_stream,//todo rename as endpoint location
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
}

// fn generate_try_operation_one_token_stream(
//     operation: &Operation,
//     server_location_name_token_stream: &proc_macro2::TokenStream,
//     str_ref_token_stream: &proc_macro2::TokenStream,
//     return_result_ok_type_token_stream: &proc_macro2::TokenStream,
//     payload_variable_initialization_token_stream: &proc_macro2::TokenStream,
//     reqwest_client_new_token_stream: &proc_macro2::TokenStream,
//     commit_header_addition_token_stream: &proc_macro2::TokenStream,
//     content_type_application_json_header_addition_token_stream: &proc_macro2::TokenStream,
//     ok_value_handle_token_stream: &proc_macro2::TokenStream,
//     table_name_stringified: &str,
//     type_variants_from_request_response_syn_variants: &[&syn::Variant],
//     desirable_status_code: proc_macro_helpers::status_code::StatusCode,
//     desirable_type_token_stream: &proc_macro2::TokenStream,
//     deserialize_response_initialization_token_stream: &proc_macro2::TokenStream,
//     unexpected_status_code_initialization_token_stream: &proc_macro2::TokenStream,
//     reqwest_initialization_token_stream: &proc_macro2::TokenStream,
//     failed_to_get_response_text_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
//     expected_type_initialization_token_stream: &proc_macro2::TokenStream,
//     from_snake_case_token_stream: &proc_macro2::TokenStream,
//     from_str_snake_case_token_stream: &proc_macro2::TokenStream,
//     proc_macro_name_upper_camel_case_ident_stringified: &str,
// ) -> proc_macro2::TokenStream {
//     let try_operation_snake_case_token_stream = naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(operation);
//     let parameters_snake_case = naming_constants::ParametersSnakeCase;
//     let payload_snake_case = naming_constants::PayloadSnakeCase;
//     let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
//     let operation_parameters_upper_camel_case_token_stream = naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
//     let operation_http_method_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(
//         &operation.http_method(),
//     );
//     let url_handle_token_stream = naming_conventions::UrlHandleSelfSnakeCaseTokenStream::url_handle_self_snake_case_token_stream(operation, table_name_stringified);
//     let code_occurence_snake_case_stringified = naming_conventions::CodeOccurenceSnakeCase;
//     let type_variants_from_request_response_syn_variants_len = type_variants_from_request_response_syn_variants.len();
//     let code_occurence_upper_camel_case = naming_conventions::CodeOccurenceUpperCamelCase;
//     let try_operation_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(operation);
//     let http_status_code_quote_token_stream = desirable_status_code.to_http_status_code_token_stream();
//     let desirable_enum_name = {
//         let status_code_enum_name_stingified = format!("{try_operation_response_variants_upper_camel_case_token_stream}{desirable_status_code}");
//         status_code_enum_name_stingified
//         .parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {status_code_enum_name_stingified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     };
//     // let (unique_status_codes, unique_status_codes_len, unique_status_codes_len_minus_one) = {
//     //     let hashmap_unique_status_codes = type_variants_from_request_response_syn_variants.iter().fold(//todo maybe not need hashmap here? maybe just unique vec?
//     //         std::collections::HashMap::<proc_macro_helpers::status_code::StatusCode, std::vec::Vec<(
//     //             &syn::Ident,
//     //             std::vec::Vec<(syn::Ident, proc_macro2::TokenStream)>,
//     //         )>>::with_capacity(type_variants_from_request_response_syn_variants_len),
//     //         |mut acc, element| {
//     //             let variant_ident = &element.ident;
//     //             let error_variant_attribute = proc_macro_helpers::status_code::StatusCode::try_from(element)
//     //             .unwrap_or_else(|error| {panic!("{proc_macro_name_upper_camel_case_ident_stringified} variant {variant_ident} failed: {error}")});
//     //             let fields_named = if let syn::Fields::Named(fields_named) = &element.fields {
//     //                 fields_named
//     //             }
//     //             else {
//     //                 panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
//     //             };
//     //             let error_variant_fields = fields_named.named.iter().map(|field|{//todo refactor
//     //                 let field_ident = field.ident.as_ref().unwrap_or_else(|| {
//     //                     panic!(
//     //                         "{proc_macro_name_upper_camel_case_ident_stringified} {}",
//     //                         naming_constants::FIELD_IDENT_IS_NONE
//     //                     )
//     //                 });
//     //                 let field_type_with_serialize_deserialize = if *field_ident == *code_occurence_snake_case_stringified {
//     //                     let code_occurence_type_token_stream = {
//     //                         if let syn::Type::Path(type_path) = &field.ty {
//     //                             let mut code_occurence_type_repeat_checker = false;
//     //                             let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
//     //                             .fold(std::string::String::new(), |mut acc, path_segment| {
//     //                                 let path_segment_ident = &path_segment.ident;
//     //                                 if *path_segment_ident == &code_occurence_upper_camel_case.to {
//     //                                     assert!(!code_occurence_type_repeat_checker, "{proc_macro_name_upper_camel_case_ident_stringified} code_occurence_ident detected more than one {code_occurence_upper_camel_case_stringified} inside type path");
//     //                                     acc.push_str(&path_segment_ident.to_string());
//     //                                     code_occurence_type_repeat_checker = true;
//     //                                 }
//     //                                 else {
//     //                                     acc.push_str(&format!("{path_segment_ident}::"));
//     //                                 }
//     //                                 acc
//     //                             });
//     //                             assert!(code_occurence_type_repeat_checker, "{proc_macro_name_upper_camel_case_ident_stringified} no {code_occurence_upper_camel_case_stringified} named field");
//     //                             code_occurence_segments_stringified_handle.parse::<proc_macro2::TokenStream>()
//     //                             .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_segments_stringified_handle} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     //                         }
//     //                         else {
//     //                             panic!(
//     //                                 "{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_snake_case_stringified} {} {}",
//     //                                 naming_constants::SUPPORTS_ONLY_STRINGIFIED,
//     //                                 naming_constants::SYN_TYPE_PATH
//     //                             );
//     //                         }
//     //                     };
//     //                     code_occurence_type_token_stream
//     //                 }
//     //                 else {
//     //                     let attribute = {
//     //                         let mut option_attribute = None;
//     //                         field.attrs.iter().for_each(|attr|{
//     //                             if attr.path().segments.len() == 1 {
//     //                                 let error_message = format!("{proc_macro_name_upper_camel_case_ident_stringified} two or more supported attributes!");
//     //                                 let attr_ident = attr.path().segments.iter().next().map_or_else(|| panic!("attr.path().segments.iter().next() is None"), |path_segment| &path_segment.ident);
//     //                                 if let Ok(value) = {
//     //                                     use std::str::FromStr;
//     //                                     proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::from_str(&attr_ident.to_string())
//     //                                 } {
//     //                                     if option_attribute.is_some() {
//     //                                         panic!("{error_message}");
//     //                                     }
//     //                                     else {
//     //                                         option_attribute = Some(value);
//     //                                     }
//     //                                 }
//     //                             }//other attributes are not for this proc_macro
//     //                         });
//     //                         option_attribute.unwrap_or_else(|| panic!(
//     //                             "{proc_macro_name_upper_camel_case_ident_stringified} option attribute {}",
//     //                             naming_constants::IS_NONE_STRINGIFIED
//     //                         ))
//     //                     };
//     //                     let supported_container = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_supported_container(
//     //                         field,
//     //                         proc_macro_name_upper_camel_case_ident_stringified,
//     //                     );
//     //                     proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_field_type_with_serialize_deserialize_version(
//     //                         attribute,
//     //                         supported_container,
//     //                         proc_macro_name_upper_camel_case_ident_stringified,
//     //                     )
//     //                 };
//     //                 (field_ident.clone(), field_type_with_serialize_deserialize)
//     //             }).collect::<Vec<(syn::Ident, proc_macro2::TokenStream)>>();
//     //             let error_variant = (
//     //                 variant_ident,
//     //                 error_variant_fields,
//     //             );
//     //             match acc.get_mut(&error_variant_attribute) {
//     //                 Some(value) => {
//     //                     value.push(error_variant);
//     //                 },
//     //                 None => {
//     //                     drop(acc.insert(error_variant_attribute, vec![error_variant]));
//     //                 }
//     //             }
//     //             acc
//     //         },
//     //     );
//     //     let unique_status_codes_len = hashmap_unique_status_codes.len();
//     //     assert!(unique_status_codes_len >= 1, "{proc_macro_name_upper_camel_case_ident_stringified} unique_status_codes_len < 1 {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE);
//     //     let unique_status_codes_len_minus_one = unique_status_codes_len.checked_sub(1).unwrap();
//     //     let unique_status_codes = hashmap_unique_status_codes
//     //         .into_keys()
//     //         .collect::<std::vec::Vec<proc_macro_helpers::status_code::StatusCode>>();
//     //     (
//     //         unique_status_codes,
//     //         unique_status_codes_len,
//     //         unique_status_codes_len_minus_one,
//     //     )
//     // };
//     // let error_value_snake_case_token_stream =
//     //     proc_macro_common::error_value_snake_case_token_stream();
//     // let status_code_enums_try_from = {
//     //     let mut is_last_element_found = false;
//     //     let desirable_status_code_case_token_stream = quote::quote! {
//     //         match serde_json::#from_str_snake_case::<#desirable_enum_name>(&response_text) {
//     //             Ok(value) => #try_operation_response_variants_upper_camel_case_token_stream::#from_snake_case(value),
//     //             Err(#error_snake_case) => {
//     //                 return Err(#try_operation_error_named_upper_camel_case_token_stream::#deserialize_response_initialization_token_stream);
//     //             }
//     //         }
//     //     };
//     //     let mut status_code_enums_try_from_variants =
//     //         std::vec::Vec::with_capacity(unique_status_codes_len.checked_add(1).unwrap());
//     //     status_code_enums_try_from_variants.push(quote::quote! {
//     //         if status_code == #http_status_code_quote_token_stream {
//     //             #desirable_status_code_case_token_stream
//     //         }
//     //     });
//     //     unique_status_codes
//     //     .into_iter()
//     //     .enumerate()
//     //     .for_each(|(index, status_code_attribute)|{
//     //         let try_operation_response_variants_desirable_attribute_token_stream = naming_conventions::TrySelfResponseVariantsStatusCodeTokenStream::try_self_response_variants_status_code_token_stream(operation, &status_code_attribute);
//     //         let http_status_code_token_stream = status_code_attribute.to_http_status_code_token_stream();
//     //         if index == unique_status_codes_len_minus_one{
//     //             is_last_element_found = true;
//     //             status_code_enums_try_from_variants.push(quote::quote! {
//     //                 else {
//     //                     return Err(#try_operation_error_named_upper_camel_case_token_stream::#unexpected_status_code_initialization_token_stream);
//     //                 }
//     //             });
//     //         }
//     //         else if desirable_status_code != status_code_attribute {
//     //             status_code_enums_try_from_variants.push(quote::quote! {
//     //                 else if status_code == #http_status_code_token_stream {
//     //                     match serde_json::#from_str_snake_case::<#try_operation_response_variants_desirable_attribute_token_stream>(&response_text) {
//     //                         Ok(value) => #try_operation_response_variants_upper_camel_case_token_stream::#from_snake_case(value),
//     //                         Err(#error_snake_case) => {
//     //                             return Err(#try_operation_error_named_upper_camel_case_token_stream::#deserialize_response_initialization_token_stream);
//     //                         }
//     //                     }
//     //                 }
//     //             });
//     //         }
//     //         else {
//     //             //clippy lint forces to add empty else
//     //         }
//     //     });
//     //     assert!(is_last_element_found, "{proc_macro_name_upper_camel_case_ident_stringified} false = is_last_element_found");
//     //     status_code_enums_try_from_variants
//     // };
//     quote::quote! {
//         pub async fn #try_operation_snake_case_token_stream<'a>(
//             #server_location_name_token_stream: #str_ref_token_stream,
//             #parameters_snake_case: #operation_parameters_upper_camel_case_token_stream,
//         ) -> Result<#return_result_ok_type_token_stream, #try_operation_error_named_upper_camel_case_token_stream> {
//             // #payload_variable_initialization_token_stream
//             // let url = format!(
//             //     #url_handle_token_stream,
//             //     #server_location_name_token_stream
//             // );
//             // // println!("{}", url);
//             // let future = #reqwest_client_new_token_stream
//             //     .#operation_http_method_snake_case_token_stream(&url)
//             //     #commit_header_addition_token_stream
//             //     #content_type_application_json_header_addition_token_stream
//             //     .body(#payload_snake_case_token_stream)
//             //     .send();
//             // let response = match future.await {
//             //     Ok(response) => response,
//             //     Err(#error_snake_case) => {
//             //         return Err(#try_operation_error_named_upper_camel_case_token_stream::#reqwest_initialization_token_stream);
//             //     }
//             // };
//             // let status_code = response.status();
//             // let headers = response.headers().clone();
//             // let response_text = match response.text().await {
//             //     Ok(response_text) => response_text,
//             //     Err(#error_snake_case) => {
//             //         return Err(#try_operation_error_named_upper_camel_case_token_stream::#failed_to_get_response_text_syn_variant_initialization_token_stream);
//             //     }
//             // };
//             // let variants = #(#status_code_enums_try_from)*;
//             // match #desirable_type_token_stream::try_from(variants) {
//             //     Ok(value) => #ok_value_handle_token_stream,
//             //     Err(#error_snake_case) => {
//             //         return Err(#try_operation_error_named_upper_camel_case_token_stream::#expected_type_initialization_token_stream);
//             //     },
//             // }
//             //
//             // #payload_token_stream
//             // #url_token_stream
//             // #future_token_stream
//             // #response_token_stream
//             // #status_code_token_stream
//             // #headers_token_stream
//             // #response_text_token_stream
//             // #expected_response_token_stream
//             // #try_operation_route_logic_error_named_with_serialize_deserialize_token_stream
//             // #return_error_token_stream
//             //
//         }
//     }
// }

// fn generate_type_variants_from_request_response_syn_variants<'a>(
//     type_variants_from_request_response_syn_variants_partial: &[&'a syn::Variant],
//     full_additional_http_status_codes_error_variants: &[&'a (
//         syn::Ident,
//         proc_macro2::TokenStream,
//         std::vec::Vec<syn::Variant>,
//     )],
// ) -> std::vec::Vec<&'a syn::Variant> {
//     let mut handle = std::vec::Vec::with_capacity(
//         type_variants_from_request_response_syn_variants_partial.len().checked_add(full_additional_http_status_codes_error_variants.len()).unwrap()
//     );
//     for element in type_variants_from_request_response_syn_variants_partial {
//         handle.push(*element);
//     }
//     for element in full_additional_http_status_codes_error_variants {
//         for element in &element.2 {
//             handle.push(element);
//         }
//     }
//     handle
// }

// fn generate_unique_status_codes(
//     desirable_status_code: proc_macro_helpers::status_code::StatusCode,
//     type_variants_from_request_response_syn_variants: &[&syn::Variant],
//     proc_macro_name_upper_camel_case_ident_stringified: &str,
// ) -> std::vec::Vec<proc_macro_helpers::status_code::StatusCode> {
//     let mut value =
//         std::vec::Vec::with_capacity(type_variants_from_request_response_syn_variants.len());
//     value.push(desirable_status_code);
//     for element in type_variants_from_request_response_syn_variants {
//         let variant_ident = &element.ident;
//         let error_variant_status_code = proc_macro_helpers::status_code::StatusCode::try_from(element)
//         .unwrap_or_else(|error| {panic!("{proc_macro_name_upper_camel_case_ident_stringified} variant {variant_ident} failed: {error}")});
//         if !value.contains(&error_variant_status_code) {
//             value.push(error_variant_status_code);
//         }
//     }
//     value
// }

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
    fn operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant(
        &self,
        proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
    ) -> syn::Variant {
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified = naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(self);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_stringified = naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_stringified(self);
        proc_macro_helpers::construct_syn_variant::construct_syn_variant_with_status_code(
            Self::OPERATION_PAYLOAD_TRY_FROM_OPERATION_PAYLOAD_WITH_SERIALIZE_DESERIALIZE_STATUS_CODE.clone(),
            &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
            vec![
                (
                    proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
                    &operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_stringified,
                    self.self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated()
                )
            ],
            &proc_macro_name_upper_camel_case_ident_stringified,
         )
    }
    fn operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant_initialization_token_stream(
        &self,
        proc_macro_name_upper_camel_case_ident_stringified: &str,
    ) -> proc_macro2::TokenStream {
        let error_snake_case = naming_constants::ErrorSnakeCase;
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(self);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream = naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(self);
        let field_code_occurence_new_50fa7a3f_0015_4480_a054_91e811b3fdba_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            #operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: #error_snake_case,
                #field_code_occurence_new_50fa7a3f_0015_4480_a054_91e811b3fdba_token_stream,
            }
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
}

#[derive(proc_macro_assistants::ToSnakeCaseStringified)]
enum OperationHttpMethod {
    Post,
    Patch,
    Delete,
}

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
enum Order {
    Asc,
    Desc,
}

impl Order {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Asc => quote::quote! {Asc},
            Self::Desc => quote::quote! {Desc},
        }
    }
}

fn acquire_pool_and_connection(
    operation: &Operation,
    postgresql_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    eprintln_error_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let try_operation_route_logic_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedUpperCamelCaseTokenStream::try_self_route_logic_error_named_upper_camel_case_token_stream(operation);
    let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(operation);
    let value_snake_case = naming_constants::ValueSnakeCase;
    let error_snake_case = naming_constants::ErrorSnakeCase;
    let from_snake_case = naming_constants::FromSnakeCase;
    let into_response_snake_case = naming_conventions::IntoResponseSnakeCase;
    let pg_connection_snake_case = naming_conventions::PgConnectionSnakeCase;
    quote::quote! {
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {//todo find out difference between acquire and try_acquire
            Ok(#value_snake_case) => #value_snake_case,
            Err(#error_snake_case) => {
                let #error_snake_case = #try_operation_route_logic_error_named_upper_camel_case_token_stream::#postgresql_syn_variant_initialization_token_stream;
                #eprintln_error_token_stream;//todo reuse it as token_stream

                let mut res = axum::response::IntoResponse::#into_response_snake_case(axum::Json(#try_operation_route_logic_response_variants_upper_camel_case_token_stream::#from_snake_case(#error_snake_case)));
                *res.status_mut() = axum::http::StatusCode::CREATED;
                // *res.headers_mut() = axum::http::HeaderMap::new();
                return res;
            }
        };
        let #pg_connection_snake_case = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(#value_snake_case) => #value_snake_case,
            Err(#error_snake_case) => {
                let #error_snake_case = #try_operation_route_logic_error_named_upper_camel_case_token_stream::#postgresql_syn_variant_initialization_token_stream;
                #eprintln_error_token_stream;

                let mut res = axum::response::IntoResponse::#into_response_snake_case(axum::Json(#try_operation_route_logic_response_variants_upper_camel_case_token_stream::#from_snake_case(#error_snake_case)));
                *res.status_mut() = axum::http::StatusCode::CREATED;
                // *res.headers_mut() = axum::http::HeaderMap::new();
                return res;
            }
        };
    }
}

fn syn_ident_to_upper_camel_case_stringified(value: &syn::Ident) -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value.to_string())
}

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
    where_inner_type_with_serialize_deserialize_handle_stringified: std::string::String,
    where_inner_type_token_stream: proc_macro2::TokenStream,
    where_inner_type_with_serialize_deserialize_token_stream: proc_macro2::TokenStream,
}

//
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
        let where_inner_type_with_serialize_deserialize_handle_stringified = rust_sqlx_map_to_postgres_type_variant.get_where_inner_type_with_serialize_deserialize_handle_stringified("");
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
            where_inner_type_with_serialize_deserialize_handle_stringified,
            where_inner_type_token_stream,
            where_inner_type_with_serialize_deserialize_token_stream,
        })
    }
}

fn generate_pub_field_ident_field_type_token_stream(element: &SynFieldWithAdditionalInfo<'_>) -> proc_macro2::TokenStream {
    let field_ident = &element.field_ident;
    let inner_type_token_stream = &element.inner_type_token_stream;
    quote::quote! {
        pub #field_ident: #inner_type_token_stream
    }
}

fn generate_field_ident_field_type_with_serialize_deserialize_token_stream(element: &SynFieldWithAdditionalInfo<'_>) -> proc_macro2::TokenStream {
    let field_ident = &element.field_ident;
    let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
    quote::quote! {
        #field_ident: #inner_type_with_serialize_deserialize_token_stream
    }
}

fn generate_let_field_ident_value_field_ident_try_from_token_stream(//todo rename
    element: &SynFieldWithAdditionalInfo<'_>,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    field_code_occurence_new_token_stream: &proc_macro2::TokenStream,
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let field_ident = &element.field_ident;
    let inner_token_stream = quote::quote! {value.#field_ident};
    let initialization_token_stream = {
        let inner_type_token_stream = &element.inner_type_token_stream;
        let from_snake_case = naming_constants::FromSnakeCase;
        match element.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
            postgresql_crud_common::FromOrTryFrom::From => {
                quote::quote!{#inner_type_token_stream::#from_snake_case(#inner_token_stream)}
            },
            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                let try_from_snake_case = naming_conventions::TryFromSnakeCase;
                let field_ident_upper_camel_case_token_stream = {
                    //todo its a temporal naming desicion. maybe it can be better
                    let value = syn_ident_to_upper_camel_case_stringified(element.field_ident);
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote!{
                    match #inner_type_token_stream::#try_from_snake_case(#inner_token_stream) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Self::Error::#field_ident_upper_camel_case_token_stream {
                                #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: error,
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
}

fn generate_let_field_ident_value_inner_type_from_token_stream(
    element: &SynFieldWithAdditionalInfo<'_>,
) -> proc_macro2::TokenStream {
    let field_ident = &element.field_ident;
    let inner_type_token_stream = &element.inner_type_token_stream;
    let from_snake_case = naming_constants::FromSnakeCase;
    quote::quote! {
        let #field_ident = #inner_type_token_stream::#from_snake_case(value.#field_ident);
    }
}

fn generate_option_vec_where_inner_type_from_or_try_from_option_vec_where_inner_type_with_serialize_deserialize_token_stream(
    value: &SynFieldWithAdditionalInfo<'_>,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let field_ident = &value.field_ident;
    let inner_token_stream = quote::quote! {value.#field_ident};
    let where_inner_type_token_stream = &value.where_inner_type_token_stream;
    let from_snake_case = naming_constants::FromSnakeCase;
    let initialization_token_stream = match value.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
        postgresql_crud_common::FromOrTryFrom::From => quote::quote!{
            match #inner_token_stream {
                Some(value) => Some(value.into_iter().map(|element|#where_inner_type_token_stream::#from_snake_case(element)).collect()),
                None => None,
            }
        },
        postgresql_crud_common::FromOrTryFrom::TryFrom => {
            let field_code_occurence_new_68674e53_54cf_4cfe_b532_2e4aecda32c5_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_ident_upper_camel_case_token_stream = {
                //todo its a temporal naming desicion. maybe it can be better
                let value = syn_ident_to_upper_camel_case_stringified(value.field_ident);
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                match #inner_token_stream {
                    Some(value) => {
                        let mut values = std::vec::Vec::with_capacity(value.len());
                        for element in value {
                            match #where_inner_type_token_stream::try_from(element) {
                                Ok(value) => {
                                    values.push(value);
                                }
                                Err(error) => {
                                    return Err(Self::Error::#field_ident_upper_camel_case_token_stream {
                                        #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: error,
                                        #field_code_occurence_new_68674e53_54cf_4cfe_b532_2e4aecda32c5_token_stream,
                                    });
                                }
                            }
                        }
                        Some(values)
                    }
                    None => None,
                }
            }
        }
    };
    quote::quote! {
        let #field_ident = #initialization_token_stream;
    }
}

fn generate_let_field_ident_value_inner_type_with_serialize_deserialize_from_token_stream(element: &SynFieldWithAdditionalInfo<'_>) -> proc_macro2::TokenStream {
    let field_ident = &element.field_ident;
    let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
    let from_snake_case = naming_constants::FromSnakeCase;
    let value_snake_case = naming_constants::ValueSnakeCase;
    quote::quote! {
        let #field_ident = #inner_type_with_serialize_deserialize_token_stream::#from_snake_case(#value_snake_case.#field_ident);//todo from or try_from
    }
}

fn generate_let_field_ident_value_option_vec_with_serialize_deserialize_token_stream(element: &SynFieldWithAdditionalInfo<'_>) -> proc_macro2::TokenStream {
    let field_ident = &element.field_ident;
    let where_inner_type_with_serialize_deserialize_token_stream = &element.where_inner_type_with_serialize_deserialize_token_stream;
    let from_snake_case = naming_constants::FromSnakeCase;
    let value_snake_case = naming_constants::ValueSnakeCase;
    quote::quote! {
        let #field_ident = match #value_snake_case.#field_ident {
            Some(#value_snake_case) => Some(#value_snake_case.into_iter().map(|element|#where_inner_type_with_serialize_deserialize_token_stream::#from_snake_case(element)).collect()),
            None => None
        };
    }
}

fn generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_token_stream(
    value: &SynFieldWithAdditionalInfo<'_>,
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
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
            let field_ident_upper_camel_case_token_stream = {
                //todo its a temporal naming desicion. maybe it can be better
                let value = syn_ident_to_upper_camel_case_stringified(value.field_ident);
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
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
}

fn generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
    value: &[SynFieldWithAdditionalInfo<'_>],
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
) -> std::vec::Vec<proc_macro2::TokenStream> {
    value.iter().map(|element| generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_token_stream(
        element,
        primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
    )).collect()
}

fn generate_where_inner_type_from_or_try_from_where_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
    value: &[SynFieldWithAdditionalInfo<'_>],
    code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence: &token_patterns::CodeOccurenceSnakeCaseDoubleDotSpaceErrorOccurenceLibCodeOccurenceCodeOccurence,
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
    eo_error_occurence_attribute_token_stream: &proc_macro2::TokenStream,
) -> std::vec::Vec<proc_macro2::TokenStream> {
    value.iter().map(|element| {
        match element.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
            postgresql_crud_common::FromOrTryFrom::From => proc_macro2::TokenStream::new(),
            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                let where_with_serialize_deserialize_error_named_token_stream = {
                    let value = element.rust_sqlx_map_to_postgres_type_variant.get_where_with_serialize_deserialize_error_named_stringified("");
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let field_ident_upper_camel_case_token_stream = {
                    //todo its a temporal naming desicion. maybe it can be better
                    let value = syn_ident_to_upper_camel_case_stringified(element.field_ident);
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote!{
                    #field_ident_upper_camel_case_token_stream {
                        #eo_error_occurence_attribute_token_stream
                        #primary_key_supported_sqlx_postgres_type_snake_case_token_stream: #where_with_serialize_deserialize_error_named_token_stream,
                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence,
                    },//must use comma here
                }
            }
        }
    }).collect()
}

fn generate_error_occurence_variant_token_stream(
    error_variant: &syn::Variant,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
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
}

fn construct_try_operation_route_logic_error_named_with_serialize_deserialize_syn_variant(
    value: &Operation,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> syn::Variant {
    let try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified = naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCaseStringified::try_self_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified(*&value);
    let try_operation_route_logic_error_named_with_serialize_deserialize_snake_case_stringified = naming_conventions::TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCaseStringified::try_self_route_logic_error_named_with_serialize_deserialize_snake_case_stringified(*&value);
    proc_macro_helpers::construct_syn_variant::construct_syn_variant(
        &try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified,
        vec![
            (
                proc_macro_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &try_operation_route_logic_error_named_with_serialize_deserialize_snake_case_stringified,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    &[&try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case_stringified],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            )
        ],
        &proc_macro_name_upper_camel_case_ident_stringified,
    )
}

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

fn generate_additional_error_variants(
    ast: &syn::DeriveInput,
    generate_postgresql_crud_attribute: GeneratePostgresqlCrudAttribute,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> std::vec::Vec<syn::Variant> {
    let generate_postgresql_crud_attribute_stringified = generate_postgresql_crud_attribute.to_string();
    let common_additional_error_variants_attribute_token_stream = proc_macro_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(
        &ast.attrs,
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
}

fn generate_error_initialization_eprintln_response_creation_token_stream(
    operation: &Operation,
    syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    try_operation_route_logic_response_variants_initialization_token_stream: &proc_macro2::TokenStream,
    status_code_token_stream: &proc_macro2::TokenStream,
    eprintln_error_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let try_operation_route_logic_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedUpperCamelCaseTokenStream::try_self_route_logic_error_named_upper_camel_case_token_stream(operation);
    let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(operation);
    let error_snake_case = naming_constants::ErrorSnakeCase;
    let response_snake_case = naming_constants::ResponseSnakeCase;
    let into_response_snake_case = naming_conventions::IntoResponseSnakeCase;
    quote::quote! {
        let #error_snake_case = #try_operation_route_logic_error_named_upper_camel_case_token_stream::#syn_variant_initialization_token_stream;
        #eprintln_error_token_stream
        let mut #response_snake_case = axum::response::IntoResponse::#into_response_snake_case(axum::Json(#try_operation_route_logic_response_variants_upper_camel_case_token_stream::#try_operation_route_logic_response_variants_initialization_token_stream));
        *#response_snake_case.status_mut() = #status_code_token_stream;
        return #response_snake_case;
    }
}


fn generate_operation_many_payload_element_token_stream(
    operation: &Operation,
    fields_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let derive_debug = token_patterns::DeriveDebug;
    let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(operation);
    quote::quote! {
        #derive_debug
        pub struct #operation_payload_element_upper_camel_case_token_stream {
            #fields_token_stream
        }
    }
}
fn generate_operation_many_payload_token_stream(
    operation: &Operation,
) -> proc_macro2::TokenStream {
    let derive_debug = token_patterns::DeriveDebug;
    let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
    let std_vec_vec_operation_payload_element_token_stream = operation.std_vec_vec_self_payload_element_token_stream();
    quote::quote! {
        #derive_debug
        pub struct #operation_payload_upper_camel_case_token_stream(pub #std_vec_vec_operation_payload_element_token_stream);
    }
}

fn generate_operation_many_payload_element_with_serialize_deserialize_token_stream(
    operation: &Operation,
    fields_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
    let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    quote::quote! {
        #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
        pub struct #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream {
            #fields_token_stream
        }
    }
}

fn generate_operation_many_payload_with_serialize_deserialize_token_stream(operation: &Operation) -> proc_macro2::TokenStream {
    let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let std_vec_vec_operation_payload_element_with_serialize_deserialize_token_stream = operation.std_vec_vec_self_payload_element_with_serialize_deserialize_token_stream();
    quote::quote! {
        #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
        pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream(pub #std_vec_vec_operation_payload_element_with_serialize_deserialize_token_stream);
    }
}

fn generate_impl_std_convert_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream(
    operation: &Operation,
    fields_named_excluding_primary_key: &std::vec::Vec<SynFieldWithAdditionalInfo<'_>>,
    fields_idents_excluding_primary_key_token_stream: &std::vec::Vec<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(operation);
    let fields_assignment_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter()
        .map(|element| generate_let_field_ident_value_inner_type_from_token_stream(element));
    proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
        &operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream,
        &operation_payload_element_upper_camel_case_token_stream,
        &{
            quote::quote! {
                #(#fields_assignment_excluding_primary_key_token_stream)*
                Self {
                    #(#fields_idents_excluding_primary_key_token_stream),*
                }
            }
        },
    )
}

fn generate_impl_std_convert_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream(
    operation: &Operation,
) -> proc_macro2::TokenStream {
    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
    let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(operation);
    let from_snake_case = naming_constants::FromSnakeCase;
    let value_snake_case = naming_constants::ValueSnakeCase;
    proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
        &operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
        &operation_payload_upper_camel_case_token_stream,
        &{
            quote::quote! {
                let mut elements = std::vec::Vec::with_capacity(#value_snake_case.0.len());
                for element in #value_snake_case.0 {//todo refactor
                    elements.push(#operation_payload_element_upper_camel_case_token_stream::#from_snake_case(element));
                }
                Self(elements)
            }
        },
    )
}

fn generate_impl_std_convert_from_operation_payload_and_paylaod_element_with_serialize_deserialize_for_operation_payload_and_payload_element_token_stream(
    operation: &Operation,
    fields_named_excluding_primary_key: &std::vec::Vec<SynFieldWithAdditionalInfo<'_>>,
    fields_idents_excluding_primary_key_token_stream: &std::vec::Vec<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let impl_std_convert_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream = generate_impl_std_convert_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream(
        &operation,
        &fields_named_excluding_primary_key,
        &fields_idents_excluding_primary_key_token_stream,
    );
    let impl_std_convert_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream(
        &operation
    );
    quote::quote! {
        #impl_std_convert_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream
        #impl_std_convert_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
    }
}

fn generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream(
    operation: &Operation,
    fields_named_excluding_primary_key: &std::vec::Vec<SynFieldWithAdditionalInfo<'_>>,
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
    let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(operation);
    let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
        &fields_named_excluding_primary_key,
        &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
    );
    quote::quote! {
        #derive_debug_thiserror_error_occurence
        pub enum #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
            #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
        }
    }
}

fn generate_impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream(
    operation: &Operation,
    fields_named_excluding_primary_key: &std::vec::Vec<SynFieldWithAdditionalInfo<'_>>,
    fields_idents_excluding_primary_key_token_stream: &std::vec::Vec<&syn::Ident>,
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(operation);
    let field_code_occurence_new_ac56a8a4_9fc6_430b_b511_a779d7e07be4_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        proc_macro_name_upper_camel_case_ident_stringified,
    );
    let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(operation);
    let fields_assignment_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter()
        .map(|element| generate_let_field_ident_value_field_ident_try_from_token_stream(
            element,
            &proc_macro_name_upper_camel_case_ident_stringified,
            &field_code_occurence_new_ac56a8a4_9fc6_430b_b511_a779d7e07be4_token_stream,
            &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
        ));
    //todo it can be std::convert::From if all #(#fields_assignment_excluding_primary_key_token_stream)* are impl from
    proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
        &operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream,
        &operation_payload_element_upper_camel_case_token_stream,
        &operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream,
        &{
            quote::quote! {
                #(#fields_assignment_excluding_primary_key_token_stream)*
                Ok(Self {
                    #(#fields_idents_excluding_primary_key_token_stream),*
                })
            }
        },
    )
}

fn generate_impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream(
    operation: &Operation,
    fields_named_excluding_primary_key: &std::vec::Vec<SynFieldWithAdditionalInfo<'_>>,
    fields_idents_excluding_primary_key_token_stream: &std::vec::Vec<&syn::Ident>,
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream(
        &operation,
        &fields_named_excluding_primary_key,
        &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
    );
    let impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream = generate_impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream(
        &operation,
        &fields_named_excluding_primary_key,
        &fields_idents_excluding_primary_key_token_stream,
        &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    quote::quote! {
        #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream
        #impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_element_upper_camel_case_token_stream
    }
}

fn generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
    operation: &Operation,
    variants_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(operation);
    quote::quote! {
        #derive_debug_thiserror_error_occurence
        pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
            #variants_token_stream
        }
    }
}

fn generate_impl_std_convert_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_upper_camel_case_token_stream(
    operation: &Operation,
    error_initialization_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(operation);
    let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
    let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(operation);
    let error_snake_case = naming_constants::ErrorSnakeCase;
    let value_snake_case = naming_constants::ValueSnakeCase;
    proc_macro_helpers::generate_impl_std_convert_try_from_token_stream::generate_impl_std_convert_try_from_token_stream(
        &operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
        &operation_payload_upper_camel_case_token_stream,
        &operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream,
        &{
            quote::quote! {
                let mut elements = std::vec::Vec::with_capacity(#value_snake_case.0.len());
                for element in #value_snake_case.0 {
                    match #operation_payload_element_upper_camel_case_token_stream::try_from(element) {
                        Ok(#value_snake_case) => {
                            elements.push(#value_snake_case);
                        },
                        Err(#error_snake_case) => {
                            return Err(Self::Error::#error_initialization_token_stream);
                        }
                    }
                }
                Ok(Self(elements))
            }
        },
    )
}

fn generate_impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream(
    operation: &Operation,
    error_variant_declaration_token_stream: &proc_macro2::TokenStream,
    error_variant_initialization_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = generate_operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream(
        &operation,
        &error_variant_declaration_token_stream
    );
    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_upper_camel_case_token_stream = generate_impl_std_convert_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_upper_camel_case_token_stream(
        &operation,
        &error_variant_initialization_token_stream,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    quote::quote! {
        #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
        #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream_for_operation_payload_upper_camel_case_token_stream
    }
}

fn generate_impl_std_convert_try_from_operation_payload_and_payload_element_with_serialize_deserialize_for_operation_payload_and_payload_element_token_stream(
    operation: &Operation,
    error_variant_declaration_token_stream: &proc_macro2::TokenStream,
    error_variant_initialization_token_stream: &proc_macro2::TokenStream,
    fields_named_excluding_primary_key: &std::vec::Vec<SynFieldWithAdditionalInfo<'_>>,
    fields_idents_excluding_primary_key_token_stream: &std::vec::Vec<&syn::Ident>,
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    let impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream = generate_impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream(
        &operation,
        &fields_named_excluding_primary_key,
        &fields_idents_excluding_primary_key_token_stream,
        &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = generate_impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream(
        &operation,
        &error_variant_declaration_token_stream,
        &error_variant_initialization_token_stream,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    quote::quote! {
        #impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream
        #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
    }
}

fn generate_impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream(
    operation: &Operation,
    error_variant_declaration_token_stream: &proc_macro2::TokenStream,
    error_variant_initialization_token_stream: &proc_macro2::TokenStream,
    fields_named_excluding_primary_key: &std::vec::Vec<SynFieldWithAdditionalInfo<'_>>,
    fields_idents_excluding_primary_key_token_stream: &std::vec::Vec<&syn::Ident>,
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
    from_or_try_from: &postgresql_crud_common::FromOrTryFrom,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    match from_or_try_from {
        postgresql_crud_common::FromOrTryFrom::From => generate_impl_std_convert_from_operation_payload_and_paylaod_element_with_serialize_deserialize_for_operation_payload_and_payload_element_token_stream(
            &operation,
            &fields_named_excluding_primary_key,
            &fields_idents_excluding_primary_key_token_stream,
        ),
        postgresql_crud_common::FromOrTryFrom::TryFrom => generate_impl_std_convert_try_from_operation_payload_and_payload_element_with_serialize_deserialize_for_operation_payload_and_payload_element_token_stream(
            &operation,
            &error_variant_declaration_token_stream,
            &error_variant_initialization_token_stream,
            &fields_named_excluding_primary_key,
            &fields_idents_excluding_primary_key_token_stream,
            &primary_key_supported_sqlx_postgres_type_snake_case_token_stream,
            &proc_macro_name_upper_camel_case_ident_stringified,
        )
    }
}

fn generate_impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream(
    operation: &Operation,
    fields_named_excluding_primary_key: &std::vec::Vec<SynFieldWithAdditionalInfo<'_>>,
    fields_idents_excluding_primary_key_token_stream: &std::vec::Vec<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let operation_payload_element_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(operation);
    let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let from_snake_case = naming_constants::FromSnakeCase;
    let fields_assignment_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter()
        .map(|element|generate_let_field_ident_value_inner_type_with_serialize_deserialize_from_token_stream(element));
    let value_snake_case = naming_constants::ValueSnakeCase;
    proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
        &operation_payload_element_upper_camel_case_token_stream,
        &operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream,
        &quote::quote!{
                #(#fields_assignment_excluding_primary_key_token_stream)*
                Self{
                    #(#fields_idents_excluding_primary_key_token_stream),*
                }
        },
    )
}

fn generate_impl_std_convert_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation: &Operation) -> proc_macro2::TokenStream {
    let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let from_snake_case = naming_constants::FromSnakeCase;
    let value_snake_case = naming_constants::ValueSnakeCase;
    proc_macro_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
        &operation_payload_upper_camel_case_token_stream,
        &operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
        &quote::quote!{
            Self(
                #value_snake_case.0.into_iter()
                .map(|element|#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream::#from_snake_case(element))
                .collect::<std::vec::Vec<#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream>>()
            )
        },
    )
}

fn generate_impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream(
    operation: &Operation,
    fields_named_excluding_primary_key: &std::vec::Vec<SynFieldWithAdditionalInfo<'_>>,
    fields_idents_excluding_primary_key_token_stream: &std::vec::Vec<&syn::Ident>,
) -> proc_macro2::TokenStream {
    let impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream = generate_impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream(
        &operation,
        &fields_named_excluding_primary_key,
        &fields_idents_excluding_primary_key_token_stream,
    );
    let impl_std_convert_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = generate_impl_std_convert_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
    quote::quote! {
        #impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream
        #impl_std_convert_from_operation_payload_upper_camel_case_token_stream_for_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream
    }
}

fn generate_parameters_token_stream(operation: &Operation) -> proc_macro2::TokenStream {
    let derive_debug = token_patterns::DeriveDebug;
    let operation_parameters_upper_camel_case_token_stream = naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
    let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
    let payload_snake_case = naming_constants::PayloadSnakeCase;
    quote::quote! {
        #derive_debug
        pub struct #operation_parameters_upper_camel_case_token_stream {//todo maybe not need additional info, so parameters wrapper potentially can be removed
            pub #payload_snake_case: #operation_payload_upper_camel_case_token_stream,
        }
    }
}

///////////////
fn generate_try_operation_route_logic_response_variants_token_stream(
    operation: &Operation,
    desirable_type_token_stream: &proc_macro2::TokenStream,
    type_variants_from_request_response_syn_variants: &std::vec::Vec<syn::Variant>,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    let derive_debug_serde_serialize_serde_deserialize = token_patterns::DeriveDebugSerdeSerializeSerdeDeserialize;
    let try_operation_route_logic_response_variants_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicResponseVariantsUpperCamelCaseTokenStream::try_self_route_logic_response_variants_upper_camel_case_token_stream(operation);
    let desirable_upper_camel_case = naming_constants::DesirableUpperCamelCase;
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
}

fn generate_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream(
    operation: &Operation,
    type_variants_from_request_response_syn_variants: &std::vec::Vec<syn::Variant>,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
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
}


fn generate_try_operation_route_logic_error_named_token_stream(
    operation: &Operation,
    type_variants_from_request_response_syn_variants: &std::vec::Vec<syn::Variant>,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    let try_operation_route_logic_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfRouteLogicErrorNamedUpperCamelCaseTokenStream::try_self_route_logic_error_named_upper_camel_case_token_stream(operation);
    let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(|element|generate_error_occurence_variant_token_stream(
        &element,
        &proc_macro_name_upper_camel_case_ident_stringified,
    ));
    quote::quote! {
        #[derive(
            Debug,
            thiserror::Error,
            error_occurence_lib::ErrorOccurence,
        )]
        pub enum #try_operation_route_logic_error_named_upper_camel_case_token_stream {
            #(#variants_token_stream),*
        }
    }
}

fn generate_parameters_logic_token_stream(
    operation: &Operation,
    from_or_try_from: &postgresql_crud_common::FromOrTryFrom,
    json_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    json_syn_variant_status_code: &proc_macro_helpers::status_code::StatusCode,
    eprintln_error_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    let value_snake_case = naming_constants::ValueSnakeCase;
    let error_snake_case = naming_constants::ErrorSnakeCase;
    let parameters_snake_case = naming_constants::ParametersSnakeCase;
    let payload_snake_case = naming_constants::PayloadSnakeCase;
    let from_snake_case = naming_constants::FromSnakeCase;
    let body_bytes_snake_case = naming_conventions::BodyBytesSnakeCase;
    let try_from_snake_case = naming_conventions::TryFromSnakeCase;
    let try_or_try_from_operation_payload_upper_camel_case_token_stream = {
        let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
        match from_or_try_from {
            postgresql_crud_common::FromOrTryFrom::From => quote::quote! {#operation_payload_upper_camel_case_token_stream::#from_snake_case(#value_snake_case)},
            postgresql_crud_common::FromOrTryFrom::TryFrom => {
                let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
                    operation,
                    &operation.operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant_initialization_token_stream(
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    ),
                    &quote::quote! {#from_snake_case(#error_snake_case)},
                    &operation.operation_payload_try_from_operation_payload_with_serialize_deserialize_status_code().to_axum_http_status_code_token_stream(),
                    &eprintln_error_token_stream,
                );
                quote::quote! {
                    match #operation_payload_upper_camel_case_token_stream::#try_from_snake_case(#value_snake_case) {
                        Ok(#value_snake_case) => #value_snake_case,
                        Err(#error_snake_case) => {
                            #error_initialization_eprintln_response_creation_token_stream
                        }
                    }
                }
            },
        }
    };
    let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
        operation,
        &json_syn_variant_initialization_token_stream,
        &quote::quote! {#from_snake_case(#error_snake_case)},
        &json_syn_variant_status_code.to_axum_http_status_code_token_stream(),
        &eprintln_error_token_stream,
    );
    let operation_parameters_upper_camel_case_token_stream = naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    quote::quote! {
        let #parameters_snake_case = #operation_parameters_upper_camel_case_token_stream {
            #payload_snake_case: match axum::Json::<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>::from_bytes(
                &#body_bytes_snake_case,
            ) {
                Ok(axum::Json(#value_snake_case)) => #try_or_try_from_operation_payload_upper_camel_case_token_stream,
                Err(#error_snake_case) => {
                    #error_initialization_eprintln_response_creation_token_stream
                }
            },
        };
    }
}

fn generate_try_operation_route_logic_snake_case_token_stream(
    operation: &Operation,
    ast: &syn::DeriveInput,
    common_additional_route_logic_token_stream: &proc_macro2::TokenStream,
    parameters_logic_token_stream: &proc_macro2::TokenStream,
    expected_updated_primary_keys_token_stream: &proc_macro2::TokenStream,
    query_string_token_stream: &proc_macro2::TokenStream,
    binded_query_token_stream: &proc_macro2::TokenStream,
    postgresql_logic_token_stream: &proc_macro2::TokenStream,
    eprintln_error_token_stream: &proc_macro2::TokenStream,
    check_body_size_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    postgresql_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    let try_operation_route_logic_snake_case_token_stream = naming_conventions::TrySelfRouteLogicSnakeCaseTokenStream::try_self_route_logic_snake_case_token_stream(operation);
    let axum_response_response_token_stream = quote::quote!{axum::response::Response};
    let parameters_snake_case = naming_constants::ParametersSnakeCase;
    let query_string_snake_case = naming_conventions::QueryStringSnakeCase;
    let binded_query_snake_case = naming_conventions::BindedQuerySnakeCase;
    let value_snake_case = naming_constants::ValueSnakeCase;
    let error_snake_case = naming_constants::ErrorSnakeCase;
    let status_code_snake_case = naming_conventions::StatusCodeSnakeCase;
    let request_parts_preparation_token_stream = {
        let error_initialization_eprintln_response_creation_token_stream = generate_error_initialization_eprintln_response_creation_token_stream(
            &operation,
            &check_body_size_syn_variant_initialization_token_stream,
            &{
                let from_snake_case = naming_constants::FromSnakeCase;
                quote::quote! {#from_snake_case(#error_snake_case)}
            },
            &quote::quote! {#status_code_snake_case},
            &eprintln_error_token_stream,
        );
        quote::quote! {
            let (parts, body) = request.into_parts();
            let headers = parts.headers;
            let body_bytes = match route_validators::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_snake_case) => {
                    let #status_code_snake_case = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&#error_snake_case);
                    #error_initialization_eprintln_response_creation_token_stream
                }
            };
        }
    };
    let additional_validators_token_stream = {
        let operation_additional_route_logic_token_stream = proc_macro_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(
            &ast.attrs,
            &operation.to_additional_route_logic().generate_path_to_attribute(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            #common_additional_route_logic_token_stream
            #operation_additional_route_logic_token_stream
        }
    };
    let acquire_pool_and_connection_token_stream = acquire_pool_and_connection(
        &operation,
        &postgresql_syn_variant_initialization_token_stream,
        &eprintln_error_token_stream,
    );
    let desirable_response_creation_token_stream = {
        let response_snake_case = naming_constants::ResponseSnakeCase;
        let into_response_snake_case = naming_conventions::IntoResponseSnakeCase;
        let desirable_upper_camel_case = naming_constants::DesirableUpperCamelCase;
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
            app_state: axum::extract::State<
                crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits,
            >,
            request: axum::extract::Request,
        ) -> #axum_response_response_token_stream {
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
}

fn generate_try_operation_error_named_token_stream(
    operation: &Operation,
    common_http_request_syn_variants: &std::vec::Vec<syn::Variant>,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> proc_macro2::TokenStream {
    let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
    let try_operation_error_named_upper_camel_case_token_stream = naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
    let syn_variants = {
        let mut value = std::vec::Vec::with_capacity(common_http_request_syn_variants.len() + 1);
        for element in common_http_request_syn_variants {
            value.push(element.clone());
        }
        value.push(construct_try_operation_route_logic_error_named_with_serialize_deserialize_syn_variant(
            &operation,
            &proc_macro_name_upper_camel_case_ident_stringified,
        ));
        value
    };
    let variants_token_stream = syn_variants.iter().map(|element|generate_error_occurence_variant_token_stream(
        &element,
        &proc_macro_name_upper_camel_case_ident_stringified,
    ));
    quote::quote! {
        #derive_debug_thiserror_error_occurence
        pub enum #try_operation_error_named_upper_camel_case_token_stream {
            #(#variants_token_stream),*
        }
    }
}

fn generate_type_variants_from_request_response_syn_variants(
    syn_variants: &std::vec::Vec<&syn::Variant>,
    from_or_try_from: &postgresql_crud_common::FromOrTryFrom,
    operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant: &syn::Variant,
    operation: &Operation,
    ast: &syn::DeriveInput,
    proc_macro_name_upper_camel_case_ident_stringified: &std::primitive::str,
) -> std::vec::Vec<syn::Variant> {
    let mut type_variants_from_request_response_syn_variants = std::vec::Vec::new();
    for element in syn_variants {
        type_variants_from_request_response_syn_variants.push((*element).clone());
    }
    let operation_additional_error_variants = generate_additional_error_variants(
        &ast,
        operation.to_additional_error_variants(),
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    for element in operation_additional_error_variants {
        type_variants_from_request_response_syn_variants.push(element.clone());
    }
    type_variants_from_request_response_syn_variants.push(operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server_syn_variant.clone());
    if *from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = operation.operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant(
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        type_variants_from_request_response_syn_variants.push(operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant.clone());
    }
    type_variants_from_request_response_syn_variants
}

///////////////////////
fn generate_operation_payload_token_stream(
    operation: &Operation,
    fields_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let derive_debug = token_patterns::DeriveDebug;
    let operation_payload_upper_camel_case_token_stream = naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(operation);
    quote::quote! {
        #derive_debug
        pub struct #operation_payload_upper_camel_case_token_stream {
            #fields_token_stream
        }
    }
}

fn generate_payload_with_serialize_deserialize_token_stream(
    operation: &Operation,
    fields_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    quote::quote! {
        #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
        pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
            #fields_token_stream
        }
    }
}

//
fn generate_operation_payload_with_serialize_deserialize_try_from_operation_payload_error_named_token_stream(
    operation: &Operation,
    variants_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
    let operation_payload_with_serialize_deserialize_try_from_operation_payload_error_named_upper_camel_case_token_stream = naming_conventions::SelfPayloadWithSerializeDeserializeTryFromSelfPayloadErrorNamedUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_try_from_self_payload_error_named_upper_camel_case_token_stream(operation);
    quote::quote! {
        #derive_debug_thiserror_error_occurence
        pub enum #operation_payload_with_serialize_deserialize_try_from_operation_payload_error_named_upper_camel_case_token_stream {
            #variants_token_stream
        }
    }
}

fn generate_operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(
    value: &SynFieldWithAdditionalInfo<'_>,
    operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    primary_key_supported_sqlx_postgres_type_snake_case_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let code_occurence_snake_case = naming_conventions::CodeOccurenceSnakeCase;
    match value.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize() {
        postgresql_crud_common::FromOrTryFrom::From => proc_macro2::TokenStream::new(),
        postgresql_crud_common::FromOrTryFrom::TryFrom => {
            let field_ident_upper_camel_case_token_stream = {
                //todo its a temporal naming desicion. maybe it can be better
                let value = syn_ident_to_upper_camel_case_stringified(value.field_ident);
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
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
}

fn generate_payload_and_payload_with_serialize_deserialize_create_many_or_update_many(
    operation: &Operation,
    fields_token_stream: &proc_macro2::TokenStream,
    fields_with_serialize_deserialize_token_stream: &proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let payload_token_stream = {
        let operation_payload_element_token_stream = generate_operation_many_payload_element_token_stream(
            &operation,
            &fields_token_stream
        );
        let operation_payload_token_stream = generate_operation_many_payload_token_stream(
            &operation,
        );
        quote::quote! {
            #operation_payload_element_token_stream
            #operation_payload_token_stream
        }
    };
    let payload_with_serialize_deserialize_token_stream = {
        let operation_payload_element_with_serialize_deserialize_token_stream = generate_operation_many_payload_element_with_serialize_deserialize_token_stream(
            &operation,
            &fields_with_serialize_deserialize_token_stream
        );
        let operation_payload_with_serialize_deserialize_token_stream = generate_operation_many_payload_with_serialize_deserialize_token_stream(
            &operation,
        );
        quote::quote! {
            #operation_payload_element_with_serialize_deserialize_token_stream
            #operation_payload_with_serialize_deserialize_token_stream
        }
    };
    (
        payload_token_stream,
        payload_with_serialize_deserialize_token_stream
    )
}

fn generate_parameters_pattern_token_stream(
    operation: &Operation,
    payload_token_stream: proc_macro2::TokenStream,
    payload_with_serialize_deserialize_token_stream: proc_macro2::TokenStream,
    impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream: proc_macro2::TokenStream,
    impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let parameters_token_stream = generate_parameters_token_stream(&operation);
    quote::quote! {
        #payload_token_stream
        #payload_with_serialize_deserialize_token_stream
        #impl_std_convert_from_or_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
        #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream
        #parameters_token_stream
    }
}