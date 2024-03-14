mod acquire_pool_and_connection;
mod check_for_none;
mod column_names_factorial;
mod extract_syn_variants_from_proc_macro_attribute;
mod from_log_and_return_error;
mod generate_postgres_transaction;
mod type_variants_from_request_response_generator;

// trait Something {
//     fn something();
// }

// impl Something for i8 {
//     fn something() {
//         println!("");
//     }
// }

// impl Something for sqlx::types::Uuid {
//     fn something() {}
// }

// Rust type	Postgres type(s)
// bool	BOOL
// i8	“CHAR”
// i16	SMALLINT, SMALLSERIAL, INT2
// i32	INT, SERIAL, INT4
// i64	BIGINT, BIGSERIAL, INT8
// f32	REAL, FLOAT4
// f64	DOUBLE PRECISION, FLOAT8
// &str, std::string::String	VARCHAR, CHAR(N), TEXT, NAME
// &[u8], std::vec::Vec<u8>	BYTEA
// ()	VOID
// PgInterval	INTERVAL
// PgRange<T>	INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE
// PgMoney	MONEY
// PgLTree	LTREE
// PgLQuery	LQUERY

//todo generate for each create update delete body length checked and for path query headers too
//todo how to write filter logic for sqlx rust postgresql types?
//todo decide where to do error log (maybe add in some places)
//todo clear unnesesary generated returns.
// unneeded `return` statement
// for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_return
// `#[warn(clippy::needless_return)]`
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

#[proc_macro_attribute]
pub fn create_many_additional_http_status_codes_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn create_one_additional_http_status_codes_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_one_additional_http_status_codes_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_many_additional_http_status_codes_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_one_additional_http_status_codes_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_many_additional_http_status_codes_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_one_additional_http_status_codes_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_many_additional_http_status_codes_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_attribute]
pub fn additional_http_status_codes_error_variants(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_derive(GeneratePostgresqlCrud)] //todo check on postgresql max length value of type
pub fn generate_postgresql_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GeneratePostgresqlCrud";
    let proc_macro_name_snake_case =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &proc_macro_name_upper_camel_case,
        );
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {e}",
            proc_macro_common::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", ast.data);
    let ident = &ast.ident;
    let ident_snake_case_stringified =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &ident.to_string(),
        );
    let proc_macro_name_upper_camel_case_ident_stringified =
        format!("{proc_macro_name_upper_camel_case} {ident}");
    let table_name_stringified = pluralizer::pluralize(&ident_snake_case_stringified, 2, false);
    let table_name_quotes_token_stream =
        proc_macro_common::generate_quotes::generate_quotes_token_stream(
            &table_name_stringified,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
    let str_ref_token_stream = proc_macro_common::str_ref_token_stream();
    let table_name_declaration_token_stream = quote::quote! {pub const TABLE_NAME: #str_ref_token_stream = #table_name_quotes_token_stream;};
    let fields_named = if let syn::Data::Struct(data_struct) = &ast.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            &fields_named.named
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Named");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let primary_key_syn_field_with_additional_info = {
        let primary_key_attr_name = "generate_postgresql_crud_primary_key";
        let mut primary_key_field_option = None;
        for element in fields_named {
            match &element.ty {
                syn::Type::Path(value) => match value.path.segments.len() == 2 {
                    true => {
                        if value.path.segments[0].ident != postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} field_type is not syn::Type::Path");
                        }
                        match value.path.segments[0].arguments {
                            syn::PathArguments::None => (),
                            _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} value.path.segments[0].arguments != syn::PathArguments::None")
                        }
                        let maybe_generic_token_stream = match &value.path.segments[1].arguments {
                            syn::PathArguments::None => quote::quote!{},
                            syn::PathArguments::AngleBracketed(value) => {
                                quote::quote!{#value}//< test_mod :: Something >
                            },
                            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} does not support syn::PathArguments::Parenthesized"),
                        };
                        match postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::try_from(&value.path.segments[1].ident.to_string() as &str) {
                            Ok(value) => match value {
                                //todo maybe support other postgresql primary key types
                                postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => match primary_key_field_option {
                                    Some(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} must have one PrimaryKey"),
                                    None => {
                                        primary_key_field_option = Some(SynFieldWithAdditionalInfo::from(element.clone()));
                                    },
                                },
                                _ => ()
                            },
                            Err(e) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} RustSqlxMapToPostgresTypeVariant::try_from failed {e}")
                        }
                    },
                    false => panic!("{proc_macro_name_upper_camel_case_ident_stringified} field_type is not syn::Type::Path")
                },
                _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} field_type is not syn::Type::Path")
            }
        }
        match primary_key_field_option {
            Some(value) => value,
            None => panic!("{proc_macro_name_upper_camel_case_ident_stringified} no {primary_key_attr_name} attribute"),
        }
    };
    let primary_key_field_ident = &primary_key_syn_field_with_additional_info.field_ident;
    let sqlx_types_uuid_stringified = naming_constants::SQLX_TYPES_UUID_STRINGIFIED;
    let sqlx_types_uuid_token_stream = {
        sqlx_types_uuid_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {sqlx_types_uuid_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_string_string_token_stream = proc_macro_common::std_string_string_token_stream();
    let field_ident_is_none_stringified = naming_constants::FIELD_IDENT_IS_NONE;
    let fields_named_wrappers_excluding_primary_key = fields_named
        .clone()
        .into_iter()
        .filter(|field| *field != primary_key_syn_field_with_additional_info.field)
        .map(|element| SynFieldWithAdditionalInfo::from(element))
        .collect::<std::vec::Vec<SynFieldWithAdditionalInfo>>();
    let fields_named_len = fields_named.len();
    if fields_named_len <= 1 {
        panic!(
            "{proc_macro_name_upper_camel_case_ident_stringified} false = fields_named.len() > 1"
        );
    }
    let fields_named_wrappers_excluding_primary_key_len = fields_named_wrappers_excluding_primary_key.len();
    let is_fields_named_wrappers_excluding_primary_key_len_equals_one = fields_named_wrappers_excluding_primary_key_len == 1;
    let primary_key_field_ident_quotes_token_stream = proc_macro_common::generate_quotes::generate_quotes_token_stream(
        &primary_key_field_ident.to_string(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let debug_token_stream =
        proc_macro_helpers::naming_conventions::debug_upper_camel_case_token_stream();
    let thiserror_error_token_stream = proc_macro_common::thiserror_error_token_stream();
    let error_occurence_error_occurence_token_stream =
        proc_macro_common::error_occurence_lib_error_occurence_token_stream();
    let utoipa_to_schema_token_stream = proc_macro_common::utoipa_to_schema_token_stream();
    let serde_serialize_token_stream = proc_macro_common::serde_serialize_token_stream();
    let serde_deserialize_token_stream = proc_macro_common::serde_deserialize_token_stream();
    let derive_debug_token_stream =
        proc_macro_helpers::wrap_derive::wrap_derive_token_stream(&[&debug_token_stream]);
    let derive_debug_thiserror_error_occurence_token_stream =
        proc_macro_helpers::wrap_derive::wrap_derive_token_stream(&[
            &debug_token_stream,
            &thiserror_error_token_stream,
            &error_occurence_error_occurence_token_stream,
        ]);
    let derive_debug_to_schema_token_stream =
        proc_macro_helpers::wrap_derive::wrap_derive_token_stream(&[
            &debug_token_stream,
            &utoipa_to_schema_token_stream,
        ]);
    let derive_debug_serialize_deserialize_token_stream =
        proc_macro_helpers::wrap_derive::wrap_derive_token_stream(&[
            &debug_token_stream,
            &serde_serialize_token_stream,
            &serde_deserialize_token_stream,
        ]);
    let derive_debug_serialize_deserialize_to_schema_token_stream =
        proc_macro_helpers::wrap_derive::wrap_derive_token_stream(&[
            &debug_token_stream,
            &serde_serialize_token_stream,
            &serde_deserialize_token_stream,
            &utoipa_to_schema_token_stream,
        ]);
    let try_from_upper_camel_case_stringified =
        proc_macro_helpers::naming_conventions::try_from_upper_camel_case_stringified();
    let from_str_upper_camel_case_stringified =
        proc_macro_helpers::naming_conventions::from_str_upper_camel_case_stringified();
    let from_str_upper_camel_case_token_stream =
        proc_macro_helpers::naming_conventions::from_str_upper_camel_case_token_stream();
    let from_str_snake_case_stringified =
        proc_macro_helpers::naming_conventions::from_str_snake_case_stringified();
    let from_str_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::from_str_snake_case_token_stream();
    let sqlx_row_token_stream = proc_macro_common::sqlx_row_token_stream();
    let std_primitive_str_sqlx_column_index_token_stream =
        quote::quote! {&'a std::primitive::str: sqlx::ColumnIndex<R>,};
    let sqlx_decode_decode_database_token_stream =
        quote::quote! {sqlx::decode::Decode<'a, R::Database>};
    let sqlx_types_type_database_token_stream = quote::quote! {sqlx::types::Type<R::Database>};
    let primary_key_uuid_wrapper_try_from_sqlx_row_name_token_stream = quote::quote! {primary_key_uuid_wrapper_try_from_sqlx_row};
    let crate_server_postgres_uuid_wrapper_token_stream =
        quote::quote! {crate::server::postgres::uuid_wrapper};
    let error_named_upper_camel_case_stringified =
        proc_macro_helpers::naming_conventions::error_named_upper_camel_case_stringified();
    let uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_stringified = format!(
        "{}{}{try_from_upper_camel_case_stringified}{}{}{}",
        proc_macro_helpers::naming_conventions::uuid_upper_camel_case_stringified(),
        proc_macro_helpers::naming_conventions::wrapper_upper_camel_case_stringified(),
        proc_macro_helpers::naming_conventions::possible_upper_camel_case_stringified(),
        proc_macro_helpers::naming_conventions::uuid_upper_camel_case_stringified(),
        proc_macro_helpers::naming_conventions::wrapper_upper_camel_case_stringified(),
    );
    let uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_token_stream = {
        uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let uuid_wrapper_try_from_possible_uuid_wrapper_snake_case_token_stream = {
        let uuid_wrapper_try_from_possible_uuid_wrapper_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_stringified.to_string()
        );
        uuid_wrapper_try_from_possible_uuid_wrapper_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {uuid_wrapper_try_from_possible_uuid_wrapper_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let uuid_wrapper_try_from_possible_uuid_wrapper_error_named_upper_camel_case_token_stream = {
        let uuid_wrapper_try_from_possible_uuid_wrapper_error_named_upper_camel_case_stringified = format!("{uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_stringified}{error_named_upper_camel_case_stringified}");
        uuid_wrapper_try_from_possible_uuid_wrapper_error_named_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {uuid_wrapper_try_from_possible_uuid_wrapper_error_named_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream = quote::quote! {#crate_server_postgres_uuid_wrapper_token_stream::#uuid_wrapper_try_from_possible_uuid_wrapper_error_named_upper_camel_case_token_stream};
    let crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream =
        quote::quote! {#crate_server_postgres_uuid_wrapper_token_stream::UuidWrapper};
    let std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream = quote::quote! {std::vec::Vec<#crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream>};
    let crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream =
        quote::quote! {#crate_server_postgres_uuid_wrapper_token_stream::PossibleUuidWrapper};
    let std_str_from_str_token_stream =
        quote::quote! {std::str::#from_str_upper_camel_case_token_stream};
    let struct_options_ident_stringified = format!(
        "{ident}{}",
        proc_macro_helpers::naming_conventions::options_upper_camel_case_stringified()
    );
    let struct_options_ident_token_stream = {
        struct_options_ident_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {struct_options_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let struct_options_token_stream = {
        let serde_skip_serializing_if_value_attribute_token_stream =
            quote::quote! {#[serde(skip_serializing_if = "Option::is_none")]};
        let field_option_primary_key_token_stream = {
            let inner_type_with_serialize_deserialize_token_stream =
                &primary_key_syn_field_with_additional_info
                    .inner_type_with_serialize_deserialize_token_stream;
            quote::quote! {
                #serde_skip_serializing_if_value_attribute_token_stream
                pub #primary_key_field_ident: std::option::Option<#inner_type_with_serialize_deserialize_token_stream>
            }
        };
        let fields_options_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| {
            let field_vis = &element.field.vis;
            let field_ident = &element.field_ident;
            let field_type_path = &element.field.ty;
            let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
            quote::quote!{
                #serde_skip_serializing_if_value_attribute_token_stream
                #field_vis #field_ident: std::option::Option<#inner_type_with_serialize_deserialize_token_stream>
            }
        });
        quote::quote! {
            #derive_debug_serialize_deserialize_token_stream
            pub struct #struct_options_ident_token_stream {
                #field_option_primary_key_token_stream,
                #(#fields_options_excluding_primary_key_token_stream),*
            }
        }
    };
    let super_path_token_stream = quote::quote! {}; //super::
    let from_ident_for_ident_options_token_stream = {
        let inner_type_with_serialize_deserialize_token_stream =
            &primary_key_syn_field_with_additional_info
                .inner_type_with_serialize_deserialize_token_stream;
        let ident_option_variant_primary_key_token_stream = quote::quote! {
            #primary_key_field_ident: Some(#inner_type_with_serialize_deserialize_token_stream::from(value.#primary_key_field_ident.0)),//todo from or try_from
        };
        let ident_option_variants_excluding_primary_key_token_stream =
            fields_named_wrappers_excluding_primary_key
                .iter()
                .map(|element| {
                    let field_ident = &element.field_ident;
                    let inner_type_with_serialize_deserialize_token_stream =
                        &element.inner_type_with_serialize_deserialize_token_stream;
                    quote::quote! {
                        #field_ident: Some(#inner_type_with_serialize_deserialize_token_stream::from(value.#field_ident.0))//todo from or try_from
                    }
                });
        quote::quote! {
            impl std::convert::From<#super_path_token_stream #ident> for #struct_options_ident_token_stream {
                fn from(value: #super_path_token_stream #ident) -> Self {
                    Self {
                        #ident_option_variant_primary_key_token_stream
                        #(#ident_option_variants_excluding_primary_key_token_stream),*
                    }
                }
            }
        }
    };
    // println!("{from_ident_for_ident_options_token_stream}");
    let column_variants = {
        let fields_named_enumerated = fields_named
            .iter()
            .enumerate()
            .collect::<std::vec::Vec<(usize, &syn::Field)>>();
        let fields_named_clone_stringified =
            fields_named.iter().collect::<std::vec::Vec<&syn::Field>>();
        let mut veced_vec = fields_named_clone_stringified
            .iter()
            .map(|field| vec![(*field).clone()])
            .collect::<std::vec::Vec<std::vec::Vec<syn::Field>>>();
        //todo rewrite in different way, coz compile time for many columns in not accaptable
        let column_variants_vec: std::vec::Vec<std::vec::Vec<syn::Field>> =
            crate::column_names_factorial::column_names_factorial(
                fields_named_enumerated,
                fields_named_clone_stringified,
                &mut veced_vec as &mut [std::vec::Vec<syn::Field>],
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        column_variants_vec
            .into_iter()
            .map(|elements| {
                elements
                    .into_iter()
                    .map(|element| SynFieldWithAdditionalInfo::from(element))
                    .collect::<std::vec::Vec<SynFieldWithAdditionalInfo>>()
            })
            .collect::<std::vec::Vec<std::vec::Vec<SynFieldWithAdditionalInfo>>>()
    };
    let structs_variants_token_stream = column_variants.iter().map(|variant_columns| {
        let struct_name_token_stream = {
            let struct_name_stringified = variant_columns.iter().fold(format!("{ident}"), |mut acc, element| {
                acc.push_str(&convert_case::Casing::to_case(
                    &element.field_ident.to_string(), 
                    convert_case::Case::UpperCamel
                ));
                acc
            });
            struct_name_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {struct_name_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let genereted_fields = variant_columns.iter().map(|variant_column|{
            let variant_column_ident = &variant_column.field_ident;
            let variant_column_type = &variant_column.field.ty;
            let inner_type_with_serialize_deserialize_token_stream = &variant_column.inner_type_with_serialize_deserialize_token_stream;
            quote::quote! {
                pub #variant_column_ident: #inner_type_with_serialize_deserialize_token_stream,//#variant_column_type
            }
        });
        quote::quote! {
            #derive_debug_token_stream
            pub struct #struct_name_token_stream {
                #(#genereted_fields)*
            }
        }
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let code_occurence_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::code_occurence_upper_camel_case_token_stream();
    let code_occurence_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::code_occurence_snake_case_token_stream();
    let error_occurence_lib_code_occurence_code_occurence_token_stream = quote::quote! {error_occurence_lib::#code_occurence_snake_case_token_stream::#code_occurence_upper_camel_case_token_stream};
    let code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream = quote::quote! {
        #code_occurence_snake_case_token_stream: #error_occurence_lib_code_occurence_code_occurence_token_stream
    };
    let eo_error_occurence_attribute_token_stream =
        proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence
            .to_attribute_view_token_stream();
    let eo_display_token_stream =
        proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay
            .to_attribute_view_token_stream();
    let eo_display_with_serialize_deserialize_token_stream = proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize.to_attribute_view_token_stream();
    let eo_display_foreign_type_token_stream =
        proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayForeignType
            .to_attribute_view_token_stream();
    let eo_vec_error_occurence_token_stream =
        proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecErrorOccurence
            .to_attribute_view_token_stream();
    let error_value_snake_case_token_stream =
        proc_macro_common::error_value_snake_case_token_stream();
    let impl_std_convert_try_from_ident_options_for_struct_variants_token_stream = {
        column_variants
            .iter()
            .map(|variant_columns| {
                let struct_name_stringified = {
                    let variant_columns_merged_upper_camel_case_stringified = variant_columns.iter().fold(std::string::String::from(""), |mut acc, element| {
                        acc.push_str(&convert_case::Casing::to_case(&element.field_ident.to_string(), convert_case::Case::UpperCamel));
                        acc
                    });
                    format!("{ident}{variant_columns_merged_upper_camel_case_stringified}")
                };
                let struct_name_token_stream = {
                    struct_name_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {struct_name_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let self_fields_token_stream = generate_self_fields_token_stream(
                    &variant_columns.iter().map(|element|&element.field).collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let ident_try_from_ident_options_error_named_upper_camel_case_token_stream = {
                    let ident_try_from_ident_options_error_named_upper_camel_case_stringified = format!(
                        "{struct_name_stringified}{try_from_upper_camel_case_stringified}{struct_options_ident_stringified}{error_named_upper_camel_case_stringified}"
                    );
                    ident_try_from_ident_options_error_named_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_try_from_ident_options_error_named_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let is_none_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::is_none_upper_camel_case_stringified();
                let postfix_is_none_snake_case_stringified = format!("_{}", proc_macro_helpers::naming_conventions::is_none_snake_case_stringified());
                let ident_try_from_ident_options_error_named_token_stream = {
                    let uuid_wrapper_try_from_possible_uuid_wrapper_primary_key_variant_token_stream = match variant_columns.iter().find(|element|element.field_ident == *primary_key_field_ident) {
                        Some(value) => match value.field_ident == *primary_key_field_ident {
                            true => quote::quote!{
                                #uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_token_stream {
                                    #eo_error_occurence_attribute_token_stream
                                    #uuid_wrapper_try_from_possible_uuid_wrapper_snake_case_token_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                                },
                            },
                            false => proc_macro2::TokenStream::new()
                        },
                        None => proc_macro2::TokenStream::new()
                    };
                    let is_none_variant_columns_token_stream = variant_columns.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        let field_ident_title_case_stringified = convert_case::Casing::to_case(&field_ident.to_string(), convert_case::Case::UpperCamel);
                        let field_ident_is_none_title_case_token_stream = {
                            let field_ident_is_none_title_case_stringified = format!("{field_ident_title_case_stringified}{is_none_upper_camel_case_stringified}");
                            field_ident_is_none_title_case_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_title_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let field_ident_is_none_snake_case_token_stream = {
                            let field_ident_is_none_snake_case_stringified = format!("{field_ident}{postfix_is_none_snake_case_stringified}");
                            field_ident_is_none_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote! {
                            #field_ident_is_none_title_case_token_stream {
                                #eo_display_with_serialize_deserialize_token_stream
                                #field_ident_is_none_snake_case_token_stream: #std_string_string_token_stream,
                                #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                            }
                        }
                    });
                    quote::quote! {
                        #derive_debug_thiserror_error_occurence_token_stream
                        pub enum #ident_try_from_ident_options_error_named_upper_camel_case_token_stream {
                            //HERE remove
                            // #uuid_wrapper_try_from_possible_uuid_wrapper_primary_key_variant_token_stream
                            #(#is_none_variant_columns_token_stream),*
                        }
                    }
                };
                let impl_std_convert_try_from_ident_options_ident_token_stream = {
                    let primary_key_field_assignment_token_stream = match variant_columns.iter().find(|element|element.field_ident == *primary_key_field_ident) {
                        Some(value) => {
                            let column_variant_ident_stringified = &value.field_ident;
                            match column_variant_ident_stringified == primary_key_field_ident {
                                true => {
                                    let field_code_occurence_new_ea56ed9e_86e6_4b3e_b116_106eb0bca826_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                        file!(),
                                        line!(),
                                        column!(),
                                        &proc_macro_name_upper_camel_case_ident_stringified,
                                    );
                                    let field_code_occurence_new_fb6f51c7_7907_4872_9b50_852f40a91c0f_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                        file!(),
                                        line!(),
                                        column!(),
                                        &proc_macro_name_upper_camel_case_ident_stringified,
                                    );
                                    quote::quote!{
                                        let #primary_key_field_ident = match value.#primary_key_field_ident {
                                            //
                                            // match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value) {
                                            //     Ok(value) => value.into_inner(),
                                            //     Err(#error_value_snake_case_token_stream) => {
                                            //         return Err(Self::Error::#uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_token_stream {
                                            //             #uuid_wrapper_try_from_possible_uuid_wrapper_snake_case_token_stream: #error_value_snake_case_token_stream,
                                            //             #field_code_occurence_new_ea56ed9e_86e6_4b3e_b116_106eb0bca826_token_stream,
                                            //         });
                                            //     }
                                            // }
                                            //
                                            Some(value) => value,
                                            None => {
                                                return Err(Self::Error::IdIsNone {
                                                    id_is_none: #std_string_string_token_stream::from("id is None"),//todo primary key field naming fix
                                                    #field_code_occurence_new_fb6f51c7_7907_4872_9b50_852f40a91c0f_token_stream,
                                                });
                                            }
                                        };
                                    }
                                },
                                false => proc_macro2::TokenStream::new()
                            }
                        },
                        None => proc_macro2::TokenStream::new()
                    };
                    let variant_columns_assignment_token_stream = variant_columns.iter().filter(|element|element.field_ident != *primary_key_field_ident).map(|element|{
                        let field_ident = &element.field_ident;
                        let field_ident_title_case_stringified = convert_case::Casing::to_case(&field_ident.to_string(), convert_case::Case::UpperCamel);
                        let field_ident_is_none_title_case_token_stream = {
                            let field_ident_is_none_title_case_stringified = format!("{field_ident_title_case_stringified}{is_none_upper_camel_case_stringified}");
                            field_ident_is_none_title_case_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_title_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let field_ident_is_none_snake_case_token_stream = {
                            let field_ident_is_none_snake_case_stringified = format!("{field_ident}{postfix_is_none_snake_case_stringified}");
                            field_ident_is_none_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let field_ident_is_none_message_snake_case_token_stream = {
                            let field_ident_is_none_snake_case_stringified = format!("\"{field_ident} is None\"");
                            field_ident_is_none_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let field_code_occurence_new_dea6a52d_6fb9_49c7_9220_6f185e417faf_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            let #field_ident = match value.#field_ident {
                                Some(value) => value,
                                None => {
                                    return Err(Self::Error::#field_ident_is_none_title_case_token_stream {
                                        #field_ident_is_none_snake_case_token_stream: #std_string_string_token_stream::from(#field_ident_is_none_message_snake_case_token_stream),
                                        #field_code_occurence_new_dea6a52d_6fb9_49c7_9220_6f185e417faf_token_stream,
                                    });
                                }
                            };
                        }
                    });
                    quote::quote! {
                        impl std::convert::TryFrom<#struct_options_ident_token_stream> for #struct_name_token_stream {
                            type Error = #ident_try_from_ident_options_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #struct_options_ident_token_stream) -> Result<Self, Self::Error> {
                                #primary_key_field_assignment_token_stream
                                #(#variant_columns_assignment_token_stream)*
                                Ok(Self {
                                    #(#self_fields_token_stream),*
                                })
                            }
                        }
                    }
                };
                quote::quote! {
                    #ident_try_from_ident_options_error_named_token_stream
                    #impl_std_convert_try_from_ident_options_ident_token_stream
                }
            })
            .collect::<std::vec::Vec<proc_macro2::TokenStream>>()
    };
    let ident_column_upper_camel_case_token_stream = {
        let ident_column_upper_camel_case_stringified = format!(
            "{ident}{}",
            proc_macro_helpers::naming_conventions::column_upper_camel_case_stringified()
        );
        ident_column_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_column_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let column_token_stream = {
        let column_variants = fields_named
            .iter()
            .map(|field| {
                let field_ident_stringified = field.ident
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                    })
                    .to_string();
                let serialize_deserialize_ident_token_stream = format!("\"{field_ident_stringified}\"").parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                let variant_ident_token_stream = {
                    let variant_ident_stringified = convert_case::Casing::to_case(&field_ident_stringified, convert_case::Case::UpperCamel);
                    variant_ident_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {variant_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    #[serde(rename(serialize = #serialize_deserialize_ident_token_stream, deserialize = #serialize_deserialize_ident_token_stream))]
                    #variant_ident_token_stream
                }
            })
            .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            #[derive(
                #debug_token_stream,
                #serde_serialize_token_stream,
                #serde_deserialize_token_stream,
                enum_extension::EnumExtension,
                strum_macros::EnumIter,
                PartialEq,
                Eq,
                #from_str_snake_case_token_stream::#from_str_upper_camel_case_token_stream,
            )]
            pub enum #ident_column_upper_camel_case_token_stream {
                #(#column_variants),*
            }
            impl std::fmt::Display for #ident_column_upper_camel_case_token_stream {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", Self::to_snake_case(self))
                }
            }
        }
    };
    let column_select_upper_camel_case_stringified = format!(
        "{}{}",
        proc_macro_helpers::naming_conventions::column_upper_camel_case_stringified(),
        proc_macro_helpers::naming_conventions::select_upper_camel_case_stringified()
    );
    let ident_column_select_upper_camel_case_token_stream = {
        let ident_column_select_upper_camel_case_stringified =
            format!("{ident}{column_select_upper_camel_case_stringified}");
        ident_column_select_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_column_select_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let options_try_from_sqlx_row_name_token_stream = quote::quote! {options_try_from_sqlx_row};
    let crate_common_serde_urlencoded_serde_url_encoded_parameter_token_stream =
        quote::quote! {crate::common::serde_urlencoded::SerdeUrlencodedParameter};
    let ident_column_select_from_str_error_named_upper_camel_case_token_stream = {
        let ident_column_select_from_str_error_named_upper_camel_case_stringified = format!("{ident}{column_select_upper_camel_case_stringified}{from_str_upper_camel_case_stringified}{error_named_upper_camel_case_stringified}");
        ident_column_select_from_str_error_named_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_column_select_from_str_error_named_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let crate_server_postgres_bind_query_try_generate_bind_increments_error_named_name_token_stream = {
        let crate_server_postgres_bind_query_try_generate_bind_increments_error_named_name_stringified = format!("crate::server::postgres::bind_query::TryGenerateBindIncrements{error_named_upper_camel_case_stringified}");
        crate_server_postgres_bind_query_try_generate_bind_increments_error_named_name_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {crate_server_postgres_bind_query_try_generate_bind_increments_error_named_name_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let column_select_token_stream = {
        let column_select_struct_token_stream = {
            let column_select_variants_token_stream = column_variants.iter().map(|column_variant|{
                let variant_ident_token_stream = {
                    let variant_ident_stringified_handle = column_variant.iter()
                        .fold(std::string::String::default(), |mut acc, field| {
                            acc.push_str(&convert_case::Casing::to_case(&field.field_ident.to_string(), convert_case::Case::UpperCamel));
                            acc
                        });
                    variant_ident_stringified_handle.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {variant_ident_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    #variant_ident_token_stream
                }
            });
            quote::quote! {
                #[derive(#debug_token_stream, #serde_serialize_token_stream, #serde_deserialize_token_stream, Clone, strum_macros::Display)]
                pub enum #ident_column_select_upper_camel_case_token_stream {
                    #(#column_select_variants_token_stream),*
                }
            }
        };
        // println!("{column_select_struct_token_stream}");
        let generate_query_token_stream = {
            let generate_query_variants_token_stream = column_variants.iter().map(|column_variant|{
                let write_ident_token_stream = {
                    let mut write_ident_stringified_handle = column_variant.iter()
                        .fold(std::string::String::default(), |mut acc, field| {
                            acc.push_str(&format!("{},", &field.field_ident));
                            acc
                        });
                    write_ident_stringified_handle.pop();
                    format!("\"{write_ident_stringified_handle}\"").parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {write_ident_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let variant_ident_token_stream = {
                    let variant_ident_stringified_handle = column_variant.iter()
                        .fold(std::string::String::default(), |mut acc, field| {
                            acc.push_str(&convert_case::Casing::to_case(&field.field_ident.to_string(), convert_case::Case::UpperCamel));
                            acc
                        });
                    variant_ident_stringified_handle.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {variant_ident_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    Self::#variant_ident_token_stream => #std_string_string_token_stream::from(#write_ident_token_stream)
                }
            });
            quote::quote! {
                impl crate::server::postgres::generate_query::GenerateQuery for #ident_column_select_upper_camel_case_token_stream {
                    fn generate_query(&self) -> #std_string_string_token_stream {
                        match self {
                            #(#generate_query_variants_token_stream),*
                        }
                    }
                }
            }
        };
        // println!("{generate_query_token_stream}");
        let impl_default_token_stream = {
            let default_select_variant_ident_token_stream = {
                let default_select_variant_ident_stringified = fields_named.iter()
                .fold(std::string::String::default(), |mut acc, field| {
                    let field_ident_stringified = field.ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        }).to_string();
                    acc.push_str(&convert_case::Casing::to_case(&field_ident_stringified, convert_case::Case::UpperCamel));
                    acc
                });
                default_select_variant_ident_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {default_select_variant_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote! {
                impl std::default::Default for #ident_column_select_upper_camel_case_token_stream {
                    fn default() -> Self {
                        Self::#default_select_variant_ident_token_stream
                    }
                }
            }
        };
        // println!("{impl_default_token_stream}");
        let from_option_self_token_stream = {
            quote::quote! {
                impl std::convert::From<std::option::Option<Self>> for #ident_column_select_upper_camel_case_token_stream {
                    fn from(option_value: std::option::Option<Self>) -> Self {
                        match option_value {
                            Some(value) => value,
                            None => Self::default(),
                        }
                    }
                }
            }
        };
        // println!("{from_option_self_token_stream}");
        let not_correct_value_token_stream = quote::quote! {not_correct_value};
        let supported_values_token_stream = quote::quote! {supported_values};
        let not_correct_upper_camel_case_token_stream = quote::quote! {NotCorrect};
        let ident_column_select_from_str_error_named_token_stream = {
            quote::quote! {
                #derive_debug_thiserror_error_occurence_token_stream
                pub enum #ident_column_select_from_str_error_named_upper_camel_case_token_stream {
                    #not_correct_upper_camel_case_token_stream {
                        #eo_display_with_serialize_deserialize_token_stream
                        #not_correct_value_token_stream: #std_string_string_token_stream,
                        #eo_display_with_serialize_deserialize_token_stream
                        #supported_values_token_stream: #std_string_string_token_stream,
                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                    },
                }
            }
        };
        // println!("{ident_column_select_from_str_error_named_token_stream}");
        let impl_std_str_from_str_for_ident_column_select_token_stream = {
            let match_acceptable_variants_token_stream = column_variants.iter().map(|column_variant|{
                let variant_ident_stringified_handle = column_variant.iter()
                .fold(std::string::String::default(), |mut acc, field| {
                    acc.push_str(&convert_case::Casing::to_case(&field.field_ident.to_string(), convert_case::Case::UpperCamel));
                    acc
                });
                let write_ident_token_stream = {
                    let write_ident_stringified = format!("\"{variant_ident_stringified_handle}\"");
                    write_ident_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {write_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let variant_ident_token_stream = {
                    variant_ident_stringified_handle.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {variant_ident_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    #write_ident_token_stream => Ok(Self::#variant_ident_token_stream)
                }
            });
            let supported_values_handle_token_stream = {
                let mut column_variants_stringified = column_variants.iter().fold(
                    std::string::String::default(),
                    |mut acc, column_variant| {
                        let variant_ident_stringified_handle = column_variant.iter().fold(
                            std::string::String::default(),
                            |mut acc, field| {
                                acc.push_str(&convert_case::Casing::to_case(
                                    &field
                                    .field_ident
                                    .to_string(),
                                    convert_case::Case::UpperCamel
                                ));
                                acc
                            },
                        );
                        acc.push_str(&format!("\\\"{variant_ident_stringified_handle}\\\","));
                        acc
                    },
                );
                column_variants_stringified.pop();
                let supported_values_handle_stringified =
                    format!("\"{column_variants_stringified}\"");
                supported_values_handle_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {supported_values_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_code_occurence_new_6da5ddda_4191_47d7_9ea2_4c0d6fb9d99d_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                impl #std_str_from_str_token_stream for #ident_column_select_upper_camel_case_token_stream {
                    type Err = #ident_column_select_from_str_error_named_upper_camel_case_token_stream;
                    fn from_str(value: #str_ref_token_stream) -> Result<Self, Self::Err> {
                        match value {
                            #(#match_acceptable_variants_token_stream),*,
                            _ => Err(Self::Err::#not_correct_upper_camel_case_token_stream {
                                #not_correct_value_token_stream: #std_string_string_token_stream::from(value),
                                #supported_values_token_stream: #std_string_string_token_stream::from(#supported_values_handle_token_stream),
                                #field_code_occurence_new_6da5ddda_4191_47d7_9ea2_4c0d6fb9d99d_token_stream,
                            }),
                        }
                    }
                }
            }
        };
        // println!("{impl_std_str_from_str_for_ident_column_select_token_stream}");
        let serde_urlencoded_parameter_token_stream = {
            quote::quote! {
                impl #crate_common_serde_urlencoded_serde_url_encoded_parameter_token_stream for #ident_column_select_upper_camel_case_token_stream {
                    fn serde_urlencoded_parameter(self) -> #std_string_string_token_stream {
                        self.to_string()
                    }
                }
            }
        };
        // println!("{serde_urlencoded_parameter_token_stream}");
        let options_try_from_sqlx_row_token_stream = {
            let declaration_primary_key_token_stream = {
                let inner_type_with_serialize_deserialize_token_stream =
                    &primary_key_syn_field_with_additional_info
                        .inner_type_with_serialize_deserialize_token_stream;
                quote::quote! {
                    let mut #primary_key_field_ident: std::option::Option<#inner_type_with_serialize_deserialize_token_stream> = None;
                }
            };
            let declaration_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                let field_ident = &element.field_ident;
                let field_type = &element.field.ty;
                let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
                quote::quote! {
                    let mut #field_ident: std::option::Option<#inner_type_with_serialize_deserialize_token_stream> = None;
                }
            });
            let assignment_token_stream = column_variants.iter().map(|column_variant|{
                let write_ident_token_stream = column_variant.iter().filter_map(|element|match &element.field == &primary_key_syn_field_with_additional_info.field {
                    true => None,
                    false => {
                        let field_ident = &element.field_ident;
                        let field_ident_string_quotes_token_stream = proc_macro_common::generate_quotes::generate_quotes_token_stream(
                            &field_ident.to_string(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let original_type_token_stream = &element.original_type_token_stream;
                        let inner_type_token_stream = &element.inner_type_token_stream;
                        let inner_type_with_serialize_deserialize_token_stream = &element.inner_type_with_serialize_deserialize_token_stream;
                        Some(quote::quote!{
                            #field_ident = {
                                let value: std::option::Option<#original_type_token_stream> = row.try_get(#field_ident_string_quotes_token_stream)?;
                                value.map(|value|
                                    #inner_type_with_serialize_deserialize_token_stream::from(#inner_type_token_stream(value))//todo from or try_from
                                )
                            };
                        })
                    },
                });
                let variant_ident_token_stream = {
                    let variant_ident_stringified_handle = column_variant.iter()
                        .fold(std::string::String::default(), |mut acc, element| {
                            acc.push_str(&convert_case::Casing::to_case(&element.field_ident.to_string(), convert_case::Case::UpperCamel));
                            acc
                        });
                    variant_ident_stringified_handle.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {variant_ident_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    Self::#variant_ident_token_stream => {
                        // #write_ident_primary_key_token_stream
                        #(#write_ident_token_stream)*
                    }
                }
            });
            let option_fields_initiation_token_stream = generate_self_fields_token_stream(
                &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let sqlx_decode_decode_and_sqlx_types_type_primary_key_token_stream = {
                let original_type_token_stream =
                    &primary_key_syn_field_with_additional_info.original_type_token_stream;
                quote::quote! {
                    std::option::Option<#original_type_token_stream>: #sqlx_decode_decode_database_token_stream,
                    std::option::Option<#original_type_token_stream>: #sqlx_types_type_database_token_stream,
                }
            };
            let sqlx_decode_decode_and_sqlx_types_type_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| {
                let field_type = &element.field.ty;
                let original_type_token_stream = &element.original_type_token_stream;
                quote::quote!{
                    std::option::Option<#original_type_token_stream>: #sqlx_decode_decode_database_token_stream,
                    std::option::Option<#original_type_token_stream>: #sqlx_types_type_database_token_stream,
                }
            });
            quote::quote! {
                impl #ident_column_select_upper_camel_case_token_stream {
                    fn #options_try_from_sqlx_row_name_token_stream<'a, R: #sqlx_row_token_stream>(
                        &self,
                        row: &'a R,
                    ) -> sqlx::Result<#struct_options_ident_token_stream>
                    where
                        #std_primitive_str_sqlx_column_index_token_stream
                        #sqlx_decode_decode_and_sqlx_types_type_primary_key_token_stream
                        #(#sqlx_decode_decode_and_sqlx_types_type_with_excluded_primary_key_token_stream)*
                    {
                        #declaration_primary_key_token_stream
                        #(#declaration_excluding_primary_key_token_stream)*
                        match self {
                            //todo why id in all cases
                            // let primary_key_try_get_result: Result<
                            //     std::option::Option<sqlx::types::Uuid>,
                            //     sqlx::Error,
                            // > = row.try_get("id");
                            #(#assignment_token_stream)*
                        }
                        Ok(#struct_options_ident_token_stream {
                            #(#option_fields_initiation_token_stream),*
                        })
                    }
                }
            }
        };
        // println!("{options_try_from_sqlx_row_token_stream}");
        quote::quote! {
            #column_select_struct_token_stream
            #generate_query_token_stream
            #impl_default_token_stream
            #from_option_self_token_stream
            #ident_column_select_from_str_error_named_token_stream
            #impl_std_str_from_str_for_ident_column_select_token_stream
            #serde_urlencoded_parameter_token_stream
            #options_try_from_sqlx_row_token_stream
        }
    };
    // println!("{column_select_token_stream}");
    let crate_server_postgres_regex_filter_regex_filter_token_stream =
        quote::quote! {crate::server::postgres::regex_filter::RegexFilter};
    let crate_server_postgres_postgres_bigint_postgres_bigint_token_stream =
        quote::quote! {crate::server::postgres::postgres_bigint::PostgresBigint};
    let primary_key_uuid_wrapper_try_from_sqlx_row_token_stream = {
        let primary_key_str_token_stream = {
            let primary_key_str_stringified = format!("\"{primary_key_field_ident}\"");
            primary_key_str_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {primary_key_str_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let row_name_token_stream = quote::quote! {row};
        let primary_key_name_token_stream = quote::quote! {primary_key};
        let inner_type_token_stream = {
            let value_stringified = primary_key_syn_field_with_additional_info.rust_sqlx_map_to_postgres_type_variant.get_inner_type_stringified("");//todo generic
            value_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let original_type_token_stream = {
            let value_stringified = primary_key_syn_field_with_additional_info.rust_sqlx_map_to_postgres_type_variant.get_original_type_stringified("");
            value_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        //todo rename coz in the future uuid_wrapper will be removed
        quote::quote! {
            fn #primary_key_uuid_wrapper_try_from_sqlx_row_name_token_stream<'a, R: #sqlx_row_token_stream>(#row_name_token_stream: &'a R) -> sqlx::Result<#inner_type_token_stream>
            //HERE
            //#crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream
            where
                #std_primitive_str_sqlx_column_index_token_stream
                //HERE
                // #sqlx_types_uuid_token_stream: #sqlx_decode_decode_database_token_stream,
                // #sqlx_types_uuid_token_stream: #sqlx_types_type_database_token_stream,
                #original_type_token_stream: #sqlx_decode_decode_database_token_stream,
                #original_type_token_stream: #sqlx_types_type_database_token_stream,
            {
                let #primary_key_name_token_stream: #original_type_token_stream = #row_name_token_stream.try_get(#primary_key_str_token_stream)?;
                // Ok(#crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::from(#primary_key_name_token_stream))
                Ok(#inner_type_token_stream::from(#primary_key_name_token_stream))
            }
        }
    };
    // println!("{primary_key_uuid_wrapper_try_from_sqlx_row_token_stream}");
    // let order_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&order_upper_camel_case_stringified);
    let order_by_upper_camel_case_stringified = format!(
        "{}{}",
        proc_macro_helpers::naming_conventions::order_upper_camel_case_stringified(),
        proc_macro_helpers::naming_conventions::by_upper_camel_case_stringified()
    );
    let order_by_snake_case_stringified =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &order_by_upper_camel_case_stringified,
        );
    let order_by_upper_camel_case_token_stream = {
        order_by_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {order_by_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let order_by_snake_case_token_stream = {
        order_by_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {order_by_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let crate_server_postgres_order_by_order_by_token_stream = quote::quote! {crate::server::postgres::#order_by_snake_case_token_stream::#order_by_upper_camel_case_token_stream};
    let crate_server_postgres_order_order_token_stream =
        quote::quote! {crate::server::postgres::order::Order};
    let limit_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::limit_snake_case_token_stream();
    let offset_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::offset_snake_case_token_stream();
    let order_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::order_snake_case_token_stream();
    let column_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::column_snake_case_token_stream();
    let ident_order_by_wrapper_stringified =
        format!("{ident}{order_by_upper_camel_case_stringified}Wrapper");
    let ident_order_by_wrapper_name_token_stream = {
        ident_order_by_wrapper_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_order_by_wrapper_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_order_by_wrapper_from_str_error_named_name_token_stream = {
        let ident_order_by_wrapper_from_str_error_named_name_stringified = format!("{ident_order_by_wrapper_stringified}{from_str_upper_camel_case_stringified}{error_named_upper_camel_case_stringified}");
        ident_order_by_wrapper_from_str_error_named_name_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_order_by_wrapper_from_str_error_named_name_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let deserialize_ident_order_by_token_stream = {
        //todo
        let ivalid_ident_order_by_handle_token_stream = {
            let ivalid_ident_order_by_handle = format!("\"Invalid {ident}OrderBy:\"");
            ivalid_ident_order_by_handle.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ivalid_ident_order_by_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let deserialize_ident_order_by_snake_case_name_token_stream = {
            let deserialize_ident_order_by_snake_case_name =
                format!("deserialize_{ident_snake_case_stringified}_order_by");
            deserialize_ident_order_by_snake_case_name.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {deserialize_ident_order_by_snake_case_name} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        quote::quote! {
            fn #deserialize_ident_order_by_snake_case_name_token_stream<'de, D>(
                deserializer: D,
            ) -> Result<#crate_server_postgres_order_by_order_by_token_stream<#ident_column_upper_camel_case_token_stream>, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let string_deserialized = {
                    use #serde_deserialize_token_stream;
                    #std_string_string_token_stream::deserialize(deserializer)?
                };
                let split_inner_url_parameters_symbol = ',';
                let default_message = format!(#ivalid_ident_order_by_handle_token_stream);
                let column_equal_str = "column=";
                let order_equal_str = "order=";
                let column = match string_deserialized.find(column_equal_str) {
                    Some(index) => match index.checked_add(column_equal_str.len()) {
                        Some(offset) => match string_deserialized.get(offset..) {
                            Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                                Some(offset_slice_next_comma_index) => {
                                    match offset_slice.get(0..offset_slice_next_comma_index) {
                                        Some(possible_column) => match {
                                            use #std_str_from_str_token_stream;
                                            #ident_column_upper_camel_case_token_stream::from_str(possible_column)
                                        } {
                                            Ok(column) => column,
                                            Err(#error_value_snake_case_token_stream) => {
                                                return Err(serde::de::Error::custom(&format!(
                                                    "{default_message} {column_equal_str} {}",
                                                    #error_value_snake_case_token_stream
                                                )));
                                            }
                                        },
                                        None => {
                                            return Err(serde::de::Error::custom(&format!(
                                                "{default_message} {column_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)"
                                            )));
                                        }
                                    }
                                }
                                None => match offset_slice.get(0..) {
                                    Some(possible_column) => match {
                                        use #std_str_from_str_token_stream;
                                        #ident_column_upper_camel_case_token_stream::from_str(possible_column)
                                    } {
                                        Ok(column) => column,
                                        Err(#error_value_snake_case_token_stream) => {
                                            return Err(serde::de::Error::custom(&format!(
                                                "{default_message} {column_equal_str} {}",
                                                #error_value_snake_case_token_stream
                                            )));
                                        }
                                    },
                                    None => {
                                        return Err(serde::de::Error::custom(&format!(
                                            "{default_message} {column_equal_str} failed to offset_slice.get(0..)"
                                        )));
                                    }
                                },
                            },
                            None => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {column_equal_str} failed to string_deserialized.get(offset..)"
                                )));
                            }
                        },
                        None => {
                            return Err(serde::de::Error::custom(&format!(
                                "{default_message} {column_equal_str} index overflow"
                            )));
                        }
                    },
                    None => {
                        return Err(serde::de::Error::custom(&format!(
                            "{default_message} {column_equal_str} not found"
                        )));
                    }
                };
                let order = match string_deserialized.find(order_equal_str) {
                    Some(index) => match index.checked_add(order_equal_str.len()) {
                        Some(offset) => match string_deserialized.get(offset..) {
                            Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                                Some(offset_slice_next_comma_index) => {
                                    match offset_slice.get(0..offset_slice_next_comma_index) {
                                        Some(possible_order) => match {
                                            use #std_str_from_str_token_stream;
                                            #crate_server_postgres_order_order_token_stream::from_str(possible_order)
                                        } {
                                            Ok(order) => Some(order),
                                            Err(#error_value_snake_case_token_stream) => {
                                                return Err(serde::de::Error::custom(&format!(
                                                    "{default_message} {order_equal_str} {}",
                                                    #error_value_snake_case_token_stream
                                                )));
                                            }
                                        },
                                        None => {
                                            return Err(serde::de::Error::custom(&format!(
                                                "{default_message} {order_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)"
                                            )));
                                        }
                                    }
                                }
                                None => match offset_slice.get(0..) {
                                    Some(possible_order) => match {
                                        use #std_str_from_str_token_stream;
                                        #crate_server_postgres_order_order_token_stream::from_str(possible_order)
                                    } {
                                        Ok(order) => Some(order),
                                        Err(#error_value_snake_case_token_stream) => {
                                            return Err(serde::de::Error::custom(&format!(
                                                "{default_message} {order_equal_str} {}",
                                                #error_value_snake_case_token_stream
                                            )));
                                        }
                                    },
                                    None => {
                                        return Err(serde::de::Error::custom(&format!(
                                            "{default_message} {order_equal_str} failed to offset_slice.get(0..)"
                                        )));
                                    }
                                },
                            },
                            None => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {order_equal_str} failed to string_deserialized.get(offset..)"
                                )));
                            }
                        },
                        None => {
                            return Err(serde::de::Error::custom(&format!(
                                "{default_message} {order_equal_str} index overflow"
                            )));
                        }
                    },
                    None => None,
                };
                Ok(#crate_server_postgres_order_by_order_by_token_stream { column, order })
            }
        }
    };
    let order_by_wrapper_token_stream = {
        let struct_token_stream = {
            let deserialize_with_name_quotes_token_stream =
                proc_macro_common::generate_quotes::generate_quotes_token_stream(
                    &format!("deserialize_{ident_snake_case_stringified}_order_by"),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
            quote::quote! {
                #derive_debug_serialize_deserialize_token_stream
                pub struct #ident_order_by_wrapper_name_token_stream(
                    #[serde(deserialize_with = #deserialize_with_name_quotes_token_stream)]
                    pub #crate_server_postgres_order_by_order_by_token_stream<#ident_column_upper_camel_case_token_stream>,
                );
            }
        };
        let impl_crate_common_serde_urlencoded_serde_urlencoded_parameter_for_ident_order_by_wrapper_token_stream = {
            quote::quote! {
                impl #crate_common_serde_urlencoded_serde_url_encoded_parameter_token_stream for #ident_order_by_wrapper_name_token_stream {
                    fn serde_urlencoded_parameter(self) -> #std_string_string_token_stream {
                        let column = &self.0.column;
                        let order = self.0.order.unwrap_or_default();//todo remove this. must be no default
                        format!("column={column},order={order}")
                    }
                }
            }
        };
        let ident_order_by_wrapper_from_str_error_named_token_stream = {
            let variants_token_stream = [
                "ColumnFromStr",
                "ColumnNoOffsetValue",
                "ColumnOffsetSliceGet",
                "ColumnStringDeserializedGet",
                "ColumnIndexCheckedAdd",
                "ColumnStringDeserializedFind",
                "OrderFromStr",
                "OrderOffsetSliceGetNone",
                "OrderStringDeserializedGetNone",
                "OrderIndexCheckedAdd",
            ].into_iter().map(|name_upper_camel_case_stringified|{
                let name_upper_camel_case_token_stream = {
                    name_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {name_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let name_snake_case_token_stream = {
                    let name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&name_upper_camel_case_stringified);
                    name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote!{
                    #name_upper_camel_case_token_stream {
                        #eo_display_with_serialize_deserialize_token_stream
                        #name_snake_case_token_stream: #std_string_string_token_stream,
                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                    }
                }
            }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            quote::quote! {
                #derive_debug_thiserror_error_occurence_token_stream
                pub enum #ident_order_by_wrapper_from_str_error_named_name_token_stream {
                    #(#variants_token_stream),*
                }
            }
        };
        let impl_std_str_from_str_for_ident_order_by_wrapper_token_stream = {
            let default_message_handle_token_stream = {
                let default_message_handle_stringified = format!("\"Invalid {ident}OrderBy:\"");
                default_message_handle_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {default_message_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_code_occurence_new_07dace83_c1c0_48ff_b40d_1914497705af_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_be62d02f_f4c0_42be_84e3_974adf6e4302_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_25bfb433_fb47_45cb_ad97_ff72d9910d72_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_e997a363_0b00_4109_b597_76e4a920bff5_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_41dfa4e8_c896_424f_83cf_e1094542055d_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_14672eb7_19d9_4093_a51b_f4acdff8c7e4_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_4e8641ae_f8b8_4025_8a55_103e95b26219_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_2d490278_ca68_4b25_9326_e302c619ebc2_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_ad63af88_549a_4e2a_908f_11f83048fcb6_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_907076fc_90b7_43dc_852d_9089ba3aa1ef_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_9bfcc646_717f_45ba_849f_67d825bd64a5_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_4dac2eb7_ae2f_42c5_9ec0_b28911f485dc_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let field_code_occurence_new_ed1f65d5_05d6_4d43_9f0d_41767ec0c089_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                impl #std_str_from_str_token_stream for #ident_order_by_wrapper_name_token_stream {
                    type Err = #ident_order_by_wrapper_from_str_error_named_name_token_stream;
                    fn from_str(value: #str_ref_token_stream) -> Result<Self, Self::Err> {
                        let string_deserialized = value.to_string();
                        let split_inner_url_parameters_symbol = ',';
                        let default_message = format!(#default_message_handle_token_stream);
                        let column_equal_str = "column=";
                        let order_equal_str = "order=";
                        let column = match string_deserialized.find(column_equal_str) {
                            Some(index) => match index.checked_add(column_equal_str.len()) {
                                Some(offset) => match string_deserialized.get(offset..) {
                                    Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                                        Some(offset_slice_next_comma_index) => {
                                            match offset_slice.get(0..offset_slice_next_comma_index) {
                                                Some(possible_column) => match #ident_column_upper_camel_case_token_stream::from_str(possible_column) {
                                                    Ok(column) => column,
                                                    Err(#error_value_snake_case_token_stream) => {
                                                        return Err(Self::Err::ColumnFromStr {
                                                            column_from_str: #error_value_snake_case_token_stream,
                                                            #field_code_occurence_new_07dace83_c1c0_48ff_b40d_1914497705af_token_stream,
                                                        });
                                                    }
                                                },
                                                None => {
                                                    return Err(Self::Err::ColumnNoOffsetValue {
                                                        column_no_offset_value: #std_string_string_token_stream::from("no offset value"),
                                                        #field_code_occurence_new_be62d02f_f4c0_42be_84e3_974adf6e4302_token_stream,
                                                    });
                                                }
                                            }
                                        }
                                        None => match offset_slice.get(0..) {
                                            Some(possible_column) => match #ident_column_upper_camel_case_token_stream::from_str(possible_column) {
                                                Ok(column) => column,
                                                Err(#error_value_snake_case_token_stream) => {
                                                    return Err(Self::Err::ColumnFromStr {
                                                        column_from_str: #error_value_snake_case_token_stream,
                                                        #field_code_occurence_new_25bfb433_fb47_45cb_ad97_ff72d9910d72_token_stream,
                                                    });
                                                }
                                            },
                                            None => {
                                                return Err(Self::Err::ColumnOffsetSliceGet {
                                                    column_offset_slice_get: #std_string_string_token_stream::from("offset_slice_get"),
                                                    #field_code_occurence_new_e997a363_0b00_4109_b597_76e4a920bff5_token_stream,
                                                });
                                            }
                                        },
                                    },
                                    None => {
                                        return Err(Self::Err::ColumnStringDeserializedGet {
                                            column_string_deserialized_get: #std_string_string_token_stream::from("string_deserialized_get"),
                                            #field_code_occurence_new_41dfa4e8_c896_424f_83cf_e1094542055d_token_stream,
                                        });
                                    }
                                },
                                None => {
                                    return Err(Self::Err::ColumnIndexCheckedAdd {
                                        column_index_checked_add: #std_string_string_token_stream::from("index_checked_add"),
                                        #field_code_occurence_new_14672eb7_19d9_4093_a51b_f4acdff8c7e4_token_stream,
                                    });
                                }
                            },
                            None => {
                                return Err(Self::Err::ColumnStringDeserializedFind {
                                    column_string_deserialized_find: #std_string_string_token_stream::from("string_deserialized_find"),
                                    #field_code_occurence_new_4e8641ae_f8b8_4025_8a55_103e95b26219_token_stream,
                                });
                            }
                        };
                        let order = match string_deserialized.find(order_equal_str) {
                            Some(index) => match index.checked_add(order_equal_str.len()) {
                                Some(offset) => match string_deserialized.get(offset..) {
                                    Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                                        Some(offset_slice_next_comma_index) => {
                                            match offset_slice.get(0..offset_slice_next_comma_index) {
                                                Some(possible_order) => match #crate_server_postgres_order_order_token_stream::from_str(possible_order) {
                                                    Ok(order) => Some(order),
                                                    Err(#error_value_snake_case_token_stream) => {
                                                        return Err(Self::Err::OrderFromStr {
                                                            order_from_str: #error_value_snake_case_token_stream,
                                                            #field_code_occurence_new_2d490278_ca68_4b25_9326_e302c619ebc2_token_stream,
                                                        });
                                                    }
                                                },
                                                None => {
                                                    return Err(Self::Err::OrderOffsetSliceGetNone {
                                                        order_offset_slice_get_none: #std_string_string_token_stream::from("order_offset_slice_get_none"),
                                                        #field_code_occurence_new_ad63af88_549a_4e2a_908f_11f83048fcb6_token_stream,
                                                    });
                                                }
                                            }
                                        }
                                        None => match offset_slice.get(0..) {
                                            Some(possible_order) => match #crate_server_postgres_order_order_token_stream::from_str(possible_order) {
                                                Ok(order) => Some(order),
                                                Err(#error_value_snake_case_token_stream) => {
                                                    return Err(Self::Err::OrderFromStr {
                                                        order_from_str: #error_value_snake_case_token_stream,
                                                        #field_code_occurence_new_907076fc_90b7_43dc_852d_9089ba3aa1ef_token_stream
                                                    });
                                                }
                                            },
                                            None => {
                                                return Err(Self::Err::OrderOffsetSliceGetNone {
                                                    order_offset_slice_get_none: #std_string_string_token_stream::from("order_offset_slice_get_none"),
                                                    #field_code_occurence_new_9bfcc646_717f_45ba_849f_67d825bd64a5_token_stream,
                                                });
                                            }
                                        },
                                    },
                                    None => {
                                        return Err(Self::Err::OrderStringDeserializedGetNone {
                                            order_string_deserialized_get_none: #std_string_string_token_stream::from("string_deserialized_get_none"),
                                            #field_code_occurence_new_4dac2eb7_ae2f_42c5_9ec0_b28911f485dc_token_stream,
                                        });
                                    }
                                },
                                None => {
                                    return Err(Self::Err::OrderIndexCheckedAdd {
                                        order_index_checked_add: #std_string_string_token_stream::from("order_index_checked_add"),
                                        #field_code_occurence_new_ed1f65d5_05d6_4d43_9f0d_41767ec0c089_token_stream,
                                    });
                                }
                            },
                            None => None,
                        };
                        Ok(Self(#crate_server_postgres_order_by_order_by_token_stream { column, order }))
                    }
                }
            }
        };
        quote::quote! {
            #struct_token_stream
            #impl_crate_common_serde_urlencoded_serde_urlencoded_parameter_for_ident_order_by_wrapper_token_stream
            #ident_order_by_wrapper_from_str_error_named_token_stream
            #impl_std_str_from_str_for_ident_order_by_wrapper_token_stream
        }
    };
    // println!("{order_by_wrapper_token_stream}");
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
            let ident_column_read_permission_name = format!("{ident}ColumnReadPermission");
            ident_column_read_permission_name.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_column_read_permission_name} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let fields_permission_token_stream = fields_named.iter().map(|field| {
            let field_ident = field.ident.as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                });
            quote::quote!{
                #field_ident: bool
            }
        });
        quote::quote! {
            pub struct #ident_column_read_permission_name_token_stream {
                #(#fields_permission_token_stream),*
            }
        }
    };
    let reference_api_location_test_token_stream = quote::quote! {&api_location};
    let additional_http_status_codes_error_variants_snake_case_stringified =
        "additional_http_status_codes_error_variants";
    let common_middlewares_error_syn_variants = crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_proc_macro_attribute(
        &ast,
        additional_http_status_codes_error_variants_snake_case_stringified,
        "postgresql_crud",//todo - its main crate name. maybe reuse it later?
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let common_middlewares_error_syn_variants_len = common_middlewares_error_syn_variants.len();
    let extraction_result_snake_case_stringified = "extraction_result";
    let parameters_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::parameters_snake_case_token_stream();
    let payload_upper_camel_case_stringified =
        proc_macro_helpers::naming_conventions::payload_upper_camel_case_stringified();
    let payload_snake_case_stringified =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &payload_upper_camel_case_stringified,
        );
    let payload_snake_case_token_stream = payload_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {payload_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let payload_extraction_result_snake_case_token_stream = {
        let payload_extraction_result_snake_case =
            format!("{payload_snake_case_token_stream}_{extraction_result_snake_case_stringified}");
        payload_extraction_result_snake_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {payload_extraction_result_snake_case} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let non_existing_primary_keys_name_token_stream = quote::quote! {non_existing_primary_keys};
    let use_futures_try_stream_ext_token_stream = quote::quote! {use futures::TryStreamExt};
    let serde_json_to_string_token_stream = quote::quote! {serde_json::to_string};
    // let payload_element_upper_camel_case_stringified = format!("{payload_upper_camel_case_stringified}Element");
    let returning_stringified = "returning";
    let returning_primary_key_stringified =
        format!(" {returning_stringified} {primary_key_field_ident}");
    let rollback_error_name_token_stream = quote::quote! {rollback_error};
    let returning_primary_key_quotes_token_stream =
        proc_macro_common::generate_quotes::generate_quotes_token_stream(
            &returning_primary_key_stringified,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
    // let path_to_crud = "crate::repositories_types::server::routes::api::cats::";
    let app_state_path =
        quote::quote! {postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync}; //todo path
    let app_state_name_token_stream = quote::quote! {app_state};
    let error_log_call_token_stream = quote::quote! {
        error_occurence_lib::error_log::ErrorLog::error_log(
            &#error_value_snake_case_token_stream,
            #app_state_name_token_stream.as_ref(),
        );
    };
    let serde_json_to_string_upper_camel_case_stringified = "SerdeJsonToString";
    let serde_json_to_string_upper_camel_case_token_stream = serde_json_to_string_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {serde_json_to_string_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let serde_json_to_string_snake_case_token_stream = {
        let serde_json_to_string_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&serde_json_to_string_upper_camel_case_stringified);
        serde_json_to_string_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {serde_json_to_string_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let serde_json_to_string_variant_initialization_token_stream = {
        let field_code_occurence_new_27b49c75_24b2_4480_ac4d_62a1f75f5646_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            #serde_json_to_string_upper_camel_case_token_stream {
                #serde_json_to_string_snake_case_token_stream: #error_value_snake_case_token_stream,
                #field_code_occurence_new_27b49c75_24b2_4480_ac4d_62a1f75f5646_token_stream,
            }
        }
    };
    let http_request_error_named_serde_json_to_string_variant_token_stream = quote::quote! {
        #serde_json_to_string_upper_camel_case_token_stream {
            #eo_display_token_stream
            #serde_json_to_string_snake_case_token_stream: serde_json::Error,
            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
        }
    };
    let std_string_string_syn_punctuated_punctuated = proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
        &["std","string","String"],
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let sqlx_error_syn_punctuated_punctuated = proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
        &["sqlx","Error"],
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_syn_punctuated_punctuated =
        generate_std_vec_vec_syn_punctuated_punctuated(
            &["crate", "server", "postgres", "uuid_wrapper", "UuidWrapper"],
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
    let std_vec_vec_crate_server_postgres_regex_filter_regex_filter_syn_punctuated_punctuated =
        generate_std_vec_vec_syn_punctuated_punctuated(
            &["crate", "server", "postgres", "regex_filter", "RegexFilter"],
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
    let code_occurence_field = syn::Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        ident: Some(
            syn::Ident::new("code_occurence", proc_macro2::Span::call_site())
        ),
        colon_token: Some(
            syn::token::Colon {
                spans: [proc_macro2::Span::call_site()],
            },
        ),
        ty: syn::Type::Path(
            syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                        &["error_occurence_lib","code_occurence","CodeOccurence"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                },
            },
        ),
    };
    let bind_query_syn_variant = {
        let variant_name_upper_camel_case_stringified = "BindQuery";
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError,
            variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![(
                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence,
                &variant_name_snake_case_stringified,
                proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                    // &["crate","server","postgres","bind_query","TryGenerateBindIncrementsErrorNamed"],//HERE
                    &["std","string","String"],
                    &proc_macro_name_upper_camel_case_ident_stringified
                ),
            )]
        )
    };
    let bind_query_variant_initialization_token_stream = {
        let field_code_occurence_new_d61d7616_3336_43be_aaa8_2144ff2d2158_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            BindQuery {
                bind_query: #error_value_snake_case_token_stream.into_serialize_deserialize_version(),
                #field_code_occurence_new_d61d7616_3336_43be_aaa8_2144ff2d2158_token_stream
            }
        }
    };
    let checked_add_syn_variant = generate_std_string_string_error_syn_variant(
        "CheckedAdd",
        proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError,
        &code_occurence_field,
        std_string_string_syn_punctuated_punctuated.clone(),
    );
    let checked_add_variant_initialization_token_stream = {
        let checked_add_upper_camel_case_stringified = "CheckedAdd";
        let checked_add_upper_camel_case_token_stream = {
            checked_add_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {checked_add_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let checked_add_snake_case_token_stream = {
            let checked_add_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&checked_add_upper_camel_case_stringified);
            checked_add_snake_case_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {checked_add_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let field_code_occurence_new_9afdf71d_e375_455f_87a3_a16947625a7a_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            #checked_add_upper_camel_case_token_stream { //todo remove it? refactor it?
                #checked_add_snake_case_token_stream: #std_string_string_token_stream::from("checked_add is None"),
                #field_code_occurence_new_9afdf71d_e375_455f_87a3_a16947625a7a_token_stream,
            }
        }
    };
    let query_and_rollback_failed_syn_variant =
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError,
            "QueryAndRollbackFailed",
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay,
                    "query_error",
                    sqlx_error_syn_punctuated_punctuated.clone(),
                ),
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay,
                    "rollback_error",
                    sqlx_error_syn_punctuated_punctuated.clone(),
                ),
            ],
        );
    let primary_key_from_row_and_failed_rollback_syn_variant =
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError,
            "PrimaryKeyFromRowAndFailedRollback",
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay,
                    "primary_key_from_row",
                    sqlx_error_syn_punctuated_punctuated.clone(),
                ),
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay,
                    "rollback_error",
                    sqlx_error_syn_punctuated_punctuated.clone(),
                ),
            ],
        );
    let non_existing_primary_keys_syn_variant = {
        let variant_name_upper_camel_case_stringified = "NonExistingPrimaryKeys";
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplay,
                    &variant_name_snake_case_stringified,
                    std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let non_existing_primary_keys_and_failed_rollback_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
        proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
        "NonExistingPrimaryKeysAndFailedRollback",
        &code_occurence_field,
        vec![
            (
                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplay,
                "non_existing_primary_keys", 
                std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_syn_punctuated_punctuated.clone()
            ),
            (
                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay,
                "rollback_error",
                sqlx_error_syn_punctuated_punctuated.clone()
            )
        ]
    );
    let commit_failed_syn_variant = {
        let variant_name_upper_camel_case_stringified = "CommitFailed";
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError,
            variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![(
                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay,
                &variant_name_snake_case_stringified,
                sqlx_error_syn_punctuated_punctuated.clone(),
            )],
        )
    };
    let not_unique_primary_keys_name_token_stream = quote::quote! {not_unique_primary_keys};
    let not_unique_primary_keys_syn_variant = {
        let variant_name_upper_camel_case_stringified = "NotUniquePrimaryKeys";
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplay,
                    &variant_name_snake_case_stringified,
                    std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let not_unique_primary_key_variant_initialization_token_stream = {
        let field_code_occurence_new_0a70da64_9e15_4760_9656_14961b286f36_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            NotUniquePrimaryKeys {
                not_unique_primary_keys,
                #field_code_occurence_new_0a70da64_9e15_4760_9656_14961b286f36_token_stream,
            }
        }
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_upper_camel_case_stringified =
        "OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapper";
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_upper_camel_case_stringified = format!(
        "{operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_upper_camel_case_stringified}{}{}",
        proc_macro_helpers::naming_conventions::in_upper_camel_case_stringified(),
        proc_macro_helpers::naming_conventions::server_upper_camel_case_stringified()
    );
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_upper_camel_case_token_stream = {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_stringified = format!(
        "{operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_upper_camel_case_stringified}{}{}",
        proc_macro_helpers::naming_conventions::in_upper_camel_case_stringified(),
        proc_macro_helpers::naming_conventions::client_upper_camel_case_stringified()
    ); //todo reuse it
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream = {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_snake_case_token_stream = {
        let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_stringified.to_string());
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_upper_camel_case_token_stream = {
        let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_snake_case_stringified = format!(
            "{operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_stringified}{}{}",
            proc_macro_helpers::naming_conventions::error_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::unnamed_upper_camel_case_stringified()
        );
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_syn_variant = {
        let variant_name_upper_camel_case_stringified = operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_upper_camel_case_stringified;
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError, //todo - is it right status code for this case?
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![(
                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay,
                &variant_name_snake_case_stringified,
                sqlx_error_syn_punctuated_punctuated.clone(),
            )],
        )
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream = {
        let field_code_occurence_new_3567ece5_74c9_4b48_a46c_8230cd728182_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_upper_camel_case_token_stream {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server: #error_value_snake_case_token_stream,
                #field_code_occurence_new_3567ece5_74c9_4b48_a46c_8230cd728182_token_stream,
            }
        }
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_declaration_token_stream = quote::quote! {
        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream {
            #eo_vec_error_occurence_token_stream
            #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_snake_case_token_stream: std::vec::Vec<#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_upper_camel_case_token_stream>,
            #code_occurence_snake_case_token_stream: #error_occurence_lib_code_occurence_code_occurence_token_stream,
        }
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_initialization_token_stream = {
        let field_code_occurence_new_bb9fbcd9_7cea_42e2_b7d8_bc42710bf35e_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream {
                #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_snake_case_token_stream: vec_errors,
                #field_code_occurence_new_bb9fbcd9_7cea_42e2_b7d8_bc42710bf35e_token_stream
            }
        }
    };
    let uuid_wrapper_try_from_possible_uuid_wrapper_in_client_token_stream =
        quote::quote! {uuid_wrapper_try_from_possible_uuid_wrapper_in_client};
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_declaration_token_stream = quote::quote! {
        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream {
            #eo_error_occurence_attribute_token_stream
            #uuid_wrapper_try_from_possible_uuid_wrapper_in_client_token_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
        }
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_initialization_token_stream = {
        let field_code_occurence_new_a1c07748_20c3_49eb_85e0_615161d95345_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream {
                #uuid_wrapper_try_from_possible_uuid_wrapper_in_client_token_stream: #error_value_snake_case_token_stream,
                #field_code_occurence_new_a1c07748_20c3_49eb_85e0_615161d95345_token_stream,
            }
        }
    };
    let not_unique_vec_syn_variants: std::vec::Vec<syn::Variant> = fields_named.iter().fold(std::vec::Vec::with_capacity(fields_named_len - 1), |mut acc, element| {
        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                naming_constants::FIELD_IDENT_IS_NONE
            )
        });
        let not_unique_field_vec_upper_camel_stringified = generate_not_unique_field_vec_upper_camel_stringified(field_ident);
        let not_unique_field_vec_snake_case_stringified = generate_not_unique_field_vec_snake_case_stringified(field_ident);
        acc.push(crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &not_unique_field_vec_upper_camel_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplayWithSerializeDeserialize,
                    &not_unique_field_vec_snake_case_stringified,
                    std_vec_vec_crate_server_postgres_regex_filter_regex_filter_syn_punctuated_punctuated.clone()
                )
            ]
        ));
        acc
    });
    let not_uuid_syn_variant = {
        let variant_name_upper_camel_case_stringified = format!(
            "{}{}",
            proc_macro_helpers::naming_conventions::not_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::uuid_upper_camel_case_stringified()
        );
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay,
                    &variant_name_snake_case_stringified,
                    proc_macro_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                        &["sqlx","types","uuid","Error"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                )
            ]
        )
    };
    let no_payload_fields_syn_variant = {
        let variant_name_upper_camel_case_stringified = format!(
            "{}{}{}",
            proc_macro_helpers::naming_conventions::no_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::payload_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::fields_upper_camel_case_stringified()
        );
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize,
                    &variant_name_snake_case_stringified,
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let no_payload_parameters_syn_variant = {
        let variant_name_upper_camel_case_stringified = format!(
            "{}{}{}",
            proc_macro_helpers::naming_conventions::no_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::payload_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::parameters_upper_camel_case_stringified()
        );
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize,
                    &variant_name_snake_case_stringified,
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let commit_header_addition_token_stream = quote::quote! {
        .header(
            postgresql_crud::COMMIT,
            git_info::PROJECT_GIT_INFO.commit,
        )
    };
    let application_json_quotes_token_stream =
        proc_macro_common::generate_quotes::generate_quotes_token_stream(
            "application/json",
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
    let content_type_application_json_header_addition_token_stream = quote::quote! {
        .header(reqwest::header::CONTENT_TYPE, #application_json_quotes_token_stream)
    };
    let axum_response_into_response_token_stream =
        proc_macro_common::axum_response_into_response_token_stream();
    let axum_extract_rejection_json_rejection_token_stream =
        proc_macro_common::axum_extract_rejection_json_rejection_token_stream();
    let sqlx_query_sqlx_postgres_token_stream = quote::quote! {sqlx::query::<sqlx::Postgres>};
    let reqwest_client_new_token_stream = quote::quote! {reqwest::Client::new()};
    let axum_extract_state_token_stream = quote::quote! {axum::extract::State};
    let axum_json_token_stream = quote::quote! {axum::Json};
    let crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream = quote::quote! {crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor};
    let crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream =
        quote::quote! {crate::server::postgres::bind_query::BindQuery::bind_value_to_query}; //todo move trait to own crate to reuse in postgresql_crud and common
    let crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream = quote::quote! {crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments};
    let crate_server_postgres_bind_query_bind_query_try_increment_token_stream =
        quote::quote! {crate::server::postgres::bind_query::BindQuery::try_increment};
    let increment_initialization_token_stream = quote::quote! {let mut increment: u64 = 0;};
    let http_status_code_token_stream = proc_macro_common::http_status_code_token_stream();
    let reqwest_header_header_map_token_stream =
        proc_macro_common::reqwest_header_header_map_token_stream();
    let reqwest_error_token_stream = proc_macro_common::reqwest_error_token_stream();
    let crate_common_api_request_unexpected_error_response_text_result_token_stream =
        quote::quote! {crate::common::api_request_unexpected_error::ResponseTextResult};
    let try_extract_value_token_stream = quote::quote! {try_extract_value};
    let server_location_name_token_stream = quote::quote! {server_location};
    let dot_space = ", ";
    // let pg_temp_stringified = "pg_temp";
    let pg_connection_token_stream = quote::quote! {pg_connection};
    let query_string_name_token_stream = quote::quote! {query_string};
    let binded_query_name_token_stream = quote::quote! {binded_query};
    let order_by_token_stream = quote::quote! {order_by};
    let current_vec_len_name_token_stream = quote::quote! {current_vec_len};
    let desirable_upper_camel_case_token_stream =
        proc_macro_helpers::naming_conventions::desirable_upper_camel_case_token_stream();
    let select_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::select_snake_case_token_stream();
    let limit_token_stream =
        proc_macro_helpers::naming_conventions::limit_snake_case_token_stream();
    let offset_token_stream =
        proc_macro_helpers::naming_conventions::offset_snake_case_token_stream();
    let rollback_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::rollback_snake_case_token_stream();
    let element_name_token_stream =
        proc_macro_helpers::naming_conventions::element_snake_case_token_stream();
    let acc_name_token_stream =
        proc_macro_helpers::naming_conventions::acc_snake_case_token_stream();
    let query_name_token_stream =
        proc_macro_helpers::naming_conventions::query_snake_case_token_stream();
    let not_uuid_upper_camel_case_stringified = format!(
        "{}{}",
        proc_macro_helpers::naming_conventions::not_upper_camel_case_stringified(),
        proc_macro_helpers::naming_conventions::uuid_upper_camel_case_stringified()
    );
    let not_uuid_token_upper_camel_case_stream = {
        not_uuid_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_uuid_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let not_uuid_token_snake_case_stream = {
        let not_uuid_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&not_uuid_upper_camel_case_stringified.to_string());
        not_uuid_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_uuid_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let underscore_vec_name_stringified = "_vec";
    let update_name_stringified =
        proc_macro_helpers::naming_conventions::update_snake_case_stringified();
    let as_name_stringified = proc_macro_helpers::naming_conventions::as_snake_case_stringified();
    let set_name_stringified = proc_macro_helpers::naming_conventions::set_snake_case_stringified();
    let from_name_stringified =
        proc_macro_helpers::naming_conventions::from_snake_case_stringified();
    let insert_name_stringified =
        proc_macro_helpers::naming_conventions::insert_snake_case_stringified();
    let into_name_stringified =
        proc_macro_helpers::naming_conventions::into_snake_case_stringified();
    let values_name_stringified =
        proc_macro_helpers::naming_conventions::values_snake_case_stringified();
    let delete_name_stringified =
        proc_macro_helpers::naming_conventions::delete_snake_case_stringified();
    let where_name_stringified =
        proc_macro_helpers::naming_conventions::where_snake_case_stringified();
    let where_name_qoutes_token_stream =
        proc_macro_common::generate_quotes::generate_quotes_token_stream(
            &where_name_stringified,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
    let and_name_stringified = proc_macro_helpers::naming_conventions::and_snake_case_stringified();
    // let any_name_stringified = "any";
    // let array_name_stringified = "array";
    let select_name_stringified =
        proc_macro_helpers::naming_conventions::select_snake_case_stringified();
    let order_by_name_stringified = "order by";
    let limit_name_stringified =
        proc_macro_helpers::naming_conventions::limit_snake_case_stringified();
    let offset_name_stringified =
        proc_macro_helpers::naming_conventions::offset_snake_case_stringified();
    let in_name_stringified = proc_macro_helpers::naming_conventions::in_snake_case_stringified();
    let unnest_name_stringified =
        proc_macro_helpers::naming_conventions::unnest_snake_case_stringified();
    let api_request_unexpected_error_module_path_token_stream =
        quote::quote! { crate::common::api_request_unexpected_error };
    let expected_type_upper_camel_case_token_stream =
        proc_macro_helpers::naming_conventions::expected_type_upper_camel_case_token_stream();
    let expected_type_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::expected_type_snake_case_token_stream();
    //todo make crate::type_variants_from_request_response_generator::construct_syn_variant(
    let (
        generate_expected_type_declaration_token_stream,
        expected_type_initialization_token_stream,
    ) = {
        let field_code_occurence_new_a2da8618_eb3a_425c_a2d8_c02d39845168_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        (
            |operation: &crate::Operation| -> proc_macro2::TokenStream {
                let try_operation_with_serialize_deserialize_token_stream = proc_macro_helpers::naming_conventions::TrySelfWithSerializeDeserializeTokenStream::try_self_with_serialize_deserialize_token_stream(operation);
                quote::quote! {
                    #expected_type_upper_camel_case_token_stream {
                        #eo_display_with_serialize_deserialize_token_stream
                        #expected_type_snake_case_token_stream: #try_operation_with_serialize_deserialize_token_stream,
                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                    }
                }
            },
            quote::quote! {
                #expected_type_upper_camel_case_token_stream {
                    #expected_type_snake_case_token_stream: #error_value_snake_case_token_stream,
                    #field_code_occurence_new_a2da8618_eb3a_425c_a2d8_c02d39845168_token_stream
                }
            },
        )
    };
    let unexpected_status_code_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::unexpected_status_code_upper_camel_case_token_stream();
    let status_code_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::status_code_snake_case_token_stream();
    let headers_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::headers_snake_case_token_stream();
    let response_text_result_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::response_text_result_snake_case_token_stream();
    let response_text_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::response_text_snake_case_token_stream();
    let (
        unexpected_status_code_declaration_token_stream,
        unexpected_status_code_initialization_token_stream,
    ) = {
        let field_code_occurence_new_9d155a81_cad2_46fd_96ae_5d53eb306083_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        (
            quote::quote! {
                #unexpected_status_code_upper_camel_case_token_stream {
                    #eo_display_token_stream
                    #status_code_snake_case_token_stream: #http_status_code_token_stream,
                    #eo_display_foreign_type_token_stream
                    #headers_snake_case_token_stream: #reqwest_header_header_map_token_stream,
                    #eo_display_foreign_type_token_stream
                    #response_text_result_snake_case_token_stream: #crate_common_api_request_unexpected_error_response_text_result_token_stream,
                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                }
            },
            quote::quote! {
                #unexpected_status_code_upper_camel_case_token_stream {
                    #status_code_snake_case_token_stream,
                    #headers_snake_case_token_stream,
                    #response_text_result_snake_case_token_stream: #api_request_unexpected_error_module_path_token_stream::ResponseTextResult::ResponseText(response_text),
                    #field_code_occurence_new_9d155a81_cad2_46fd_96ae_5d53eb306083_token_stream
                }
            },
        )
    };
    let failed_to_get_response_text_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::failed_to_get_response_text_upper_camel_case_token_stream();
    let reqwest_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::reqwest_snake_case_token_stream();
    let (
        failed_to_get_response_text_declaration_token_stream,
        failed_to_get_response_text_initialization_token_stream,
    ) = {
        let field_code_occurence_new_4528b9ed_5b9d_486b_af78_345e1b9d95cc_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        (
            quote::quote! {
                #failed_to_get_response_text_upper_camel_case_token_stream {
                    #eo_display_foreign_type_token_stream
                    #reqwest_snake_case_token_stream: #reqwest_error_token_stream,
                    #eo_display_token_stream
                    #status_code_snake_case_token_stream: #http_status_code_token_stream,
                    #eo_display_foreign_type_token_stream
                    #headers_snake_case_token_stream: #reqwest_header_header_map_token_stream,
                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                }
            },
            quote::quote! {
                #failed_to_get_response_text_upper_camel_case_token_stream {
                    #reqwest_snake_case_token_stream: #error_value_snake_case_token_stream,
                    #status_code_snake_case_token_stream,
                    #headers_snake_case_token_stream,
                    #field_code_occurence_new_4528b9ed_5b9d_486b_af78_345e1b9d95cc_token_stream
                }
            },
        )
    };
    let deserialize_response_upper_camel_case_token_stream =
        proc_macro_helpers::naming_conventions::deserialize_response_upper_camel_case_token_stream(
        );
    let serde_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::serde_snake_case_token_stream();
    let (
        deserialize_response_declaration_token_stream,
        deserialize_response_initialization_token_stream,
    ) = {
        let field_code_occurence_new_ddbdaed4_ec8e_4fc7_bcc7_152edc67f22b_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        (
            quote::quote! {
                #deserialize_response_upper_camel_case_token_stream {
                    #eo_display_token_stream
                    #serde_snake_case_token_stream: serde_json::Error,
                    #eo_display_token_stream
                    #status_code_snake_case_token_stream: #http_status_code_token_stream,
                    #eo_display_foreign_type_token_stream
                    #headers_snake_case_token_stream: #reqwest_header_header_map_token_stream,
                    #eo_display_with_serialize_deserialize_token_stream
                    #response_text_snake_case_token_stream: #std_string_string_token_stream,
                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                }
            },
            quote::quote! {
                #deserialize_response_upper_camel_case_token_stream {
                    #serde_snake_case_token_stream: #error_value_snake_case_token_stream,
                    #status_code_snake_case_token_stream,
                    #headers_snake_case_token_stream,
                    #response_text_snake_case_token_stream,
                    #field_code_occurence_new_ddbdaed4_ec8e_4fc7_bcc7_152edc67f22b_token_stream
                }
            },
        )
    };
    let reqwest_upper_camel_case_token_stream =
        proc_macro_helpers::naming_conventions::reqwest_upper_camel_case_token_stream();
    let (reqwest_declaration_token_stream, reqwest_initialization_token_stream) = {
        let field_code_occurence_new_484abc36_0756_417a_a448_7956000c2d8c_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        (
            quote::quote! {
                #reqwest_upper_camel_case_token_stream {
                    #eo_display_foreign_type_token_stream
                    #reqwest_snake_case_token_stream: #reqwest_error_token_stream,
                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                }
            },
            quote::quote! {
                #reqwest_upper_camel_case_token_stream {
                    #reqwest_snake_case_token_stream: #error_value_snake_case_token_stream,
                    #field_code_occurence_new_484abc36_0756_417a_a448_7956000c2d8c_token_stream,
                }
            },
        )
    };
    let common_error_syn_variants = {
        let sqlx_postgres_error_named_syn_variants =
            proc_macro_helpers::enum_variants::sqlx_postgres_error_named_syn_variants(
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        let json_extractor_error_named_syn_variants =
            proc_macro_helpers::enum_variants::json_extractor_error_named_syn_variants(
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        let mut common_error_variants_vec = std::vec::Vec::with_capacity(
            common_middlewares_error_syn_variants_len
                + sqlx_postgres_error_named_syn_variants.len()
                + 1,
        );
        for element in sqlx_postgres_error_named_syn_variants {
            common_error_variants_vec.push(element);
        }
        for element in json_extractor_error_named_syn_variants {
            common_error_variants_vec.push(element);
        }
        let unexpected_case_syn_variant = {
            let variant_name_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::unexpected_case_upper_camel_case_stringified();
            let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
            crate::type_variants_from_request_response_generator::construct_syn_variant(
                proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError,
                &variant_name_upper_camel_case_stringified,
                &code_occurence_field,
                vec![
                    (
                        proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize,
                        &variant_name_snake_case_stringified,
                        std_string_string_syn_punctuated_punctuated.clone()
                    )
                ]
            )
        };
        common_error_variants_vec.push(unexpected_case_syn_variant);
        common_error_variants_vec
    };
    let fields_named_excluding_primary_key = fields_named_wrappers_excluding_primary_key
        .iter()
        .map(|element| &element.field)
        .collect::<std::vec::Vec<&syn::Field>>();
    // let fields_named_idents_comma_excluding_primary_key_token_stream = generate_self_fields_token_stream(
    //     &fields_named_excluding_primary_key,
    //     &proc_macro_name_upper_camel_case_ident_stringified,
    // ).iter().map(|element|quote::quote!{#element,}).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let fields_named_idents_comma_token_stream = generate_self_fields_token_stream(
        &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
        &proc_macro_name_upper_camel_case_ident_stringified,
    )
    .iter()
    .map(|element| quote::quote! {#element,})
    .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let select_full_variant_token_stream = {
        let select_full_variant_stringified = fields_named.iter().fold(std::string::String::default(), |mut acc, field| {
            let field_ident_stringified = field.ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                }).to_string();
            acc.push_str(&convert_case::Casing::to_case(&field_ident_stringified, convert_case::Case::UpperCamel));
            acc
        });
        select_full_variant_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {select_full_variant_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let primary_keys_token_stream = quote::quote! {primary_keys};
    let primary_key_token_stream = quote::quote! {primary_key};
    let into_inner_type_vec_snake_case_token_stream = quote::quote! {into_inner_type_vec};
    let (create_many_token_stream, create_many_http_request_test_token_stream) = {
        let operation = Operation::CreateMany;
        let operation_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation);
        let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(&operation);
        let operation_payload_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation);
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(&operation);
        let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(&operation);
        let try_operation_response_variants_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(&operation);
        let try_operation_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfUpperCamelCaseTokenStream::try_self_upper_camel_case_token_stream(&operation);
        let operation_payload_element_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation);
        let additional_http_status_codes_error_variants = vec![]; //todo find out why rust analyzer crashes
                                                                  // crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_method_proc_macro_attribute(
                                                                  //     &ast,
                                                                  //     &operation_name_snake_case_stringified,
                                                                  //     additional_http_status_codes_error_variants_snake_case_stringified,
                                                                  //     &proc_macro_name_snake_case,
                                                                  //     &proc_macro_name_upper_camel_case_ident_stringified
                                                                  // );
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence,
                    &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
                    proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
                        &operation
                    )
                )
            ]
        );
        let type_variants_from_request_response_syn_variants = {
            let full_additional_http_status_codes_error_variants =
                generate_full_additional_http_status_codes_error_variants(
                    common_middlewares_error_syn_variants.iter().collect(),
                    additional_http_status_codes_error_variants.iter().collect(),
                );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response =
                    std::vec::Vec::with_capacity(common_error_syn_variants.len() + 3);
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                type_variants_from_request_response.push(&bind_query_syn_variant);
                type_variants_from_request_response.push(&operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_syn_variant);
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants,
            )
        };
        let desirable_status_code = proc_macro_helpers::status_code::StatusCode::Tvfrr201Created;
        let unique_status_codes = generate_unique_status_codes(
            &desirable_status_code,
            &type_variants_from_request_response_syn_variants,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let payload_token_stream = {
                let operation_payload_element_token_stream = {
                    let fields_with_excluded_primary_key_token_stream =
                        fields_named_wrappers_excluding_primary_key
                            .iter()
                            .map(|element| {
                                generate_pub_field_ident_field_type_token_stream(
                                    element,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                )
                            });
                    quote::quote! {
                        #derive_debug_token_stream
                        pub struct #operation_payload_element_upper_camel_case_token_stream {
                            #(#fields_with_excluded_primary_key_token_stream),*
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_element_token_stream
                    #derive_debug_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream(pub std::vec::Vec<#operation_payload_element_upper_camel_case_token_stream>);
                }
            };
            // println!("{payload_token_stream}");
            let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
            let payload_with_serialize_deserialize_token_stream = {
                let operation_payload_element_with_serialize_deserialize_token_stream = {
                    let fields_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| generate_field_ident_field_type_with_serialize_deserialize_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    quote::quote! {
                        #derive_debug_serialize_deserialize_to_schema_token_stream
                        pub struct #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream {
                            #(#fields_with_excluded_primary_key_token_stream),*
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_element_with_serialize_deserialize_token_stream
                    #derive_debug_serialize_deserialize_token_stream
                    pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream(std::vec::Vec<#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream>);
                }
            };
            // println!("{payload_with_serialize_deserialize_token_stream}");
            let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream = {
                    let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                    let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream = {
                        let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                            &fields_named_wrappers_excluding_primary_key,
                            &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream
                        );
                        quote::quote! {
                            #derive_debug_thiserror_error_occurence_token_stream
                            pub enum #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                                //todo remove useless variant and add later protentian errors from conversion from with serialize_deserialize
                                #not_uuid_token_upper_camel_case_stream {
                                    #eo_display_with_serialize_deserialize_token_stream
                                    #not_uuid_token_snake_case_stream: std::string::String,//HERE #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                                },
                                #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                            }
                        }
                    };
                    let field_code_occurence_new_591cf20c_c2de_4d33_a7b0_785e8796f0ce_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter()
                        .map(|element| generate_let_field_ident_value_field_ident_try_from_token_stream(
                            element,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                            &field_code_occurence_new_591cf20c_c2de_4d33_a7b0_785e8796f0ce_token_stream,
                        ));
                    let fields_idents_excluding_primary_key_token_stream =
                        fields_named_wrappers_excluding_primary_key
                            .iter()
                            .map(|element| &element.field_ident);
                    //todo it can be std::convert::From if all #(#fields_assignment_excluding_primary_key_token_stream)* are impl from 
                    quote::quote! {
                        #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream
                        impl std::convert::TryFrom<#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_element_upper_camel_case_token_stream {
                            type Error = #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                #(#fields_assignment_excluding_primary_key_token_stream)*
                                Ok(Self {
                                    #(#fields_idents_excluding_primary_key_token_stream),*
                                })
                            }
                        }
                    }
                };
                let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                    let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = {
                        quote::quote! {
                            #derive_debug_thiserror_error_occurence_token_stream
                            pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                                //todo remove useless variant and add later protentian errors from conversion from with serialize_deserialize
                                #not_uuid_token_upper_camel_case_stream {
                                    #eo_display_with_serialize_deserialize_token_stream
                                    #not_uuid_token_snake_case_stream: std::string::String,//HERE//#crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream
                                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                                },
                            }
                        }
                    };
                    quote::quote! {
                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream

                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                let mut elements = std::vec::Vec::with_capacity(value.0.len());
                                for element in value.0 {
                                    match #operation_payload_element_upper_camel_case_token_stream::try_from(element) {
                                        Ok(value) => {
                                            elements.push(value);
                                        }
                                        Err(#error_value_snake_case_token_stream) => todo!()
                                    }
                                }
                                Ok(Self(elements))
                            }
                        }
                    }
                };
                quote::quote! {
                    #impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream = {
                    let fields_assignment_excluding_primary_key_token_stream =
                        fields_named_wrappers_excluding_primary_key
                            .iter()
                            .map(|element| {
                                generate_let_field_ident_value_field_ident_from_token_stream(
                                    element,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                )
                            });
                    let fields_idents_excluding_primary_key_token_stream =
                        fields_named_wrappers_excluding_primary_key
                            .iter()
                            .map(|element| &element.field_ident);
                    quote::quote! {
                        impl std::convert::From<#operation_payload_element_upper_camel_case_token_stream> for #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream {
                            fn from(value: #operation_payload_element_upper_camel_case_token_stream) -> Self {
                                #(#fields_assignment_excluding_primary_key_token_stream)*
                                Self{
                                    #(#fields_idents_excluding_primary_key_token_stream),*
                                }
                            }
                        }
                    }
                };
                quote::quote! {
                    #impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream
                    impl std::convert::From<#operation_payload_upper_camel_case_token_stream> for #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        fn from(value: #operation_payload_upper_camel_case_token_stream) -> Self {
                            Self(
                                value.0.into_iter()
                                .map(|element|#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream::from(element))
                                .collect::<std::vec::Vec<#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream>>()
                            )
                        }
                    }
                }
            };
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            quote::quote! {
                #payload_token_stream
                #payload_with_serialize_deserialize_token_stream
                #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream

                #derive_debug_token_stream
                pub struct #operation_parameters_upper_camel_case_token_stream {
                    pub #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream,
                }
            }
        };
        // println!("{parameters_token_stream}");
        let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_token_stream = {
            let inner_type_with_serialize_deserialize_error_named_token_stream = {
                let value_stringified = primary_key_syn_field_with_additional_info.rust_sqlx_map_to_postgres_type_variant.get_inner_type_with_serialize_deserialize_error_named_stringified();
                value_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote! {
                #derive_debug_thiserror_error_occurence_token_stream
                pub enum #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_upper_camel_case_token_stream {
                    #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream(
                        // #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream
                        #inner_type_with_serialize_deserialize_error_named_token_stream
                    ),
                }
            }
        };
        // println!("{operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_token_stream}");
        let try_operation_error_with_middleware_error_variants_token_stream = {
            let inner_type_with_serialize_deserialize_token_stream = {
                let value_stringified = primary_key_syn_field_with_additional_info.rust_sqlx_map_to_postgres_type_variant.get_inner_type_with_serialize_deserialize_stringified("");
                value_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            crate::type_variants_from_request_response_generator::type_variants_from_request_response_generator(
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#inner_type_with_serialize_deserialize_token_stream>},//HERE #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream 
                &derive_debug_thiserror_error_occurence_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
                &generate_expected_type_declaration_token_stream,
                &unexpected_status_code_declaration_token_stream,
                &failed_to_get_response_text_declaration_token_stream,
                &deserialize_response_declaration_token_stream,
                &reqwest_declaration_token_stream,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (http_request_token_stream, http_request_test_token_stream) = {
            let try_operation_error_named_token_stream = {
                let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                let expected_type_declaration_token_stream =
                    generate_expected_type_declaration_token_stream(&operation);
                quote::quote! {
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        //todo remove request_error variant
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_declaration_token_stream,
                        #expected_type_declaration_token_stream,
                        #unexpected_status_code_declaration_token_stream,
                        #failed_to_get_response_text_declaration_token_stream,
                        #deserialize_response_declaration_token_stream,
                        #reqwest_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let primary_key_inner_type_token_stream = {
                let value_stringified = primary_key_syn_field_with_additional_info.rust_sqlx_map_to_postgres_type_variant.get_inner_type_stringified("");
                value_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let primary_key_inner_type_with_serialize_deserialize_token_stream = {
                let value_stringified = primary_key_syn_field_with_additional_info.rust_sqlx_map_to_postgres_type_variant.get_inner_type_with_serialize_deserialize_stringified("");
                value_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let http_request_token_stream = generate_http_request_many_token_stream(
                &server_location_name_token_stream,
                &str_ref_token_stream,
                // &std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &quote::quote! {std::vec::Vec<#primary_key_inner_type_token_stream>},
                &serde_json_to_string_token_stream,
                &serde_json_to_string_variant_initialization_token_stream,
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_upper_camel_case_token_stream,
                &operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream,
                &operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_initialization_token_stream,
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#primary_key_inner_type_with_serialize_deserialize_token_stream>},//HERE //#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream
                &deserialize_response_initialization_token_stream,
                &unexpected_status_code_initialization_token_stream,
                &reqwest_initialization_token_stream,
                &failed_to_get_response_text_initialization_token_stream,
                &expected_type_initialization_token_stream,
                &primary_key_syn_field_with_additional_info,
            );
            let http_request_test_token_stream = {
                let element_fields_initialization_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                    let field_ident = element.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    let field_type = &element.ty;
                    quote::quote!{
                        #field_ident: #field_type::default()
                    }
                }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                let test_content_token_stream = quote::quote! {
                    let #primary_keys_token_stream = match #try_operation_snake_case_token_stream(
                        #reference_api_location_test_token_stream,
                        #operation_parameters_upper_camel_case_token_stream {
                            #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream(vec![
                                #operation_payload_element_upper_camel_case_token_stream{
                                    #(#element_fields_initialization_token_stream),*
                                }
                            ])
                        },
                    )
                    .await
                    {
                        Ok(value) => {
                            println!("{value:#?}");
                            value
                        },
                        Err(#error_value_snake_case_token_stream) => panic!(
                            "{}",
                            #error_value_snake_case_token_stream
                        ),//todo remove panic maybe?
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream,
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let query_string_token_stream = {
                    let column_names = fields_named_wrappers_excluding_primary_key.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, element)| {
                        let field_ident = &element.field_ident;
                        let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::global_variables::hardcode::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                        match incremented_index == fields_named_wrappers_excluding_primary_key_len {
                            true => {
                                acc.push_str(&format!("{field_ident}"));
                            },
                            false => {
                                acc.push_str(&format!("{field_ident}{dot_space}"));
                            },
                        }
                        acc
                    });
                    let column_increments = {
                        let mut column_increments = fields_named_wrappers_excluding_primary_key.iter()
                            .enumerate().fold(std::string::String::default(), |mut acc, (index, _)| {
                                acc.push_str(&format!("${}, ", index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::global_variables::hardcode::CHECKED_ADD_NONE_OVERFLOW_MESSAGE))));
                                acc
                            });
                        column_increments.pop();
                        column_increments.pop();
                        column_increments
                    };
                    let query_stringified = format!(
                        "\"{insert_name_stringified} {into_name_stringified} {table_name_stringified} ({column_names}) {select_name_stringified} {column_names} {from_name_stringified} {unnest_name_stringified}({column_increments}) {as_name_stringified} a({column_names}){returning_primary_key_stringified}\""
                    );
                    query_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let column_vecs_token_stream = {
                        let column_vecs_handle_token_stream = {
                            let value = fields_named_wrappers_excluding_primary_key.iter().map(|element| {
                                let field_ident_underscore_vec_stringified = {
                                    let field_ident = &element.field_ident;
                                    format!("{field_ident}{underscore_vec_name_stringified}")
                                };
                                field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            });
                            quote::quote!{#(#value),*}
                        };
                        match is_fields_named_wrappers_excluding_primary_key_len_equals_one {
                            true => column_vecs_handle_token_stream,
                            false => quote::quote!{(#column_vecs_handle_token_stream)}
                        }
                    };
                    let column_vecs_with_capacity_token_stream = {
                        let column_vecs_with_capacity_handle_token_stream = {
                            let value = fields_named_wrappers_excluding_primary_key.iter().map(|_|quote::quote!{std::vec::Vec::with_capacity(#current_vec_len_name_token_stream)});
                            quote::quote!{#(#value),*}
                        };
                        match is_fields_named_wrappers_excluding_primary_key_len_equals_one {
                            true => column_vecs_with_capacity_handle_token_stream,
                            false => quote::quote!{(#column_vecs_with_capacity_handle_token_stream)}
                        }
                    };
                    let columns_acc_push_elements_token_stream = fields_named_wrappers_excluding_primary_key.iter().enumerate().map(|(index, element)|{
                        let field_ident = &element.field_ident;
                        let index_token_stream = match is_fields_named_wrappers_excluding_primary_key_len_equals_one {
                            true => proc_macro2::TokenStream::new(),
                            false => {
                                let index_stringified = index.to_string();
                                let value_token_stream = index_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                quote::quote!{.#value_token_stream}
                            }
                        };
                        //space need to to concat token stream correctly
                        quote::quote!{#acc_name_token_stream #index_token_stream.push(#element_name_token_stream.#field_ident);}
                    });
                    let column_query_bind_vecs_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        let field_ident_underscore_vec_token_stream = {
                            let field_ident_underscore_vec_stringified = {
                                let field_ident = &element.field_ident;
                                format!("{field_ident}{underscore_vec_name_stringified}")
                            };
                            field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let inner_type_token_stream = &element.inner_type_token_stream;
                        quote::quote!{
                            #query_name_token_stream = #query_name_token_stream.bind(
                                //todo add ::<T> for serde json <T> case. for others just empty token stream
                                #inner_type_token_stream::#into_inner_type_vec_snake_case_token_stream(#field_ident_underscore_vec_token_stream)
                            );
                        }
                    });
                    quote::quote! {
                        let mut #query_name_token_stream = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        let #current_vec_len_name_token_stream = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.0.len();
                        let #column_vecs_token_stream = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.0.into_iter().fold(#column_vecs_with_capacity_token_stream,
                        |mut #acc_name_token_stream, #element_name_token_stream| {
                            #(#columns_acc_push_elements_token_stream)*
                            #acc_name_token_stream
                        });
                        #(#column_query_bind_vecs_token_stream)*
                        #query_name_token_stream
                    }
                };
                // println!("{binded_query_token_stream}");
                let from_log_and_return_error_token_stream =
                    crate::from_log_and_return_error::from_log_and_return_error(
                        &try_operation_upper_camel_case_token_stream,
                        &error_log_call_token_stream,
                        &try_operation_response_variants_token_stream,
                    );
                let acquire_pool_and_connection_token_stream =
                    crate::acquire_pool_and_connection::acquire_pool_and_connection(
                        &from_log_and_return_error_token_stream,
                        &pg_connection_token_stream,
                    );
                let primary_key_inner_type_with_serialize_deserialize_token_stream = {
                    let value_stringified = primary_key_syn_field_with_additional_info.rust_sqlx_map_to_postgres_type_variant.get_inner_type_with_serialize_deserialize_stringified("");
                    value_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    let #query_string_name_token_stream = {
                        #query_string_token_stream
                    };
                    println!("{}", #query_string_name_token_stream);
                    let #binded_query_name_token_stream = {
                        #binded_query_token_stream
                    };
                    #acquire_pool_and_connection_token_stream
                    let mut rows = #binded_query_name_token_stream.fetch(#pg_connection_token_stream.as_mut());
                    let mut vec_values = std::vec::Vec::new();
                    while let Some(row) = {
                        match {
                            use futures::TryStreamExt;
                            rows.try_next()
                        }
                        .await
                        {
                            Ok(value) => value,
                            Err(#error_value_snake_case_token_stream) => {
                                let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::from(#error_value_snake_case_token_stream);
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                            }
                        }
                    } {
                        match {
                            use #sqlx_row_token_stream;
                            row.try_get::<#sqlx_types_uuid_token_stream, #str_ref_token_stream>(#primary_key_field_ident_quotes_token_stream)
                        } {
                            Ok(value) => {
                                vec_values.push(
                                    // #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream //HERE
                                    #primary_key_inner_type_with_serialize_deserialize_token_stream::from(value),
                                );
                            }
                            Err(#error_value_snake_case_token_stream) => {
                                let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                            }
                        }
                    }
                    #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(vec_values)
                }
            };
            // println!("{try_operation_token_stream}");
            let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                &table_name_stringified,
                &unique_status_codes,
                &application_json_quotes_token_stream,
                &table_name_quotes_token_stream,
                &operation_payload_upper_camel_case_token_stream,
                &operation,
            );
            let field_code_occurence_new_91c61a45_6c97_47cc_ac96_65bdcfec0494_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> impl #axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(#error_value_snake_case_token_stream) => {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: #error_value_snake_case_token_stream,
                                        #field_code_occurence_new_91c61a45_6c97_47cc_ac96_65bdcfec0494_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            },
                            Err(#error_value_snake_case_token_stream) => {
                                return #error_value_snake_case_token_stream;
                            }
                        },
                    };
                    println!("{:#?}", #parameters_snake_case_token_stream);
                    {
                        #try_operation_token_stream
                    }
                }
            }
        };
        // println!("{route_handler_token_stream}");
        let common_middlewares_error_syn_variants_from_impls =
            generate_common_middlewares_error_syn_variants_from_impls(
                common_middlewares_error_syn_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                additional_http_status_codes_error_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote! {
                #parameters_token_stream
                #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &create_many_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (create_one_token_stream, create_one_http_request_test_token_stream) = {
        let operation = Operation::CreateOne;
        let operation_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation);
        let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(&operation);
        let operation_payload_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation);
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(&operation);
        let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(&operation);
        let try_operation_response_variants_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(&operation);
        let try_operation_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfUpperCamelCaseTokenStream::try_self_upper_camel_case_token_stream(&operation);
        let additional_http_status_codes_error_variants = vec![]; //todo find out why rust analyzer crashes
                                                                  // crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_method_proc_macro_attribute(
                                                                  //     &ast,
                                                                  //     &operation_name_snake_case_stringified,
                                                                  //     additional_http_status_codes_error_variants_snake_case_stringified,
                                                                  //     &proc_macro_name_snake_case,
                                                                  //     &proc_macro_name_upper_camel_case_ident_stringified
                                                                  // );
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence,
                    &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
                    proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
                        &operation
                    )
                )
            ]
        );
        let type_variants_from_request_response_syn_variants = {
            let full_additional_http_status_codes_error_variants =
                generate_full_additional_http_status_codes_error_variants(
                    common_middlewares_error_syn_variants.iter().collect(),
                    additional_http_status_codes_error_variants.iter().collect(),
                );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response =
                    std::vec::Vec::with_capacity(common_error_syn_variants.len() + 2);
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response.push(&operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants,
            )
        };
        let desirable_status_code = proc_macro_helpers::status_code::StatusCode::Tvfrr201Created;
        let unique_status_codes = generate_unique_status_codes(
            &desirable_status_code,
            &type_variants_from_request_response_syn_variants,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let payload_token_stream = {
                let fields_with_excluded_primary_key_token_stream =
                    fields_named_wrappers_excluding_primary_key
                        .iter()
                        .map(|element| {
                            generate_pub_field_ident_field_type_token_stream(
                                element,
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            )
                        });
                quote::quote! {
                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream {
                        #(#fields_with_excluded_primary_key_token_stream),*
                    }
                }
            };
            // println!("{payload_token_stream}");
            let payload_with_serialize_deserialize_token_stream = {
                let fields_with_excluded_primary_key_token_stream =
                    fields_named_wrappers_excluding_primary_key
                        .iter()
                        .map(|element| {
                            generate_field_ident_field_type_with_serialize_deserialize_token_stream(
                                element,
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            )
                        });
                quote::quote! {
                    #derive_debug_serialize_deserialize_to_schema_token_stream
                    pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        #(#fields_with_excluded_primary_key_token_stream),*
                    }
                }
            };
            // println!("{payload_with_serialize_deserialize_token_stream}");
            let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = {
                    let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                        &fields_named_wrappers_excluding_primary_key,
                        &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream
                    );
                    quote::quote! {
                        #derive_debug_thiserror_error_occurence_token_stream
                        pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                            //todo errors in case of conversion to original type from with_serialize_deserialize failed and remove not_uuid_token_upper_camel_case_stream - useless
                            #not_uuid_token_upper_camel_case_stream {
                                #eo_error_occurence_attribute_token_stream
                                #not_uuid_token_snake_case_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                            },
                            #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                        }
                    }
                };
                // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                    let field_code_occurence_new_3763990f_5c49_47d0_a774_5ef584cd1236_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter()
                        .map(|element| generate_let_field_ident_value_field_ident_try_from_token_stream(
                            element,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                            &field_code_occurence_new_3763990f_5c49_47d0_a774_5ef584cd1236_token_stream,
                        ));
                    let fields_idents_excluding_primary_key_token_stream =
                        fields_named_wrappers_excluding_primary_key
                            .iter()
                            .map(|element| &element.field_ident);
                    quote::quote! {
                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                #(#fields_assignment_excluding_primary_key_token_stream)*
                                Ok(Self {
                                    #(#fields_idents_excluding_primary_key_token_stream),*
                                })
                            }
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let fields_assignment_excluding_primary_key_token_stream =
                    fields_named_wrappers_excluding_primary_key
                        .iter()
                        .map(|element| {
                            generate_let_field_ident_value_field_ident_from_token_stream(
                                element,
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            )
                        });
                let fields_idents_excluding_primary_key_token_stream =
                    fields_named_wrappers_excluding_primary_key
                        .iter()
                        .map(|element| &element.field_ident);
                quote::quote! {
                    impl std::convert::From<#operation_payload_upper_camel_case_token_stream> for #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        fn from(value: #operation_payload_upper_camel_case_token_stream) -> Self {
                            #(#fields_assignment_excluding_primary_key_token_stream)*
                            Self{
                                #(#fields_idents_excluding_primary_key_token_stream),*
                            }
                        }
                    }
                }
            };
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            quote::quote! {
                #payload_token_stream
                #payload_with_serialize_deserialize_token_stream
                #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream

                #derive_debug_token_stream
                pub struct #operation_parameters_upper_camel_case_token_stream {
                    pub #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream,
                }
            }
        };
        // println!("{parameters_token_stream}");
        let try_operation_error_with_middleware_error_variants_token_stream = {
            crate::type_variants_from_request_response_generator::type_variants_from_request_response_generator(
                &desirable_status_code,
                &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
                &generate_expected_type_declaration_token_stream,
                &unexpected_status_code_declaration_token_stream,
                &failed_to_get_response_text_declaration_token_stream,
                &deserialize_response_declaration_token_stream,
                &reqwest_declaration_token_stream,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (http_request_token_stream, http_request_test_token_stream) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let expected_type_declaration_token_stream =
                    generate_expected_type_declaration_token_stream(&operation);
                quote::quote! {
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_declaration_token_stream,
                        #expected_type_declaration_token_stream,
                        #unexpected_status_code_declaration_token_stream,
                        #failed_to_get_response_text_declaration_token_stream,
                        #deserialize_response_declaration_token_stream,
                        #reqwest_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_try_operation_token_stream(
                &server_location_name_token_stream,
                &str_ref_token_stream,
                &crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &quote::quote! {
                    let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(&#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)) {
                        Ok(value) => value,
                        Err(#error_value_snake_case_token_stream) => {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                        }
                    };
                },
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &quote::quote! {
                    match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value) {
                        Ok(value) => Ok(value),
                        Err(#error_value_snake_case_token_stream) => Err(
                            #try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_initialization_token_stream
                        )
                    }
                },
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream, //todo reuse it
                &deserialize_response_initialization_token_stream,
                &unexpected_status_code_initialization_token_stream,
                &reqwest_initialization_token_stream,
                &failed_to_get_response_text_initialization_token_stream,
                &expected_type_initialization_token_stream,
            );
            let http_request_test_token_stream = {
                let element_fields_initialization_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                    let field_ident = element.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    let field_type = &element.ty;
                    quote::quote!{
                        #field_ident: #field_type::default()
                    }
                }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                let test_content_token_stream = quote::quote! {
                    let #primary_key_token_stream = match #try_operation_snake_case_token_stream(
                        #reference_api_location_test_token_stream,
                        #operation_parameters_upper_camel_case_token_stream {
                            #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
                                #(#element_fields_initialization_token_stream),*
                            }
                        },
                    )
                    .await
                    {
                        Ok(value) => {
                            println!("{value:#?}");
                            value
                        },
                        Err(#error_value_snake_case_token_stream) => panic!(
                            "{}",
                            #error_value_snake_case_token_stream
                        )
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream,
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let query_string_token_stream = {
                    let (column_names, column_increments) = {
                        fields_named_wrappers_excluding_primary_key.iter().enumerate().fold((
                            std::string::String::default(),
                            std::string::String::default()
                        ), |mut acc, (index, element)| {
                            let field_ident = &element.field_ident;
                            let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::global_variables::hardcode::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                            match incremented_index == fields_named_wrappers_excluding_primary_key_len {
                                true => {
                                    acc.0.push_str(&format!("{field_ident}"));
                                    acc.1.push_str(&format!("${incremented_index}"));
                                },
                                false => {
                                    acc.0.push_str(&format!("{field_ident}{dot_space}"));
                                    acc.1.push_str(&format!("${incremented_index}{dot_space}"));
                                },
                            }
                            acc
                        })
                    };
                    let query_stringified = format!("\"{insert_name_stringified} {into_name_stringified} {table_name_stringified}({column_names}) {values_name_stringified} ({column_increments}){returning_primary_key_stringified}\"");
                    query_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        quote::quote!{
                            query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#field_ident, query);
                        }
                    });
                    quote::quote! {
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        #(#binded_query_modifications_token_stream)*
                        query
                    }
                };
                // println!("{binded_query_token_stream}");
                let from_log_and_return_error_token_stream =
                    crate::from_log_and_return_error::from_log_and_return_error(
                        &try_operation_upper_camel_case_token_stream,
                        &error_log_call_token_stream,
                        &try_operation_response_variants_token_stream,
                    );
                let acquire_pool_and_connection_token_stream =
                    crate::acquire_pool_and_connection::acquire_pool_and_connection(
                        &from_log_and_return_error_token_stream,
                        &pg_connection_token_stream,
                    );
                quote::quote! {
                    let #query_string_name_token_stream = {
                        #query_string_token_stream
                    };
                    println!("{}", #query_string_name_token_stream);
                    let #binded_query_name_token_stream = {
                        #binded_query_token_stream
                    };
                    #acquire_pool_and_connection_token_stream
                    match #binded_query_name_token_stream.fetch_one(#pg_connection_token_stream.as_mut()).await {
                        Ok(value) => match {
                            use #sqlx_row_token_stream;
                            value.try_get::<#sqlx_types_uuid_token_stream, #str_ref_token_stream>(#primary_key_field_ident_quotes_token_stream)
                        } {
                            Ok(value) => #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value)),
                            Err(#error_value_snake_case_token_stream) => {
                                let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                            }
                        },
                        Err(#error_value_snake_case_token_stream) => {
                            #from_log_and_return_error_token_stream
                        }
                    }
                }
            };
            let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //todo different parameters
                &table_name_stringified,
                &unique_status_codes,
                &application_json_quotes_token_stream,
                &table_name_quotes_token_stream,
                &operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                &operation,
            );
            let field_code_occurence_new_db0d95a8_8dcc_4228_80ec_e5dce2003333_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> impl #axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(#error_value_snake_case_token_stream) => {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: #error_value_snake_case_token_stream,
                                        #field_code_occurence_new_db0d95a8_8dcc_4228_80ec_e5dce2003333_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            },
                            Err(#error_value_snake_case_token_stream) => {
                                return #error_value_snake_case_token_stream;
                            }
                        },
                    };
                    println!("{:#?}", #parameters_snake_case_token_stream);
                    {
                        #try_operation_token_stream
                    }
                }
            }
        };
        // println!("{route_handler_token_stream}");
        let common_middlewares_error_syn_variants_from_impls =
            generate_common_middlewares_error_syn_variants_from_impls(
                common_middlewares_error_syn_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                additional_http_status_codes_error_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &create_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (read_many_token_stream, read_many_http_request_test_token_stream) = {
        let operation = Operation::ReadMany;
        let operation_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation);
        let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(&operation);
        let operation_payload_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation);
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(&operation);
        let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(&operation);
        let try_operation_response_variants_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(&operation);
        let try_operation_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfUpperCamelCaseTokenStream::try_self_upper_camel_case_token_stream(&operation);
        let additional_http_status_codes_error_variants = vec![]; //todo find out why rust analyzer crashes
                                                                  // crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_method_proc_macro_attribute(
                                                                  //     &ast,
                                                                  //     &operation_name_snake_case_stringified,
                                                                  //     additional_http_status_codes_error_variants_snake_case_stringified,
                                                                  //     &proc_macro_name_snake_case,
                                                                  //     &proc_macro_name_upper_camel_case_ident_stringified
                                                                  // );
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence,
                    &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
                    proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
                        &operation
                    )
                )
            ]
        );
        let type_variants_from_request_response_syn_variants = {
            let full_additional_http_status_codes_error_variants =
                generate_full_additional_http_status_codes_error_variants(
                    common_middlewares_error_syn_variants.iter().collect(),
                    additional_http_status_codes_error_variants.iter().collect(),
                );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
                    common_error_syn_variants.len() + not_unique_vec_syn_variants.len() + 4,
                );
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                for element in &not_unique_vec_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                type_variants_from_request_response.push(&not_unique_primary_keys_syn_variant);
                type_variants_from_request_response.push(&bind_query_syn_variant);
                type_variants_from_request_response.push(&not_uuid_syn_variant);
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants,
            )
        };
        let desirable_status_code = proc_macro_helpers::status_code::StatusCode::Tvfrr200Ok;
        let unique_status_codes = generate_unique_status_codes(
            &desirable_status_code,
            &type_variants_from_request_response_syn_variants,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let payload_token_stream = {
                let fields_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                    let field_ident = &element.field_ident;
                    quote::quote!{
                        pub #field_ident: std::option::Option<std::vec::Vec<#crate_server_postgres_regex_filter_regex_filter_token_stream>>,
                    }
                });
                quote::quote! {
                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream {
                        pub #primary_key_field_ident: std::option::Option<#std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream>,
                        #(#fields_with_excluded_primary_key_token_stream)*
                        pub #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream,
                        pub #order_by_token_stream: #crate_server_postgres_order_by_order_by_token_stream<#ident_column_upper_camel_case_token_stream>,
                        pub #limit_token_stream: #crate_server_postgres_postgres_bigint_postgres_bigint_token_stream,
                        pub #offset_token_stream: #crate_server_postgres_postgres_bigint_postgres_bigint_token_stream,
                    }
                }
            };
            // println!("{payload_token_stream}");
            let payload_with_serialize_deserialize_token_stream = {
                let fields_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                    let field_ident = &element.field_ident;
                    quote::quote!{
                        #field_ident: std::option::Option<std::vec::Vec<#crate_server_postgres_regex_filter_regex_filter_token_stream>>,
                    }
                });
                quote::quote! {
                    #derive_debug_serialize_deserialize_token_stream
                    pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        #primary_key_field_ident: std::option::Option<std::vec::Vec<#std_string_string_token_stream>>,//todo maybe possible uuid wrapper
                        #(#fields_with_excluded_primary_key_token_stream)*
                        #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream,
                        #order_by_token_stream: #crate_server_postgres_order_by_order_by_token_stream<#ident_column_upper_camel_case_token_stream>,
                        #limit_token_stream: #crate_server_postgres_postgres_bigint_postgres_bigint_token_stream,
                        #offset_token_stream: #crate_server_postgres_postgres_bigint_postgres_bigint_token_stream,
                    }
                }
            };
            // println!("{payload_with_serialize_deserialize_token_stream}");
            let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = {
                    let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                        &fields_named_wrappers_excluding_primary_key,
                        &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream
                    );
                    quote::quote! {
                        #derive_debug_thiserror_error_occurence_token_stream
                        pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                            #not_uuid_token_upper_camel_case_stream {
                                #eo_error_occurence_attribute_token_stream
                                #not_uuid_token_snake_case_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                            },
                            #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                        }
                    }
                };
                // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                    let primary_key_field_assignment_token_stream = {
                        let field_code_occurence_new_ed55593d_d353_440c_8145_c1c712bc5ace_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote! {
                            let #primary_key_field_ident = match value.#primary_key_field_ident {
                                Some(value) => match value.into_iter()
                                    .map(|element|#crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(element)))
                                    .collect::<Result<
                                        #std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                                        #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream
                                    >>()
                                {
                                    Ok(value) => Some(value),
                                    Err(#error_value_snake_case_token_stream) => {
                                        return Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                            #not_uuid_token_snake_case_stream: #error_value_snake_case_token_stream,
                                            #field_code_occurence_new_ed55593d_d353_440c_8145_c1c712bc5ace_token_stream,
                                        });
                                    }
                                },
                                None => None
                            };
                        }
                    };
                    let field_code_occurence_new_5ad5fc24_5f04_4749_bf9e_17f74223d293_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter()
                        .map(|element| generate_let_field_ident_value_field_ident_try_from_token_stream(
                            element,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                            &field_code_occurence_new_5ad5fc24_5f04_4749_bf9e_17f74223d293_token_stream,
                        ));
                    quote::quote! {
                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                #primary_key_field_assignment_token_stream
                                #(#fields_assignment_excluding_primary_key_token_stream)*
                                let #select_snake_case_token_stream = value.#select_snake_case_token_stream;
                                let #order_by_snake_case_token_stream = value.#order_by_snake_case_token_stream;
                                let #limit_snake_case_token_stream = value.#limit_snake_case_token_stream;
                                let #offset_snake_case_token_stream = value.#offset_snake_case_token_stream;
                                Ok(Self {
                                    #(#fields_named_idents_comma_token_stream)*
                                    #select_snake_case_token_stream,
                                    #order_by_snake_case_token_stream,
                                    #limit_snake_case_token_stream,
                                    #offset_snake_case_token_stream,
                                })
                            }
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let primary_key_field_assignment_token_stream = {
                    quote::quote! {
                        let #primary_key_field_ident = match value.#primary_key_field_ident {
                            Some(value) => Some(value.into_iter().map(|element|element.to_string()).collect::<std::vec::Vec<#std_string_string_token_stream>>()),
                            None => None
                        };
                    }
                };
                let fields_assignment_excluding_primary_key_token_stream =
                    fields_named_wrappers_excluding_primary_key
                        .iter()
                        .map(|element| {
                            generate_let_field_ident_value_field_ident_from_token_stream(
                                element,
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            )
                        });
                quote::quote! {
                    impl std::convert::From<#operation_payload_upper_camel_case_token_stream> for #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        fn from(value: #operation_payload_upper_camel_case_token_stream) -> Self {
                            #primary_key_field_assignment_token_stream
                            #(#fields_assignment_excluding_primary_key_token_stream)*
                            let #select_snake_case_token_stream = value.#select_snake_case_token_stream;
                            let #order_by_snake_case_token_stream = value.#order_by_snake_case_token_stream;
                            let #limit_snake_case_token_stream = value.#limit_snake_case_token_stream;
                            let #offset_snake_case_token_stream = value.#offset_snake_case_token_stream;
                            Self{
                                #(#fields_named_idents_comma_token_stream)*
                                #select_snake_case_token_stream,
                                #order_by_snake_case_token_stream,
                                #limit_snake_case_token_stream,
                                #offset_snake_case_token_stream,
                            }
                        }
                    }
                }
            };
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            quote::quote! {
                #payload_token_stream
                #payload_with_serialize_deserialize_token_stream
                #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream

                #derive_debug_token_stream
                pub struct #operation_parameters_upper_camel_case_token_stream {
                    pub #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream,
                }
            }
        };
        // println!("{parameters_token_stream}");
        let try_operation_error_with_middleware_error_variants_token_stream = {
            crate::type_variants_from_request_response_generator::type_variants_from_request_response_generator(
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#struct_options_ident_token_stream>},//todo reuse it
                &derive_debug_thiserror_error_occurence_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
                &generate_expected_type_declaration_token_stream,
                &unexpected_status_code_declaration_token_stream,
                &failed_to_get_response_text_declaration_token_stream,
                &deserialize_response_declaration_token_stream,
                &reqwest_declaration_token_stream,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (http_request_token_stream, http_request_test_token_stream) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let expected_type_declaration_token_stream =
                    generate_expected_type_declaration_token_stream(&operation);
                quote::quote! {
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #expected_type_declaration_token_stream,
                        #unexpected_status_code_declaration_token_stream,
                        #failed_to_get_response_text_declaration_token_stream,
                        #deserialize_response_declaration_token_stream,
                        #reqwest_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_try_operation_token_stream(
                &server_location_name_token_stream,
                &str_ref_token_stream,
                &quote::quote! {std::vec::Vec<#struct_options_ident_token_stream>},
                &quote::quote! {
                    let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(
                        &#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)
                    ) {
                        Ok(value) => value,
                        Err(#error_value_snake_case_token_stream) => {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                        }
                    };
                },
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &quote::quote! {
                    Ok(value)
                },
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &quote::quote! {std::vec::Vec::<#struct_options_ident_token_stream>}, //todo reuse it
                &deserialize_response_initialization_token_stream,
                &unexpected_status_code_initialization_token_stream,
                &reqwest_initialization_token_stream,
                &failed_to_get_response_text_initialization_token_stream,
                &expected_type_initialization_token_stream,
            );
            let http_request_test_token_stream = {
                let order_initialization_token_stream = Order::Desc.to_token_stream();
                let fields_initialization_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                    let field_ident = element.ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    quote::quote!{
                        #field_ident: None,//todo maybe generate all the possible versions for what need to have?
                    }
                }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                let test_content_token_stream = quote::quote! {
                    match #try_operation_snake_case_token_stream(
                        #reference_api_location_test_token_stream,
                        //todo - builder pattern?
                        #operation_parameters_upper_camel_case_token_stream {
                            #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
                                #primary_key_field_ident: Some(#primary_keys_token_stream.clone()),
                                #(#fields_initialization_excluding_primary_key_token_stream)*
                                #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream::#select_full_variant_token_stream,
                                #order_by_snake_case_token_stream: #crate_server_postgres_order_by_order_by_token_stream {
                                    #column_snake_case_token_stream: #ident_column_upper_camel_case_token_stream::Name,
                                    #order_snake_case_token_stream: Some(#crate_server_postgres_order_order_token_stream::#order_initialization_token_stream),//todo remove option here
                                },
                                #limit_snake_case_token_stream: #crate_server_postgres_postgres_bigint_postgres_bigint_token_stream(#limit_snake_case_token_stream),
                                #offset_snake_case_token_stream: #crate_server_postgres_postgres_bigint_postgres_bigint_token_stream(#offset_snake_case_token_stream),
                            }
                        },
                    )
                    .await
                    {
                        Ok(value) => {
                            println!("{value:#?}");
                            value
                        },
                        Err(#error_value_snake_case_token_stream) => panic!(
                            "{}",
                            #error_value_snake_case_token_stream
                        )
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream,
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let filter_unique_parameters_token_stream = {
                    let filter_unique_parameters_primary_key_token_stream = quote::quote! {
                        if let Some(#primary_key_field_ident) = &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident {
                            let #not_unique_primary_keys_name_token_stream = {
                                let mut vec = std::vec::Vec::with_capacity(#primary_key_field_ident.len());
                                let mut #not_unique_primary_keys_name_token_stream = std::vec::Vec::with_capacity(#primary_key_field_ident.len());
                                for element in #primary_key_field_ident {
                                    let handle = element;
                                    match vec.contains(&handle) {
                                        true => {
                                            #not_unique_primary_keys_name_token_stream.push(element.clone());
                                        },
                                        false => {
                                            vec.push(element);
                                        }
                                    }
                                }
                                #not_unique_primary_keys_name_token_stream
                            };
                            if let false = #not_unique_primary_keys_name_token_stream.is_empty() {
                                let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#not_unique_primary_key_variant_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                            }
                        }
                    };
                    let filter_unique_parameters_other_columns_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let field_handle_token_stream = {
                            let field_handle_stringified = format!("{field_ident}_handle");
                            field_handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let not_unique_field_vec_snake_case_token_stream = {
                            let not_unique_field_vec_snake_case_stringified = generate_not_unique_field_vec_snake_case_stringified(field_ident);
                            not_unique_field_vec_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_unique_field_vec_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let not_unique_field_vec_vec_upper_camel_token_stream = {
                            let not_unique_field_vec_upper_camel_stringified = generate_not_unique_field_vec_upper_camel_stringified(field_ident);
                            not_unique_field_vec_upper_camel_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_unique_field_vec_upper_camel_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let field_code_occurence_new_eb1a9553_449e_4767_9e5c_c1856b77bd4e_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote!{
                            let #field_handle_token_stream = match #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#field_ident {
                                Some(value) => {
                                    let is_unique = {
                                        let mut vec = std::vec::Vec::with_capacity(value.len());
                                        let mut is_unique = true;
                                        for element in &value {
                                            match vec.contains(&element) {
                                                true => {
                                                    is_unique = false;
                                                    break;
                                                }
                                                false => {
                                                    vec.push(element);
                                                }
                                            }
                                        }
                                        is_unique
                                    };
                                    match is_unique {
                                        true => Some(value),
                                        false => {
                                            let #not_unique_field_vec_snake_case_token_stream = {
                                                let mut vec = std::vec::Vec::with_capacity(value.len());
                                                let mut #not_unique_field_vec_snake_case_token_stream = std::vec::Vec::with_capacity(value.len());
                                                for element in value {
                                                    match vec.contains(&element) {
                                                        true => {
                                                            #not_unique_field_vec_snake_case_token_stream.push(element);
                                                        }
                                                        false => {
                                                            vec.push(element);
                                                        }
                                                    }
                                                }
                                                #not_unique_field_vec_snake_case_token_stream
                                            };
                                            let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#not_unique_field_vec_vec_upper_camel_token_stream {
                                                #not_unique_field_vec_snake_case_token_stream,
                                                #field_code_occurence_new_eb1a9553_449e_4767_9e5c_c1856b77bd4e_token_stream,
                                            };
                                            #error_log_call_token_stream
                                            return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
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
                            let prefix_false_handle_stringified =
                                format!("\" {and_name_stringified}\"");
                            prefix_false_handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {prefix_false_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let handle_token_stream = {
                            let handle_stringified = format!("\"{{}} {primary_key_field_ident} {in_name_stringified} ({select_name_stringified} {unnest_name_stringified}(${{}}))\"");
                            handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote! {
                            if let Some(value) = &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident {
                                let prefix = match additional_parameters.is_empty() {
                                    true => #where_name_qoutes_token_stream,
                                    false => #prefix_false_handle_token_stream,
                                };
                                match increment.checked_add(1) {
                                    Some(value) => {
                                        increment = value;
                                    },
                                    None => {
                                        //todo - think what to do with #crate_server_postgres_bind_query_try_generate_bind_increments_error_named_name_token_stream and how handle it
                                        let e = #crate_server_postgres_bind_query_try_generate_bind_increments_error_named_name_token_stream::#checked_add_variant_initialization_token_stream;
                                        return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
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
                    let additional_parameters_modification_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        let handle_token_stream = {
                            let handle_stringified = format!("\"{field_ident} ~ {{value}} \"");
                            handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let prefix_false_handle_token_stream = {
                            let prefix_false_handle_stringified = format!("\" {and_name_stringified}\"");
                            prefix_false_handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {prefix_false_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let field_handle_token_stream = {
                            let field_handle_stringified = format!("{field_ident}_handle");
                            field_handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{
                            if let Some(value) = &#field_handle_token_stream {
                                let prefix = match additional_parameters.is_empty() {
                                    true => #where_name_qoutes_token_stream,
                                    false => #prefix_false_handle_token_stream,
                                };
                                let bind_increments = {
                                    let mut bind_increments = #std_string_string_token_stream::default();
                                    for (index, element) in value.iter().enumerate() {
                                        match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(
                                            element,
                                            &mut increment
                                        ) {
                                            Ok(value) => {
                                                let handle = format!(#handle_token_stream);
                                                match index == 0 {
                                                    true => {
                                                        bind_increments.push_str(&handle);
                                                    },
                                                    false => {
                                                        bind_increments.push_str(&format!("{} {handle}", element.conjuctive_operator));
                                                    },
                                                }
                                            },
                                            Err(#error_value_snake_case_token_stream) => {
                                                return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
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
                        let handle_stringified = format!("\"{select_name_stringified} {{}} {from_name_stringified} {table_name_stringified} {{}}\"");
                        handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let additional_parameters_order_by_handle_token_stream = {
                        let additional_parameters_order_by_handle_stringified =
                            format!("\"{{}}{order_by_name_stringified} {{}} {{}}\"");
                        additional_parameters_order_by_handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_order_by_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let additional_parameters_limit_handle_token_stream = {
                        let additional_parameters_limit_handle_stringified =
                            format!("\"{{}}{limit_name_stringified} {{}}\"");
                        additional_parameters_limit_handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_limit_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let additional_parameters_offset_handle_token_stream = {
                        let additional_parameters_offset_handle_stringified =
                            format!("\"{{}}{offset_name_stringified} {{}}\"");
                        additional_parameters_offset_handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_offset_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote! {
                        format!(
                            #handle_token_stream,
                            crate::server::postgres::generate_query::GenerateQuery::generate_query(
                                &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#select_snake_case_token_stream
                            ),
                            {
                                #increment_initialization_token_stream
                                let mut additional_parameters = #std_string_string_token_stream::default();
                                #additional_parameters_primary_key_modification_token_stream
                                #(#additional_parameters_modification_token_stream)*
                                {
                                    let prefix = match additional_parameters.is_empty() {
                                        true => "",
                                        false => " ",
                                    };
                                    let value = &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#order_by_token_stream;
                                    let order_stringified = match &value.order {
                                        Some(order) => order.to_string(),
                                        None => #crate_server_postgres_order_order_token_stream::default().to_string(),
                                    };
                                    additional_parameters.push_str(&format!(
                                        #additional_parameters_order_by_handle_token_stream,
                                        prefix,
                                        value.column,
                                        order_stringified
                                    ));
                                }
                                {
                                    let prefix = match additional_parameters.is_empty() {
                                        true => "",
                                        false => " ",
                                    };
                                    let value = match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(
                                        &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.limit,
                                        &mut increment
                                    ) {
                                        Ok(value) => value,
                                        Err(#error_value_snake_case_token_stream) => {
                                            return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
                                        },
                                    };
                                    additional_parameters.push_str(&format!(
                                        #additional_parameters_limit_handle_token_stream,
                                        prefix,
                                        value
                                    ));
                                }
                                {
                                    let prefix = match additional_parameters.is_empty() {
                                        true => "",
                                        false => " ",
                                    };
                                    let value = match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(
                                        &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.offset,
                                        &mut increment
                                    ) {
                                        Ok(value) => value,
                                        Err(#error_value_snake_case_token_stream) => {
                                            return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
                                        },
                                    };
                                    additional_parameters.push_str(&format!(
                                        #additional_parameters_offset_handle_token_stream,
                                        prefix,
                                        value
                                    ));
                                }
                                additional_parameters
                            }
                        )
                    }
                };
                let binded_query_token_stream = {
                    let binded_query_primary_key_modification_token_stream = quote::quote! {
                        if let Some(value) = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident {
                            query = query.bind(value.into_iter().map(|element|element.into_inner().clone()).collect::<std::vec::Vec<#sqlx_types_uuid_token_stream>>());
                        }
                    };
                    let binded_query_modifications_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        let field_handle_token_stream = {
                            let field_handle_stringified = format!("{field_ident}_handle");
                            field_handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{
                            if let Some(values) = #field_handle_token_stream {
                                for value in values {
                                    query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
                                        value, query,
                                    );
                                }
                            }
                        }
                    });
                    quote::quote! {
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        #binded_query_primary_key_modification_token_stream
                        #(#binded_query_modifications_token_stream)*
                        query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case_token_stream.#payload_snake_case_token_stream.limit,
                            query,
                        );
                        query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case_token_stream.#payload_snake_case_token_stream.offset,
                            query,
                        );
                        query
                    }
                };
                let from_log_and_return_error_token_stream =
                    crate::from_log_and_return_error::from_log_and_return_error(
                        &try_operation_upper_camel_case_token_stream,
                        &error_log_call_token_stream,
                        &try_operation_response_variants_token_stream,
                    );
                let acquire_pool_and_connection_token_stream =
                    crate::acquire_pool_and_connection::acquire_pool_and_connection(
                        &from_log_and_return_error_token_stream,
                        &pg_connection_token_stream,
                    );
                quote::quote! {
                    #filter_unique_parameters_token_stream
                    let #query_string_name_token_stream = {
                        #query_string_token_stream
                    };
                    println!("{}", #query_string_name_token_stream);
                    let #binded_query_name_token_stream = {
                        #binded_query_token_stream
                    };
                    let vec_values = {
                        #acquire_pool_and_connection_token_stream
                        let mut rows = #binded_query_name_token_stream.fetch(#pg_connection_token_stream.as_mut());
                        let mut vec_values = std::vec::Vec::new();
                        while let Some(row) = {
                            match {
                                #use_futures_try_stream_ext_token_stream;
                                rows.try_next()
                            }
                            .await
                            {
                                Ok(value) => value,
                                Err(#error_value_snake_case_token_stream) => {
                                    #from_log_and_return_error_token_stream;
                                }
                            }
                        } {
                            match #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#select_snake_case_token_stream.#options_try_from_sqlx_row_name_token_stream(&row) {
                                Ok(value) => {
                                    vec_values.push(value);
                                }
                                Err(#error_value_snake_case_token_stream) => {
                                    #from_log_and_return_error_token_stream;
                                }
                            }
                        }
                        vec_values
                    };
                    #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(vec_values)
                }
            };
            // println!("{try_operation_token_stream}");
            let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                &table_name_stringified,
                &unique_status_codes,
                &application_json_quotes_token_stream,
                &table_name_quotes_token_stream,
                &operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                &operation,
            );
            let field_code_occurence_new_1d57484c_3c67_4f5f_81a6_ec8efc9c6896_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> impl #axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(#error_value_snake_case_token_stream) => {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: #error_value_snake_case_token_stream,
                                        #field_code_occurence_new_1d57484c_3c67_4f5f_81a6_ec8efc9c6896_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            },
                            Err(#error_value_snake_case_token_stream) => {
                                return #error_value_snake_case_token_stream;
                            }
                        },
                    };
                    println!("{:#?}", #parameters_snake_case_token_stream);
                    {
                        #try_operation_token_stream
                    }
                }
            }
        };
        // println!("{route_handler_token_stream}");
        let common_middlewares_error_syn_variants_from_impls =
            generate_common_middlewares_error_syn_variants_from_impls(
                common_middlewares_error_syn_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                additional_http_status_codes_error_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream,
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
        let operation_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation);
        let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(&operation);
        let operation_payload_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation);
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(&operation);
        let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(&operation);
        let try_operation_response_variants_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(&operation);
        let try_operation_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfUpperCamelCaseTokenStream::try_self_upper_camel_case_token_stream(&operation);
        let additional_http_status_codes_error_variants = vec![]; //todo find out why rust analyzer crashes
                                                                  // crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_method_proc_macro_attribute(
                                                                  //     &ast,
                                                                  //     &operation_name_snake_case_stringified,
                                                                  //     additional_http_status_codes_error_variants_snake_case_stringified,
                                                                  //     &proc_macro_name_snake_case,
                                                                  //     &proc_macro_name_upper_camel_case_ident_stringified
                                                                  // );
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence,
                    &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
                    proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
                        &operation
                    )
                )
            ]
        );
        let type_variants_from_request_response_syn_variants = {
            let full_additional_http_status_codes_error_variants =
                generate_full_additional_http_status_codes_error_variants(
                    common_middlewares_error_syn_variants.iter().collect(),
                    additional_http_status_codes_error_variants.iter().collect(),
                );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response =
                    std::vec::Vec::with_capacity(common_error_syn_variants.len() + 1);
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants,
            )
        };
        let desirable_status_code = proc_macro_helpers::status_code::StatusCode::Tvfrr200Ok;
        let unique_status_codes = generate_unique_status_codes(
            &desirable_status_code,
            &type_variants_from_request_response_syn_variants,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let payload_token_stream = {
                quote::quote! {
                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream {
                        pub #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                        pub #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream,
                    }
                }
            };
            // println!("{payload_token_stream}");
            let payload_with_serialize_deserialize_token_stream = {
                quote::quote! {
                    #derive_debug_serialize_deserialize_token_stream
                    pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                        #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream,
                    }
                }
            };
            // println!("{payload_with_serialize_deserialize_token_stream}");
            let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = {
                    quote::quote! {
                        #derive_debug_thiserror_error_occurence_token_stream
                        pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                            #not_uuid_token_upper_camel_case_stream {
                                #eo_error_occurence_attribute_token_stream
                                #not_uuid_token_snake_case_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                            },
                        }
                    }
                };
                // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                    let field_code_occurence_new_3b778bbe_aec5_4ebe_af05_11074800c690_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                let #primary_key_field_ident = match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value.#primary_key_field_ident) {
                                    Ok(value) => value,
                                    Err(#error_value_snake_case_token_stream) => {
                                        return Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                            #not_uuid_token_snake_case_stream: #error_value_snake_case_token_stream,
                                            #field_code_occurence_new_3b778bbe_aec5_4ebe_af05_11074800c690_token_stream,
                                        });
                                    }
                                };
                                let #select_snake_case_token_stream = value.#select_snake_case_token_stream;
                                Ok(Self {
                                    #primary_key_field_ident,
                                    #select_snake_case_token_stream
                                })
                            }
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                quote::quote! {
                    impl std::convert::From<#operation_payload_upper_camel_case_token_stream> for #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        fn from(value: #operation_payload_upper_camel_case_token_stream) -> Self {
                            let #primary_key_field_ident = #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value.#primary_key_field_ident);
                            let #select_snake_case_token_stream = value.#select_snake_case_token_stream;
                            Self {
                                #primary_key_field_ident,
                                #select_snake_case_token_stream,
                            }
                        }
                    }
                }
            };
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            quote::quote! {
                #payload_token_stream
                #payload_with_serialize_deserialize_token_stream
                #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream

                #derive_debug_token_stream
                pub struct #operation_parameters_upper_camel_case_token_stream {
                    pub #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream,
                }
            }
        };
        // println!("{parameters_token_stream}");
        let try_operation_error_with_middleware_error_variants_token_stream = {
            crate::type_variants_from_request_response_generator::type_variants_from_request_response_generator(
                &desirable_status_code,
                &struct_options_ident_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
                &generate_expected_type_declaration_token_stream,
                &unexpected_status_code_declaration_token_stream,
                &failed_to_get_response_text_declaration_token_stream,
                &deserialize_response_declaration_token_stream,
                &reqwest_declaration_token_stream,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (
            http_request_token_stream,
            http_request_test_expect_success_token_stream,
            http_request_test_expect_fail_token_stream,
        ) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let expected_type_declaration_token_stream =
                    generate_expected_type_declaration_token_stream(&operation);
                quote::quote! {
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #expected_type_declaration_token_stream,
                        #unexpected_status_code_declaration_token_stream,
                        #failed_to_get_response_text_declaration_token_stream,
                        #deserialize_response_declaration_token_stream,
                        #reqwest_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_try_operation_token_stream(
                &server_location_name_token_stream,
                &str_ref_token_stream,
                &struct_options_ident_token_stream,
                &quote::quote! {
                    let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(&#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)) {
                        Ok(value) => value,
                        Err(#error_value_snake_case_token_stream) => {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                        }
                    };
                },
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &quote::quote! {
                    Ok(value)
                },
                &table_name_stringified,
                &operation,
                //
                &proc_macro_name_upper_camel_case_ident_stringified,
                &type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &struct_options_ident_token_stream,
                &deserialize_response_initialization_token_stream,
                &unexpected_status_code_initialization_token_stream,
                &reqwest_initialization_token_stream,
                &failed_to_get_response_text_initialization_token_stream,
                &expected_type_initialization_token_stream,
            );
            let http_request_test_expect_success_token_stream = {
                let test_content_token_stream = quote::quote! {
                    match #try_operation_snake_case_token_stream(
                        #reference_api_location_test_token_stream,
                        #operation_parameters_upper_camel_case_token_stream {
                            #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
                                #primary_key_field_ident: #primary_key_token_stream.clone(),//todo
                                #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream::#select_full_variant_token_stream
                            }
                        },
                    )
                    .await
                    {
                        Ok(value) => println!("{value:#?}"),
                        Err(#error_value_snake_case_token_stream) => panic!("{}", #error_value_snake_case_token_stream)
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            let http_request_test_expect_fail_token_stream = {
                let test_content_token_stream = quote::quote! {
                    match #try_operation_snake_case_token_stream(
                        #reference_api_location_test_token_stream,
                        #operation_parameters_upper_camel_case_token_stream {
                            #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
                                #primary_key_field_ident: #primary_key_token_stream.clone(),//todo
                                #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream::#select_full_variant_token_stream
                            }
                        },
                    )
                    .await
                    {
                        Ok(value) => panic!("{value:#?}"),
                        Err(#error_value_snake_case_token_stream) => println!("{}", #error_value_snake_case_token_stream)
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_expect_success_token_stream,
                http_request_test_expect_fail_token_stream,
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let query_string_token_stream = {
                    let query_token_stream = {
                        let query_stringified = format!("\"{select_name_stringified} {{}} {from_name_stringified} {table_name_stringified} {where_name_stringified} {primary_key_field_ident} = $1\"");
                        query_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote! {
                        format!(
                            #query_token_stream,
                            crate::server::postgres::generate_query::GenerateQuery::generate_query(&#select_snake_case_token_stream),
                        )
                    }
                };
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = quote::quote! {
                        let query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident, query);
                    };
                    quote::quote! {
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        #binded_query_modifications_token_stream
                        query
                    }
                };
                let from_log_and_return_error_token_stream =
                    crate::from_log_and_return_error::from_log_and_return_error(
                        &try_operation_upper_camel_case_token_stream,
                        &error_log_call_token_stream,
                        &try_operation_response_variants_token_stream,
                    );
                let acquire_pool_and_connection_token_stream =
                    crate::acquire_pool_and_connection::acquire_pool_and_connection(
                        &from_log_and_return_error_token_stream,
                        &pg_connection_token_stream,
                    );
                quote::quote! {
                    let #select_snake_case_token_stream = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#select_snake_case_token_stream;
                    let #query_string_name_token_stream = {
                        #query_string_token_stream
                    };
                    println!("{}", #query_string_name_token_stream);
                    let #binded_query_name_token_stream = {
                        #binded_query_token_stream
                    };
                    #acquire_pool_and_connection_token_stream
                    match #binded_query_name_token_stream.fetch_one(#pg_connection_token_stream.as_mut()).await {
                        Ok(row) => match #select_snake_case_token_stream.#options_try_from_sqlx_row_name_token_stream(&row) {
                            Ok(value) => #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(value),
                            Err(#error_value_snake_case_token_stream) => {
                                #from_log_and_return_error_token_stream
                            },
                        },
                        Err(#error_value_snake_case_token_stream) => {
                            #from_log_and_return_error_token_stream
                        },
                    }
                }
            };
            // println!("{try_operation_token_stream}");
            let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                &table_name_stringified,
                &unique_status_codes,
                &application_json_quotes_token_stream,
                &table_name_quotes_token_stream,
                &operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                &operation,
            );
            let field_code_occurence_new_cd714ff2_3a40_4e0d_8930_e43d2f69ffc0_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> impl #axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(#error_value_snake_case_token_stream) => {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: #error_value_snake_case_token_stream,
                                        #field_code_occurence_new_cd714ff2_3a40_4e0d_8930_e43d2f69ffc0_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            },
                            Err(#error_value_snake_case_token_stream) => {
                                return #error_value_snake_case_token_stream;
                            }
                        },
                    };
                    println!("{:#?}", #parameters_snake_case_token_stream);
                    {
                        #try_operation_token_stream
                    }
                }
            }
        };
        // println!("{route_handler_token_stream}");
        let common_middlewares_error_syn_variants_from_impls =
            generate_common_middlewares_error_syn_variants_from_impls(
                common_middlewares_error_syn_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                additional_http_status_codes_error_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_expect_success_token_stream,
            http_request_test_expect_fail_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &read_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (update_many_token_stream, update_many_http_request_test_token_stream) = {
        let operation = Operation::UpdateMany;
        let operation_name_upper_camel_case_stringified = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&operation);
        let operation_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation);
        let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(&operation);
        let operation_payload_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation);
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(&operation);
        let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(&operation);
        let try_operation_response_variants_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(&operation);
        let try_operation_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfUpperCamelCaseTokenStream::try_self_upper_camel_case_token_stream(&operation);
        let operation_payload_element_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadElementUpperCamelCaseTokenStream::self_payload_element_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(&operation);
        let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadElementTryFromSelfPayloadElementWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::self_payload_element_try_from_self_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
        let additional_http_status_codes_error_variants = vec![]; //todo find out why rust analyzer crashes
                                                                  // crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_method_proc_macro_attribute(
                                                                  //     &ast,
                                                                  //     &operation_name_snake_case_stringified,
                                                                  //     additional_http_status_codes_error_variants_snake_case_stringified,
                                                                  //     &proc_macro_name_snake_case,
                                                                  //     &proc_macro_name_upper_camel_case_ident_stringified
                                                                  // );
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence,
                    &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
                    proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
                        &operation
                    )
                )
            ]
        );
        let type_variants_from_request_response_syn_variants = {
            let full_additional_http_status_codes_error_variants =
                generate_full_additional_http_status_codes_error_variants(
                    common_middlewares_error_syn_variants.iter().collect(),
                    additional_http_status_codes_error_variants.iter().collect(),
                );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response =
                    std::vec::Vec::with_capacity(common_error_syn_variants.len() + 10);
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                type_variants_from_request_response.push(&not_unique_primary_keys_syn_variant);
                type_variants_from_request_response.push(&bind_query_syn_variant);
                type_variants_from_request_response.push(&checked_add_syn_variant);
                type_variants_from_request_response.push(&no_payload_fields_syn_variant);
                type_variants_from_request_response.push(&commit_failed_syn_variant);
                type_variants_from_request_response.push(&non_existing_primary_keys_syn_variant);
                type_variants_from_request_response
                    .push(&primary_key_from_row_and_failed_rollback_syn_variant);
                type_variants_from_request_response
                    .push(&non_existing_primary_keys_and_failed_rollback_syn_variant);
                type_variants_from_request_response.push(&query_and_rollback_failed_syn_variant);
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants,
            )
        };
        let desirable_status_code = proc_macro_helpers::status_code::StatusCode::Tvfrr200Ok;
        let unique_status_codes = generate_unique_status_codes(
            &desirable_status_code,
            &type_variants_from_request_response_syn_variants,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let payload_token_stream = {
                let operation_payload_element_token_stream = {
                    //todo make sure name and color both are not None(make it option<value>, not just a value)
                    let fields_with_excluded_primary_key_token_stream =
                        fields_named_wrappers_excluding_primary_key
                            .iter()
                            .map(|element| {
                                generate_pub_field_ident_field_type_token_stream(
                                    element,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                )
                            });
                    quote::quote! {
                        #derive_debug_to_schema_token_stream
                        pub struct #operation_payload_element_upper_camel_case_token_stream {
                            pub #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                            #(#fields_with_excluded_primary_key_token_stream),*
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_element_token_stream

                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream(pub std::vec::Vec<#operation_payload_element_upper_camel_case_token_stream>);
                }
            };
            // println!("{payload_token_stream}");
            let operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadElementWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_element_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
            let payload_with_serialize_deserialize_token_stream = {
                let operation_payload_element_with_serialize_deserialize_token_stream = {
                    let fields_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_field_ident_field_type_with_serialize_deserialize_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    quote::quote! {
                        #derive_debug_serialize_deserialize_to_schema_token_stream
                        pub struct #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream {
                            #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                            #(#fields_with_excluded_primary_key_token_stream),*
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_element_with_serialize_deserialize_token_stream

                    #derive_debug_serialize_deserialize_to_schema_token_stream
                    pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream(std::vec::Vec<#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream>);
                }
            };
            // println!("{payload_with_serialize_deserialize_token_stream}");
            let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = {
                    let value = format!(
                        "{operation_name_upper_camel_case_stringified}PayloadTryFrom{operation_name_upper_camel_case_stringified}PayloadWithSerializeDeserialize{error_named_upper_camel_case_stringified}"
                    );
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = {
                    quote::quote! {
                        #derive_debug_thiserror_error_occurence_token_stream
                        pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                            #not_uuid_token_upper_camel_case_stream {
                                #eo_display_token_stream
                                #not_uuid_token_snake_case_stream: sqlx::types::uuid::Error,
                                #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                            },
                        }
                        //todo generate it
                        impl std::convert::From<
                            #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream
                        > for #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                            fn from(value: #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream) -> Self {
                                match value {
                                    #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream::#not_uuid_token_upper_camel_case_stream {
                                        #not_uuid_token_snake_case_stream,
                                        #code_occurence_snake_case_token_stream,
                                    } => Self::#not_uuid_token_upper_camel_case_stream {
                                        #not_uuid_token_snake_case_stream,
                                        #code_occurence_snake_case_token_stream,
                                    }
                                }
                            }
                        }
                    }
                };
                let impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream = {
                    let operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream = {
                        let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                            &fields_named_wrappers_excluding_primary_key,
                            &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream
                        );
                        quote::quote! {
                            #derive_debug_thiserror_error_occurence_token_stream
                            pub enum #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                                #not_uuid_token_upper_camel_case_stream {
                                    #eo_display_token_stream
                                    #not_uuid_token_snake_case_stream: sqlx::types::uuid::Error,
                                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                                },
                                #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                            }
                        }
                    };
                    // println!("{operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream}");
                    let field_code_occurence_new_77f303a5_de96_4f73_a274_f2195cb619b1_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let fields_assignments_token_stream = fields_named_wrappers_excluding_primary_key.iter()
                        .map(|element| generate_let_field_ident_value_field_ident_try_from_token_stream(
                            element,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                            &field_code_occurence_new_77f303a5_de96_4f73_a274_f2195cb619b1_token_stream,
                        ));
                    let self_init_fields_token_stream = generate_self_fields_token_stream(
                        &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>()
                            as &[&syn::Field],
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let field_code_occurence_new_814b41f8_3219_4b62_bc0b_02a26d23b262_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream
                        impl std::convert::TryFrom<#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_element_upper_camel_case_token_stream {
                            type Error = #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                let #primary_key_field_ident = match #sqlx_types_uuid_token_stream::parse_str(value.#primary_key_field_ident.to_inner()) {
                                    Ok(value) => #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::from(value),
                                    Err(#error_value_snake_case_token_stream) => {
                                        return Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                            #not_uuid_token_snake_case_stream: #error_value_snake_case_token_stream,
                                            #field_code_occurence_new_814b41f8_3219_4b62_bc0b_02a26d23b262_token_stream,
                                        });
                                    },
                                };
                                #(#fields_assignments_token_stream)*
                                Ok(Self{
                                    #(#self_init_fields_token_stream),*
                                })
                            }
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream

                    impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                        type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                        fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                            match value.0.into_iter()//todo rewrite as try_from
                                .map(|element|#operation_payload_element_upper_camel_case_token_stream::try_from(element))
                                .collect::<Result<
                                    std::vec::Vec<#operation_payload_element_upper_camel_case_token_stream>,
                                    #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream
                                >>()
                            {
                                Ok(value) => Ok(Self(value)),
                                Err(#error_value_snake_case_token_stream) => Err(Self::Error::from(#error_value_snake_case_token_stream)),
                            }
                        }
                    }
                }
            };
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream = {
                    let fields_assignments_token_stream =
                        fields_named_wrappers_excluding_primary_key
                            .iter()
                            .map(|element| {
                                generate_let_field_ident_value_field_ident_from_token_stream(
                                    element,
                                    &proc_macro_name_upper_camel_case_ident_stringified,
                                )
                            });
                    let self_init_fields_token_stream = generate_self_fields_token_stream(
                        &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>()
                            as &[&syn::Field],
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        impl std::convert::From<#operation_payload_element_upper_camel_case_token_stream> for #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream {
                            fn from(value: #operation_payload_element_upper_camel_case_token_stream) -> Self {
                                let #primary_key_field_ident = #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value.#primary_key_field_ident);
                                #(#fields_assignments_token_stream)*
                                Self {
                                    #(#self_init_fields_token_stream),*
                                }
                            }
                        }
                    }
                };
                // println!("{impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream}");
                quote::quote! {
                    #impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream

                    impl std::convert::From<#operation_payload_upper_camel_case_token_stream> for #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        fn from(value: #operation_payload_upper_camel_case_token_stream) -> Self {
                            Self(
                                value.0.into_iter()
                                .map(|element|#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream::from(element))
                                .collect()
                            )
                        }
                    }
                }
            };
            quote::quote! {
                #payload_token_stream
                #payload_with_serialize_deserialize_token_stream
                #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream

                #derive_debug_token_stream
                pub struct #operation_parameters_upper_camel_case_token_stream {
                    pub #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream,
                }
            }
        };
        // println!("{parameters_token_stream}");
        let try_operation_error_with_middleware_error_variants_token_stream = {
            crate::type_variants_from_request_response_generator::type_variants_from_request_response_generator(
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>},//todo reuse
                &derive_debug_thiserror_error_occurence_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
                &generate_expected_type_declaration_token_stream,
                &unexpected_status_code_declaration_token_stream,
                &failed_to_get_response_text_declaration_token_stream,
                &deserialize_response_declaration_token_stream,
                &reqwest_declaration_token_stream,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (http_request_token_stream, http_request_test_token_stream) = {
            let try_operation_error_named_token_stream = {
                let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                let expected_type_declaration_token_stream =
                    generate_expected_type_declaration_token_stream(&operation);
                quote::quote! {
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_declaration_token_stream,
                        #expected_type_declaration_token_stream,
                        #unexpected_status_code_declaration_token_stream,
                        #failed_to_get_response_text_declaration_token_stream,
                        #deserialize_response_declaration_token_stream,
                        #reqwest_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_http_request_many_token_stream(
                &server_location_name_token_stream,
                &str_ref_token_stream,
                &std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &serde_json_to_string_token_stream,
                &serde_json_to_string_variant_initialization_token_stream,
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_upper_camel_case_token_stream,
                &operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream,
                &operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_initialization_token_stream,
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>},//todo reuse
                &deserialize_response_initialization_token_stream,
                &unexpected_status_code_initialization_token_stream,
                &reqwest_initialization_token_stream,
                &failed_to_get_response_text_initialization_token_stream,
                &expected_type_initialization_token_stream,
                &primary_key_syn_field_with_additional_info,
            );
            let http_request_test_token_stream = {
                let fields_initialization_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                    let field_ident = element.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    let field_type = &element.ty;
                    quote::quote!{
                        #field_ident: #field_type::default()
                    }
                }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                let test_content_token_stream = quote::quote! {
                    match #try_operation_snake_case_token_stream(
                        &api_location,
                        #operation_parameters_upper_camel_case_token_stream {
                            #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream (
                                #primary_keys_token_stream.clone().into_iter().map(|element| {
                                    #operation_payload_element_upper_camel_case_token_stream {
                                        #primary_key_field_ident: element,
                                        #(#fields_initialization_excluding_primary_key_token_stream),*//todo make sure name and color both are not None(make it option<value>, not just a value)
                                    }
                                }).collect()
                            )
                        }
                    )
                    .await
                    {
                        Ok(value) => println!("{value:#?}"),
                        Err(#error_value_snake_case_token_stream) => panic!("{}", #error_value_snake_case_token_stream)
                    }
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream,
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let expected_updated_primary_keys_token_stream = quote::quote! {
                    #parameters_snake_case_token_stream
                    .#payload_snake_case_token_stream
                    .0
                    .iter()
                    .map(|element| element.#primary_key_field_ident.clone()) //todo - maybe its not a good idea to remove .clone here coz in macro dont know what type
                    .collect::<#std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream>()
                };
                let query_string_token_stream = {
                    let column_names = fields_named.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, field)| {
                        let field_ident = field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
                        let possible_dot_space = match (
                            index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::global_variables::hardcode::CHECKED_ADD_NONE_OVERFLOW_MESSAGE))
                        ) == fields_named_len {
                            true => "",
                            false => dot_space,
                        };
                        acc.push_str(&format!("{field_ident}{possible_dot_space}"));
                        acc
                    });
                    let column_increments = fields_named.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, _)| {
                        let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::global_variables::hardcode::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                        let possible_dot_space = match (incremented_index) == fields_named_len {
                            true => "",
                            false => dot_space,
                        };
                        acc.push_str(&format!("${incremented_index}{possible_dot_space}"));
                        acc
                    });
                    let declarations = fields_named_wrappers_excluding_primary_key.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, element)| {
                        let field_ident = &element.field_ident;
                        let possible_dot_space = match (
                            index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::global_variables::hardcode::CHECKED_ADD_NONE_OVERFLOW_MESSAGE))
                        ) == fields_named_wrappers_excluding_primary_key_len {
                            true => "",
                            false => dot_space,
                        };
                        acc.push_str(&format!("{field_ident} = data.{field_ident}{possible_dot_space}"));
                        acc
                    });
                    let query_stringified = format!("\"{update_name_stringified} {table_name_stringified} {as_name_stringified} t {set_name_stringified} {declarations} {from_name_stringified} ({select_name_stringified} * {from_name_stringified} {unnest_name_stringified}({column_increments})) as data({column_names}) where t.{primary_key_field_ident} = data.{primary_key_field_ident} {returning_stringified} data.{primary_key_field_ident}\"");
                    query_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let binded_query_token_stream = {
                    //todo remove () if in fields named only one element
                    let column_vecs_token_stream = fields_named.iter().map(|field|{
                        let field_ident_underscore_vec_stringified = {
                            let field_ident = field.ident.as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                });
                            format!("{field_ident}{underscore_vec_name_stringified}")
                        };
                        field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    });
                    let column_vecs_with_capacity_token_stream = fields_named.iter().map(|_|quote::quote!{std::vec::Vec::with_capacity(#current_vec_len_name_token_stream)});
                    let columns_acc_push_elements_token_stream = fields_named.iter().enumerate().map(|(index, field)|{
                        let field_ident = field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
                        let index_token_stream = {
                            let index_stringified = format!("{index}");
                            index_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{#acc_name_token_stream.#index_token_stream.push(#element_name_token_stream.#field_ident);}
                    });
                    let column_query_bind_primary_key_vec_token_stream = {
                        let field_ident_underscore_vec_token_stream = {
                            let field_ident_underscore_vec_stringified = format!(
                                "{primary_key_field_ident}{underscore_vec_name_stringified}"
                            );
                            field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote! {
                            #query_name_token_stream = #query_name_token_stream.bind(
                                #field_ident_underscore_vec_token_stream
                                .into_iter()
                                .map(|element| element.into_inner())
                                .collect::<std::vec::Vec<#sqlx_types_uuid_token_stream>>()
                            );
                        }
                    };
                    let column_query_bind_vecs_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        let field_ident_underscore_vec_token_stream = {
                            let field_ident_underscore_vec_stringified = {
                                let field_ident = &element.field_ident;
                                format!("{field_ident}{underscore_vec_name_stringified}")
                            };
                            field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{#query_name_token_stream = #query_name_token_stream.bind(#field_ident_underscore_vec_token_stream);}
                    });
                    quote::quote! {
                        let mut #query_name_token_stream = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        let #current_vec_len_name_token_stream = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.0.len();
                        let (
                            #(#column_vecs_token_stream),*
                        ) = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.0.into_iter().fold((
                            #(#column_vecs_with_capacity_token_stream),*
                        ), |mut #acc_name_token_stream, #element_name_token_stream| {
                            #(#columns_acc_push_elements_token_stream)*
                            #acc_name_token_stream
                        });
                        #column_query_bind_primary_key_vec_token_stream
                        #(#column_query_bind_vecs_token_stream)*
                        #query_name_token_stream
                    }
                };
                let from_log_and_return_error_token_stream =
                    crate::from_log_and_return_error::from_log_and_return_error(
                        &try_operation_upper_camel_case_token_stream,
                        &error_log_call_token_stream,
                        &try_operation_response_variants_token_stream,
                    );
                let acquire_pool_and_connection_token_stream =
                    crate::acquire_pool_and_connection::acquire_pool_and_connection(
                        &from_log_and_return_error_token_stream,
                        &pg_connection_token_stream,
                    );
                let generate_postgres_transaction_token_stream =
                    crate::generate_postgres_transaction::generate_postgres_transaction(
                        &expected_updated_primary_keys_token_stream,
                        &query_string_name_token_stream,
                        &query_string_token_stream,
                        &binded_query_token_stream,
                        &acquire_pool_and_connection_token_stream,
                        &pg_connection_token_stream,
                        &binded_query_name_token_stream,
                        &use_futures_try_stream_ext_token_stream,
                        &primary_key_uuid_wrapper_try_from_sqlx_row_name_token_stream,
                        &from_log_and_return_error_token_stream,
                        &rollback_error_name_token_stream,
                        &non_existing_primary_keys_name_token_stream,
                        &rollback_snake_case_token_stream,
                        &try_operation_response_variants_token_stream,
                        &desirable_upper_camel_case_token_stream,
                        &try_operation_upper_camel_case_token_stream,
                        &error_log_call_token_stream,
                        &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                quote::quote! {
                    {
                        let #not_unique_primary_keys_name_token_stream = {
                            let mut vec = std::vec::Vec::with_capacity(#parameters_snake_case_token_stream.#payload_snake_case_token_stream.0.len());
                            let mut #not_unique_primary_keys_name_token_stream = std::vec::Vec::with_capacity(#parameters_snake_case_token_stream.#payload_snake_case_token_stream.0.len());
                            for element in &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.0 {
                                let handle = &element.#primary_key_field_ident;
                                match vec.contains(&handle) {
                                    true => {
                                        #not_unique_primary_keys_name_token_stream.push(element.#primary_key_field_ident.clone());
                                    },
                                    false => {
                                        vec.push(&element.#primary_key_field_ident);
                                    }
                                }
                            }
                            #not_unique_primary_keys_name_token_stream
                        };
                        if let false = #not_unique_primary_keys_name_token_stream.is_empty() {
                            let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#not_unique_primary_key_variant_initialization_token_stream;
                            #error_log_call_token_stream
                            return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                        }
                    }
                    #generate_postgres_transaction_token_stream
                }
            };
            // println!("{try_operation_token_stream}");
            let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                &table_name_stringified,
                &unique_status_codes,
                &application_json_quotes_token_stream,
                &table_name_quotes_token_stream,
                &operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                &operation,
            );
            let field_code_occurence_new_04274f8d_c9a4_41d1_9bdc_39432371c33f_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream<'a>(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> impl #axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(#error_value_snake_case_token_stream) => {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: #error_value_snake_case_token_stream,
                                        #field_code_occurence_new_04274f8d_c9a4_41d1_9bdc_39432371c33f_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            },
                            Err(#error_value_snake_case_token_stream) => {
                                return #error_value_snake_case_token_stream;
                            }
                        },
                    };
                    println!("{:#?}", #parameters_snake_case_token_stream);
                    {
                        #try_operation_token_stream
                    }
                }
            }
        };
        // println!("{route_handler_token_stream}");
        let common_middlewares_error_syn_variants_from_impls =
            generate_common_middlewares_error_syn_variants_from_impls(
                common_middlewares_error_syn_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                additional_http_status_codes_error_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &update_many_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    //todo WHY ITS RETURN SUCCESS EVEN IF ROW DOES NOT EXISTS?
    let (update_one_token_stream, update_one_http_request_test_token_stream) = {
        let operation = Operation::UpdateOne;
        let operation_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation);
        let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(&operation);
        let operation_payload_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation);
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(&operation);
        let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(&operation);
        let try_operation_response_variants_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(&operation);
        let try_operation_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfUpperCamelCaseTokenStream::try_self_upper_camel_case_token_stream(&operation);
        let additional_http_status_codes_error_variants = vec![]; //todo find out why rust analyzer crashes
                                                                  // crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_method_proc_macro_attribute(
                                                                  //     &ast,
                                                                  //     &operation_name_snake_case_stringified,
                                                                  //     &additional_http_status_codes_error_variants_snake_case_stringified,
                                                                  //     &proc_macro_name_snake_case,
                                                                  //     &proc_macro_name_upper_camel_case_ident_stringified
                                                                  // );
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence,
                    &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
                    proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
                        &operation
                    )
                )
            ]
        );
        let type_variants_from_request_response_syn_variants = {
            let full_additional_http_status_codes_error_variants =
                generate_full_additional_http_status_codes_error_variants(
                    common_middlewares_error_syn_variants.iter().collect(),
                    additional_http_status_codes_error_variants.iter().collect(),
                );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response =
                    std::vec::Vec::with_capacity(common_error_syn_variants.len() + 4);
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                type_variants_from_request_response.push(&bind_query_syn_variant);
                type_variants_from_request_response.push(&no_payload_fields_syn_variant);
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response.push(&operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants,
            )
        };
        let desirable_status_code = proc_macro_helpers::status_code::StatusCode::Tvfrr200Ok;
        let unique_status_codes = generate_unique_status_codes(
            &desirable_status_code,
            &type_variants_from_request_response_syn_variants,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let payload_token_stream = {
                let fields_with_excluded_primary_key_token_stream =
                    fields_named_wrappers_excluding_primary_key
                        .iter()
                        .map(|element| {
                            let field_ident = &element.field_ident;
                            let field_type = &element.field.ty;
                            quote::quote! {
                                pub #field_ident: std::option::Option<#field_type>
                            }
                        });
                quote::quote! {
                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream {
                        pub #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                        #(#fields_with_excluded_primary_key_token_stream),*
                    }
                }
            };
            // println!("{payload_token_stream}");
            let payload_with_serialize_deserialize_token_stream = {
                let fields_with_excluded_primary_key_token_stream =
                    fields_named_wrappers_excluding_primary_key
                        .iter()
                        .map(|element| {
                            let field_ident = &element.field_ident;
                            let field_type = &element.field.ty;
                            quote::quote! {
                                #field_ident: std::option::Option<#field_type>
                            }
                        });
                quote::quote! {
                    #derive_debug_serialize_deserialize_to_schema_token_stream
                    pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                        #(#fields_with_excluded_primary_key_token_stream),*
                    }
                }
            };
            // println!("{payload_with_serialize_deserialize_token_stream}");
            let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = {
                    let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                        &fields_named_wrappers_excluding_primary_key,
                        &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream
                    );
                    quote::quote! {
                        #derive_debug_thiserror_error_occurence_token_stream
                        pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                            #not_uuid_token_upper_camel_case_stream {
                                #eo_error_occurence_attribute_token_stream
                                #not_uuid_token_snake_case_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                            },
                            #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                        }
                    }
                };
                // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                    let field_code_occurence_new_1755cb35_9932_42ce_8a4a_edd53bb789a1_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter()
                        .map(|element| generate_let_field_ident_value_field_ident_try_from_token_stream(
                            element,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                            &field_code_occurence_new_1755cb35_9932_42ce_8a4a_edd53bb789a1_token_stream,
                        ));
                    let fields_idents_excluding_primary_key_token_stream =
                        fields_named_wrappers_excluding_primary_key
                            .iter()
                            .map(|element| &element.field_ident);
                    let field_code_occurence_new_9d290620_cad2_47ab_900e_da3f3d08307f_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                let #primary_key_field_ident = match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value.#primary_key_field_ident) {
                                    Ok(value) => value,
                                    Err(#error_value_snake_case_token_stream) => {
                                        return Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                            #not_uuid_token_snake_case_stream: #error_value_snake_case_token_stream,
                                            #field_code_occurence_new_9d290620_cad2_47ab_900e_da3f3d08307f_token_stream,
                                        });
                                    }
                                };
                                //todo with_serialize_deserialize
                                #(#fields_assignment_excluding_primary_key_token_stream)*
                                Ok(Self {
                                    #primary_key_field_ident,
                                    #(#fields_idents_excluding_primary_key_token_stream),*
                                })
                            }
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let fields_assignment_excluding_primary_key_token_stream =
                    fields_named_wrappers_excluding_primary_key
                        .iter()
                        .map(|element| {
                            generate_let_field_ident_value_field_ident_from_token_stream(
                                element,
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            )
                        });
                let fields_idents_excluding_primary_key_token_stream =
                    fields_named_wrappers_excluding_primary_key
                        .iter()
                        .map(|element| &element.field_ident);
                quote::quote! {
                    impl std::convert::From<#operation_payload_upper_camel_case_token_stream> for #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        fn from(value: #operation_payload_upper_camel_case_token_stream) -> Self {
                            let #primary_key_field_ident = #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value.#primary_key_field_ident);
                            #(#fields_assignment_excluding_primary_key_token_stream)*
                            Self{
                                #primary_key_field_ident,
                                #(#fields_idents_excluding_primary_key_token_stream),*
                            }
                        }
                    }
                }
            };
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            quote::quote! {
                #payload_token_stream
                #payload_with_serialize_deserialize_token_stream
                #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream

                #derive_debug_token_stream
                pub struct #operation_parameters_upper_camel_case_token_stream {
                    pub #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream,
                }
            }
        };
        // println!("{parameters_token_stream}");
        let try_operation_error_with_middleware_error_variants_token_stream = {
            crate::type_variants_from_request_response_generator::type_variants_from_request_response_generator(
                &desirable_status_code,
                &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
                &generate_expected_type_declaration_token_stream,
                &unexpected_status_code_declaration_token_stream,
                &failed_to_get_response_text_declaration_token_stream,
                &deserialize_response_declaration_token_stream,
                &reqwest_declaration_token_stream,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (http_request_token_stream, http_request_test_token_stream) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let expected_type_declaration_token_stream =
                    generate_expected_type_declaration_token_stream(&operation);
                quote::quote! {
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_declaration_token_stream,
                        #expected_type_declaration_token_stream,
                        #unexpected_status_code_declaration_token_stream,
                        #failed_to_get_response_text_declaration_token_stream,
                        #deserialize_response_declaration_token_stream,
                        #reqwest_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_try_operation_token_stream(
                &server_location_name_token_stream,
                &str_ref_token_stream,
                &crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &quote::quote! {
                    let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(&#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)) {
                        Ok(value) => value,
                        Err(#error_value_snake_case_token_stream) => {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                        }
                    };
                },
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &quote::quote! {
                    match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value) {
                        Ok(value) => Ok(value),
                        Err(#error_value_snake_case_token_stream) => Err(#try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_initialization_token_stream )
                    }
                },
                &table_name_stringified,
                &operation,
                //
                &proc_macro_name_upper_camel_case_ident_stringified,
                &type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                &deserialize_response_initialization_token_stream,
                &unexpected_status_code_initialization_token_stream,
                &reqwest_initialization_token_stream,
                &failed_to_get_response_text_initialization_token_stream,
                &expected_type_initialization_token_stream,
            );
            let http_request_test_token_stream = {
                let fields_initialization_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                    let field_ident = element.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    let field_type = &element.ty;
                    quote::quote!{
                        #field_ident: Some(#field_type::default())
                    }
                }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                let test_content_token_stream = quote::quote! {
                    let #primary_key_token_stream = match #try_operation_snake_case_token_stream(
                        #reference_api_location_test_token_stream,
                        #operation_parameters_upper_camel_case_token_stream {
                            #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
                                #primary_key_field_ident: #primary_key_token_stream.clone(),
                                #(#fields_initialization_excluding_primary_key_token_stream),*
                            }
                        }
                    )
                    .await
                    {
                        Ok(value) => {
                            println!("{value:#?}");
                            value
                        },
                        Err(#error_value_snake_case_token_stream) => panic!("{}", #error_value_snake_case_token_stream)
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream,
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let check_for_none_token_stream_excluding_primary_key =
                    crate::check_for_none::check_for_none(
                        fields_named,
                        &primary_key_syn_field_with_additional_info.field,
                        &proc_macro_name_upper_camel_case_ident_stringified,
                        dot_space,
                        &try_operation_response_variants_token_stream,
                        true,
                    );
                let query_string_token_stream = {
                    let additional_parameters_modification_token_stream = {
                        fields_named_wrappers_excluding_primary_key.iter().enumerate().map(|(index, element)| {
                            let field_ident = &element.field_ident;
                            let handle_token_stream = {
                                let possible_dot_space = match (
                                    index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::global_variables::hardcode::CHECKED_ADD_NONE_OVERFLOW_MESSAGE))
                                ) == fields_named_wrappers_excluding_primary_key_len {
                                    true => "",
                                    false => dot_space,
                                };
                                let handle_stringified = format!("\"{field_ident} = ${{increment}}{possible_dot_space}\"");
                                handle_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            quote::quote!{
                                if let Some(value) = &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#field_ident {
                                    match #crate_server_postgres_bind_query_bind_query_try_increment_token_stream(value, &mut increment) {
                                        Ok(_) => {
                                            query.push_str(&format!(#handle_token_stream));//add dot_space for all elements except last
                                        },
                                        Err(#error_value_snake_case_token_stream) => {
                                            return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
                                        },
                                    }
                                }
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>()
                    };
                    let additional_parameters_primary_key_modification_token_stream = {
                        let query_part_token_stream = {
                            let query_part_stringified =
                                format!("\" where {primary_key_field_ident} = ${{increment}}\""); //todo where
                            query_part_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_part_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote! {
                            match #crate_server_postgres_bind_query_bind_query_try_increment_token_stream(&#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident, &mut increment) {
                                Ok(_) => {
                                    query.push_str(&format!(#query_part_token_stream));
                                },
                                Err(#error_value_snake_case_token_stream) => {
                                    return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
                                },
                            }
                        }
                    };
                    let handle_token_stream = {
                        let handle_stringified = format!("\"{update_name_stringified} {table_name_stringified} {set_name_stringified} \""); //todo where
                        handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote! {
                        #increment_initialization_token_stream
                        let mut query = #std_string_string_token_stream::from(#handle_token_stream);
                        #(#additional_parameters_modification_token_stream)*
                        #additional_parameters_primary_key_modification_token_stream
                        query.push_str(&format!(#returning_primary_key_quotes_token_stream));
                        query
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        let field_ident = &element.field_ident;
                        quote::quote!{
                            if let Some(value) = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#field_ident {
                                query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
                                    value,
                                    query,
                                );
                            }
                        }
                    });
                    let binded_query_primary_key_modification_token_stream = quote::quote! {
                        query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident,
                            query,
                        );
                    };
                    quote::quote! {
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        #(#binded_query_modifications_token_stream)*
                        #binded_query_primary_key_modification_token_stream
                        query
                    }
                };
                let from_log_and_return_error_token_stream =
                    crate::from_log_and_return_error::from_log_and_return_error(
                        &try_operation_upper_camel_case_token_stream,
                        &error_log_call_token_stream,
                        &try_operation_response_variants_token_stream,
                    );
                let acquire_pool_and_connection_token_stream =
                    crate::acquire_pool_and_connection::acquire_pool_and_connection(
                        &from_log_and_return_error_token_stream,
                        &pg_connection_token_stream,
                    );
                quote::quote! {
                    #check_for_none_token_stream_excluding_primary_key
                    let #query_string_name_token_stream = {
                        #query_string_token_stream
                    };
                    println!("{}", #query_string_name_token_stream);
                    let #binded_query_name_token_stream = {
                        #binded_query_token_stream
                    };
                    #acquire_pool_and_connection_token_stream
                    match #binded_query_name_token_stream
                        .fetch_one(#pg_connection_token_stream.as_mut())
                        .await
                    {
                        Ok(value) => match {
                            use #sqlx_row_token_stream;
                            value.try_get::<#sqlx_types_uuid_token_stream, #str_ref_token_stream>(#primary_key_field_ident_quotes_token_stream)
                        } {
                            Ok(value) => #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value)),
                            Err(#error_value_snake_case_token_stream) => {
                                let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                            }
                        },
                        Err(#error_value_snake_case_token_stream) => {
                            #from_log_and_return_error_token_stream;
                        }
                    }
                }
            };
            // println!("{try_operation_token_stream}");
            let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                &table_name_stringified,
                &unique_status_codes,
                &application_json_quotes_token_stream,
                &table_name_quotes_token_stream,
                &operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                &operation,
            );
            let field_code_occurence_new_112d570b_90ed_44d6_a8bf_d855372006cb_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream<'a>(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> impl #axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(#error_value_snake_case_token_stream) => {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: #error_value_snake_case_token_stream,
                                        #field_code_occurence_new_112d570b_90ed_44d6_a8bf_d855372006cb_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            },
                            Err(#error_value_snake_case_token_stream) => {
                                return #error_value_snake_case_token_stream;
                            }
                        },
                    };
                    println!("{:#?}", #parameters_snake_case_token_stream);
                    {
                        #try_operation_token_stream
                    }
                }
            }
        };
        // println!("{route_handler_token_stream}");
        let common_middlewares_error_syn_variants_from_impls =
            generate_common_middlewares_error_syn_variants_from_impls(
                common_middlewares_error_syn_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                additional_http_status_codes_error_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &update_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (delete_many_token_stream, delete_many_http_request_test_token_stream) = {
        let operation = Operation::DeleteMany;
        let operation_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation);
        let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(&operation);
        let operation_payload_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation);
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(&operation);
        let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(&operation);
        let try_operation_response_variants_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(&operation);
        let try_operation_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfUpperCamelCaseTokenStream::try_self_upper_camel_case_token_stream(&operation);
        let additional_http_status_codes_error_variants = vec![]; //todo find out why rust analyzer crashes
                                                                  // crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_method_proc_macro_attribute(
                                                                  //     &ast,
                                                                  //     &operation_name_snake_case_stringified,
                                                                  //     additional_http_status_codes_error_variants_snake_case_stringified,
                                                                  //     &proc_macro_name_snake_case,
                                                                  //     &proc_macro_name_upper_camel_case_ident_stringified
                                                                  // );
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence,
                    &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
                    proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
                        &operation
                    )
                )
            ]
        );
        let type_variants_from_request_response_syn_variants = {
            let full_additional_http_status_codes_error_variants =
                generate_full_additional_http_status_codes_error_variants(
                    common_middlewares_error_syn_variants.iter().collect(),
                    additional_http_status_codes_error_variants.iter().collect(),
                );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
                    common_error_syn_variants.len() + not_unique_vec_syn_variants.len() + 12,
                );
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                for element in &not_unique_vec_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                type_variants_from_request_response.push(&not_unique_primary_keys_syn_variant);
                type_variants_from_request_response.push(&bind_query_syn_variant);
                type_variants_from_request_response.push(&no_payload_fields_syn_variant);
                type_variants_from_request_response.push(&no_payload_parameters_syn_variant);
                type_variants_from_request_response.push(&non_existing_primary_keys_syn_variant);
                type_variants_from_request_response
                    .push(&non_existing_primary_keys_and_failed_rollback_syn_variant);
                type_variants_from_request_response
                    .push(&primary_key_from_row_and_failed_rollback_syn_variant);
                type_variants_from_request_response.push(&commit_failed_syn_variant);
                type_variants_from_request_response.push(&query_and_rollback_failed_syn_variant);
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response.push(&operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants,
            )
        };
        let desirable_status_code = proc_macro_helpers::status_code::StatusCode::Tvfrr200Ok;
        let unique_status_codes = generate_unique_status_codes(
            &desirable_status_code,
            &type_variants_from_request_response_syn_variants,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let payload_token_stream = {
                let fields_with_excluded_primary_key_token_stream =
                    fields_named_wrappers_excluding_primary_key
                        .iter()
                        .map(|element| {
                            let field_ident = &element.field_ident;
                            quote::quote! {
                                pub #field_ident: std::option::Option<std::vec::Vec<#crate_server_postgres_regex_filter_regex_filter_token_stream>>//todo
                            }
                        });
                quote::quote! {
                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream {
                        pub #primary_key_field_ident: std::option::Option<#std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream>,
                        #(#fields_with_excluded_primary_key_token_stream),*
                    }
                }
            };
            // println!("{payload_token_stream}");
            let payload_with_serialize_deserialize_token_stream = {
                let fields_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                    let field_ident = &element.field_ident;
                    quote::quote!{
                        #field_ident: std::option::Option<std::vec::Vec<#crate_server_postgres_regex_filter_regex_filter_token_stream>>
                    }
                });
                quote::quote! {
                    #derive_debug_serialize_deserialize_token_stream
                    pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        #primary_key_field_ident: std::option::Option<std::vec::Vec<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>>,
                        #(#fields_with_excluded_primary_key_token_stream),*
                    }
                }
            };
            // println!("{payload_with_serialize_deserialize_token_stream}");
            let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = {
                    let inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream = generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
                        &fields_named_wrappers_excluding_primary_key,
                        &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream
                    );
                    quote::quote! {
                        #derive_debug_thiserror_error_occurence_token_stream
                        pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                            #not_uuid_token_upper_camel_case_stream {
                                #eo_error_occurence_attribute_token_stream
                                #not_uuid_token_snake_case_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                            },
                            #(#inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variants_token_stream)*
                        }
                    }
                };
                // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                    let field_code_occurence_new_0e89db69_45c2_4be1_adf0_91dff2118591_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let fields_assignments_token_stream = fields_named_wrappers_excluding_primary_key.iter()
                        .map(|element| generate_let_field_ident_value_field_ident_try_from_token_stream(
                            element,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                            &field_code_occurence_new_0e89db69_45c2_4be1_adf0_91dff2118591_token_stream,
                        ));
                    let self_init_fields_token_stream = generate_self_fields_token_stream(
                        &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>()
                            as &[&syn::Field],
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let field_code_occurence_new_9a8f5b56_b6d4_4539_8f07_a13997efd268_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote! {
                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                let #primary_key_field_ident = match value.#primary_key_field_ident {
                                    Some(value) => match value.into_iter().map(|element|#crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(element)).collect::<Result<
                                        #std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                                        #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream
                                    >>() {
                                        Ok(value) => Some(value),
                                        Err(#error_value_snake_case_token_stream) => {
                                            return Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                                #not_uuid_token_snake_case_stream: #error_value_snake_case_token_stream,
                                                #field_code_occurence_new_9a8f5b56_b6d4_4539_8f07_a13997efd268_token_stream,
                                            });
                                        },
                                    },
                                    None => None,
                                };
                                #(#fields_assignments_token_stream)*
                                Ok(Self {
                                    #(#self_init_fields_token_stream),*
                                })
                            }
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let fields_assignments_token_stream = fields_named_wrappers_excluding_primary_key
                    .iter()
                    .map(|element| {
                        generate_let_field_ident_value_field_ident_from_token_stream(
                            element,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        )
                    });
                let self_init_fields_token_stream = generate_self_fields_token_stream(
                    &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>()
                        as &[&syn::Field],
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote! {
                    impl std::convert::From<#operation_payload_upper_camel_case_token_stream> for #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        fn from(value: #operation_payload_upper_camel_case_token_stream) -> Self {
                            let #primary_key_field_ident = match value.#primary_key_field_ident {
                                Some(value) => Some(value.into_iter()
                                    .map(|element|#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(element))
                                    .collect::<std::vec::Vec<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>>()),
                                None => None,
                            };
                            #(#fields_assignments_token_stream)*
                            Self{
                                #(#self_init_fields_token_stream),*
                            }
                        }
                    }
                }
            };
            quote::quote! {
                #payload_token_stream
                #payload_with_serialize_deserialize_token_stream
                #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream

                #derive_debug_token_stream
                pub struct #operation_parameters_upper_camel_case_token_stream {
                    pub #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream,
                }
            }
        };
        // println!("{parameters_token_stream}");
        let try_operation_error_with_middleware_error_variants_token_stream = {
            crate::type_variants_from_request_response_generator::type_variants_from_request_response_generator(
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>},//todo reuse
                &derive_debug_thiserror_error_occurence_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
                &generate_expected_type_declaration_token_stream,
                &unexpected_status_code_declaration_token_stream,
                &failed_to_get_response_text_declaration_token_stream,
                &deserialize_response_declaration_token_stream,
                &reqwest_declaration_token_stream,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (http_request_token_stream, http_request_test_token_stream) = {
            let try_operation_error_named_token_stream = {
                let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
                let expected_type_declaration_token_stream =
                    generate_expected_type_declaration_token_stream(&operation);
                quote::quote! {
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_declaration_token_stream,
                        #expected_type_declaration_token_stream,
                        #unexpected_status_code_declaration_token_stream,
                        #failed_to_get_response_text_declaration_token_stream,
                        #deserialize_response_declaration_token_stream,
                        #reqwest_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_http_request_many_token_stream(
                &server_location_name_token_stream,
                &str_ref_token_stream,
                &std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &serde_json_to_string_token_stream,
                &serde_json_to_string_variant_initialization_token_stream,
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_upper_camel_case_token_stream,
                &operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream,
                &operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_initialization_token_stream,
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>},//todo reuse
                &deserialize_response_initialization_token_stream,
                &unexpected_status_code_initialization_token_stream,
                &reqwest_initialization_token_stream,
                &failed_to_get_response_text_initialization_token_stream,
                &expected_type_initialization_token_stream,
                &primary_key_syn_field_with_additional_info,
            );
            let http_request_test_token_stream = {
                let fields_initialization_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
                    let field_ident = element.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    quote::quote!{
                        #field_ident: None
                        //or and support
                        // Some(vec![crate::server::postgres::regex_filter::RegexFilter {
                        //     regex: #std_string_string_token_stream::from("test"),
                        //     conjuctive_operator: crate::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                        // }])
                    }
                }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                let test_content_token_stream = quote::quote! {
                    match #try_operation_snake_case_token_stream(
                        #reference_api_location_test_token_stream,
                        //todo - builder pattern?
                        #operation_parameters_upper_camel_case_token_stream {
                            #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
                                #primary_key_field_ident: Some(
                                    #primary_keys_token_stream.clone()
                                    // vec![
                                    //     crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(
                                    //         crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(id)
                                    //     ).unwrap()
                                    // ]
                                ),
                                #(#fields_initialization_excluding_primary_key_token_stream),*
                            }
                        },
                    )
                    .await
                    {
                        Ok(value) => println!("{value:#?}"),
                        // let vec_cat_id: Vec<
                        //     super::DogId,
                        // > = value
                        //     .into_iter()
                        //     .filter_map(|value| match value.id {
                        //         Some(id) => Some(
                        //             super::DogId {
                        //                 id,
                        //             },
                        //         ),
                        //         None => None,
                        //     })
                        //     .collect();
                        // println!("{vec_cat_id:#?}");
                        Err(#error_value_snake_case_token_stream) => panic!("{}", #error_value_snake_case_token_stream)
                    }
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream,
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let check_for_none_token_stream = crate::check_for_none::check_for_none(
                    fields_named,
                    &primary_key_syn_field_with_additional_info.field,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                    dot_space,
                    &try_operation_response_variants_token_stream,
                    false,
                );
                let parameters_match_token_stream = fields_named.iter().map(|field| {
                    let field_ident = field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    quote::quote!{
                        &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#field_ident
                    }
                });
                let parameters_match_primary_key_some_other_none_token_stream = fields_named.iter().map(|field| {
                    let field_ident = field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    match field_ident == primary_key_field_ident {
                        true => quote::quote!{Some(#primary_key_field_ident)},
                        false => quote::quote!{None}
                    }
                });
                let from_log_and_return_error_token_stream =
                    crate::from_log_and_return_error::from_log_and_return_error(
                        &try_operation_upper_camel_case_token_stream,
                        &error_log_call_token_stream,
                        &try_operation_response_variants_token_stream,
                    );
                let acquire_pool_and_connection_token_stream =
                    crate::acquire_pool_and_connection::acquire_pool_and_connection(
                        &from_log_and_return_error_token_stream,
                        &pg_connection_token_stream,
                    );
                let generate_postgres_transaction_token_stream = {
                    let filter_unique_parameters_token_stream = {
                        let filter_unique_parameters_primary_key_token_stream = quote::quote! {
                            let #not_unique_primary_keys_name_token_stream = {
                                let mut vec = std::vec::Vec::with_capacity(#primary_key_field_ident.len());
                                let mut #not_unique_primary_keys_name_token_stream = std::vec::Vec::with_capacity(#primary_key_field_ident.len());
                                for element in #primary_key_field_ident {
                                    let handle = element;
                                    match vec.contains(&handle) {
                                        true => {
                                            #not_unique_primary_keys_name_token_stream.push(element.clone());
                                        },
                                        false => {
                                            vec.push(element);
                                        }
                                    }
                                }
                                #not_unique_primary_keys_name_token_stream
                            };
                            if let false = #not_unique_primary_keys_name_token_stream.is_empty() {
                                let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#not_unique_primary_key_variant_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                            }
                        };
                        quote::quote! {
                            #filter_unique_parameters_primary_key_token_stream
                        }
                    };
                    let expected_updated_primary_keys_token_stream = quote::quote! {
                        #primary_key_field_ident
                        .iter()
                        .map(|element| element.clone()) //todo - maybe its not a good idea to remove .clone here coz in macro dont know what type
                        .collect::<#std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream>()
                    };
                    let query_string_primary_key_some_other_none_token_stream = {
                        let handle_stringified = format!("\"{delete_name_stringified} {from_name_stringified} {table_name_stringified} {where_name_stringified} {primary_key_field_ident} {in_name_stringified} ({select_name_stringified} {unnest_name_stringified}($1)){returning_primary_key_stringified}\"");
                        handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let binded_query_primary_key_some_other_none_token_stream = quote::quote! {
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        query = query.bind(
                            #primary_key_field_ident
                            .into_iter()
                            .map(|element| element.clone().into_inner())
                            .collect::<std::vec::Vec<#sqlx_types_uuid_token_stream>>()
                        );
                        query
                    };
                    let generate_postgres_transaction_token_stream =
                        crate::generate_postgres_transaction::generate_postgres_transaction(
                            &expected_updated_primary_keys_token_stream,
                            &query_string_name_token_stream,
                            &query_string_primary_key_some_other_none_token_stream,
                            &binded_query_primary_key_some_other_none_token_stream,
                            &acquire_pool_and_connection_token_stream,
                            &pg_connection_token_stream,
                            &binded_query_name_token_stream,
                            &use_futures_try_stream_ext_token_stream,
                            &primary_key_uuid_wrapper_try_from_sqlx_row_name_token_stream,
                            &from_log_and_return_error_token_stream,
                            &rollback_error_name_token_stream,
                            &non_existing_primary_keys_name_token_stream,
                            &rollback_snake_case_token_stream,
                            &try_operation_response_variants_token_stream,
                            &desirable_upper_camel_case_token_stream,
                            &try_operation_upper_camel_case_token_stream,
                            &error_log_call_token_stream,
                            &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                    quote::quote! {
                        #filter_unique_parameters_token_stream
                        #generate_postgres_transaction_token_stream
                    }
                };
                let generate_postgres_execute_query_token_stream = {
                    //todo rename execute into something else
                    let filter_unique_parameters_token_stream = {
                        let filter_unique_parameters_primary_key_token_stream = quote::quote! {
                            if let Some(#primary_key_field_ident) = &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident {
                                let #not_unique_primary_keys_name_token_stream = {
                                    let mut vec = std::vec::Vec::with_capacity(#primary_key_field_ident.len());
                                    let mut #not_unique_primary_keys_name_token_stream = std::vec::Vec::with_capacity(#primary_key_field_ident.len());
                                    for element in #primary_key_field_ident {
                                        let handle = element;
                                        match vec.contains(&handle) {
                                            true => {
                                                #not_unique_primary_keys_name_token_stream.push(element.clone());
                                            },
                                            false => {
                                                vec.push(element);
                                            }
                                        }
                                    }
                                    #not_unique_primary_keys_name_token_stream
                                };
                                if let false = #not_unique_primary_keys_name_token_stream.is_empty() {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#not_unique_primary_key_variant_initialization_token_stream;
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            }
                        };
                        let filter_unique_parameters_other_columns_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                            let field_ident = &element.field_ident;
                            let field_handle_token_stream = {
                                let field_handle_stringified = format!("{field_ident}_handle");
                                field_handle_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            let not_unique_field_vec_snake_case_token_stream = {
                                let not_unique_field_vec_snake_case_stringified = generate_not_unique_field_vec_snake_case_stringified(field_ident);
                                not_unique_field_vec_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_unique_field_vec_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            let not_unique_field_vec_vec_upper_camel_token_stream = {
                                let not_unique_field_vec_upper_camel_stringified = generate_not_unique_field_vec_upper_camel_stringified(field_ident);
                                not_unique_field_vec_upper_camel_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {not_unique_field_vec_upper_camel_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            let field_code_occurence_new_a4cd6c7d_3d82_4ee7_84f0_ca63ddb894e1_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                                file!(),
                                line!(),
                                column!(),
                                &proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            quote::quote!{
                                let #field_handle_token_stream = match #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#field_ident {
                                    Some(value) => {
                                        let is_unique = {
                                            let mut vec = std::vec::Vec::with_capacity(value.len());
                                            let mut is_unique = true;
                                            for element in &value {
                                                match vec.contains(&element) {
                                                    true => {
                                                        is_unique = false;
                                                        break;
                                                    }
                                                    false => {
                                                        vec.push(element);
                                                    }
                                                }
                                            }
                                            is_unique
                                        };
                                        match is_unique {
                                            true => Some(value),
                                            false => {
                                                let #not_unique_field_vec_snake_case_token_stream = {
                                                    let mut vec = std::vec::Vec::with_capacity(value.len());
                                                    let mut #not_unique_field_vec_snake_case_token_stream = std::vec::Vec::with_capacity(value.len());
                                                    for element in value {
                                                        match vec.contains(&element) {
                                                            true => {
                                                                #not_unique_field_vec_snake_case_token_stream.push(element);
                                                            }
                                                            false => {
                                                                vec.push(element);
                                                            }
                                                        }
                                                    }
                                                    #not_unique_field_vec_snake_case_token_stream
                                                };
                                                let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#not_unique_field_vec_vec_upper_camel_token_stream {
                                                    #not_unique_field_vec_snake_case_token_stream,
                                                    #field_code_occurence_new_a4cd6c7d_3d82_4ee7_84f0_ca63ddb894e1_token_stream,
                                                };
                                                #error_log_call_token_stream
                                                return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                            }
                                        }
                                    },
                                    None => None
                                };
                            }
                        });
                        quote::quote! {
                            #filter_unique_parameters_primary_key_token_stream
                            #(#filter_unique_parameters_other_columns_token_stream)*
                        }
                    };
                    let query_string_token_stream = {
                        let additional_parameters_modification_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                            let field_ident = &element.field_ident;
                            let field_handle_token_stream = {
                                let field_handle_stringified = format!("{field_ident}_handle");
                                field_handle_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            let handle_token_stream = {
                                let handle_stringified = format!("\"{field_ident} = ${{increment}}\"");
                                handle_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            quote::quote!{
                                if let Some(value) = &#field_handle_token_stream {
                                    match #crate_server_postgres_bind_query_bind_query_try_increment_token_stream(
                                        value,
                                        &mut increment
                                    ) {
                                        Ok(_) => {
                                            let handle = format!(#handle_token_stream);
                                            match additional_parameters.is_empty() {
                                                true => {
                                                    additional_parameters.push_str(&handle);
                                                },
                                                false => {
                                                    additional_parameters.push_str(&format!(" AND {handle}"));
                                                },
                                            }
                                        },
                                        Err(#error_value_snake_case_token_stream) => {
                                            return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
                                        },
                                    }
                                }
                            }
                        });
                        let additional_parameters_primary_key_modification_token_stream = {
                            let handle_token_stream = {
                                let handle_stringified = format!(
                                    "\" {primary_key_field_ident} {in_name_stringified} ({{}})\""
                                );
                                handle_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            let additional_parameters_and_token_stream = {
                                let additional_parameters_and_stringified =
                                    format!("\" {and_name_stringified}\"");
                                additional_parameters_and_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_and_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            quote::quote! {
                                if let Some(#primary_key_field_ident) = &#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident {
                                    if let false = additional_parameters.is_empty() {
                                        additional_parameters.push_str(#additional_parameters_and_token_stream);
                                    }
                                    additional_parameters.push_str(&format!(
                                        #handle_token_stream,
                                        {
                                            let mut additional_parameters = #std_string_string_token_stream::default();
                                            for element in #primary_key_field_ident {
                                                match #crate_server_postgres_bind_query_bind_query_try_increment_token_stream(
                                                    element,
                                                    &mut increment,
                                                ) {
                                                    Ok(_) => {
                                                        additional_parameters.push_str(&format!("${increment},"));
                                                    }
                                                    Err(#error_value_snake_case_token_stream) => {
                                                        return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
                                                    }
                                                }
                                            }
                                            additional_parameters.pop();
                                            additional_parameters
                                        }
                                    ));
                                }
                            }
                        };
                        let handle_token_stream = {
                            let handle_stringified = format!("\"{delete_name_stringified} {from_name_stringified} {table_name_stringified} {where_name_stringified} {{}}{returning_primary_key_stringified}\"");
                            handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote! {
                            format!(
                                #handle_token_stream,
                                {
                                    #increment_initialization_token_stream
                                    let mut additional_parameters = #std_string_string_token_stream::default();
                                    #(#additional_parameters_modification_token_stream)*
                                    #additional_parameters_primary_key_modification_token_stream
                                    additional_parameters
                                }
                            )
                        }
                    };
                    let binded_query_token_stream = {
                        let binded_query_modifications_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                            let field_ident = &element.field_ident;
                            let field_handle_token_stream = {
                                let field_handle_stringified = format!("{field_ident}_handle");
                                field_handle_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            quote::quote!{
                                if let Some(value) = #field_handle_token_stream {
                                    query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(value, query);
                                }
                            }
                        });
                        let binded_query_primary_key_modifications_token_stream = quote::quote! {
                            if let Some(#primary_key_field_ident) = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident {
                                for element in #primary_key_field_ident {
                                    query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(element, query);
                                }
                            }
                        };
                        quote::quote! {
                            let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                            #(#binded_query_modifications_token_stream)*
                            #binded_query_primary_key_modifications_token_stream
                            query
                        }
                    };
                    quote::quote! {
                        #filter_unique_parameters_token_stream
                        let #query_string_name_token_stream = {
                            #query_string_token_stream
                        };
                        println!("{}", #query_string_name_token_stream);
                        let #binded_query_name_token_stream = {
                            #binded_query_token_stream
                        };
                        #acquire_pool_and_connection_token_stream
                        let mut rows = #binded_query_name_token_stream.fetch(#pg_connection_token_stream.as_mut());
                        let mut vec_values = std::vec::Vec::new();
                        while let Some(row) = {
                            match {
                                use futures::TryStreamExt;
                                rows.try_next()
                            }
                            .await
                            {
                                Ok(value) => value,
                                Err(#error_value_snake_case_token_stream) => {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::from(#error_value_snake_case_token_stream);
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            }
                        } {
                            match {
                                use #sqlx_row_token_stream;
                                row.try_get::<#sqlx_types_uuid_token_stream, #str_ref_token_stream>(#primary_key_field_ident_quotes_token_stream)
                            } {
                                Ok(value) => {
                                    vec_values.push(
                                        #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value),
                                    );
                                }
                                Err(#error_value_snake_case_token_stream) => {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream;
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            }
                        }
                        #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(vec_values)
                    }
                };
                quote::quote! {
                    #check_for_none_token_stream
                    match (#(#parameters_match_token_stream),*) {
                        (#(#parameters_match_primary_key_some_other_none_token_stream),*) => {
                            #generate_postgres_transaction_token_stream
                        }
                        _ => {
                            #generate_postgres_execute_query_token_stream
                        }
                    }
                }
            };
            // println!("{try_operation_token_stream}");
            let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                &table_name_stringified,
                &unique_status_codes,
                &application_json_quotes_token_stream,
                &table_name_quotes_token_stream,
                &operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                &operation,
            );
            let field_code_occurence_new_39b5c2be_b5d4_4d33_b3e9_152543c33dca_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream<'a>(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> impl #axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(#error_value_snake_case_token_stream) => {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: #error_value_snake_case_token_stream,
                                        #field_code_occurence_new_39b5c2be_b5d4_4d33_b3e9_152543c33dca_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            },
                            Err(#error_value_snake_case_token_stream) => {
                                return #error_value_snake_case_token_stream;
                            }
                        },
                    };
                    println!("{:#?}", #parameters_snake_case_token_stream);
                    {
                        #try_operation_token_stream
                    }
                }
            }
        };
        // println!("{route_handler_token_stream}");
        let common_middlewares_error_syn_variants_from_impls =
            generate_common_middlewares_error_syn_variants_from_impls(
                common_middlewares_error_syn_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                additional_http_status_codes_error_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &delete_many_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (delete_one_token_stream, delete_one_http_request_test_token_stream) = {
        let operation = Operation::DeleteOne;
        let operation_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation);
        let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(&operation);
        let operation_payload_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadUpperCamelCaseTokenStream::self_payload_upper_camel_case_token_stream(&operation);
        let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseStringified::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_stringified(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_token_stream(&operation);
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeSnakeCaseTokenStream::self_payload_try_from_self_payload_with_serialize_deserialize_snake_case_token_stream(&operation);
        let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(&operation);
        let try_operation_response_variants_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(&operation);
        let try_operation_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfUpperCamelCaseTokenStream::try_self_upper_camel_case_token_stream(&operation);
        let additional_http_status_codes_error_variants = vec![]; //todo find out why rust analyzer crashes
                                                                  // crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_method_proc_macro_attribute(
                                                                  //     &ast,
                                                                  //     &operation_name_snake_case_stringified,
                                                                  //     additional_http_status_codes_error_variants_snake_case_stringified,
                                                                  //     &proc_macro_name_snake_case,
                                                                  //     &proc_macro_name_upper_camel_case_ident_stringified
                                                                  // );
        let operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr400BadRequest,
            &operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence,
                    &proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_stringified),
                    proc_macro_helpers::naming_conventions::SelfPayloadTryFromSelfPayloadWithSerializeDeserializeUpperCamelCasePunctuated::self_payload_try_from_self_payload_with_serialize_deserialize_upper_camel_case_punctuated(
                        &operation
                    )
                )
            ]
        );
        let type_variants_from_request_response_syn_variants = {
            let full_additional_http_status_codes_error_variants =
                generate_full_additional_http_status_codes_error_variants(
                    common_middlewares_error_syn_variants.iter().collect(),
                    additional_http_status_codes_error_variants.iter().collect(),
                );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response =
                    std::vec::Vec::with_capacity(common_error_syn_variants.len() + 2);
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                //todo why no bind query error here?
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response.push(&operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants,
            )
        };
        let desirable_status_code = proc_macro_helpers::status_code::StatusCode::Tvfrr200Ok;
        let unique_status_codes = generate_unique_status_codes(
            &desirable_status_code,
            &type_variants_from_request_response_syn_variants,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        let parameters_token_stream = {
            let payload_token_stream = {
                quote::quote! {
                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream {
                        pub #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                    }
                }
            };
            // println!("{payload_token_stream}");
            let payload_with_serialize_deserialize_token_stream = {
                quote::quote! {
                    #derive_debug_serialize_deserialize_to_schema_token_stream
                    pub struct #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                    }
                }
            };
            // println!("{payload_with_serialize_deserialize_token_stream}");
            let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::PayloadTryFromPayloadWithSerializeDeserializeErrorNamedUpperCamelCaseTokenStream::payload_try_from_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream(&operation);
                let operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream = {
                    quote::quote! {
                        #derive_debug_thiserror_error_occurence_token_stream
                        pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                            #not_uuid_token_upper_camel_case_stream {
                                #eo_error_occurence_attribute_token_stream
                                #not_uuid_token_snake_case_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                            },
                        }
                    }
                };
                // println!("{operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream}");
                let field_code_occurence_new_66343753_b4dc_4b64_b7a6_3f206033a0b1_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    file!(),
                    line!(),
                    column!(),
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                let impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream = {
                    quote::quote! {
                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value.#primary_key_field_ident) {
                                    Ok(value) => Ok(Self { #primary_key_field_ident: value }),
                                    Err(#error_value_snake_case_token_stream) => Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                        #not_uuid_token_snake_case_stream: #error_value_snake_case_token_stream,
                                        #field_code_occurence_new_66343753_b4dc_4b64_b7a6_3f206033a0b1_token_stream,
                                    }),
                                }
                            }
                        }
                    }
                };
                quote::quote! {
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                quote::quote! {
                    impl std::convert::From<#operation_payload_upper_camel_case_token_stream> for #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                        fn from(value: #operation_payload_upper_camel_case_token_stream) -> Self {
                            let #primary_key_field_ident = #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value.#primary_key_field_ident);
                            Self{
                                #primary_key_field_ident
                            }
                        }
                    }
                }
            };
            // println!("{impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream}");
            quote::quote! {
                #payload_token_stream
                #payload_with_serialize_deserialize_token_stream
                #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                #impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream

                #derive_debug_token_stream
                pub struct #operation_parameters_upper_camel_case_token_stream {
                    pub #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream,
                }
            }
        };
        // println!("{parameters_token_stream}");
        let try_operation_error_with_middleware_error_variants_token_stream = {
            crate::type_variants_from_request_response_generator::type_variants_from_request_response_generator(
                &desirable_status_code,
                &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                &type_variants_from_request_response_syn_variants,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
                &generate_expected_type_declaration_token_stream,
                &unexpected_status_code_declaration_token_stream,
                &failed_to_get_response_text_declaration_token_stream,
                &deserialize_response_declaration_token_stream,
                &reqwest_declaration_token_stream,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (http_request_token_stream, http_request_test_token_stream) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let expected_type_declaration_token_stream =
                    generate_expected_type_declaration_token_stream(&operation);
                quote::quote! {
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_declaration_token_stream,
                        #expected_type_declaration_token_stream,
                        #unexpected_status_code_declaration_token_stream,
                        #failed_to_get_response_text_declaration_token_stream,
                        #deserialize_response_declaration_token_stream,
                        #reqwest_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_try_operation_token_stream(
                &server_location_name_token_stream,
                &str_ref_token_stream,
                &crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &quote::quote! {
                    //todo maybe for all cases use this? = remove this parameter and write it inside generate_try_operation_token_stream
                    let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(&#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)) {
                        Ok(value) => value,
                        Err(#error_value_snake_case_token_stream) => {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                        }
                    };
                },
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &quote::quote! {
                    match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value) {
                        Ok(value) => Ok(value),
                        Err(#error_value_snake_case_token_stream) => Err(#try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_initialization_token_stream )
                    }
                },
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                &type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                &deserialize_response_initialization_token_stream,
                &unexpected_status_code_initialization_token_stream,
                &reqwest_initialization_token_stream,
                &failed_to_get_response_text_initialization_token_stream,
                &expected_type_initialization_token_stream,
            );
            let http_request_test_token_stream = {
                let test_content_token_stream = quote::quote! {
                    match #try_operation_snake_case_token_stream(
                        #reference_api_location_test_token_stream,
                        #operation_parameters_upper_camel_case_token_stream {
                            #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
                                #primary_key_field_ident: #primary_key_token_stream.clone()
                            }
                        },
                    )
                    .await
                    {
                        Ok(value) => println!("{value:#?}"),
                        Err(#error_value_snake_case_token_stream) => panic!("{}", #error_value_snake_case_token_stream)
                    }
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote! {
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream,
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let query_string_token_stream = {
                    let additional_parameters_primary_key_modification_token_stream = {
                        let query_part_token_stream = {
                            let query_part_stringified =
                                format!("\" {primary_key_field_ident} = $1\""); //todo where
                            query_part_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_part_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote! {
                            query.push_str(&format!(#query_part_token_stream));
                        }
                    };
                    let handle_token_stream = {
                        let handle_stringified = format!("\"{delete_name_stringified} {from_name_stringified} {table_name_stringified} {where_name_stringified}\""); //todo where
                        handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote! {
                        let mut query = format!(#handle_token_stream);
                        #additional_parameters_primary_key_modification_token_stream
                        query.push_str(&format!(#returning_primary_key_quotes_token_stream));
                        query
                    }
                };
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = quote::quote! {
                        query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident, query);
                    };
                    quote::quote! {
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        #binded_query_modifications_token_stream
                        query
                    }
                };
                let from_log_and_return_error_token_stream =
                    crate::from_log_and_return_error::from_log_and_return_error(
                        &try_operation_upper_camel_case_token_stream,
                        &error_log_call_token_stream,
                        &try_operation_response_variants_token_stream,
                    );
                let acquire_pool_and_connection_token_stream =
                    crate::acquire_pool_and_connection::acquire_pool_and_connection(
                        &from_log_and_return_error_token_stream,
                        &pg_connection_token_stream,
                    );
                quote::quote! {
                    let #query_string_name_token_stream = {
                        #query_string_token_stream
                    };
                    println!("{}", #query_string_name_token_stream);
                    let #binded_query_name_token_stream = {
                        #binded_query_token_stream
                    };
                    #acquire_pool_and_connection_token_stream
                    match #binded_query_name_token_stream
                        .fetch_one(#pg_connection_token_stream.as_mut())
                        .await
                    {
                        Ok(value) => match {
                            use #sqlx_row_token_stream;
                            value.try_get::<#sqlx_types_uuid_token_stream, #str_ref_token_stream>(#primary_key_field_ident_quotes_token_stream)
                        } {
                            Ok(value) => #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value)),
                            Err(#error_value_snake_case_token_stream) => {
                                let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                            }
                        },
                        Err(#error_value_snake_case_token_stream) => {
                            #from_log_and_return_error_token_stream;
                        }
                    }
                }
            };
            // println!("{try_operation_token_stream}");
            let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                &table_name_stringified,
                &unique_status_codes,
                &application_json_quotes_token_stream,
                &table_name_quotes_token_stream,
                &operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                &operation,
            );
            let field_code_occurence_new_32b2a167_ab66_4ee6_8e59_3839fa83d830_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                file!(),
                line!(),
                column!(),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream<'a>(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> impl #axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(#error_value_snake_case_token_stream) => {
                                    let #error_value_snake_case_token_stream = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: #error_value_snake_case_token_stream,
                                        #field_code_occurence_new_32b2a167_ab66_4ee6_8e59_3839fa83d830_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(#error_value_snake_case_token_stream);
                                }
                            },
                            Err(#error_value_snake_case_token_stream) => {
                                return #error_value_snake_case_token_stream;
                            }
                        },
                    };
                    println!("{:#?}", #parameters_snake_case_token_stream);
                    {
                        #try_operation_token_stream
                    }
                }
            }
        };
        // println!("{route_handler_token_stream}");
        let common_middlewares_error_syn_variants_from_impls =
            generate_common_middlewares_error_syn_variants_from_impls(
                common_middlewares_error_syn_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                additional_http_status_codes_error_variants
                    .iter()
                    .collect::<std::vec::Vec<&(
                        syn::Ident,
                        proc_macro2::TokenStream,
                        std::vec::Vec<syn::Variant>,
                    )>>(),
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote! {
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream,
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &delete_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let emulate_crud_api_usage_test_token_stream = {
        let ident_emulate_crud_api_usage_test_snake_case_token_stream = {
            let ident_emulate_crud_api_usage_test_snake_case_stringified =
                format!("{ident_snake_case_stringified}_emulate_crud_api_usage_test");
            ident_emulate_crud_api_usage_test_snake_case_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_emulate_crud_api_usage_test_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        quote::quote! {
            #[test]
            fn #ident_emulate_crud_api_usage_test_snake_case_token_stream() {
                async fn find_out_if_it_works() {
                    let api_location = #std_string_string_token_stream::from("http://127.0.0.1:8080");//todo port from env or config maybe?
                    let limit = 1000;
                    let offset = 0;
                    #create_many_http_request_test_token_stream
                    #read_many_http_request_test_token_stream
                    #update_many_http_request_test_token_stream
                    #read_many_http_request_test_token_stream
                    #delete_many_http_request_test_token_stream
                    #read_many_http_request_test_token_stream
                    #create_one_http_request_test_token_stream
                    #read_one_http_request_test_expect_success_token_stream
                    #update_one_http_request_test_token_stream
                    #read_one_http_request_test_expect_success_token_stream
                    #delete_one_http_request_test_token_stream
                    #read_one_http_request_test_expect_fail_token_stream
                }
                match tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(num_cpus::get())
                    .enable_all()
                    .build()
                {
                    Err(#error_value_snake_case_token_stream) => {
                        panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {:#?}", #error_value_snake_case_token_stream)
                    }
                    Ok(runtime) => {
                        runtime.block_on(find_out_if_it_works());
                    }
                }
            }
        }
    };
    // println!("{emulate_crud_api_usage_test_token_stream}");
    let common_token_stream = quote::quote! {
        #table_name_declaration_token_stream
        #struct_options_token_stream
        #from_ident_for_ident_options_token_stream
        #(#structs_variants_token_stream)*
        #(#impl_std_convert_try_from_ident_options_for_struct_variants_token_stream)*
        #column_token_stream
        #column_select_token_stream
        #primary_key_uuid_wrapper_try_from_sqlx_row_token_stream
        #deserialize_ident_order_by_token_stream
        #order_by_wrapper_token_stream
        #allow_methods_token_stream
        #ident_column_read_permission_token_stream
        // #[cfg(test)]
        // mod test_try_create_many {
            #emulate_crud_api_usage_test_token_stream
        // }
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &common_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    //comment out coz its impossible to correctly generate tokens for debug purposes
    // let _mod_name_snake_case_token_stream = {
    //     let value_stringified = format!("{proc_macro_name_snake_case}_{table_name_stringified}");
    //     value_stringified.parse::<proc_macro2::TokenStream>()
    //     .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    // };
    //todo pub and private impl quote group
    let gen = quote::quote! {
        //comment out coz its impossible to correctly generate tokens
        // pub mod #mod_name_snake_case_token_stream {/
            // #common_token_stream//+

            // #create_many_token_stream//+
            // #create_one_token_stream
            // #read_many_token_stream
            // #read_one_token_stream
            // #update_many_token_stream
            // #update_one_token_stream
            // #delete_many_token_stream
            // #delete_one_token_stream
        // }
    };
    // if ident == "" {
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &gen,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    // }
    gen.into()
}

fn generate_std_vec_vec_syn_punctuated_punctuated(
    parts_vec: &[&str],
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> {
    let parts_vec_len = parts_vec.len();
    match parts_vec_len >= 1 {
        true => {
            let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();
            handle.push_value(
                syn::PathSegment {
                    ident: proc_macro2::Ident::new("std", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                }
            );
            handle.push_punct(syn::token::Colon2{
                spans: [proc_macro2::Span::call_site(),proc_macro2::Span::call_site()],
            });
            handle.push_value(
                syn::PathSegment {
                    ident: proc_macro2::Ident::new("vec", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                }
            );
            handle.push_punct(syn::token::Colon2{
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
                                        let parts_vec_len_minus_one = parts_vec_len - 1;
                                        let mut std_vec_vec_generic_type = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();
                                        for (index, element) in parts_vec.iter().enumerate() {
                                            std_vec_vec_generic_type.push_value(
                                                syn::PathSegment {
                                                    ident: proc_macro2::Ident::new(element, proc_macro2::Span::call_site()),
                                                    arguments: syn::PathArguments::None,
                                                }
                                            );
                                            match index < parts_vec_len_minus_one {
                                                true => {
                                                    std_vec_vec_generic_type.push_punct(syn::token::Colon2{
                                                        spans: [proc_macro2::Span::call_site(),proc_macro2::Span::call_site()],
                                                    });
                                                }
                                                false => ()
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
        },
        false => panic!("{proc_macro_name_upper_camel_case_ident_stringified} generate_simple_syn_punctuated_punctuated parts_vec_len.len() > 1 == false for {parts_vec:?}")
    }
}

#[derive(Debug)]
struct SynFieldWithAdditionalInfo {
    field: syn::Field,
    field_ident: syn::Ident,
    rust_sqlx_map_to_postgres_type_variant:
        postgresql_crud_common::RustSqlxMapToPostgresTypeVariant, //todo maybe not need to add here
    maybe_generic_token_stream: proc_macro2::TokenStream, //todo maybe not need to add here
    path_token_stream: proc_macro2::TokenStream,
    original_type_token_stream: proc_macro2::TokenStream,
    inner_type_token_stream: proc_macro2::TokenStream,
    inner_type_with_serialize_deserialize_token_stream: proc_macro2::TokenStream,
}
impl std::convert::From<syn::Field> for SynFieldWithAdditionalInfo {
    fn from(value: syn::Field) -> Self {
        let name = "SynFieldWithAdditionalInfo from syn::Field error";
        let field = value.clone();
        let field_ident = value
            .ident
            .unwrap_or_else(|| panic!("{name} field ident is none"));
        let (rust_sqlx_map_to_postgres_type_variant, maybe_generic_token_stream) = match &value.ty {
            syn::Type::Path(value) => {
                match value.path.segments.len() == 2 {
                    true => {
                        if value.path.segments[0].ident
                            != postgresql_crud_common::POSTGRESQL_CRUD_SNAKE_CASE
                        {
                            panic!("{name} field_type is not syn::Type::Path");
                        }
                        match value.path.segments[0].arguments {
                        syn::PathArguments::None => (),
                        _ => panic!("{name} value.path.segments[0].arguments != syn::PathArguments::None")
                    }
                        let rust_sqlx_map_to_postgres_type_variant =
                            match postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::try_from(
                                &value.path.segments[1].ident.to_string() as &str,
                            ) {
                                Ok(value) => value,
                                Err(e) => panic!(
                                    "{name} RustSqlxMapToPostgresTypeVariant::try_from failed {e}"
                                ),
                            };
                        let maybe_generic_token_stream = match &value.path.segments[1].arguments {
                            syn::PathArguments::None => quote::quote! {},
                            syn::PathArguments::AngleBracketed(value) => {
                                quote::quote! {#value} //< test_mod :: Something >
                            }
                            syn::PathArguments::Parenthesized(_) => {
                                panic!("{name} does not support syn::PathArguments::Parenthesized")
                            }
                        };
                        (
                            rust_sqlx_map_to_postgres_type_variant,
                            maybe_generic_token_stream,
                        )
                    }
                    false => panic!("{name} field_type is not syn::Type::Path"),
                }
            }
            _ => panic!("{name} field_type is not syn::Type::Path"),
        };
        let path_token_stream = {
            let path_stringified = &rust_sqlx_map_to_postgres_type_variant.get_path_stringified(); //todo generic for json
            path_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{name} {path_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let original_type_token_stream = {
            let original_type_stringified =
                &rust_sqlx_map_to_postgres_type_variant.get_original_type_stringified(""); //todo generic for json
            original_type_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{name} {original_type_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let inner_type_token_stream = {
            let inner_type_stringified =
                &rust_sqlx_map_to_postgres_type_variant.get_inner_type_stringified(""); //todo generic for json
            inner_type_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{name} {inner_type_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let inner_type_with_serialize_deserialize_token_stream = {
            let inner_type_with_serialize_deserialize_stringified =
                &rust_sqlx_map_to_postgres_type_variant
                    .get_inner_type_with_serialize_deserialize_stringified(""); //todo generic for json
            inner_type_with_serialize_deserialize_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{name} {inner_type_with_serialize_deserialize_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        Self {
            field,
            field_ident,
            rust_sqlx_map_to_postgres_type_variant, //todo maybe not need to add here
            maybe_generic_token_stream,             //todo maybe not need to add here
            path_token_stream,
            original_type_token_stream,
            inner_type_token_stream,
            inner_type_with_serialize_deserialize_token_stream,
        }
    }
}

fn generate_common_middlewares_error_syn_variants_from_impls(
    common_middlewares_error_syn_variants: std::vec::Vec<&(
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec<syn::Variant>,
    )>,
    additional_http_status_codes_error_variants: std::vec::Vec<&(
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec<syn::Variant>,
    )>,
    operation: &Operation,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let error_syn_variants = generate_full_additional_http_status_codes_error_variants(
        common_middlewares_error_syn_variants,
        additional_http_status_codes_error_variants,
    );
    let try_operation_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfUpperCamelCaseTokenStream::try_self_upper_camel_case_token_stream(operation);
    let value = error_syn_variants.iter().map(|element|{
        let element_path_ident_token_stream = {
            let element_path_ident_stringified = format!("{}{}", element.1, element.0);
            element_path_ident_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {element_path_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let variants = element.2.iter().map(|element|{
            let element_ident = &element.ident;
            let fields_named = if let syn::Fields::Named(fields_named) = &element.fields {
                &fields_named.named
            } else {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {element_ident} supports only syn::Fields::Named");
            };
            let fields_token_stream = generate_self_fields_token_stream(
                &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
                proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {
                #element_path_ident_token_stream::#element_ident {
                    #(#fields_token_stream),*
                } => Self::#element_ident {
                    #(#fields_token_stream),*
                }
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            impl std::convert::From<#element_path_ident_token_stream> for #try_operation_upper_camel_case_token_stream {
                fn from(value: #element_path_ident_token_stream) -> Self {
                    match value {
                        #(#variants),*
                    }
                }
            }
        }
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    quote::quote! {#(#value)*}
}

fn generate_not_unique_field_vec_upper_camel_stringified(
    field_ident: &syn::Ident,
) -> std::string::String {
    format!("NotUnique{}Vec", {//todo reuse naming
        convert_case::Casing::to_case(
            &field_ident
            .to_string(),
            convert_case::Case::UpperCamel
        )
    })
}

fn generate_not_unique_field_vec_snake_case_stringified(
    field_ident: &syn::Ident,
) -> std::string::String {
    format!("not_unique_{field_ident}_vec")
}

fn generate_self_fields_token_stream<'a>(
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

fn generate_pub_field_ident_field_type_token_stream(
    element: &SynFieldWithAdditionalInfo,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let field_ident = &element.field_ident;
    let field_type = &element.field.ty;
    let inner_type_token_stream = &element.inner_type_token_stream;
    quote::quote! {
        pub #field_ident: #inner_type_token_stream
    }
}

fn generate_field_ident_field_type_with_serialize_deserialize_token_stream(
    element: &SynFieldWithAdditionalInfo,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let field_ident = &element.field_ident;
    let field_type = &element.field.ty;
    let inner_type_with_serialize_deserialize_token_stream =
        &element.inner_type_with_serialize_deserialize_token_stream;
    quote::quote! {
        pub #field_ident: #inner_type_with_serialize_deserialize_token_stream
    }
}

fn generate_let_field_ident_value_field_ident_try_from_token_stream(
    element: &SynFieldWithAdditionalInfo,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    field_code_occurence_new_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let field_ident = &element.field_ident;
    let inner_token_stream = quote::quote! {value.#field_ident};
    let initialization_token_stream = {
        let inner_type_token_stream = &element.inner_type_token_stream;
        let from_token_stream = {
            let from_snake_case_token_stream = proc_macro_helpers::naming_conventions::from_snake_case_token_stream();
            quote::quote!{#inner_type_token_stream::#from_snake_case_token_stream(#inner_token_stream)}
        };
        let try_from_token_stream = {
            let try_from_snake_case_token_stream = proc_macro_helpers::naming_conventions::try_from_snake_case_token_stream();
            let postgresql_crud_common_supported_sqlx_postgres_type_snake_case_token_stream = {
                let value_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
                    &postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)
                );
                value_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_ident_upper_camel_case_token_stream = {
                //todo its a temporal naming desicion. maybe it can be better
                let value_stringified = syn_ident_to_upper_camel_case_stringified(&element.field_ident);
                value_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                match #inner_type_token_stream::#try_from_snake_case_token_stream(#inner_token_stream) {
                    Ok(value) => value,
                    Err(e) => {
                        return Err(Self::Error::#field_ident_upper_camel_case_token_stream {
                            #postgresql_crud_common_supported_sqlx_postgres_type_snake_case_token_stream: e,
                            #field_code_occurence_new_token_stream
                        });
                    }
                }
            }
        };
        match element.rust_sqlx_map_to_postgres_type_variant {
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlName => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlNameNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => try_from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull => try_from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => try_from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull => try_from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => try_from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull => try_from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => try_from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull => try_from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => try_from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull => try_from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => try_from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull => try_from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate => try_from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull => try_from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime => try_from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull => try_from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid => try_from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull => try_from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => try_from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonBNotNull => from_token_stream,

            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => from_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => from_token_stream,
        }
    };
    quote::quote! {
        let #field_ident = #initialization_token_stream;
    }
}

fn generate_let_field_ident_value_field_ident_from_token_stream(
    element: &SynFieldWithAdditionalInfo,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let field_ident = &element.field_ident;
    let inner_type_with_serialize_deserialize_token_stream =
        &element.inner_type_with_serialize_deserialize_token_stream;
    quote::quote! {
        let #field_ident = #inner_type_with_serialize_deserialize_token_stream::from(value.#field_ident);//todo from or try_from
    }
}

fn generate_http_request_many_token_stream(
    server_location_name_token_stream: &proc_macro2::TokenStream,
    str_ref_token_stream: &proc_macro2::TokenStream,
    std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream: &proc_macro2::TokenStream,
    serde_json_to_string_token_stream: &proc_macro2::TokenStream,
    serde_json_to_string_variant_initialization_token_stream: &proc_macro2::TokenStream,
    reqwest_client_new_token_stream: &proc_macro2::TokenStream,
    commit_header_addition_token_stream: &proc_macro2::TokenStream,
    content_type_application_json_header_addition_token_stream: &proc_macro2::TokenStream,
    crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream: &proc_macro2::TokenStream,
    operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_initialization_token_stream: &proc_macro2::TokenStream,
    table_name_stringified: &str,
    operation: &Operation,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    type_variants_from_request_response_syn_variants: &std::vec::Vec<&syn::Variant>,
    desirable_status_code: &proc_macro_helpers::status_code::StatusCode,
    desirable_type_token_stream: &proc_macro2::TokenStream,
    deserialize_response_initialization_token_stream: &proc_macro2::TokenStream,
    unexpected_status_code_initialization_token_stream: &proc_macro2::TokenStream,
    reqwest_initialization_token_stream: &proc_macro2::TokenStream,
    failed_to_get_response_text_initialization_token_stream: &proc_macro2::TokenStream,
    expected_type_initialization_token_stream: &proc_macro2::TokenStream,
    primary_key_syn_field_with_additional_info: &SynFieldWithAdditionalInfo,
) -> proc_macro2::TokenStream {
    let parameters_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::parameters_snake_case_token_stream();
    let payload_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::payload_snake_case_token_stream();
    let operation_http_method_snake_case_token_stream =
        proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(
            &operation.http_method(),
        );
    let url_handle_token_stream = proc_macro_helpers::naming_conventions::UrlHandleSelfSnakeCaseTokenStream::url_handle_self_snake_case_token_stream(operation, table_name_stringified);
    let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(operation);
    let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
    let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let type_variants_from_request_response_syn_variants_len =
        type_variants_from_request_response_syn_variants.len();
    let code_occurence_snake_case_stringified =
        proc_macro_helpers::naming_conventions::code_occurence_snake_case_stringified();
    let code_occurence_upper_camel_case_stringified =
        proc_macro_helpers::naming_conventions::code_occurence_upper_camel_case_stringified();
    let try_operation_response_variants_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(operation);
    let http_status_code_quote_token_stream =
        desirable_status_code.to_http_status_code_token_stream();
    let (unique_status_codes, unique_status_codes_len, unique_status_codes_len_minus_one) = {
        let hashmap_unique_status_codes = type_variants_from_request_response_syn_variants.iter().fold(//todo maybe not need hashmap here? maybe just unique vec?
            std::collections::HashMap::<proc_macro_helpers::status_code::StatusCode, std::vec::Vec<(
                &syn::Ident,
                std::vec::Vec<(syn::Ident, proc_macro2::TokenStream)>,
            )>>::with_capacity(type_variants_from_request_response_syn_variants_len),
            |mut acc, element| {
                let variant_ident = &element.ident;
                let error_variant_attribute = proc_macro_helpers::status_code::StatusCode::try_from(element)
                .unwrap_or_else(|e| {panic!("{proc_macro_name_upper_camel_case_ident_stringified} variant {variant_ident} failed: {e}")});
                let fields_named = if let syn::Fields::Named(fields_named) = &element.fields {
                    fields_named
                }
                else {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
                };
                let error_variant_fields = fields_named.named.iter().map(|field|{
                    let field_ident = field.ident.as_ref().unwrap_or_else(|| {
                        panic!(
                            "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                            naming_constants::FIELD_IDENT_IS_NONE
                        )
                    });
                    let field_type_with_serialize_deserialize = match *field_ident == *code_occurence_snake_case_stringified {
                        true => {
                            let code_occurence_type_token_stream = {
                                if let syn::Type::Path(type_path) = &field.ty {
                                    let mut code_occurence_type_repeat_checker = false;
                                    let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
                                    .fold(String::from(""), |mut acc, path_segment| {
                                        let path_segment_ident = &path_segment.ident;
                                        match *path_segment_ident == code_occurence_upper_camel_case_stringified {
                                            true => {
                                                if code_occurence_type_repeat_checker {
                                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} code_occurence_ident detected more than one {code_occurence_upper_camel_case_stringified} inside type path");
                                                }
                                                acc.push_str(&path_segment_ident.to_string());
                                                code_occurence_type_repeat_checker = true;
                                            },
                                            false => acc.push_str(&format!("{path_segment_ident}::")),
                                        }
                                        acc
                                    });
                                    if !code_occurence_type_repeat_checker {
                                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} no {code_occurence_upper_camel_case_stringified} named field");
                                    }
                                    code_occurence_segments_stringified_handle.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_segments_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                }
                                else {
                                    let syn_type_path_stringified = proc_macro_helpers::naming_conventions::syn_type_path_stringified();
                                    panic!(
                                        "{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_snake_case_stringified} {} {syn_type_path_stringified}",
                                        naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                    );
                                }
                            };
                            code_occurence_type_token_stream
                        },
                        false => {
                            let attribute = {
                                let mut option_attribute = None;
                                field.attrs.iter().for_each(|attr|{
                                    if attr.path.segments.len() == 1 {
                                        let error_message = format!("{proc_macro_name_upper_camel_case_ident_stringified} two or more supported attributes!");
                                        let attr_ident = match attr.path.segments.iter().next() {
                                            Some(path_segment) => &path_segment.ident,
                                            None => panic!("attr.path.segments.iter().next() is None"),
                                        };
                                        if let Ok(value) = {
                                            use std::str::FromStr;
                                            proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::from_str(&attr_ident.to_string())
                                        } {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(value);
                                            }
                                        }
                                    }//other attributes are not for this proc_macro
                                });
                                option_attribute.unwrap_or_else(|| panic!(
                                    "{proc_macro_name_upper_camel_case_ident_stringified} option attribute {}",
                                    naming_constants::IS_NONE_STRINGIFIED
                                ))
                            };
                            let supported_container = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_supported_container(
                                field,
                                proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_field_type_with_serialize_deserialize_version(
                                attribute,
                                supported_container,
                                proc_macro_name_upper_camel_case_ident_stringified,
                            )
                        },
                    };
                    (field_ident.clone(), field_type_with_serialize_deserialize)
                }).collect::<Vec<(syn::Ident, proc_macro2::TokenStream)>>();
                let error_variant = (
                    variant_ident,
                    error_variant_fields,
                );
                match acc.get_mut(&error_variant_attribute) {
                    Some(value) => {
                        value.push(error_variant);
                    },
                    None => {
                        acc.insert(error_variant_attribute, vec![error_variant]);
                    }
                }
                acc
            },
        );
        let unique_status_codes_len = hashmap_unique_status_codes.len();
        if unique_status_codes_len < 1 {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} unique_status_codes_len < 1 {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE);
        }
        let unique_status_codes_len_minus_one = unique_status_codes_len - 1;
        let unique_status_codes = hashmap_unique_status_codes
            .into_keys()
            .collect::<std::vec::Vec<proc_macro_helpers::status_code::StatusCode>>();
        (
            unique_status_codes,
            unique_status_codes_len,
            unique_status_codes_len_minus_one,
        )
    };
    let desirable_enum_name = {
        let status_code_enum_name_stingified = format!("{try_operation_response_variants_upper_camel_case_token_stream}{desirable_status_code}");
        status_code_enum_name_stingified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {status_code_enum_name_stingified} {}",proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let error_value_snake_case_token_stream =
        proc_macro_common::error_value_snake_case_token_stream();
    let status_code_enums_try_from = {
        let mut is_last_element_found = false;
        let desirable_status_code_case_token_stream = quote::quote! {
            match serde_json::from_str::<#desirable_enum_name>(&response_text) {
                Ok(value) => #try_operation_response_variants_upper_camel_case_token_stream::from(value),
                Err(#error_value_snake_case_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#deserialize_response_initialization_token_stream);
                }
            }
        };
        let mut status_code_enums_try_from_variants =
            std::vec::Vec::with_capacity(unique_status_codes_len + 1);
        status_code_enums_try_from_variants.push(quote::quote! {
            if status_code == #http_status_code_quote_token_stream {
                #desirable_status_code_case_token_stream
            }
        });
        unique_status_codes
        .into_iter()
        .enumerate()
        .for_each(|(index, status_code_attribute)|{
            let try_operation_response_variants_desirable_attribute_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsStatusCodeTokenStream::try_self_response_variants_status_code_token_stream(operation, &status_code_attribute);
            let http_status_code_token_stream = status_code_attribute.to_http_status_code_token_stream();
            match index == unique_status_codes_len_minus_one{
                true => {
                    is_last_element_found = true;
                    status_code_enums_try_from_variants.push(quote::quote! {
                        else {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#unexpected_status_code_initialization_token_stream);
                        }
                    });
                },
                false => {
                    if *desirable_status_code != status_code_attribute {
                        status_code_enums_try_from_variants.push(quote::quote! {
                            else if status_code == #http_status_code_token_stream {
                                match serde_json::from_str::<#try_operation_response_variants_desirable_attribute_token_stream>(&response_text) {
                                    Ok(value) => #try_operation_response_variants_upper_camel_case_token_stream::from(value),
                                    Err(#error_value_snake_case_token_stream) => {
                                        return Err(#try_operation_error_named_upper_camel_case_token_stream::#deserialize_response_initialization_token_stream);
                                    }
                                }
                            }
                        });
                    }
                },
            }
        });
        if !is_last_element_found {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} false = is_last_element_found");
        }
        status_code_enums_try_from_variants
    };
    let inner_type_token_stream = {
        let value_stringified = primary_key_syn_field_with_additional_info.rust_sqlx_map_to_postgres_type_variant.get_inner_type_stringified("");
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}",proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    quote::quote! {
        pub async fn #try_operation_snake_case_token_stream<'a>(
            #server_location_name_token_stream: #str_ref_token_stream,
            #parameters_snake_case_token_stream: #operation_parameters_upper_camel_case_token_stream,
        ) -> Result<
            #std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
            #try_operation_error_named_upper_camel_case_token_stream,
        > {
            let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(&#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)) {
                Ok(value) => value,
                Err(#error_value_snake_case_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                }
            };
            let url = format!(
                #url_handle_token_stream,
                #server_location_name_token_stream,
            );
            let future = #reqwest_client_new_token_stream
                .#operation_http_method_snake_case_token_stream(&url)
                #commit_header_addition_token_stream
                #content_type_application_json_header_addition_token_stream
                .body(#payload_snake_case_token_stream)
                .send();
            let response = match future.await {
                Ok(response) => response,
                Err(#error_value_snake_case_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#reqwest_initialization_token_stream);
                }
            };
            let status_code = response.status();
            let headers = response.headers().clone();
            let response_text = match response.text().await {
                Ok(response_text) => response_text,
                Err(#error_value_snake_case_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#failed_to_get_response_text_initialization_token_stream);
                }
            };
            let variants = #(#status_code_enums_try_from)*;
            match #desirable_type_token_stream::try_from(variants) {
                Ok(value) => {
                    let mut vec_values = std::vec::Vec::with_capacity(value.len());
                    let mut vec_errors = std::vec::Vec::with_capacity(value.len());
                    for element in value {
                        //HERE  #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream
                        match #inner_type_token_stream::try_from(element) {
                            Ok(value) => {
                                vec_values.push(value);
                            }
                            Err(#error_value_snake_case_token_stream) => {
                                vec_errors.push(
                                    #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream(e)
                                );
                            }
                        }
                    }
                    if let false = vec_errors.is_empty() {
                        return Err(#try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_initialization_token_stream);
                    }
                    Ok(vec_values)
                },
                Err(#error_value_snake_case_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#expected_type_initialization_token_stream);
                }
            }
        }
    }
}

fn generate_try_operation_token_stream(
    server_location_name_token_stream: &proc_macro2::TokenStream,
    str_ref_token_stream: &proc_macro2::TokenStream,
    return_result_ok_type_token_stream: &proc_macro2::TokenStream,
    payload_variable_initialization_token_stream: &proc_macro2::TokenStream,
    reqwest_client_new_token_stream: &proc_macro2::TokenStream,
    commit_header_addition_token_stream: &proc_macro2::TokenStream,
    content_type_application_json_header_addition_token_stream: &proc_macro2::TokenStream,
    ok_value_handle_token_stream: &proc_macro2::TokenStream,
    table_name_stringified: &str,
    operation: &Operation,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    type_variants_from_request_response_syn_variants: &std::vec::Vec<&syn::Variant>,
    desirable_status_code: &proc_macro_helpers::status_code::StatusCode,
    desirable_type_token_stream: &proc_macro2::TokenStream,
    deserialize_response_initialization_token_stream: &proc_macro2::TokenStream,
    unexpected_status_code_initialization_token_stream: &proc_macro2::TokenStream,
    reqwest_initialization_token_stream: &proc_macro2::TokenStream,
    failed_to_get_response_text_initialization_token_stream: &proc_macro2::TokenStream,
    expected_type_initialization_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let parameters_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::parameters_snake_case_token_stream();
    let payload_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::payload_snake_case_token_stream();
    let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
    let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
    let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(operation);
    let operation_http_method_snake_case_token_stream =
        proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(
            &operation.http_method(),
        );
    let url_handle_token_stream = proc_macro_helpers::naming_conventions::UrlHandleSelfSnakeCaseTokenStream::url_handle_self_snake_case_token_stream(operation, table_name_stringified);
    let code_occurence_snake_case_stringified =
        proc_macro_helpers::naming_conventions::code_occurence_snake_case_stringified();
    let type_variants_from_request_response_syn_variants_len =
        type_variants_from_request_response_syn_variants.len();
    let code_occurence_upper_camel_case_stringified =
        proc_macro_helpers::naming_conventions::code_occurence_upper_camel_case_stringified();
    let try_operation_response_variants_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(operation);
    let http_status_code_quote_token_stream =
        desirable_status_code.to_http_status_code_token_stream();
    let desirable_enum_name = {
        let status_code_enum_name_stingified = format!("{try_operation_response_variants_upper_camel_case_token_stream}{desirable_status_code}");
        status_code_enum_name_stingified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {status_code_enum_name_stingified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let (unique_status_codes, unique_status_codes_len, unique_status_codes_len_minus_one) = {
        let hashmap_unique_status_codes = type_variants_from_request_response_syn_variants.iter().fold(//todo maybe not need hashmap here? maybe just unique vec?
            std::collections::HashMap::<proc_macro_helpers::status_code::StatusCode, std::vec::Vec<(
                &syn::Ident,
                std::vec::Vec<(syn::Ident, proc_macro2::TokenStream)>,
            )>>::with_capacity(type_variants_from_request_response_syn_variants_len),
            |mut acc, element| {
                let variant_ident = &element.ident;
                let error_variant_attribute = proc_macro_helpers::status_code::StatusCode::try_from(element)
                .unwrap_or_else(|e| {panic!("{proc_macro_name_upper_camel_case_ident_stringified} variant {variant_ident} failed: {e}")});
                let fields_named = if let syn::Fields::Named(fields_named) = &element.fields {
                    fields_named
                }
                else {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} expected fields would be named");
                };
                let error_variant_fields = fields_named.named.iter().map(|field|{
                    let field_ident = field.ident.as_ref().unwrap_or_else(|| {
                        panic!(
                            "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                            naming_constants::FIELD_IDENT_IS_NONE
                        )
                    });
                    let field_type_with_serialize_deserialize = match *field_ident == *code_occurence_snake_case_stringified {
                        true => {
                            let code_occurence_type_token_stream = {
                                if let syn::Type::Path(type_path) = &field.ty {
                                    let mut code_occurence_type_repeat_checker = false;
                                    let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
                                    .fold(String::from(""), |mut acc, path_segment| {
                                        let path_segment_ident = &path_segment.ident;
                                        match *path_segment_ident == code_occurence_upper_camel_case_stringified {
                                            true => {
                                                if code_occurence_type_repeat_checker {
                                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} code_occurence_ident detected more than one {code_occurence_upper_camel_case_stringified} inside type path");
                                                }
                                                acc.push_str(&path_segment_ident.to_string());
                                                code_occurence_type_repeat_checker = true;
                                            },
                                            false => acc.push_str(&format!("{path_segment_ident}::")),
                                        }
                                        acc
                                    });
                                    if !code_occurence_type_repeat_checker {
                                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} no {code_occurence_upper_camel_case_stringified} named field");
                                    }
                                    code_occurence_segments_stringified_handle.parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_segments_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                }
                                else {
                                    let syn_type_path_stringified = proc_macro_helpers::naming_conventions::syn_type_path_stringified();
                                    panic!(
                                        "{proc_macro_name_upper_camel_case_ident_stringified} {code_occurence_snake_case_stringified} {} {syn_type_path_stringified}",
                                        naming_constants::SUPPORTS_ONLY_STRINGIFIED
                                    );
                                }
                            };
                            code_occurence_type_token_stream
                        },
                        false => {
                            let attribute = {
                                let mut option_attribute = None;
                                field.attrs.iter().for_each(|attr|{
                                    if attr.path.segments.len() == 1 {
                                        let error_message = format!("{proc_macro_name_upper_camel_case_ident_stringified} two or more supported attributes!");
                                        let attr_ident = match attr.path.segments.iter().next() {
                                            Some(path_segment) => &path_segment.ident,
                                            None => panic!("attr.path.segments.iter().next() is None"),
                                        };
                                        if let Ok(value) = {
                                            use std::str::FromStr;
                                            proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::from_str(&attr_ident.to_string())
                                        } {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(value);
                                            }
                                        }
                                    }//other attributes are not for this proc_macro
                                });
                                option_attribute.unwrap_or_else(|| panic!(
                                    "{proc_macro_name_upper_camel_case_ident_stringified} option attribute {}",
                                    naming_constants::IS_NONE_STRINGIFIED
                                ))
                            };
                            let supported_container = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_supported_container(
                                field,
                                proc_macro_name_upper_camel_case_ident_stringified,
                            );
                            proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_field_type_with_serialize_deserialize_version(
                                attribute,
                                supported_container,
                                proc_macro_name_upper_camel_case_ident_stringified,
                            )
                        },
                    };
                    (field_ident.clone(), field_type_with_serialize_deserialize)
                }).collect::<Vec<(syn::Ident, proc_macro2::TokenStream)>>();
                let error_variant = (
                    variant_ident,
                    error_variant_fields,
                );
                match acc.get_mut(&error_variant_attribute) {
                    Some(value) => {
                        value.push(error_variant);
                    },
                    None => {
                        acc.insert(error_variant_attribute, vec![error_variant]);
                    }
                }
                acc
            },
        );
        let unique_status_codes_len = hashmap_unique_status_codes.len();
        if unique_status_codes_len < 1 {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} unique_status_codes_len < 1 {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE);
        }
        let unique_status_codes_len_minus_one = unique_status_codes_len - 1;
        let unique_status_codes = hashmap_unique_status_codes
            .into_keys()
            .collect::<std::vec::Vec<proc_macro_helpers::status_code::StatusCode>>();
        (
            unique_status_codes,
            unique_status_codes_len,
            unique_status_codes_len_minus_one,
        )
    };
    let error_value_snake_case_token_stream =
        proc_macro_common::error_value_snake_case_token_stream();
    let status_code_enums_try_from = {
        let mut is_last_element_found = false;
        let desirable_status_code_case_token_stream = quote::quote! {
            match serde_json::from_str::<#desirable_enum_name>(&response_text) {
                Ok(value) => #try_operation_response_variants_upper_camel_case_token_stream::from(value),
                Err(#error_value_snake_case_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#deserialize_response_initialization_token_stream);
                }
            }
        };
        let mut status_code_enums_try_from_variants =
            std::vec::Vec::with_capacity(unique_status_codes_len + 1);
        status_code_enums_try_from_variants.push(quote::quote! {
            if status_code == #http_status_code_quote_token_stream {
                #desirable_status_code_case_token_stream
            }
        });
        unique_status_codes
        .into_iter()
        .enumerate()
        .for_each(|(index, status_code_attribute)|{
            let try_operation_response_variants_desirable_attribute_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsStatusCodeTokenStream::try_self_response_variants_status_code_token_stream(operation, &status_code_attribute);
            let http_status_code_token_stream = status_code_attribute.to_http_status_code_token_stream();
            match index == unique_status_codes_len_minus_one{
                true => {
                    is_last_element_found = true;
                    status_code_enums_try_from_variants.push(quote::quote! {
                        else {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#unexpected_status_code_initialization_token_stream);
                        }
                    });
                },
                false => {
                    if *desirable_status_code != status_code_attribute {
                        status_code_enums_try_from_variants.push(quote::quote! {
                            else if status_code == #http_status_code_token_stream {
                                match serde_json::from_str::<#try_operation_response_variants_desirable_attribute_token_stream>(&response_text) {
                                    Ok(value) => #try_operation_response_variants_upper_camel_case_token_stream::from(value),
                                    Err(#error_value_snake_case_token_stream) => {
                                        return Err(#try_operation_error_named_upper_camel_case_token_stream::#deserialize_response_initialization_token_stream);
                                    }
                                }
                            }
                        });
                    }
                },
            }
        });
        if !is_last_element_found {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} false = is_last_element_found");
        }
        status_code_enums_try_from_variants
    };
    quote::quote! {
        pub async fn #try_operation_snake_case_token_stream<'a>(
            #server_location_name_token_stream: #str_ref_token_stream,
            #parameters_snake_case_token_stream: #operation_parameters_upper_camel_case_token_stream,
        ) -> Result<#return_result_ok_type_token_stream, #try_operation_error_named_upper_camel_case_token_stream> {
            #payload_variable_initialization_token_stream
            let url = format!(
                #url_handle_token_stream,
                #server_location_name_token_stream
            );
            // println!("{}", url);
            let future = #reqwest_client_new_token_stream
                .#operation_http_method_snake_case_token_stream(&url)
                #commit_header_addition_token_stream
                #content_type_application_json_header_addition_token_stream
                .body(#payload_snake_case_token_stream)
                .send();
            let response = match future.await {
                Ok(response) => response,
                Err(#error_value_snake_case_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#reqwest_initialization_token_stream);
                }
            };
            let status_code = response.status();
            let headers = response.headers().clone();
            let response_text = match response.text().await {
                Ok(response_text) => response_text,
                Err(#error_value_snake_case_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#failed_to_get_response_text_initialization_token_stream);
                }
            };
            let variants = #(#status_code_enums_try_from)*;
            match #desirable_type_token_stream::try_from(variants) {
                Ok(value) => #ok_value_handle_token_stream,
                Err(#error_value_snake_case_token_stream) => {
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#expected_type_initialization_token_stream);
                },
            }
        }
    }
}

fn generate_type_variants_from_request_response_syn_variants<'a>(
    type_variants_from_request_response_syn_variants_partial: std::vec::Vec<&'a syn::Variant>,
    full_additional_http_status_codes_error_variants: std::vec::Vec<&'a (
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec<syn::Variant>,
    )>,
) -> std::vec::Vec<&'a syn::Variant> {
    let mut handle = std::vec::Vec::with_capacity(
        type_variants_from_request_response_syn_variants_partial.len()
            + full_additional_http_status_codes_error_variants.len(),
    );
    for element in &type_variants_from_request_response_syn_variants_partial {
        handle.push(*element);
    }
    for element in &full_additional_http_status_codes_error_variants {
        for element in &element.2 {
            handle.push(element);
        }
    }
    handle
}

fn generate_unique_status_codes(
    desirable_status_code: &proc_macro_helpers::status_code::StatusCode,
    type_variants_from_request_response_syn_variants: &[&syn::Variant],
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> std::vec::Vec<proc_macro_helpers::status_code::StatusCode> {
    let mut value =
        std::vec::Vec::with_capacity(type_variants_from_request_response_syn_variants.len());
    value.push(desirable_status_code.clone());
    type_variants_from_request_response_syn_variants.iter().for_each(|element|{
        let variant_ident = &element.ident;
        let error_variant_status_code = proc_macro_helpers::status_code::StatusCode::try_from(element)
        .unwrap_or_else(|e| {panic!("{proc_macro_name_upper_camel_case_ident_stringified} variant {variant_ident} failed: {e}")});
        match value.contains(&error_variant_status_code) {
            true => (),
            false => {
                value.push(error_variant_status_code);
            }
        }
    });
    value
}

fn generate_swagger_open_api_token_stream(
    table_name_stringified: &str,
    unique_status_codes: &[proc_macro_helpers::status_code::StatusCode],
    application_json_quotes_token_stream: &proc_macro2::TokenStream,
    table_name_quotes_token_stream: &proc_macro2::TokenStream,
    content_type_token_stream: &proc_macro2::TokenStream,
    operation: &Operation,
) -> proc_macro2::TokenStream {
    let swagger_url_path_quotes_token_stream = proc_macro_helpers::naming_conventions::SwaggerUrlPathSelfQuotesTokenStream::swagger_url_path_self_quotes_token_stream(operation, table_name_stringified);
    let content_type_snake_case_token_stream = quote::quote! {content_type};
    let description_snake_case_token_stream = quote::quote! {description};
    let responses_token_stream = unique_status_codes.iter().map(|element|{
        let status_token_stream = element.to_status_code_token_stream();
        let description_token_stream = element.to_status_code_description_token_stream();
        let body_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsStatusCodeTokenStream::try_self_response_variants_status_code_token_stream(operation, element);
        quote::quote!{
            (
                status = #status_token_stream,
                #description_snake_case_token_stream = #description_token_stream,
                body = #body_token_stream,
                #content_type_snake_case_token_stream = #application_json_quotes_token_stream
            )
        }
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let method_snake_case_token_stream =
        proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(
            &operation.http_method(),
        );
    let request_body_token_stream = {
        let request_body_description_token_stream = {
            let value_stringified = proc_macro_common::generate_quotes::generate_quotes_stringified(&format!(
                "{table_name_stringified} {} {}",
                proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(operation),
                proc_macro_helpers::naming_conventions::payload_snake_case_stringified()
            ));
            value_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        quote::quote! {
            request_body(
                content = #content_type_token_stream,
                #description_snake_case_token_stream = #request_body_description_token_stream,
                #content_type_snake_case_token_stream = #application_json_quotes_token_stream
            )
        }
    };
    let path_snake_case_token_stream =
        proc_macro_helpers::naming_conventions::path_snake_case_token_stream();
    quote::quote! {
        #[utoipa::path(
            #method_snake_case_token_stream,
            #path_snake_case_token_stream = #swagger_url_path_quotes_token_stream,
            operation_id = #swagger_url_path_quotes_token_stream,
            tag = #table_name_quotes_token_stream,
            #request_body_token_stream,
            responses(
                #(#responses_token_stream),*
            ),
        )]
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
    fn http_method(&self) -> OperationHttpMethod {
        match self {
            Self::CreateMany => OperationHttpMethod::Post,
            Self::CreateOne => OperationHttpMethod::Post,
            Self::ReadMany => OperationHttpMethod::Post,
            Self::ReadOne => OperationHttpMethod::Post,
            Self::UpdateMany => OperationHttpMethod::Patch,
            Self::UpdateOne => OperationHttpMethod::Patch,
            Self::DeleteMany => OperationHttpMethod::Delete,
            Self::DeleteOne => OperationHttpMethod::Delete,
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
//         let value_stringified = format!("{operation_name_snake_case_stringified}_test");
//         value_stringified.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
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
//                 Err(#error_value_snake_case_token_stream) => {
//                     panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {:#?}", #error_value_snake_case_token_stream)
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

fn generate_full_additional_http_status_codes_error_variants<'a>(
    common_middlewares_error_syn_variants: std::vec::Vec<&'a (
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec<syn::Variant>,
    )>,
    additional_http_status_codes_error_variants: std::vec::Vec<&'a (
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec<syn::Variant>,
    )>,
) -> std::vec::Vec<&'a (
    syn::Ident,
    proc_macro2::TokenStream,
    std::vec::Vec<syn::Variant>,
)> {
    let mut handle = std::vec::Vec::with_capacity(
        common_middlewares_error_syn_variants.len()
            + additional_http_status_codes_error_variants.len(),
    );
    for element in common_middlewares_error_syn_variants {
        handle.push(element);
    }
    for element in additional_http_status_codes_error_variants {
        handle.push(element);
    }
    handle
}

fn generate_std_string_string_error_syn_variant(
    variant_name_upper_camel_case_stringified: &str,
    status_code: proc_macro_helpers::status_code::StatusCode,
    code_occurence_field: &syn::Field,
    std_string_string_syn_punctuated_punctuated: syn::punctuated::Punctuated<
        syn::PathSegment,
        syn::token::Colon2,
    >,
) -> syn::Variant {
    let variant_name_snake_case_stringified =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &variant_name_upper_camel_case_stringified,
        );
    crate::type_variants_from_request_response_generator::construct_syn_variant(
        status_code,
        variant_name_upper_camel_case_stringified,
        code_occurence_field,
        vec![
            (
                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize,
                &variant_name_snake_case_stringified,
                std_string_string_syn_punctuated_punctuated
            )
        ]
    )
}

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

fn syn_ident_to_upper_camel_case_stringified(value: &syn::Ident) -> std::string::String {
    proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value.to_string())
}

fn generate_inner_type_from_or_try_from_inner_type_with_serialize_deserialize_error_variant_vec_token_stream(
    value: &std::vec::Vec<SynFieldWithAdditionalInfo>,
    code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream: &proc_macro2::TokenStream
) -> std::vec::Vec<proc_macro2::TokenStream> {
    value.iter().map(|element| {
        let no_error_variant_token_stream = proc_macro2::TokenStream::new();
        let rust_sqlx_map_to_postgres_type_variant = &element.rust_sqlx_map_to_postgres_type_variant;
        let error_variant_token_stream = {
            let postgresql_crud_common_supported_sqlx_postgres_type = postgresql_crud_common::SupportedSqlxPostgresType::from(rust_sqlx_map_to_postgres_type_variant);
            let postgresql_crud_common_supported_sqlx_postgres_type_snake_case_token_stream = {
                let value_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
                    &postgresql_crud_common_supported_sqlx_postgres_type
                );
                value_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let with_serialize_deserialize_error_named_token_stream = {
                let value_stringified = postgresql_crud_common_supported_sqlx_postgres_type.get_with_serialize_deserialize_error_named_stringified();
                value_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_ident_upper_camel_case_token_stream = {
                //todo its a temporal naming desicion. maybe it can be better
                let value_stringified = syn_ident_to_upper_camel_case_stringified(&element.field_ident);
                value_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #field_ident_upper_camel_case_token_stream {
                    #[eo_display]//todo - maybe need to support not only #[eo_display]
                    #postgresql_crud_common_supported_sqlx_postgres_type_snake_case_token_stream: #with_serialize_deserialize_error_named_token_stream,
                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                },//must use comma here
            }
        };
        match rust_sqlx_map_to_postgres_type_variant {
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBool => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveBoolAsPostgresqlBoolNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallInt => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallIntNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerial => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlSmallSerialNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2 => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI16AsPostgresqlInt2NotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlIntNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerial => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlSerialNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4 => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI32AsPostgresqlInt4NotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigInt => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigIntNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerial => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlBigSerialNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8 => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveI64AsPostgresqlInt8NotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlReal => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlRealNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4 => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF32AsPostgresqlFloat4NotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecision => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8 => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdPrimitiveF64AsPostgresqlFloat8NotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarchar => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlVarcharNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharN => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCharNNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlText => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlTextNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlName => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlNameNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiText => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdStringStringAsPostgresqlCiTextNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlBytea => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull => error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull => error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull => error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumeric => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBigDecimalAsPostgresqlNumericNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumeric => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesDecimalAsPostgresqlNumericNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDate => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTime => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull => error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull => error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull => error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDate => error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeDateAsPostgresqlDateNotNull => error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTime => error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesTimeTimeAsPostgresqlTimeNotNull => error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuid => error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNull => error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInet => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlInetNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidr => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::StdNetIpAddrAsPostgresqlCidrNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBit => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlBitNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBit => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesBitVecAsPostgresqlVarBitNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJson => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonB => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SqlxTypesJsonTAsPostgresqlJsonBNotNull => no_error_variant_token_stream,
    
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJson => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonNotNull => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonB => no_error_variant_token_stream,
            postgresql_crud_common::RustSqlxMapToPostgresTypeVariant::SerdeJsonValueAsPostgresqlJsonBNotNull => no_error_variant_token_stream,
        }
    }).collect()
}

#[derive(Debug)]
enum FromOrTryFrom {
    From,
    TryFrom,
}

trait RustSqlxMapToPostgresTypeVariantFromOrTryFromTokenStream {
    fn inner_type_from_or_try_from_inner_type_with_serialize_deserialize(&self) -> FromOrTryFrom;
}
impl RustSqlxMapToPostgresTypeVariantFromOrTryFromTokenStream for postgresql_crud_common::RustSqlxMapToPostgresTypeVariant {
    fn inner_type_from_or_try_from_inner_type_with_serialize_deserialize(&self) -> FromOrTryFrom {
        match self {
            Self::StdPrimitiveBoolAsPostgresqlBool => FromOrTryFrom::From,
            Self::StdPrimitiveBoolAsPostgresqlBoolNotNull => FromOrTryFrom::From,

            Self::StdPrimitiveI16AsPostgresqlSmallInt => FromOrTryFrom::From,
            Self::StdPrimitiveI16AsPostgresqlSmallIntNotNull => FromOrTryFrom::From,
            Self::StdPrimitiveI16AsPostgresqlSmallSerial => FromOrTryFrom::From,
            Self::StdPrimitiveI16AsPostgresqlSmallSerialNotNull => FromOrTryFrom::From,
            Self::StdPrimitiveI16AsPostgresqlInt2 => FromOrTryFrom::From,
            Self::StdPrimitiveI16AsPostgresqlInt2NotNull => FromOrTryFrom::From,

            Self::StdPrimitiveI32AsPostgresqlInt => FromOrTryFrom::From,
            Self::StdPrimitiveI32AsPostgresqlIntNotNull => FromOrTryFrom::From,
            Self::StdPrimitiveI32AsPostgresqlSerial => FromOrTryFrom::From,
            Self::StdPrimitiveI32AsPostgresqlSerialNotNull => FromOrTryFrom::From,
            Self::StdPrimitiveI32AsPostgresqlInt4 => FromOrTryFrom::From,
            Self::StdPrimitiveI32AsPostgresqlInt4NotNull => FromOrTryFrom::From,

            Self::StdPrimitiveI64AsPostgresqlBigInt => FromOrTryFrom::From,
            Self::StdPrimitiveI64AsPostgresqlBigIntNotNull => FromOrTryFrom::From,
            Self::StdPrimitiveI64AsPostgresqlBigSerial => FromOrTryFrom::From,
            Self::StdPrimitiveI64AsPostgresqlBigSerialNotNull => FromOrTryFrom::From,
            Self::StdPrimitiveI64AsPostgresqlInt8 => FromOrTryFrom::From,
            Self::StdPrimitiveI64AsPostgresqlInt8NotNull => FromOrTryFrom::From,

            Self::StdPrimitiveF32AsPostgresqlReal => FromOrTryFrom::From,
            Self::StdPrimitiveF32AsPostgresqlRealNotNull => FromOrTryFrom::From,
            Self::StdPrimitiveF32AsPostgresqlFloat4 => FromOrTryFrom::From,
            Self::StdPrimitiveF32AsPostgresqlFloat4NotNull => FromOrTryFrom::From,

            Self::StdPrimitiveF64AsPostgresqlDoublePrecision => FromOrTryFrom::From,
            Self::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull => FromOrTryFrom::From,
            Self::StdPrimitiveF64AsPostgresqlFloat8 => FromOrTryFrom::From,
            Self::StdPrimitiveF64AsPostgresqlFloat8NotNull => FromOrTryFrom::From,

            Self::StdStringStringAsPostgresqlVarchar => FromOrTryFrom::From,
            Self::StdStringStringAsPostgresqlVarcharNotNull => FromOrTryFrom::From,
            Self::StdStringStringAsPostgresqlCharN => FromOrTryFrom::From,
            Self::StdStringStringAsPostgresqlCharNNotNull => FromOrTryFrom::From,
            Self::StdStringStringAsPostgresqlText => FromOrTryFrom::From,
            Self::StdStringStringAsPostgresqlTextNotNull => FromOrTryFrom::From,
            Self::StdStringStringAsPostgresqlName => FromOrTryFrom::From,
            Self::StdStringStringAsPostgresqlNameNotNull => FromOrTryFrom::From,
            Self::StdStringStringAsPostgresqlCiText => FromOrTryFrom::From,
            Self::StdStringStringAsPostgresqlCiTextNotNull => FromOrTryFrom::From,

            Self::StdVecVecStdPrimitiveU8AsPostgresqlBytea => FromOrTryFrom::From,
            Self::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgIntervalAsPostgresqlInterval => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange => FromOrTryFrom::TryFrom,
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull => FromOrTryFrom::TryFrom,

            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange => FromOrTryFrom::TryFrom,
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull => FromOrTryFrom::TryFrom,

            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange => FromOrTryFrom::TryFrom,
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull => FromOrTryFrom::TryFrom,

            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoney => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiText => FromOrTryFrom::From,
            Self::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull => FromOrTryFrom::From,

            Self::SqlxTypesBigDecimalAsPostgresqlNumeric => FromOrTryFrom::From,
            Self::SqlxTypesBigDecimalAsPostgresqlNumericNotNull => FromOrTryFrom::From,

            Self::SqlxTypesDecimalAsPostgresqlNumeric => FromOrTryFrom::From,
            Self::SqlxTypesDecimalAsPostgresqlNumericNotNull => FromOrTryFrom::From,

            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz => FromOrTryFrom::From,
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull => FromOrTryFrom::From,

            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz => FromOrTryFrom::From,
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull => FromOrTryFrom::From,

            Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp => FromOrTryFrom::From,
            Self::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull => FromOrTryFrom::From,

            Self::SqlxTypesChronoNaiveDateAsPostgresqlDate => FromOrTryFrom::From,
            Self::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull => FromOrTryFrom::From,

            Self::SqlxTypesChronoNaiveTimeAsPostgresqlTime => FromOrTryFrom::From,
            Self::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull => FromOrTryFrom::From,

            Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz => FromOrTryFrom::TryFrom,
            Self::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull => FromOrTryFrom::TryFrom,

            Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp => FromOrTryFrom::TryFrom,
            Self::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull => FromOrTryFrom::TryFrom,

            Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz => FromOrTryFrom::TryFrom,
            Self::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull => FromOrTryFrom::TryFrom,

            Self::SqlxTypesTimeDateAsPostgresqlDate => FromOrTryFrom::TryFrom,
            Self::SqlxTypesTimeDateAsPostgresqlDateNotNull => FromOrTryFrom::TryFrom,

            Self::SqlxTypesTimeTimeAsPostgresqlTime => FromOrTryFrom::TryFrom,
            Self::SqlxTypesTimeTimeAsPostgresqlTimeNotNull => FromOrTryFrom::TryFrom,

            Self::SqlxTypesUuidUuidAsPostgresqlUuid => FromOrTryFrom::TryFrom,
            Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNull => FromOrTryFrom::TryFrom,
            Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey => FromOrTryFrom::TryFrom,

            Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet => FromOrTryFrom::From,
            Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull => FromOrTryFrom::From,
            Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr => FromOrTryFrom::From,
            Self::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull => FromOrTryFrom::From,

            Self::StdNetIpAddrAsPostgresqlInet => FromOrTryFrom::From,
            Self::StdNetIpAddrAsPostgresqlInetNotNull => FromOrTryFrom::From,
            Self::StdNetIpAddrAsPostgresqlCidr => FromOrTryFrom::From,
            Self::StdNetIpAddrAsPostgresqlCidrNotNull => FromOrTryFrom::From,

            Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr => FromOrTryFrom::From,
            Self::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull => FromOrTryFrom::From,

            Self::SqlxTypesBitVecAsPostgresqlBit => FromOrTryFrom::From,
            Self::SqlxTypesBitVecAsPostgresqlBitNotNull => FromOrTryFrom::From,
            Self::SqlxTypesBitVecAsPostgresqlVarBit => FromOrTryFrom::From,
            Self::SqlxTypesBitVecAsPostgresqlVarBitNotNull => FromOrTryFrom::From,

            Self::SqlxTypesJsonTAsPostgresqlJson => FromOrTryFrom::From,//todo
            Self::SqlxTypesJsonTAsPostgresqlJsonNotNull => FromOrTryFrom::From,//todo
            Self::SqlxTypesJsonTAsPostgresqlJsonB => FromOrTryFrom::From,//todo
            Self::SqlxTypesJsonTAsPostgresqlJsonBNotNull => FromOrTryFrom::From,//todo

            Self::SerdeJsonValueAsPostgresqlJson => FromOrTryFrom::From,
            Self::SerdeJsonValueAsPostgresqlJsonNotNull => FromOrTryFrom::From,
            Self::SerdeJsonValueAsPostgresqlJsonB => FromOrTryFrom::From,
            Self::SerdeJsonValueAsPostgresqlJsonBNotNull => FromOrTryFrom::From,
        }
    }
}
