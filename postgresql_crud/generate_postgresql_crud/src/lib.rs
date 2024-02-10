mod column_names_factorial;
mod check_for_none;
mod acquire_pool_and_connection;
mod from_log_and_return_error;
mod generate_postgres_transaction;
mod type_variants_from_request_response_generator;
mod extract_syn_variants_from_proc_macro_attribute;

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

#[proc_macro_derive(
    GeneratePostgresqlCrud,
    attributes(
        generate_postgresql_crud_primary_key,

        generate_postgresql_crud_bool,
        generate_postgresql_crud_char,
        generate_postgresql_crud_smallint,
        generate_postgresql_crud_smallserial,
        generate_postgresql_crud_int2,
        generate_postgresql_crud_int,
        generate_postgresql_crud_serial, 
        generate_postgresql_crud_int4,
        generate_postgresql_crud_bigint,
        generate_postgresql_crud_bigserial, 
        generate_postgresql_crud_int8,
        generate_postgresql_crud_real, 
        generate_postgresql_crud_float4,
        generate_postgresql_crud_double_precision,
        generate_postgresql_crud_float8,
        generate_postgresql_crud_varchar,
        generate_postgresql_crud_charn, //wtf????
        generate_postgresql_crud_text,
        generate_postgresql_crud_name,
        generate_postgresql_crud_bytea,
        generate_postgresql_crud_void,
        generate_postgresql_crud_interval,
        generate_postgresql_crud_int8range,
        generate_postgresql_crud_int4range,
        generate_postgresql_crud_tsrange,
        generate_postgresql_crud_tstzrange,
        generate_postgresql_crud_daterange,
        generate_postgresql_crud_numrange,
        generate_postgresql_crud_money,
        generate_postgresql_crud_ltree,
        generate_postgresql_crud_lquery,

        generate_postgresql_crud_numeric,

        generate_postgresql_crud_timestamptz,
        generate_postgresql_crud_timestamp,
        generate_postgresql_crud_date,
        generate_postgresql_crud_time,
        generate_postgresql_crud_timetz,
        generate_postgresql_crud_uuid,

        generate_postgresql_crud_inet,
        generate_postgresql_crud_cidr,

        generate_postgresql_crud_macaddr,

        generate_postgresql_crud_bit,
        generate_postgresql_crud_varbit,

        generate_postgresql_crud_json,
        generate_postgresql_crud_jsonb
    )
)]//todo check on postgresql max length value of type
pub fn generate_postgresql_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {//todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "PostgresqlCrud";
    let proc_macro_name_snake_case = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&proc_macro_name_upper_camel_case);
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {e}",
            proc_macro_common::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", ast.data);
    let ident = &ast.ident;
    let ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&ident.to_string());
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let table_name_stringified = pluralizer::pluralize(&ident_snake_case_stringified, 2, false);
    let table_name_quotes_token_stream = proc_macro_common::generate_quotes::generate_quotes_token_stream(
        &table_name_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let server_location_type_token_stream = quote::quote!{&str};
    let table_name_declaration_token_stream = quote::quote! {pub const TABLE_NAME: #server_location_type_token_stream = #table_name_quotes_token_stream;};
    let fields_named = if let syn::Data::Struct(data_struct) = &ast.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            &fields_named.named
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Named");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let primary_key_field = {
        let primary_key_attr_name = "generate_postgresql_crud_primary_key";
        let mut primary_key_field_option = None;
        for field_named in fields_named {
            let attrs = &field_named.attrs;
            if let 1 = attrs.len() {
                match attrs.first() {
                    Some(attr) => match proc_macro_helpers::error_occurence::generate_path_from_segments::generate_path_from_segments(&attr.path.segments) == primary_key_attr_name {
                        true => match primary_key_field_option {
                            Some(_) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} must have one primary_key attribute"),
                            None => {
                                primary_key_field_option = Some(field_named.clone());
                            },
                        },
                        false => (),
                    },
                    None => panic!("{proc_macro_name_upper_camel_case_ident_stringified} field_named.attrs.len() == 1, but attrs.get(0) == None"),
                }
            }
        }
        match primary_key_field_option {
            Some(value) => value,
            None => panic!("{proc_macro_name_upper_camel_case_ident_stringified} no {primary_key_attr_name} attribute"),
        }
    };
    let primary_key_field_type = &primary_key_field.ty;
    // println!("{primary_key_field:#?}");
    let sqlx_types_uuid_stringified = naming_constants::SQLX_TYPES_UUID_STRINGIFIED;
    let sqlx_types_uuid_token_stream = {
        sqlx_types_uuid_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {sqlx_types_uuid_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    {
        let primary_key_field_ident_handle = quote::quote!{#primary_key_field_type}.to_string().replace(' ', "");
        if primary_key_field_ident_handle != sqlx_types_uuid_stringified {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} primary_key is not type {sqlx_types_uuid_stringified}");
        }
    }
    let primary_key_field_ident = primary_key_field.ident.as_ref()
        .unwrap_or_else(|| {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} primary_key_field.ident is None")
        });
    let std_string_string_token_stream = quote::quote!{std::string::String};
    let field_ident_is_none_stringified = naming_constants::FIELD_IDENT_IS_NONE;
    let fields_named_wrappers_excluding_primary_key = fields_named.clone().into_iter().filter(|field|*field != primary_key_field).map(|element|{
        let field_ident = element.ident
            .as_ref()
            .unwrap_or_else(|| {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
            });
        let attrs = &element.attrs;
        let (
            supported_attribute_type,
            supported_field_type
        ) = match attrs.iter().fold(None, |mut acc, element| {
            let generated_path = proc_macro_helpers::error_occurence::generate_path_from_segments::generate_path_from_segments(&element.path.segments);
            let supported_attribute_type = {
                use std::str::FromStr;
                SupportedAttributeType::from_str(&generated_path)
            };
            match supported_attribute_type {
                Ok(value) => match acc {
                    Some(acc_value) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} supported only one attribute per field, detected both: {acc_value} and {value}"),
                    None => {
                        acc = Some(value);
                    }
                },
                Err(e) => panic!("{proc_macro_name_upper_camel_case_ident_stringified} SupportedAttributeType::from_str {generated_path} error: {e}")
            }
            acc
        }) {
            Some(supported_attribute_type) => {
                let ty = &element.ty;
                let ty_stringified = quote::quote!{#ty}.to_string().replace(' ', "");
                let supported_field_type = {
                    use std::str::FromStr;
                    SupportedFieldType::from_str(ty_stringified.as_str()).unwrap_or_else(|_| panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} {ty_stringified} SupportedFieldType::try_from failed. supported: {:?}", 
                        SupportedFieldType::into_array().into_iter().map(|element|element.to_string()).collect::<std::vec::Vec<std::string::String>>()
                    ))
                };
                match try_match_supported_attribute_type_with_supported_field_type(&supported_attribute_type, &supported_field_type) {
                    true => (
                        supported_attribute_type,
                        supported_field_type
                    ),
                    false => panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} supported_attribute_type {supported_attribute_type} is not matching to supported_field_type {supported_field_type}, see https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html", 
                    )
                }
            }
            None => panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} no field attribute found for {field_ident}, supported: {:?}", 
                SupportedAttributeType::into_array().into_iter().map(|element|element.to_string()).collect::<std::vec::Vec<std::string::String>>()
            )
        };
        FieldNamedWrapperExcludingPrimaryKey {
            field: element,
            supported_attribute_type,
            supported_field_type
        }
    }).collect::<std::vec::Vec<FieldNamedWrapperExcludingPrimaryKey>>();
    let fields_named_len = fields_named.len();
    if fields_named_len <= 1 {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} false = fields_named.len() > 1" );
    }
    let fields_named_wrappers_excluding_primary_key_len = fields_named_wrappers_excluding_primary_key.len();
    let primary_key_field_ident_quotes_token_stream = proc_macro_common::generate_quotes::generate_quotes_token_stream(
        &primary_key_field_ident.to_string(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let debug_token_stream = quote::quote!{Debug};
    let thiserror_error_token_stream = quote::quote!{thiserror::Error};
    let error_occurence_error_occurence_token_stream = quote::quote!{error_occurence_lib::ErrorOccurence};
    let utoipa_to_schema_token_stream = quote::quote!{utoipa::ToSchema};
    let serde_serialize_token_stream = quote::quote!{serde::Serialize};
    let serde_deserialize_token_stream = quote::quote!{serde::Deserialize};
    let derive_debug_token_stream = quote::quote!{#[derive(#debug_token_stream)]};
    let derive_debug_thiserror_error_occurence_token_stream = quote::quote!{#[derive(#debug_token_stream, #thiserror_error_token_stream, #error_occurence_error_occurence_token_stream)]};
    let derive_debug_to_schema_token_stream = quote::quote!{#[derive(#debug_token_stream, #utoipa_to_schema_token_stream)]};
    let derive_debug_serialize_deserialize_token_stream = quote::quote!{#[derive(#debug_token_stream, #serde_serialize_token_stream, #serde_deserialize_token_stream)]};
    let derive_debug_serialize_deserialize_to_schema_token_stream = quote::quote!{#[derive(#debug_token_stream, #serde_serialize_token_stream, #serde_deserialize_token_stream, #utoipa_to_schema_token_stream)]};
    let try_from_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::try_from_upper_camel_case_stringified();
    let from_str_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::from_str_upper_camel_case_stringified();
    let from_str_upper_camel_case_token_stream = {
        from_str_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {from_str_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let from_str_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&from_str_upper_camel_case_stringified.to_string());
    let from_str_snake_case_token_stream = {
        from_str_snake_case_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {from_str_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let sqlx_row_token_stream = quote::quote!{sqlx::Row};
    let std_primitive_str_sqlx_column_index_token_stream = quote::quote!{&'a std::primitive::str: sqlx::ColumnIndex<R>,};
    let sqlx_decode_decode_database_token_stream = quote::quote!{sqlx::decode::Decode<'a, R::Database>};
    let sqlx_types_type_database_token_stream = quote::quote!{sqlx::types::Type<R::Database>};
    let primary_key_uuid_wrapper_try_from_sqlx_row_name_token_stream = quote::quote!{primary_key_uuid_wrapper_try_from_sqlx_row};
    let crate_server_postgres_uuid_wrapper_token_stream = quote::quote!{crate::server::postgres::uuid_wrapper};
    let error_named_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::error_named_upper_camel_case_stringified();
    let uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_stringified = format!("UuidWrapper{try_from_upper_camel_case_stringified}PossibleUuidWrapper");
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
    let crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream = quote::quote!{#crate_server_postgres_uuid_wrapper_token_stream::#uuid_wrapper_try_from_possible_uuid_wrapper_error_named_upper_camel_case_token_stream};
    let crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream = quote::quote!{#crate_server_postgres_uuid_wrapper_token_stream::UuidWrapper};
    let std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream = quote::quote!{std::vec::Vec<#crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream>};
    let crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream = quote::quote!{#crate_server_postgres_uuid_wrapper_token_stream::PossibleUuidWrapper};
    let std_str_from_str_token_stream = quote::quote!{std::str::#from_str_upper_camel_case_token_stream};
    let struct_options_ident_stringified = format!(
        "{ident}{}",
        proc_macro_helpers::naming_conventions::options_upper_camel_case_stringified()
    );
    let struct_options_ident_token_stream = {
        struct_options_ident_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {struct_options_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let struct_options_token_stream = {
        let serde_skip_serializing_if_value_attribute_token_stream = quote::quote!{#[serde(skip_serializing_if = "Option::is_none")]};
        let field_option_primary_key_token_stream = quote::quote!{
            #serde_skip_serializing_if_value_attribute_token_stream
            pub #primary_key_field_ident: std::option::Option<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>
        };
        let fields_options_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| {
            let field_vis = &element.field.vis;
            let field_ident = &element.field.ident;
            let field_type_path = &element.field.ty;
            quote::quote!{
                #serde_skip_serializing_if_value_attribute_token_stream
                #field_vis #field_ident: std::option::Option<#field_type_path>
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
    let from_ident_for_ident_options_token_stream = {
        let ident_option_variant_primary_key_token_stream = quote::quote!{
            #primary_key_field_ident: Some(value.#primary_key_field_ident.into()),
        };
        let ident_option_variants_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| {
            let field_ident = element.field.ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                });
            quote::quote!{
                #field_ident: Some(value.#field_ident.into())//todo what if type does not implement serialize deserialize
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
    let column_variants = {
        let fields_named_enumerated = fields_named
            .iter()
            .enumerate()
            .collect::<std::vec::Vec<(usize, &syn::Field)>>();
        let fields_named_clone_stringified = fields_named.iter().collect::<std::vec::Vec<&syn::Field>>();
        let mut veced_vec = fields_named_clone_stringified
            .iter()
            .map(|field| vec![(*field).clone()])
            .collect::<std::vec::Vec<std::vec::Vec<syn::Field>>>();
        crate::column_names_factorial::column_names_factorial(
            fields_named_enumerated,
            fields_named_clone_stringified,
            &mut veced_vec as &mut [std::vec::Vec<syn::Field>],
            &proc_macro_name_upper_camel_case_ident_stringified
        )
    };
    let structs_variants_token_stream = {
        column_variants
            .iter()
            .map(|variant_columns| {
                let struct_name_token_stream = {
                    let mut struct_name_stringified = format!("{ident}");
                    variant_columns.iter().for_each(|variant_column| {
                        use convert_case::Casing;
                        let column_title_cased = variant_column.ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            })
                            .to_string().to_case(convert_case::Case::Title);
                        struct_name_stringified.push_str(&column_title_cased);
                    });
                    struct_name_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {struct_name_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let genereted_fields = variant_columns.iter().map(|variant_column|{
                    let variant_column_ident = variant_column.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    let variant_column_type = &variant_column.ty;
                    quote::quote! {
                        pub #variant_column_ident: #variant_column_type,
                    }
                });
                quote::quote! {
                    #derive_debug_token_stream
                    pub struct #struct_name_token_stream {
                        #(#genereted_fields)*
                    }
                }
            })
            .collect::<std::vec::Vec<proc_macro2::TokenStream>>()
    };
    let code_occurence_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::code_occurence_upper_camel_case_token_stream();
    let code_occurence_snake_case_token_stream = proc_macro_helpers::naming_conventions::code_occurence_snake_case_token_stream();
    let error_occurence_lib_code_occurence_code_occurence_token_stream = quote::quote!{error_occurence_lib::#code_occurence_snake_case_token_stream::#code_occurence_upper_camel_case_token_stream};
    let code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream = quote::quote!{
        #code_occurence_snake_case_token_stream: #error_occurence_lib_code_occurence_code_occurence_token_stream
    };
    let eo_error_occurence_attribute_token_stream = proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence.to_attribute_view_token_stream();
    let eo_display_token_stream = proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay.to_attribute_view_token_stream();
    let eo_display_with_serialize_deserialize_token_stream = proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize.to_attribute_view_token_stream();
    let eo_display_foreign_type_token_stream = proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayForeignType.to_attribute_view_token_stream();
    let eo_vec_error_occurence_token_stream = proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecErrorOccurence.to_attribute_view_token_stream();
    // let value_token_stream = quote::quote! {value};
    let impl_std_convert_try_from_ident_options_for_struct_variants_token_stream = {
        column_variants
            .iter()
            .map(|variant_columns| {
                let struct_name_stringified = {
                    let variant_columns_merged_upper_camel_case_stringified = variant_columns.iter().fold(std::string::String::from(""), |mut acc, element| {
                        use convert_case::Casing;
                        let column_title_cased = element.ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            })
                            .to_string().to_case(convert_case::Case::Title);
                        acc.push_str(&column_title_cased);
                        acc
                    });
                    format!("{ident}{variant_columns_merged_upper_camel_case_stringified}")
                };
                let struct_name_token_stream = {
                    struct_name_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {struct_name_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let self_fields_token_stream = generate_self_fields_token_stream(
                    &variant_columns.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
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
                    let uuid_wrapper_try_from_possible_uuid_wrapper_primary_key_variant_token_stream = match variant_columns.iter().find(|element| {
                        element.ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            })
                            == primary_key_field_ident
                    }) {
                        Some(value) => {
                            let column_variant_ident_stringified = value.ident.as_ref().unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
                            match column_variant_ident_stringified == primary_key_field_ident {
                                true => quote::quote!{
                                    #uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_token_stream {
                                        #eo_error_occurence_attribute_token_stream
                                        #uuid_wrapper_try_from_possible_uuid_wrapper_snake_case_token_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                        #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                                    },
                                },
                                false => proc_macro2::TokenStream::new()
                            }
                        },
                        None => proc_macro2::TokenStream::new()  
                    };
                    let is_none_variant_columns_token_stream = variant_columns.iter().map(|element|{
                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                        let field_ident_title_case_stringified = {
                            use convert_case::Casing;
                            field_ident.to_string().to_case(convert_case::Case::Title)
                        };
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
                            #uuid_wrapper_try_from_possible_uuid_wrapper_primary_key_variant_token_stream
                            #(#is_none_variant_columns_token_stream),*
                        }
                    }
                };
                let impl_std_convert_try_from_ident_options_ident_token_stream = {
                    let primary_key_field_assignment_token_stream = match variant_columns.iter().find(|element| {
                        element.ident
                            .as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            })
                            == primary_key_field_ident
                    }) {
                        Some(value) => {
                            let column_variant_ident_stringified = value.ident.as_ref().unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
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
                                            Some(value) => match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value) {
                                                Ok(value) => value.into_inner(),
                                                Err(e) => {
                                                    return Err(Self::Error::#uuid_wrapper_try_from_possible_uuid_wrapper_upper_camel_case_token_stream {
                                                        #uuid_wrapper_try_from_possible_uuid_wrapper_snake_case_token_stream: e,
                                                        #field_code_occurence_new_ea56ed9e_86e6_4b3e_b116_106eb0bca826_token_stream,
                                                    });
                                                }
                                            },
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
                    let variant_columns_assignment_token_stream = variant_columns.iter().filter(|element|{
                        element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        })
                         != primary_key_field_ident
                    }).map(|element|{
                        let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                        let field_ident_title_case_stringified = {
                            use convert_case::Casing;
                            field_ident.to_string().to_case(convert_case::Case::Title)
                        };
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
                    use convert_case::Casing;
                    let variant_ident_stringified = field_ident_stringified.to_case(convert_case::Case::Title);
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
        let ident_column_select_upper_camel_case_stringified = format!("{ident}{column_select_upper_camel_case_stringified}");
        ident_column_select_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_column_select_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let options_try_from_sqlx_row_name_token_stream = quote::quote!{options_try_from_sqlx_row};
    let crate_common_serde_urlencoded_serde_url_encoded_parameter_token_stream = quote::quote!{crate::common::serde_urlencoded::SerdeUrlencodedParameter};
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
                            use convert_case::Casing;
                            let field_ident_stringified = field.ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                }).to_string().to_case(convert_case::Case::Title);
                            acc.push_str(&field_ident_stringified);
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
                            let field_ident_stringified = field.ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                });
                            acc.push_str(&format!("{field_ident_stringified},"));
                            acc
                        });
                    write_ident_stringified_handle.pop();
                    format!("\"{write_ident_stringified_handle}\"").parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {write_ident_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let variant_ident_token_stream = {
                    let variant_ident_stringified_handle = column_variant.iter()
                        .fold(std::string::String::default(), |mut acc, field| {
                            use convert_case::Casing;
                            let field_ident_stringified = field.ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                }).to_string().to_case(convert_case::Case::Title);
                            acc.push_str(&field_ident_stringified);
                            acc
                        });
                    variant_ident_stringified_handle.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {variant_ident_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    Self::#variant_ident_token_stream => #std_string_string_token_stream::from(#write_ident_token_stream)
                }
            });
            quote::quote!{
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
                    use convert_case::Casing;
                    let field_ident_stringified = field.ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        }).to_string().to_case(convert_case::Case::Title);
                    acc.push_str(&field_ident_stringified);
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
        let not_correct_value_token_stream = quote::quote!{not_correct_value};
        let supported_values_token_stream = quote::quote!{supported_values};
        let not_correct_upper_camel_case_token_stream = quote::quote!{NotCorrect};
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
                    use convert_case::Casing;
                    let field_ident_stringified = field.ident
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        }).to_string().to_case(convert_case::Case::Title);
                    acc.push_str(&field_ident_stringified);
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
                let mut column_variants_stringified = column_variants.iter().fold(std::string::String::default(), |mut acc, column_variant| {
                    let variant_ident_stringified_handle = column_variant.iter()
                        .fold(std::string::String::default(), |mut acc, field| {
                            use convert_case::Casing;
                            let field_ident_stringified = field.ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                }).to_string().to_case(convert_case::Case::Title);
                            acc.push_str(&field_ident_stringified);
                            acc
                        });
                    acc.push_str(&format!("\\\"{variant_ident_stringified_handle}\\\","));
                    acc
                });
                column_variants_stringified.pop();
                let supported_values_handle_stringified = format!("\"{column_variants_stringified}\"");
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
                    fn from_str(value: #server_location_type_token_stream) -> Result<Self, Self::Err> {
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
                quote::quote! {
                    let mut #primary_key_field_ident: std::option::Option<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream> = None;
                }
            };
            let declaration_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                let field_ident = element.field.ident
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                    });
                let field_type = &element.field.ty;
                quote::quote! {
                    let mut #field_ident: std::option::Option<#field_type> = None;
                }
            });
            let assignment_token_stream = column_variants.iter().map(|column_variant|{
                let write_ident_primary_key_token_stream = {
                    quote::quote!{
                        let primary_key_try_get_result: Result<std::option::Option<#sqlx_types_uuid_token_stream>, sqlx::Error> = row.try_get(#primary_key_field_ident_quotes_token_stream);
                        #primary_key_field_ident = match primary_key_try_get_result {
                            Ok(option_primary_key) => option_primary_key.map(|value| #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value)),
                            Err(e) => {
                                return Err(e); //todo custom type
                            }
                        };
                    }
                };
                let write_ident_token_stream = column_variant.iter().filter_map(|field|match field == &primary_key_field {
                    true => None,
                    false => {
                        let field_ident = field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                        let field_ident_string_quotes_token_stream = proc_macro_common::generate_quotes::generate_quotes_token_stream(
                            &field_ident.to_string(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        Some(quote::quote!{
                            #field_ident = row.try_get(#field_ident_string_quotes_token_stream)?;
                        })
                    },
                });
                let variant_ident_token_stream = {
                    let variant_ident_stringified_handle = column_variant.iter()
                        .fold(std::string::String::default(), |mut acc, field| {
                            use convert_case::Casing;
                            let field_ident_stringified = field.ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                }).to_string().to_case(convert_case::Case::Title);
                            acc.push_str(&field_ident_stringified);
                            acc
                        });
                    variant_ident_stringified_handle.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {variant_ident_stringified_handle} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                quote::quote! {
                    Self::#variant_ident_token_stream => {
                        #write_ident_primary_key_token_stream
                        #(#write_ident_token_stream)*
                    }
                }
            });
            let option_fields_initiation_token_stream = generate_self_fields_token_stream(
                &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let sqlx_decode_decode_and_sqlx_types_type_primary_key_token_stream = quote::quote!{
                std::option::Option<#sqlx_types_uuid_token_stream>: #sqlx_decode_decode_database_token_stream,
                std::option::Option<#sqlx_types_uuid_token_stream>: #sqlx_types_type_database_token_stream,
            };
            let sqlx_decode_decode_and_sqlx_types_type_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| {
                let field_type = &element.field.ty;
                quote::quote!{
                    std::option::Option<#field_type>: #sqlx_decode_decode_database_token_stream,
                    std::option::Option<#field_type>: #sqlx_types_type_database_token_stream,
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
    let crate_server_postgres_regex_filter_regex_filter_token_stream = quote::quote!{crate::server::postgres::regex_filter::RegexFilter};
    let crate_server_postgres_postgres_bigint_postgres_bigint_token_stream = quote::quote!{crate::server::postgres::postgres_bigint::PostgresBigint};
    let primary_key_uuid_wrapper_try_from_sqlx_row_token_stream = {
        let primary_key_str_token_stream = {
            let primary_key_str_stringified = format!("\"{primary_key_field_ident}\"");
            primary_key_str_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {primary_key_str_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let row_name_token_stream = quote::quote!{row};
        let primary_key_name_token_stream = quote::quote!{primary_key};
        quote::quote! {
            fn #primary_key_uuid_wrapper_try_from_sqlx_row_name_token_stream<'a, R: #sqlx_row_token_stream>(#row_name_token_stream: &'a R) -> sqlx::Result<#crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream>
            where
                #std_primitive_str_sqlx_column_index_token_stream
                #sqlx_types_uuid_token_stream: #sqlx_decode_decode_database_token_stream,
                #sqlx_types_uuid_token_stream: #sqlx_types_type_database_token_stream,
            {
                let #primary_key_name_token_stream: #sqlx_types_uuid_token_stream = #row_name_token_stream.try_get(#primary_key_str_token_stream)?;
                Ok(#crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::from(#primary_key_name_token_stream))
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
    let order_by_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&order_by_upper_camel_case_stringified);
    let order_by_upper_camel_case_token_stream = {
        order_by_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {order_by_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let order_by_snake_case_token_stream = {
        order_by_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {order_by_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let crate_server_postgres_order_by_order_by_token_stream = quote::quote!{crate::server::postgres::#order_by_snake_case_token_stream::#order_by_upper_camel_case_token_stream};
    let crate_server_postgres_order_order_token_stream = quote::quote!{crate::server::postgres::order::Order};
    let limit_snake_case_token_stream = proc_macro_helpers::naming_conventions::limit_snake_case_token_stream();
    let offset_snake_case_token_stream = proc_macro_helpers::naming_conventions::offset_snake_case_token_stream();
    let order_snake_case_token_stream = proc_macro_helpers::naming_conventions::order_snake_case_token_stream();
    let column_snake_case_token_stream = proc_macro_helpers::naming_conventions::column_snake_case_token_stream();
    let ident_order_by_wrapper_stringified = format!("{ident}{order_by_upper_camel_case_stringified}Wrapper");
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
            let deserialize_ident_order_by_snake_case_name = format!("deserialize_{ident_snake_case_stringified}_order_by");
            deserialize_ident_order_by_snake_case_name.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {deserialize_ident_order_by_snake_case_name} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        quote::quote!{
            fn #deserialize_ident_order_by_snake_case_name_token_stream<'de, D>(
                deserializer: D,
            ) -> Result<#crate_server_postgres_order_by_order_by_token_stream<#ident_column_upper_camel_case_token_stream>, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let string_deserialized = {
                    use #serde_deserialize_token_stream;
                    std::string::String::deserialize(deserializer)?
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
                                            Err(e) => {
                                                return Err(serde::de::Error::custom(&format!(
                                                    "{default_message} {column_equal_str} {e}"
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
                                        Err(e) => {
                                            return Err(serde::de::Error::custom(&format!(
                                                "{default_message} {column_equal_str} {e}"
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
                                            Err(e) => {
                                                return Err(serde::de::Error::custom(&format!(
                                                    "{default_message} {order_equal_str} {e}"
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
                                        Err(e) => {
                                            return Err(serde::de::Error::custom(&format!(
                                                "{default_message} {order_equal_str} {e}"
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
            let deserialize_with_name_quotes_token_stream = proc_macro_common::generate_quotes::generate_quotes_token_stream(
                &format!("deserialize_{ident_snake_case_stringified}_order_by"),
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote!{
                #derive_debug_serialize_deserialize_token_stream
                pub struct #ident_order_by_wrapper_name_token_stream(
                    #[serde(deserialize_with = #deserialize_with_name_quotes_token_stream)]
                    pub #crate_server_postgres_order_by_order_by_token_stream<#ident_column_upper_camel_case_token_stream>,
                );
            }
        };
        let impl_crate_common_serde_urlencoded_serde_urlencoded_parameter_for_ident_order_by_wrapper_token_stream = {
            quote::quote!{
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
            quote::quote!{
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
            quote::quote!{
                impl #std_str_from_str_token_stream for #ident_order_by_wrapper_name_token_stream {
                    type Err = #ident_order_by_wrapper_from_str_error_named_name_token_stream;
                    fn from_str(value: #server_location_type_token_stream) -> Result<Self, Self::Err> {
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
                                                    Err(e) => {
                                                        return Err(Self::Err::ColumnFromStr {
                                                            column_from_str: e,
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
                                                Err(e) => {
                                                    return Err(Self::Err::ColumnFromStr {
                                                        column_from_str: e,
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
                                                    Err(e) => {
                                                        return Err(Self::Err::OrderFromStr {
                                                            order_from_str: e,
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
                                                Err(e) => {
                                                    return Err(Self::Err::OrderFromStr {
                                                        order_from_str: e,
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
        quote::quote!{
            #struct_token_stream
            #impl_crate_common_serde_urlencoded_serde_urlencoded_parameter_for_ident_order_by_wrapper_token_stream
            #ident_order_by_wrapper_from_str_error_named_token_stream
            #impl_std_str_from_str_for_ident_order_by_wrapper_token_stream
        }
    };
    // println!("{order_by_wrapper_token_stream}");
    let allow_methods_token_stream = {
        quote::quote!{
            pub const ALLOW_METHODS: [http::Method;4] = [http::Method::GET, http::Method::POST, http::Method::PATCH, http::Method::DELETE];//todo new axum version does not support it or something - find out
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
        quote::quote!{
            pub struct #ident_column_read_permission_name_token_stream {
                #(#fields_permission_token_stream),*
            }
        }
    };
    let reference_api_location_test_token_stream = quote::quote!{&api_location};
    let additional_http_status_codes_error_variants_snake_case_stringified = "additional_http_status_codes_error_variants";
    let common_middlewares_error_syn_variants = crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_proc_macro_attribute(
        &ast,
        additional_http_status_codes_error_variants_snake_case_stringified,
        &proc_macro_name_snake_case,
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let common_middlewares_error_syn_variants_len = common_middlewares_error_syn_variants.len();
    let extraction_result_snake_case_stringified = "extraction_result";
    let parameters_snake_case_token_stream = proc_macro_helpers::naming_conventions::parameters_snake_case_token_stream();
    let payload_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::payload_upper_camel_case_stringified();
    let payload_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&payload_upper_camel_case_stringified);
    let payload_snake_case_token_stream = payload_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {payload_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let payload_extraction_result_snake_case_token_stream = {
        let payload_extraction_result_snake_case = format!("{payload_snake_case_token_stream}_{extraction_result_snake_case_stringified}");
        payload_extraction_result_snake_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {payload_extraction_result_snake_case} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let non_existing_primary_keys_name_token_stream = quote::quote!{non_existing_primary_keys};
    let expected_updated_primary_keys_name_token_stream = quote::quote!{expected_updated_primary_keys};
    let use_futures_try_stream_ext_token_stream = quote::quote!{use futures::TryStreamExt};
    let serde_json_to_string_token_stream = quote::quote!{serde_json::to_string};
    // let payload_element_upper_camel_case_stringified = format!("{payload_upper_camel_case_stringified}Element");
    let request_error_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::request_error_upper_camel_case_stringified();
    let returning_stringified = "returning";
    let returning_primary_key_stringified = format!(" {returning_stringified} {primary_key_field_ident}");
    let primary_key_vec_name_token_stream = quote::quote!{primary_key_vec};
    let rollback_error_name_token_stream = quote::quote!{rollback_error};
    let returning_primary_key_quotes_token_stream = proc_macro_common::generate_quotes::generate_quotes_token_stream(
        &returning_primary_key_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let request_error_upper_camel_case_token_stream = request_error_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {request_error_upper_camel_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let request_error_snake_case_token_stream = {
        let request_error_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&request_error_upper_camel_case_stringified);
        request_error_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {request_error_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    // let path_to_crud = "crate::repositories_types::server::routes::api::cats::";
    let app_state_path = quote::quote!{postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync};//todo path
    let app_state_name_token_stream = quote::quote!{app_state};
    let error_log_call_token_stream = quote::quote!{
        error_occurence_lib::error_log::ErrorLog::error_log(
            &error,
            #app_state_name_token_stream.as_ref(),
        );
    };
    let request_error_variant_initialization_token_stream = {
        let field_code_occurence_new_9758f49b_3415_42fe_b2c7_7439c2c4f586_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            #request_error_upper_camel_case_token_stream {
                #request_error_snake_case_token_stream: e,
                #field_code_occurence_new_9758f49b_3415_42fe_b2c7_7439c2c4f586_token_stream,
            }
        }
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
        quote::quote!{
            #serde_json_to_string_upper_camel_case_token_stream {
                #serde_json_to_string_snake_case_token_stream: e,
                #field_code_occurence_new_27b49c75_24b2_4480_ac4d_62a1f75f5646_token_stream,
            }
        }
    };
    let http_request_error_named_serde_json_to_string_variant_token_stream = quote::quote!{
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
    let std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_syn_punctuated_punctuated = generate_std_vec_vec_syn_punctuated_punctuated(
        &["crate","server","postgres","uuid_wrapper","UuidWrapper"],
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let std_vec_vec_crate_server_postgres_regex_filter_regex_filter_syn_punctuated_punctuated = generate_std_vec_vec_syn_punctuated_punctuated(
        &["crate","server","postgres","regex_filter","RegexFilter"],
        &proc_macro_name_upper_camel_case_ident_stringified
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
                    &["crate","server","postgres","bind_query","TryGenerateBindIncrementsErrorNamed"],
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
        quote::quote!{
            BindQuery { 
                bind_query: e.into_serialize_deserialize_version(), 
                #field_code_occurence_new_d61d7616_3336_43be_aaa8_2144ff2d2158_token_stream
            }
        }
    };
    let checked_add_syn_variant = generate_std_string_string_error_syn_variant(
        "CheckedAdd",
        proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError,
        &code_occurence_field,
        std_string_string_syn_punctuated_punctuated.clone()
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
        quote::quote!{
            #checked_add_upper_camel_case_token_stream { //todo remove it? refactor it?
                #checked_add_snake_case_token_stream: #std_string_string_token_stream::from("checked_add is None"), 
                #field_code_occurence_new_9afdf71d_e375_455f_87a3_a16947625a7a_token_stream, 
            }
        }
    };
    let query_and_rollback_failed_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
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
            )   
        ]
    );
    let query_and_rollback_failed_variant_initialization_token_stream = {
        let field_code_occurence_new_254f2939_bca7_4b8a_b737_cd9bbbbdd5df_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            QueryAndRollbackFailed {
                query_error: e,
                #rollback_error_name_token_stream,
                #field_code_occurence_new_254f2939_bca7_4b8a_b737_cd9bbbbdd5df_token_stream,
            }
        }
    };
    let primary_key_from_row_and_failed_rollback_syn_variant = crate::type_variants_from_request_response_generator::construct_syn_variant(
        proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError,
        "PrimaryKeyFromRowAndFailedRollback",
        &code_occurence_field,
        vec![
            (
                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay, 
                "primary_key_from_row",
                sqlx_error_syn_punctuated_punctuated.clone() 
            ),
            (
                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay, 
                "rollback_error", 
                sqlx_error_syn_punctuated_punctuated.clone()
            )
        ]
    );
    let primary_key_from_row_and_failed_rollback_variant_initialization_token_stream = {
        let field_code_occurence_new_494adabc_50aa_4d57_acc8_4a0444df7d28_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            PrimaryKeyFromRowAndFailedRollback {
                primary_key_from_row: e,
                #rollback_error_name_token_stream,
                #field_code_occurence_new_494adabc_50aa_4d57_acc8_4a0444df7d28_token_stream,
            }
        }
    };
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
    let non_existing_primary_keys_variant_initialization_token_stream = {
        let field_code_occurence_new_4853d33a_b7e0_45df_8024_98ba66d26973_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            NonExistingPrimaryKeys {
                #non_existing_primary_keys_name_token_stream,
                #field_code_occurence_new_4853d33a_b7e0_45df_8024_98ba66d26973_token_stream,
            }
        }
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
    let non_existing_primary_keys_and_failed_rollback_variant_initialization_token_stream = {
        let field_code_occurence_new_5e07939c_0aa6_4f48_9f1f_5d3866c651ab_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            NonExistingPrimaryKeysAndFailedRollback {
                #non_existing_primary_keys_name_token_stream,
                #rollback_error_name_token_stream: e,
                #field_code_occurence_new_5e07939c_0aa6_4f48_9f1f_5d3866c651ab_token_stream,
            }
        }
    };
    let commit_failed_syn_variant = {
        let variant_name_upper_camel_case_stringified = "CommitFailed";
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::type_variants_from_request_response_generator::construct_syn_variant(
            proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError,
            variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay, 
                    &variant_name_snake_case_stringified, 
                    sqlx_error_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let commit_failed_variant_initialization_token_stream = {
        let field_code_occurence_new_52fad21a_c2cd_40f2_85af_dfec05be9d22_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            CommitFailed {
                commit_failed: e,
                #field_code_occurence_new_52fad21a_c2cd_40f2_85af_dfec05be9d22_token_stream,
            }
        }
    };
    let not_unique_primary_keys_name_token_stream = quote::quote!{not_unique_primary_keys};
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
        quote::quote!{
            NotUniquePrimaryKeys {
                not_unique_primary_keys,
                #field_code_occurence_new_0a70da64_9e15_4760_9656_14961b286f36_token_stream,
            }
        }
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_upper_camel_case_stringified = "OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapper";
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
    );//todo reuse it
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
            proc_macro_helpers::status_code::StatusCode::Tvfrr500InternalServerError,//todo - is it right status code for this case?
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay, 
                    &variant_name_snake_case_stringified, 
                    sqlx_error_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream = {
        let field_code_occurence_new_3567ece5_74c9_4b48_a46c_8230cd728182_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote!{
            #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_upper_camel_case_token_stream {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server: e,
                #field_code_occurence_new_3567ece5_74c9_4b48_a46c_8230cd728182_token_stream,
            }
        }
    };
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_declaration_token_stream = quote::quote!{
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
        quote::quote!{
            #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream {
                #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_snake_case_token_stream: vec_errors,
                #field_code_occurence_new_bb9fbcd9_7cea_42e2_b7d8_bc42710bf35e_token_stream
            }
        }
    };
    let uuid_wrapper_try_from_possible_uuid_wrapper_in_client_token_stream = quote::quote!{uuid_wrapper_try_from_possible_uuid_wrapper_in_client};
    let operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_declaration_token_stream = quote::quote!{
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
        quote::quote!{
            #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream {
                #uuid_wrapper_try_from_possible_uuid_wrapper_in_client_token_stream: e,
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
    let commit_header_addition_token_stream = quote::quote!{
        .header(
            postgresql_crud::COMMIT,
            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit,
        )
    };
    let application_json_quotes_token_stream = proc_macro_common::generate_quotes::generate_quotes_token_stream(
        "application/json",
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let content_type_application_json_header_addition_token_stream = quote::quote!{
        .header(reqwest::header::CONTENT_TYPE, #application_json_quotes_token_stream)
    };
    let impl_axum_response_into_response_token_stream = quote::quote!{impl axum::response::IntoResponse};
    let crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream = quote::quote!{crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor};
    let axum_extract_rejection_json_rejection_token_stream = quote::quote!{axum::extract::rejection::JsonRejection};
    let use_sqlx_acquire_token_stream = quote::quote!{use sqlx::Acquire};
    let sqlx_query_sqlx_postgres_token_stream = quote::quote!{sqlx::query::<sqlx::Postgres>};
    let reqwest_client_new_token_stream = quote::quote!{reqwest::Client::new()};
    let axum_extract_state_token_stream = quote::quote!{axum::extract::State};
    let axum_json_token_stream = quote::quote!{axum::Json};
    let crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream = quote::quote!{crate::server::postgres::bind_query::BindQuery::bind_value_to_query};
    let crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream = quote::quote!{crate::server::postgres::bind_query::BindQuery::try_generate_bind_increments};
    let crate_server_postgres_bind_query_bind_query_try_increment_token_stream = quote::quote!{crate::server::postgres::bind_query::BindQuery::try_increment};
    let increment_initialization_token_stream = quote::quote!{let mut increment: u64 = 0;};
    let try_extract_value_token_stream = quote::quote!{try_extract_value};
    let server_location_name_token_stream = quote::quote!{server_location};
    let dot_space = ", ";
    // let pg_temp_stringified = "pg_temp";
    let pg_connection_token_stream = quote::quote!{pg_connection};
    let query_string_name_token_stream = quote::quote!{query_string};
    let binded_query_name_token_stream = quote::quote!{binded_query};
    let postgres_transaction_token_stream = quote::quote!{postgres_transaction};
    let order_by_token_stream = quote::quote!{order_by};
    let current_vec_len_name_token_stream = quote::quote!{current_vec_len};
    let desirable_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::desirable_upper_camel_case_token_stream();
    let select_snake_case_token_stream = proc_macro_helpers::naming_conventions::select_snake_case_token_stream();
    let limit_token_stream = proc_macro_helpers::naming_conventions::limit_snake_case_token_stream();
    let offset_token_stream = proc_macro_helpers::naming_conventions::offset_snake_case_token_stream();
    let rollback_snake_case_token_stream = proc_macro_helpers::naming_conventions::rollback_snake_case_token_stream();
    let commit_token_stream = proc_macro_helpers::naming_conventions::commit_snake_case_token_stream();
    let begin_token_stream = proc_macro_helpers::naming_conventions::begin_snake_case_token_stream();
    let element_name_token_stream = proc_macro_helpers::naming_conventions::element_snake_case_token_stream();
    let acc_name_token_stream = proc_macro_helpers::naming_conventions::acc_snake_case_token_stream();
    let query_name_token_stream = proc_macro_helpers::naming_conventions::query_snake_case_token_stream();
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
    let update_name_stringified = proc_macro_helpers::naming_conventions::update_snake_case_stringified();
    let as_name_stringified = proc_macro_helpers::naming_conventions::as_snake_case_stringified();
    let set_name_stringified = proc_macro_helpers::naming_conventions::set_snake_case_stringified();
    let from_name_stringified = proc_macro_helpers::naming_conventions::from_snake_case_stringified();
    let insert_name_stringified = proc_macro_helpers::naming_conventions::insert_snake_case_stringified();
    let into_name_stringified = proc_macro_helpers::naming_conventions::into_snake_case_stringified();
    let values_name_stringified = proc_macro_helpers::naming_conventions::values_snake_case_stringified();
    let delete_name_stringified = proc_macro_helpers::naming_conventions::delete_snake_case_stringified();
    let where_name_stringified = proc_macro_helpers::naming_conventions::where_snake_case_stringified();
    let where_name_qoutes_token_stream = proc_macro_common::generate_quotes::generate_quotes_token_stream(
        &where_name_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let and_name_stringified = proc_macro_helpers::naming_conventions::and_snake_case_stringified();
    // let any_name_stringified = "any";
    // let array_name_stringified = "array";
    let select_name_stringified = proc_macro_helpers::naming_conventions::select_snake_case_stringified();
    let order_by_name_stringified = "order by";
    let limit_name_stringified = proc_macro_helpers::naming_conventions::limit_snake_case_stringified();
    let offset_name_stringified = proc_macro_helpers::naming_conventions::offset_snake_case_stringified();
    let in_name_stringified = proc_macro_helpers::naming_conventions::in_snake_case_stringified();
    let unnest_name_stringified = proc_macro_helpers::naming_conventions::unnest_snake_case_stringified();
    let common_error_syn_variants = {
        let sqlx_postgres_error_named_syn_variants = proc_macro_helpers::enum_variants::sqlx_postgres_error_named_syn_variants(&proc_macro_name_upper_camel_case_ident_stringified);
        let json_extractor_error_named_syn_variants = proc_macro_helpers::enum_variants::json_extractor_error_named_syn_variants(&proc_macro_name_upper_camel_case_ident_stringified);
        let mut common_error_variants_vec = std::vec::Vec::with_capacity(common_middlewares_error_syn_variants_len + sqlx_postgres_error_named_syn_variants.len() + 1);
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
    let fields_named_excluding_primary_key = fields_named_wrappers_excluding_primary_key.iter().map(|element|&element.field).collect::<std::vec::Vec<&syn::Field>>();
    // let fields_named_idents_comma_excluding_primary_key_token_stream = generate_self_fields_token_stream(
    //     &fields_named_excluding_primary_key,
    //     &proc_macro_name_upper_camel_case_ident_stringified,
    // ).iter().map(|element|quote::quote!{#element,}).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let fields_named_idents_comma_token_stream = generate_self_fields_token_stream(
        &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
        &proc_macro_name_upper_camel_case_ident_stringified,
    ).iter().map(|element|quote::quote!{#element,}).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let select_full_variant_token_stream = {
        let select_full_variant_stringified = fields_named.iter().fold(std::string::String::default(), |mut acc, field| {
            use convert_case::Casing;
            let field_ident_stringified = field.ident
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                }).to_string().to_case(convert_case::Case::Title);
            acc.push_str(&field_ident_stringified);
            acc
        });
        select_full_variant_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {select_full_variant_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let primary_keys_token_stream = quote::quote!{primary_keys};
    let primary_key_token_stream = quote::quote!{primary_key};
    let (
        create_many_token_stream,
        create_many_http_request_test_token_stream
    ) = {
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
        let additional_http_status_codes_error_variants = vec![];//todo find out why rust analyzer crashes
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
            let full_additional_http_status_codes_error_variants = generate_full_additional_http_status_codes_error_variants(
                common_middlewares_error_syn_variants.iter().collect(),
                additional_http_status_codes_error_variants.iter().collect()
            );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
                    common_error_syn_variants.len() +
                    3
                );
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
                full_additional_http_status_codes_error_variants
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
                    let fields_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| generate_pub_field_ident_field_type_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    quote::quote!{
                        #derive_debug_serialize_deserialize_to_schema_token_stream
                        pub struct #operation_payload_element_upper_camel_case_token_stream {
                            #(#fields_with_excluded_primary_key_token_stream),*
                        }
                    }
                };
                quote::quote!{
                    #operation_payload_element_token_stream
                    #derive_debug_serialize_deserialize_to_schema_token_stream
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
                    quote::quote!{
                        #derive_debug_serialize_deserialize_to_schema_token_stream
                        pub struct #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream {
                            #(#fields_with_excluded_primary_key_token_stream),*
                        }
                    }
                };
                quote::quote!{
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
                        quote::quote!{
                            #derive_debug_thiserror_error_occurence_token_stream
                            pub enum #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                                //todo remove useless variant and add later protentian errors from conversion from with serialize_deserialize
                                #not_uuid_token_upper_camel_case_stream {
                                    #eo_error_occurence_attribute_token_stream
                                    #not_uuid_token_snake_case_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                                },
                            }
                        }
                    };
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_try_from_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    let fields_idents_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        element.field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            })
                    });
                    quote::quote!{
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
                        quote::quote!{
                            #derive_debug_thiserror_error_occurence_token_stream
                            pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                                //todo remove useless variant and add later protentian errors from conversion from with serialize_deserialize
                                #not_uuid_token_upper_camel_case_stream {
                                    #eo_error_occurence_attribute_token_stream
                                    #not_uuid_token_snake_case_stream: #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream,
                                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                                },
                            }
                        }
                    };
                    quote::quote!{
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
                                        Err(e) => todo!()
                                    }
                                }
                                Ok(Self(elements))
                            }
                        }
                    }
                };
                quote::quote!{
                    #impl_std_convert_try_from_operation_payload_element_with_serialize_deserialize_for_operation_payload_element_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream = {
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_from_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    let fields_idents_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        element.field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            })
                    });
                    quote::quote!{
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
                quote::quote!{
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
            quote::quote!{
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
            quote::quote!{
                #derive_debug_thiserror_error_occurence_token_stream
                pub enum #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_upper_camel_case_token_stream {
                    #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_upper_camel_case_token_stream(
                        #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream
                    ),
                }
            }
        };
        // println!("{operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_token_stream}");
        let try_operation_error_with_middleware_error_variants_token_stream = {
            crate::type_variants_from_request_response_generator::type_variants_from_request_response_generator(
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>},//todo reuse
                &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &eo_display_token_stream,
                &eo_display_foreign_type_token_stream,
                &eo_display_with_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                type_variants_from_request_response_syn_variants.clone(),
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (
            http_request_token_stream,
            http_request_test_token_stream
        ) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(&operation);
                quote::quote!{
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #request_error_upper_camel_case_token_stream {
                            #eo_error_occurence_attribute_token_stream
                            #request_error_snake_case_token_stream: #try_operation_request_error_upper_camel_case_token_stream,
                            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                        },
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_http_request_many_token_stream(
                &server_location_name_token_stream,
                &server_location_type_token_stream,
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
                &request_error_variant_initialization_token_stream,
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>},//todo reuse
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
                let test_content_token_stream = quote::quote!{
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
                        Err(e) => panic!("{e}"),
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote!{
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let query_string_token_stream = {
                    let column_names = {
                        let fields_named_filtered = fields_named_wrappers_excluding_primary_key.iter()
                        .map(|element|&element.field)
                        .collect::<std::vec::Vec<&syn::Field>>();
                        fields_named_filtered.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, field)| {
                            let field_ident = field.ident.as_ref().unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
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
                        })
                    };
                    let column_increments = {
                        let mut column_increments = fields_named_wrappers_excluding_primary_key.iter()
                        .map(|element|&element.field)
                        .collect::<std::vec::Vec<&syn::Field>>()
                        .iter()
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
                    let column_vecs_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| {
                        let field_ident_underscore_vec_stringified = {
                            let field_ident = element.field.ident.as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                });
                            format!("{field_ident}{underscore_vec_name_stringified}")
                        };
                        field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    });
                    let handle_fields_named_wrappers_excluding_primary_key = fields_named_wrappers_excluding_primary_key.iter()
                    .map(|element|&element.field)
                    .collect::<std::vec::Vec<&syn::Field>>();
                    let column_vecs_with_capacity_token_stream = handle_fields_named_wrappers_excluding_primary_key.iter().map(|_|quote::quote!{std::vec::Vec::with_capacity(#current_vec_len_name_token_stream)});
                    let columns_acc_push_elements_token_stream = handle_fields_named_wrappers_excluding_primary_key.iter()
                    .enumerate().map(|(index, field)|{
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
                    let column_query_bind_vecs_token_stream = handle_fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        let field_ident_underscore_vec_token_stream = {
                            let field_ident_underscore_vec_stringified = {
                                let field_ident = element.ident.as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                    });
                                format!("{field_ident}{underscore_vec_name_stringified}")
                            };
                            field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{#query_name_token_stream = #query_name_token_stream.bind(#field_ident_underscore_vec_token_stream);}
                    });
                    quote::quote!{
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
                        #(#column_query_bind_vecs_token_stream)*
                        #query_name_token_stream
                    }
                };
                // println!("{binded_query_token_stream}");
                let from_log_and_return_error_token_stream = crate::from_log_and_return_error::from_log_and_return_error(
                    &try_operation_upper_camel_case_token_stream,
                    &error_log_call_token_stream,
                    &try_operation_response_variants_token_stream,
                );
                let acquire_pool_and_connection_token_stream = crate::acquire_pool_and_connection::acquire_pool_and_connection(
                    &from_log_and_return_error_token_stream,
                    &pg_connection_token_stream
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
                            Err(e) => {
                                let error = #try_operation_upper_camel_case_token_stream::from(e);
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(error);
                            }
                        }
                    } {
                        match {
                            use #sqlx_row_token_stream;
                            row.try_get::<#sqlx_types_uuid_token_stream, #server_location_type_token_stream>(#primary_key_field_ident_quotes_token_stream)
                        } {
                            Ok(value) => {
                                vec_values.push(
                                    #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value),
                                );
                            }
                            Err(e) => {
                                let error = #try_operation_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(error);
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
            quote::quote!{
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> #impl_axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value, 
                                Err(e) => {
                                    let error = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: e, 
                                        #field_code_occurence_new_91c61a45_6c97_47cc_ac96_65bdcfec0494_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            },
                            Err(err) => {
                                return err;
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
        let common_middlewares_error_syn_variants_from_impls = generate_common_middlewares_error_syn_variants_from_impls(
            common_middlewares_error_syn_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            additional_http_status_codes_error_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            &operation,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote!{
                #parameters_token_stream
                #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_error_unnamed_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &create_many_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (
        create_one_token_stream,
        create_one_http_request_test_token_stream
     ) = {
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
        let additional_http_status_codes_error_variants = vec![];//todo find out why rust analyzer crashes
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
            let full_additional_http_status_codes_error_variants = generate_full_additional_http_status_codes_error_variants(
                common_middlewares_error_syn_variants.iter().collect(),
                additional_http_status_codes_error_variants.iter().collect()
            );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
                    common_error_syn_variants.len() +
                    2
                );
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response.push(&operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants
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
                let fields_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| generate_pub_field_ident_field_type_token_stream(
                    element,
                    &proc_macro_name_upper_camel_case_ident_stringified
                ));
                quote::quote!{
                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream {
                        #(#fields_with_excluded_primary_key_token_stream),*
                    }
                }
            };
            // println!("{payload_token_stream}");
            let payload_with_serialize_deserialize_token_stream = {
                let fields_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| generate_field_ident_field_type_with_serialize_deserialize_token_stream(
                    element,
                    &proc_macro_name_upper_camel_case_ident_stringified
                ));
                quote::quote!{
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
                    quote::quote!{
                        #derive_debug_thiserror_error_occurence_token_stream
                        pub enum #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                            //todo errors in case of conversion to original type from with_serialize_deserialize failed and remove not_uuid_token_upper_camel_case_stream - useless
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
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_try_from_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    let fields_idents_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        element.field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            })
                    });
                    quote::quote!{
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
                quote::quote!{
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_from_token_stream(
                    element,
                    &proc_macro_name_upper_camel_case_ident_stringified
                ));
                let fields_idents_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                    element.field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        })
                });
                quote::quote!{
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
            quote::quote!{
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
                &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &eo_display_token_stream,
                &eo_display_foreign_type_token_stream,
                &eo_display_with_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                type_variants_from_request_response_syn_variants.clone(),//todo remove .clone()
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (
            http_request_token_stream,
            http_request_test_token_stream
         ) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(&operation);
                quote::quote!{
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #request_error_upper_camel_case_token_stream {
                            #eo_error_occurence_attribute_token_stream
                            #request_error_snake_case_token_stream: #try_operation_request_error_upper_camel_case_token_stream,
                            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                        },
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_try_operation_token_stream_new(
                &server_location_name_token_stream,
                &server_location_type_token_stream,
                &crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &quote::quote!{
                    let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(&#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)) {
                        Ok(value) => value,
                        Err(e) => {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                        }
                    };
                },
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &quote::quote!{
                    match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value) {
                        Ok(value) => Ok(value),
                        Err(e) => Err(#try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_initialization_token_stream )
                    }
                },
                &request_error_variant_initialization_token_stream,
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,//todo reuse it
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
                let test_content_token_stream = quote::quote!{
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
                        Err(e) => panic!("{e}")
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote!{
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let query_string_token_stream = {
                    let (
                        column_names,
                        column_increments
                    ) = {
                        let fields_named_filtered = fields_named_wrappers_excluding_primary_key.iter().map(|element|&element.field).collect::<std::vec::Vec<&syn::Field>>();
                        fields_named_filtered.iter().enumerate().fold((
                            std::string::String::default(),
                            std::string::String::default()
                        ), |mut acc, (index, field)| {
                            let field_ident = field.ident.as_ref().unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
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
                        let field_ident = element.field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
                        quote::quote!{
                            query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#field_ident, query);
                        }
                    });
                    quote::quote!{
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        #(#binded_query_modifications_token_stream)*
                        query
                    }
                };
                // println!("{binded_query_token_stream}");
                let from_log_and_return_error_token_stream = crate::from_log_and_return_error::from_log_and_return_error(
                    &try_operation_upper_camel_case_token_stream,
                    &error_log_call_token_stream,
                    &try_operation_response_variants_token_stream,
                );
                let acquire_pool_and_connection_token_stream = crate::acquire_pool_and_connection::acquire_pool_and_connection(
                    &from_log_and_return_error_token_stream,
                    &pg_connection_token_stream
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
                            value.try_get::<#sqlx_types_uuid_token_stream, #server_location_type_token_stream>(#primary_key_field_ident_quotes_token_stream)
                        } {
                            Ok(value) => #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value)),
                            Err(e) => {
                                let error = #try_operation_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(error);
                            }
                        },
                        Err(e) => {
                            #from_log_and_return_error_token_stream
                        }
                    }
                }
            };
            let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(//todo different parameters
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
            quote::quote!{
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> #impl_axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value, 
                                Err(e) => {
                                    let error = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: e, 
                                        #field_code_occurence_new_db0d95a8_8dcc_4228_80ec_e5dce2003333_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            },
                            Err(err) => {
                                return err;
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
        let common_middlewares_error_syn_variants_from_impls = generate_common_middlewares_error_syn_variants_from_impls(
            common_middlewares_error_syn_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            additional_http_status_codes_error_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            &operation,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote!{
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &create_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (
        read_many_token_stream,
        read_many_http_request_test_token_stream
    ) = {
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
        let additional_http_status_codes_error_variants = vec![];//todo find out why rust analyzer crashes
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
            let full_additional_http_status_codes_error_variants = generate_full_additional_http_status_codes_error_variants(
                common_middlewares_error_syn_variants.iter().collect(),
                additional_http_status_codes_error_variants.iter().collect()
            );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
                    common_error_syn_variants.len() +
                    not_unique_vec_syn_variants.len() +
                    4
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
                full_additional_http_status_codes_error_variants
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
                    let field_ident = element.field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    quote::quote!{
                        pub #field_ident: std::option::Option<std::vec::Vec<#crate_server_postgres_regex_filter_regex_filter_token_stream>>,
                    }
                });
                quote::quote!{
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
                    let field_ident = element.field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    quote::quote!{
                        #field_ident: std::option::Option<std::vec::Vec<#crate_server_postgres_regex_filter_regex_filter_token_stream>>,
                    }
                });
                quote::quote!{
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
                    quote::quote!{
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
                    let primary_key_field_assignment_token_stream = {
                        let field_code_occurence_new_ed55593d_d353_440c_8145_c1c712bc5ace_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                            file!(),
                            line!(),
                            column!(),
                            &proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        quote::quote!{
                            let #primary_key_field_ident = match value.#primary_key_field_ident {
                                Some(value) => match value.into_iter()
                                    .map(|element|#crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(element)))
                                    .collect::<Result<
                                        #std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                                        #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream
                                    >>() 
                                {
                                    Ok(value) => Some(value),
                                    Err(e) => {
                                        return Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                            #not_uuid_token_snake_case_stream: e,
                                            #field_code_occurence_new_ed55593d_d353_440c_8145_c1c712bc5ace_token_stream,
                                        });
                                    }
                                },
                                None => None
                            };
                        }
                    };
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_try_from_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    quote::quote!{
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
                quote::quote!{
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let primary_key_field_assignment_token_stream = {
                    quote::quote!{
                        let #primary_key_field_ident = match value.#primary_key_field_ident {
                            Some(value) => Some(value.into_iter().map(|element|element.to_string()).collect::<std::vec::Vec<#std_string_string_token_stream>>()),
                            None => None
                        };
                    }
                };
                let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_from_token_stream(
                    element,
                    &proc_macro_name_upper_camel_case_ident_stringified
                ));
                quote::quote!{
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
            quote::quote!{
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
                &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &eo_display_token_stream,
                &eo_display_foreign_type_token_stream,
                &eo_display_with_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                type_variants_from_request_response_syn_variants.clone(),
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (
            http_request_token_stream,
            http_request_test_token_stream
        ) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(&operation);
                quote::quote!{
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #request_error_upper_camel_case_token_stream {
                            #eo_error_occurence_attribute_token_stream
                            #request_error_snake_case_token_stream: #try_operation_request_error_upper_camel_case_token_stream,
                            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                        },
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_try_operation_token_stream_new(
                &server_location_name_token_stream,
                &server_location_type_token_stream,
                &quote::quote!{std::vec::Vec<#struct_options_ident_token_stream>},
                &quote::quote!{
                    let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(
                        &#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)
                    ) {
                        Ok(value) => value,
                        Err(e) => {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                        }
                    };
                },
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &quote::quote!{
                    Ok(value)
                },
                &request_error_variant_initialization_token_stream,
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#struct_options_ident_token_stream>},//todo reuse it
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
                let test_content_token_stream = quote::quote!{
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
                        Err(e) => panic!("{e}")
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote!{
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let filter_unique_parameters_token_stream = {
                    let filter_unique_parameters_primary_key_token_stream = quote::quote!{
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
                                let error = #try_operation_upper_camel_case_token_stream::#not_unique_primary_key_variant_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(error);
                            }
                        }
                    };
                    let filter_unique_parameters_other_columns_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element| {
                        let field_ident = element.field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
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
                                            let error = #try_operation_upper_camel_case_token_stream::#not_unique_field_vec_vec_upper_camel_token_stream {
                                                #not_unique_field_vec_snake_case_token_stream,
                                                #field_code_occurence_new_eb1a9553_449e_4767_9e5c_c1856b77bd4e_token_stream,
                                            };
                                            #error_log_call_token_stream
                                            return #try_operation_response_variants_token_stream::from(error);
                                        }
                                    }
                                }
                                None => None,
                            };
                        }
                    });
                    quote::quote!{
                        #filter_unique_parameters_primary_key_token_stream
                        #(#filter_unique_parameters_other_columns_token_stream)*
                    }
                };
                let query_string_token_stream = {
                    let additional_parameters_primary_key_modification_token_stream = {
                        let prefix_false_handle_token_stream = {
                            let prefix_false_handle_stringified = format!("\" {and_name_stringified}\"");
                            prefix_false_handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {prefix_false_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let handle_token_stream = {
                            let handle_stringified = format!("\"{{}} {primary_key_field_ident} {in_name_stringified} ({select_name_stringified} {unnest_name_stringified}(${{}}))\"");
                            handle_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{
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
                        let field_ident = element.field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
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
                                    let mut bind_increments = std::string::String::default();
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
                                            Err(e) => {
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
                        let additional_parameters_order_by_handle_stringified = format!("\"{{}}{order_by_name_stringified} {{}} {{}}\"");
                        additional_parameters_order_by_handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_order_by_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let additional_parameters_limit_handle_token_stream = {
                        let additional_parameters_limit_handle_stringified = format!("\"{{}}{limit_name_stringified} {{}}\"");
                        additional_parameters_limit_handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_limit_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    let additional_parameters_offset_handle_token_stream = {
                        let additional_parameters_offset_handle_stringified = format!("\"{{}}{offset_name_stringified} {{}}\"");
                        additional_parameters_offset_handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_offset_handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote!{
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
                                        Err(e) => {
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
                                        Err(e) => {
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
                    let binded_query_primary_key_modification_token_stream = quote::quote!{
                        if let Some(value) = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident {
                            query = query.bind(value.into_iter().map(|element|element.into_inner().clone()).collect::<std::vec::Vec<#sqlx_types_uuid_token_stream>>());
                        }
                    };
                    let binded_query_modifications_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        let field_ident = element.field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
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
                    quote::quote!{
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
                let from_log_and_return_error_token_stream = crate::from_log_and_return_error::from_log_and_return_error(
                    &try_operation_upper_camel_case_token_stream,
                    &error_log_call_token_stream,
                    &try_operation_response_variants_token_stream,
                );
                let acquire_pool_and_connection_token_stream = crate::acquire_pool_and_connection::acquire_pool_and_connection(
                    &from_log_and_return_error_token_stream,
                    &pg_connection_token_stream
                );
                quote::quote!{
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
                                Err(e) => {
                                    #from_log_and_return_error_token_stream;
                                }
                            }
                        } {
                            match #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#select_snake_case_token_stream.#options_try_from_sqlx_row_name_token_stream(&row) {
                                Ok(value) => {
                                    vec_values.push(value);
                                }
                                Err(e) => {
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
            quote::quote!{
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> #impl_axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(e) => {
                                    let error = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: e,
                                        #field_code_occurence_new_1d57484c_3c67_4f5f_81a6_ec8efc9c6896_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            },
                            Err(err) => {
                                return err;
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
        let common_middlewares_error_syn_variants_from_impls = generate_common_middlewares_error_syn_variants_from_impls(
            common_middlewares_error_syn_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            additional_http_status_codes_error_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            &operation,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote!{
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream
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
        read_one_http_request_test_expect_fail_token_stream
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
        let additional_http_status_codes_error_variants = vec![];//todo find out why rust analyzer crashes
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
            let full_additional_http_status_codes_error_variants = generate_full_additional_http_status_codes_error_variants(
                common_middlewares_error_syn_variants.iter().collect(),
                additional_http_status_codes_error_variants.iter().collect()
            );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
                    common_error_syn_variants.len() +
                    1
                );
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants
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
                quote::quote!{
                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream {
                        pub #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                        pub #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream,
                    }
                }
            };
            // println!("{payload_token_stream}");
            let payload_with_serialize_deserialize_token_stream = {
                quote::quote!{
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
                    quote::quote!{
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
                    quote::quote!{
                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                let #primary_key_field_ident = match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value.#primary_key_field_ident) {
                                    Ok(value) => value,
                                    Err(e) => {
                                        return Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                            #not_uuid_token_snake_case_stream: e,
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
                quote::quote!{
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                quote::quote!{
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
            quote::quote!{
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
                &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &eo_display_token_stream,
                &eo_display_foreign_type_token_stream,
                &eo_display_with_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                type_variants_from_request_response_syn_variants.clone(),//todo remove .clone()
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (
            http_request_token_stream,
            http_request_test_expect_success_token_stream,
            http_request_test_expect_fail_token_stream
         ) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(&operation);
                quote::quote!{
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #request_error_upper_camel_case_token_stream {
                            #eo_error_occurence_attribute_token_stream
                            #request_error_snake_case_token_stream: #try_operation_request_error_upper_camel_case_token_stream,
                            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                        },
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_try_operation_token_stream_new(
                &server_location_name_token_stream,
                &server_location_type_token_stream,
                &struct_options_ident_token_stream,
                &quote::quote!{
                    let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(&#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)) {
                        Ok(value) => value,
                        Err(e) => {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                        }
                    };
                },
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &quote::quote!{
                    Ok(value)
                },
                &request_error_variant_initialization_token_stream,
                &table_name_stringified,
                &operation,
                //
                &proc_macro_name_upper_camel_case_ident_stringified,
                type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &struct_options_ident_token_stream,
            );
            let http_request_test_expect_success_token_stream = {
                let test_content_token_stream = quote::quote!{
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
                        Err(e) => panic!("{e}")
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            let http_request_test_expect_fail_token_stream = {
                let test_content_token_stream = quote::quote!{
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
                        Err(e) => println!("{e}")
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote!{
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_expect_success_token_stream,
                http_request_test_expect_fail_token_stream
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
                    quote::quote!{
                        format!(
                            #query_token_stream,
                            crate::server::postgres::generate_query::GenerateQuery::generate_query(&#select_snake_case_token_stream),
                        )
                    }
                };
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = quote::quote!{
                        let query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident, query);
                    };
                    quote::quote!{
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        #binded_query_modifications_token_stream
                        query
                    }
                };
                let from_log_and_return_error_token_stream = crate::from_log_and_return_error::from_log_and_return_error(
                    &try_operation_upper_camel_case_token_stream,
                    &error_log_call_token_stream,
                    &try_operation_response_variants_token_stream,
                );
                let acquire_pool_and_connection_token_stream = crate::acquire_pool_and_connection::acquire_pool_and_connection(
                    &from_log_and_return_error_token_stream,
                    &pg_connection_token_stream
                );
                quote::quote!{
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
                            Err(e) => {
                                #from_log_and_return_error_token_stream
                            },
                        },
                        Err(e) => {
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
            quote::quote!{
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> #impl_axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value, 
                                Err(e) => {
                                    let error = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: e, 
                                        #field_code_occurence_new_cd714ff2_3a40_4e0d_8930_e43d2f69ffc0_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            },
                            Err(err) => {
                                return err;
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
        let common_middlewares_error_syn_variants_from_impls = generate_common_middlewares_error_syn_variants_from_impls(
            common_middlewares_error_syn_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            additional_http_status_codes_error_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            &operation,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote!{
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_expect_success_token_stream,
            http_request_test_expect_fail_token_stream
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &read_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (
        update_many_token_stream,
        update_many_http_request_test_token_stream
     ) = {
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
        let additional_http_status_codes_error_variants = vec![];//todo find out why rust analyzer crashes
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
            let full_additional_http_status_codes_error_variants = generate_full_additional_http_status_codes_error_variants(
                common_middlewares_error_syn_variants.iter().collect(),
                additional_http_status_codes_error_variants.iter().collect()
            );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
                    common_error_syn_variants.len() +
                    10
                );
                for element in &common_error_syn_variants {
                    type_variants_from_request_response.push(element);
                }
                type_variants_from_request_response.push(&not_unique_primary_keys_syn_variant);
                type_variants_from_request_response.push(&bind_query_syn_variant);
                type_variants_from_request_response.push(&checked_add_syn_variant);
                type_variants_from_request_response.push(&no_payload_fields_syn_variant);
                type_variants_from_request_response.push(&commit_failed_syn_variant);
                type_variants_from_request_response.push(&non_existing_primary_keys_syn_variant);
                type_variants_from_request_response.push(&primary_key_from_row_and_failed_rollback_syn_variant);
                type_variants_from_request_response.push(&non_existing_primary_keys_and_failed_rollback_syn_variant);
                type_variants_from_request_response.push(&query_and_rollback_failed_syn_variant);
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants
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
                    let fields_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_pub_field_ident_field_type_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    quote::quote!{
                        #derive_debug_to_schema_token_stream
                        pub struct #operation_payload_element_upper_camel_case_token_stream {
                            pub #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                            #(#fields_with_excluded_primary_key_token_stream),*
                        }
                    }
                };
                quote::quote!{
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
                    quote::quote!{
                        #derive_debug_serialize_deserialize_to_schema_token_stream
                        pub struct #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream {
                            #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                            #(#fields_with_excluded_primary_key_token_stream),*
                        }
                    }
                };
                quote::quote!{
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
                    quote::quote!{
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
                        quote::quote!{
                            #derive_debug_thiserror_error_occurence_token_stream
                            pub enum #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream {
                                #not_uuid_token_upper_camel_case_stream {
                                    #eo_display_token_stream
                                    #not_uuid_token_snake_case_stream: sqlx::types::uuid::Error,
                                    #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                                },
                            }
                        }
                    };
                    // println!("{operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream}");
                    let fields_assignments_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_try_from_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    let self_init_fields_token_stream = generate_self_fields_token_stream(
                        &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let field_code_occurence_new_814b41f8_3219_4b62_bc0b_02a26d23b262_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote!{
                        #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_token_stream
                        impl std::convert::TryFrom<#operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_element_upper_camel_case_token_stream {
                            type Error = #operation_payload_element_try_from_operation_payload_element_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_element_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                let #primary_key_field_ident = match #sqlx_types_uuid_token_stream::parse_str(value.#primary_key_field_ident.to_inner()) {
                                    Ok(value) => #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::from(value),
                                    Err(e) => {
                                        return Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                            #not_uuid_token_snake_case_stream: e,
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
                quote::quote!{
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
                                Err(e) => Err(Self::Error::from(e)),
                            }
                        }
                    }
                }
            };
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let impl_std_convert_from_operation_payload_element_for_operation_payload_element_with_serialize_deserialize_token_stream = {
                    let fields_assignments_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_from_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    let self_init_fields_token_stream = generate_self_fields_token_stream(
                        &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote!{
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
                quote::quote!{
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
            quote::quote!{
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
                &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &eo_display_token_stream,
                &eo_display_foreign_type_token_stream,
                &eo_display_with_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                type_variants_from_request_response_syn_variants.clone(),//todo remove .clone()
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (
            http_request_token_stream,
            http_request_test_token_stream
         ) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(&operation);
                quote::quote!{
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #request_error_upper_camel_case_token_stream {
                            #eo_error_occurence_attribute_token_stream
                            #request_error_snake_case_token_stream: #try_operation_request_error_upper_camel_case_token_stream,
                            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                        },
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_http_request_many_token_stream(
                &server_location_name_token_stream,
                &server_location_type_token_stream,
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
                &request_error_variant_initialization_token_stream,
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>},//todo reuse
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
                let test_content_token_stream = quote::quote!{
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
                        Err(e) => panic!("{e}")
                    }
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote!{
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let expected_updated_primary_keys_token_stream = quote::quote!{
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
                    let declarations = {
                        let fields_named_filtered = fields_named_wrappers_excluding_primary_key.iter().map(|element|&element.field).collect::<std::vec::Vec<&syn::Field>>();
                        fields_named_filtered.iter().enumerate().fold(std::string::String::default(), |mut acc, (index, field)| {
                            let field_ident = field.ident.as_ref().unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
                            let possible_dot_space = match (
                                index.checked_add(1).unwrap_or_else(|| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {index} {}", proc_macro_common::global_variables::hardcode::CHECKED_ADD_NONE_OVERFLOW_MESSAGE))
                            ) == fields_named_wrappers_excluding_primary_key_len {
                                true => "",
                                false => dot_space,
                            };
                            acc.push_str(&format!("{field_ident} = data.{field_ident}{possible_dot_space}"));
                            acc
                        })
                    };
                    let query_stringified = format!("\"{update_name_stringified} {table_name_stringified} {as_name_stringified} t {set_name_stringified} {declarations} {from_name_stringified} ({select_name_stringified} * {from_name_stringified} {unnest_name_stringified}({column_increments})) as data({column_names}) where t.{primary_key_field_ident} = data.{primary_key_field_ident} {returning_stringified} data.{primary_key_field_ident}\"");
                    query_stringified.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let binded_query_token_stream = {
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
                            let field_ident_underscore_vec_stringified = format!("{primary_key_field_ident}{underscore_vec_name_stringified}");
                            field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{
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
                                let field_ident = element.field.ident.as_ref()
                                    .unwrap_or_else(|| {
                                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                    });
                                format!("{field_ident}{underscore_vec_name_stringified}")
                            };
                            field_ident_underscore_vec_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_underscore_vec_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{#query_name_token_stream = #query_name_token_stream.bind(#field_ident_underscore_vec_token_stream);}
                    });
                    quote::quote!{
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
                let from_log_and_return_error_token_stream = crate::from_log_and_return_error::from_log_and_return_error(
                    &try_operation_upper_camel_case_token_stream,
                    &error_log_call_token_stream,
                    &try_operation_response_variants_token_stream,
                );
                let acquire_pool_and_connection_token_stream = crate::acquire_pool_and_connection::acquire_pool_and_connection(
                    &from_log_and_return_error_token_stream,
                    &pg_connection_token_stream
                );
                let generate_postgres_transaction_token_stream = crate::generate_postgres_transaction::generate_postgres_transaction(
                    &expected_updated_primary_keys_token_stream,
                    &query_string_name_token_stream,
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &acquire_pool_and_connection_token_stream,
                    &use_sqlx_acquire_token_stream,
                    &pg_connection_token_stream,
                    &begin_token_stream,
                    &binded_query_name_token_stream,
                    &use_futures_try_stream_ext_token_stream,
                    &query_and_rollback_failed_variant_initialization_token_stream,
                    &primary_key_uuid_wrapper_try_from_sqlx_row_name_token_stream,
                    &from_log_and_return_error_token_stream,
                    &rollback_error_name_token_stream,
                    &primary_key_from_row_and_failed_rollback_variant_initialization_token_stream,
                    &non_existing_primary_keys_name_token_stream,
                    &expected_updated_primary_keys_name_token_stream,
                    &primary_key_vec_name_token_stream,
                    &rollback_snake_case_token_stream,
                    &non_existing_primary_keys_variant_initialization_token_stream,
                    &non_existing_primary_keys_and_failed_rollback_variant_initialization_token_stream,
                    &postgres_transaction_token_stream,
                    &commit_token_stream,
                    &try_operation_response_variants_token_stream,
                    &desirable_upper_camel_case_token_stream,
                    &try_operation_upper_camel_case_token_stream,
                    &commit_failed_variant_initialization_token_stream,
                    &error_log_call_token_stream,
                    &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                );
                quote::quote!{
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
                            let error = #try_operation_upper_camel_case_token_stream::#not_unique_primary_key_variant_initialization_token_stream;
                            #error_log_call_token_stream
                            return #try_operation_response_variants_token_stream::from(error);
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
            quote::quote!{
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream<'a>(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> #impl_axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(e) => {
                                    let error = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: e,
                                        #field_code_occurence_new_04274f8d_c9a4_41d1_9bdc_39432371c33f_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            },
                            Err(err) => {
                                return err;
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
        let common_middlewares_error_syn_variants_from_impls = generate_common_middlewares_error_syn_variants_from_impls(
            common_middlewares_error_syn_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            additional_http_status_codes_error_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            &operation,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote!{
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &update_many_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    //todo WHY ITS RETURN SUCCESS EVEN IF ROW DOES NOT EXISTS?
    let (
        update_one_token_stream,
        update_one_http_request_test_token_stream
     ) = {
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
        let additional_http_status_codes_error_variants = vec![];//todo find out why rust analyzer crashes
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
            let full_additional_http_status_codes_error_variants = generate_full_additional_http_status_codes_error_variants(
                common_middlewares_error_syn_variants.iter().collect(),
                additional_http_status_codes_error_variants.iter().collect()
            );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
                    common_error_syn_variants.len() +
                    4
                );
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
                full_additional_http_status_codes_error_variants
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
                    let field_ident = element.field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    let field_type = &element.field.ty;
                    quote::quote!{
                        pub #field_ident: std::option::Option<#field_type>
                    }
                });
                quote::quote!{
                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream {
                        pub #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                        #(#fields_with_excluded_primary_key_token_stream),*
                    }
                }
            };
            // println!("{payload_token_stream}");
            let payload_with_serialize_deserialize_token_stream = {
                let fields_with_excluded_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                    let field_ident = element.field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    let field_type = &element.field.ty;
                    quote::quote!{
                        #field_ident: std::option::Option<#field_type>//todo with serialize deserialize conversion variants
                    }   
                });
                quote::quote!{
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
                    quote::quote!{
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
                    let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_try_from_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    let fields_idents_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                        element.field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            })
                    });
                    let field_code_occurence_new_9d290620_cad2_47ab_900e_da3f3d08307f_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote!{
                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                let #primary_key_field_ident = match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value.#primary_key_field_ident) {
                                    Ok(value) => value,
                                    Err(e) => {
                                        return Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                            #not_uuid_token_snake_case_stream: e,
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
                quote::quote!{
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let fields_assignment_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_from_token_stream(
                    element,
                    &proc_macro_name_upper_camel_case_ident_stringified
                ));
                let fields_idents_excluding_primary_key_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                    element.field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        })
                });
                quote::quote!{
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
            quote::quote!{
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
                &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &eo_display_token_stream,
                &eo_display_foreign_type_token_stream,
                &eo_display_with_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                type_variants_from_request_response_syn_variants.clone(),//todo remove .clone()
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (
            http_request_token_stream,
            http_request_test_token_stream
         ) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(&operation);
                quote::quote!{
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #request_error_upper_camel_case_token_stream {
                            #eo_error_occurence_attribute_token_stream
                            #request_error_snake_case_token_stream: #try_operation_request_error_upper_camel_case_token_stream,
                            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                        },
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_try_operation_token_stream_new(
                &server_location_name_token_stream,
                &server_location_type_token_stream,
                &crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &quote::quote!{
                    let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(&#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)) {
                        Ok(value) => value,
                        Err(e) => {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                        }
                    };
                },
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &quote::quote!{
                    match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value) {
                        Ok(value) => Ok(value),
                        Err(e) => Err(#try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_initialization_token_stream )
                    }
                },
                &request_error_variant_initialization_token_stream,
                &table_name_stringified,
                &operation,
                //
                &proc_macro_name_upper_camel_case_ident_stringified,
                type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
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
                let test_content_token_stream = quote::quote!{
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
                        Err(e) => panic!("{e}")
                    };
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote!{
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let check_for_none_token_stream_excluding_primary_key = crate::check_for_none::check_for_none(
                    fields_named,
                    &primary_key_field,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                    dot_space,
                    &try_operation_response_variants_token_stream,
                    true
                );
                let query_string_token_stream = {
                    let additional_parameters_modification_token_stream = {
                        let fields_named_filtered = fields_named_wrappers_excluding_primary_key.iter().map(|element|&element.field).collect::<std::vec::Vec<&syn::Field>>();
                        fields_named_filtered.iter().enumerate().map(|(index, field)| {
                            let field_ident = field.ident.as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                });
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
                                        Err(e) => {
                                            return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
                                        },
                                    }
                                }
                            }
                        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>()
                    };
                    let additional_parameters_primary_key_modification_token_stream = {
                        let query_part_token_stream = {
                            let query_part_stringified = format!("\" where {primary_key_field_ident} = ${{increment}}\"");//todo where
                            query_part_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_part_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{
                            match #crate_server_postgres_bind_query_bind_query_try_increment_token_stream(&#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident, &mut increment) {
                                Ok(_) => {
                                    query.push_str(&format!(#query_part_token_stream));
                                },
                                Err(e) => {
                                    return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
                                },
                            }
                        }
                    };
                    let handle_token_stream = {
                        let handle_stringified = format!("\"{update_name_stringified} {table_name_stringified} {set_name_stringified} \"");//todo where
                        handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote!{
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
                        let field_ident = element.field.ident.as_ref()
                            .unwrap_or_else(|| {
                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                            });
                        quote::quote!{
                            if let Some(value) = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#field_ident {
                                query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
                                    value,
                                    query,
                                );
                            }
                        }
                    });
                    let binded_query_primary_key_modification_token_stream = quote::quote!{
                        query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident,
                            query,
                        );
                    };
                    quote::quote!{
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        #(#binded_query_modifications_token_stream)*
                        #binded_query_primary_key_modification_token_stream
                        query
                    }
                };
                let from_log_and_return_error_token_stream = crate::from_log_and_return_error::from_log_and_return_error(
                    &try_operation_upper_camel_case_token_stream,
                    &error_log_call_token_stream,
                    &try_operation_response_variants_token_stream,
                );
                let acquire_pool_and_connection_token_stream = crate::acquire_pool_and_connection::acquire_pool_and_connection(
                    &from_log_and_return_error_token_stream,
                    &pg_connection_token_stream
                );
                quote::quote!{
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
                            value.try_get::<#sqlx_types_uuid_token_stream, #server_location_type_token_stream>(#primary_key_field_ident_quotes_token_stream)
                        } {
                            Ok(value) => #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value)),
                            Err(e) => {
                                let error = #try_operation_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(error);
                            }
                        },
                        Err(e) => {
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
            quote::quote!{
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream<'a>(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> #impl_axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(e) => {
                                    let error = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: e,
                                        #field_code_occurence_new_112d570b_90ed_44d6_a8bf_d855372006cb_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            },
                            Err(err) => {
                                return err;
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
        let common_middlewares_error_syn_variants_from_impls = generate_common_middlewares_error_syn_variants_from_impls(
            common_middlewares_error_syn_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            additional_http_status_codes_error_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            &operation,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote!{
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &update_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (
        delete_many_token_stream,
        delete_many_http_request_test_token_stream
     ) = {
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
        let additional_http_status_codes_error_variants = vec![];//todo find out why rust analyzer crashes
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
            let full_additional_http_status_codes_error_variants = generate_full_additional_http_status_codes_error_variants(
                common_middlewares_error_syn_variants.iter().collect(),
                additional_http_status_codes_error_variants.iter().collect()
            );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
                    common_error_syn_variants.len() +
                    not_unique_vec_syn_variants.len() +
                    12
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
                type_variants_from_request_response.push(&non_existing_primary_keys_and_failed_rollback_syn_variant);
                type_variants_from_request_response.push(&primary_key_from_row_and_failed_rollback_syn_variant);
                type_variants_from_request_response.push(&commit_failed_syn_variant);
                type_variants_from_request_response.push(&query_and_rollback_failed_syn_variant);
                type_variants_from_request_response.push(&operation_payload_try_from_operation_payload_with_serialize_deserialize_syn_variant);
                type_variants_from_request_response.push(&operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_syn_variant);
                type_variants_from_request_response
            };
            generate_type_variants_from_request_response_syn_variants(
                type_variants_from_request_response_syn_variants_partial,
                full_additional_http_status_codes_error_variants
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
                    let field_ident = element.field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    quote::quote!{
                        pub #field_ident: std::option::Option<std::vec::Vec<#crate_server_postgres_regex_filter_regex_filter_token_stream>>//todo
                    }
                });
                quote::quote!{
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
                    let field_ident = element.field.ident.as_ref()
                        .unwrap_or_else(|| {
                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                        });
                    quote::quote!{
                        #field_ident: std::option::Option<std::vec::Vec<#crate_server_postgres_regex_filter_regex_filter_token_stream>>
                    }
                });
                quote::quote!{
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
                    quote::quote!{
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
                    let fields_assignments_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_try_from_token_stream(
                        element,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    let self_init_fields_token_stream = generate_self_fields_token_stream(
                        &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    let field_code_occurence_new_9a8f5b56_b6d4_4539_8f07_a13997efd268_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                        file!(),
                        line!(),
                        column!(),
                        &proc_macro_name_upper_camel_case_ident_stringified,
                    );
                    quote::quote!{
                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                let #primary_key_field_ident = match value.#primary_key_field_ident {
                                    Some(value) => match value.into_iter().map(|element|#crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(element)).collect::<Result<
                                        #std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,                                      
                                        #crate_server_postgres_uuid_wrapper_uuid_wrapper_try_from_possible_uuid_wrapper_error_named_token_stream
                                    >>() {
                                        Ok(value) => Some(value),
                                        Err(e) => {
                                            return Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                                #not_uuid_token_snake_case_stream: e,
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
                quote::quote!{
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                let fields_assignments_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|generate_let_field_ident_value_field_ident_from_token_stream(
                    element,
                    &proc_macro_name_upper_camel_case_ident_stringified
                ));
                let self_init_fields_token_stream = generate_self_fields_token_stream(
                    &fields_named.iter().collect::<std::vec::Vec<&syn::Field>>() as &[&syn::Field],
                    &proc_macro_name_upper_camel_case_ident_stringified,
                );
                quote::quote!{
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
            quote::quote!{
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
                &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &eo_display_token_stream,
                &eo_display_foreign_type_token_stream,
                &eo_display_with_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                type_variants_from_request_response_syn_variants.clone(),//todo remove .clone()
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (
            http_request_token_stream,
            http_request_test_token_stream
         ) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(&operation);
                quote::quote!{
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #request_error_upper_camel_case_token_stream {
                            #eo_error_occurence_attribute_token_stream
                            #request_error_snake_case_token_stream: #try_operation_request_error_upper_camel_case_token_stream,
                            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                        },
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_many_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_http_request_many_token_stream(
                &server_location_name_token_stream,
                &server_location_type_token_stream,
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
                &request_error_variant_initialization_token_stream,
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &quote::quote!{std::vec::Vec::<#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream>},//todo reuse
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
                        //     regex: std::string::String::from("test"),
                        //     conjuctive_operator: crate::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                        // }])
                    }
                }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                let test_content_token_stream = quote::quote!{
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
                        Err(e) => panic!("{e}")
                    }
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote!{
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream
            )
        };
        // println!("{http_request_token_stream}");
        let route_handler_token_stream = {
            let operation_snake_case_token_stream = operation_name_snake_case_stringified.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {operation_name_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let try_operation_token_stream = {
                let check_for_none_token_stream = crate::check_for_none::check_for_none(
                    fields_named,
                    &primary_key_field,
                    &proc_macro_name_upper_camel_case_ident_stringified,
                    dot_space,
                    &try_operation_response_variants_token_stream,
                    false
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
                let from_log_and_return_error_token_stream = crate::from_log_and_return_error::from_log_and_return_error(
                    &try_operation_upper_camel_case_token_stream,
                    &error_log_call_token_stream,
                    &try_operation_response_variants_token_stream,
                );
                let acquire_pool_and_connection_token_stream = crate::acquire_pool_and_connection::acquire_pool_and_connection(
                    &from_log_and_return_error_token_stream,
                    &pg_connection_token_stream
                );
                let generate_postgres_transaction_token_stream = {
                    let filter_unique_parameters_token_stream = {
                        let filter_unique_parameters_primary_key_token_stream = quote::quote!{
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
                                let error = #try_operation_upper_camel_case_token_stream::#not_unique_primary_key_variant_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(error);
                            }
                        };
                        quote::quote!{
                            #filter_unique_parameters_primary_key_token_stream
                        }
                    };
                    let expected_updated_primary_keys_token_stream = quote::quote!{
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
                    let binded_query_primary_key_some_other_none_token_stream = quote::quote!{
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        query = query.bind(
                            #primary_key_field_ident
                            .into_iter()
                            .map(|element| element.clone().into_inner())
                            .collect::<std::vec::Vec<#sqlx_types_uuid_token_stream>>()
                        );
                        query
                    };
                    let generate_postgres_transaction_token_stream = crate::generate_postgres_transaction::generate_postgres_transaction(
                        &expected_updated_primary_keys_token_stream,
                        &query_string_name_token_stream,
                        &query_string_primary_key_some_other_none_token_stream,
                        &binded_query_primary_key_some_other_none_token_stream,
                        &acquire_pool_and_connection_token_stream,
                        &use_sqlx_acquire_token_stream,
                        &pg_connection_token_stream,
                        &begin_token_stream,
                        &binded_query_name_token_stream,
                        &use_futures_try_stream_ext_token_stream,
                        &query_and_rollback_failed_variant_initialization_token_stream,
                        &primary_key_uuid_wrapper_try_from_sqlx_row_name_token_stream,
                        &from_log_and_return_error_token_stream,
                        &rollback_error_name_token_stream,
                        &primary_key_from_row_and_failed_rollback_variant_initialization_token_stream,
                        &non_existing_primary_keys_name_token_stream,
                        &expected_updated_primary_keys_name_token_stream,
                        &primary_key_vec_name_token_stream,
                        &rollback_snake_case_token_stream,
                        &non_existing_primary_keys_variant_initialization_token_stream,
                        &non_existing_primary_keys_and_failed_rollback_variant_initialization_token_stream,
                        &postgres_transaction_token_stream,
                        &commit_token_stream,
                        &try_operation_response_variants_token_stream,
                        &desirable_upper_camel_case_token_stream,
                        &try_operation_upper_camel_case_token_stream,
                        &commit_failed_variant_initialization_token_stream,
                        &error_log_call_token_stream,
                        &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
                    );
                    quote::quote!{
                        #filter_unique_parameters_token_stream
                        #generate_postgres_transaction_token_stream
                    }
                };
                let generate_postgres_execute_query_token_stream = {//todo rename execute into something else
                    let filter_unique_parameters_token_stream = {
                        let filter_unique_parameters_primary_key_token_stream = quote::quote!{
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
                                    let error = #try_operation_upper_camel_case_token_stream::#not_unique_primary_key_variant_initialization_token_stream;
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            }
                        };
                        let filter_unique_parameters_other_columns_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                            let field_ident = element.field.ident.as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                });
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
                                                let error = #try_operation_upper_camel_case_token_stream::#not_unique_field_vec_vec_upper_camel_token_stream {
                                                    #not_unique_field_vec_snake_case_token_stream,
                                                    #field_code_occurence_new_a4cd6c7d_3d82_4ee7_84f0_ca63ddb894e1_token_stream,
                                                };
                                                #error_log_call_token_stream
                                                return #try_operation_response_variants_token_stream::from(error);
                                            }
                                        }
                                    },
                                    None => None
                                };
                            }
                        });
                        quote::quote!{
                            #filter_unique_parameters_primary_key_token_stream
                            #(#filter_unique_parameters_other_columns_token_stream)*
                        }
                    };
                    let query_string_token_stream = {
                        let additional_parameters_modification_token_stream = fields_named_wrappers_excluding_primary_key.iter().map(|element|{
                            let field_ident = element.field.ident.as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                });
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
                                        Err(e) => {
                                            return #try_operation_response_variants_token_stream::#bind_query_variant_initialization_token_stream;
                                        },
                                    }
                                }
                            }
                        });
                        let additional_parameters_primary_key_modification_token_stream = {
                            let handle_token_stream = {
                                let handle_stringified = format!("\" {primary_key_field_ident} {in_name_stringified} ({{}})\"");
                                handle_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            let additional_parameters_and_token_stream = {
                                let additional_parameters_and_stringified = format!("\" {and_name_stringified}\"");
                                additional_parameters_and_stringified.parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_parameters_and_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            quote::quote!{
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
                                                    Err(e) => {
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
                        quote::quote!{
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
                            let field_ident = element.field.ident.as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} {field_ident_is_none_stringified}")
                                });
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
                        let binded_query_primary_key_modifications_token_stream = quote::quote!{
                            if let Some(#primary_key_field_ident) = #parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident {
                                for element in #primary_key_field_ident {
                                    query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(element, query);
                                }
                            }
                        };
                        quote::quote!{
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
                                Err(e) => {
                                    let error = #try_operation_upper_camel_case_token_stream::from(e);
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            }
                        } {
                            match {
                                use #sqlx_row_token_stream;
                                row.try_get::<#sqlx_types_uuid_token_stream, #server_location_type_token_stream>(#primary_key_field_ident_quotes_token_stream)
                            } {
                                Ok(value) => {
                                    vec_values.push(
                                        #crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value),
                                    );
                                }
                                Err(e) => {
                                    let error = #try_operation_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream;
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            }
                        }
                        #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(vec_values)
                    }
                };
                quote::quote!{
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
            quote::quote!{
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream<'a>(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> #impl_axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(e) => {
                                    let error = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: e,
                                        #field_code_occurence_new_39b5c2be_b5d4_4d33_b3e9_152543c33dca_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            },
                            Err(err) => {
                                return err;
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
        let common_middlewares_error_syn_variants_from_impls = generate_common_middlewares_error_syn_variants_from_impls(
            common_middlewares_error_syn_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            additional_http_status_codes_error_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            &operation,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote!{
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &delete_many_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let (
        delete_one_token_stream,
        delete_one_http_request_test_token_stream
     ) = {
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
        let additional_http_status_codes_error_variants = vec![];//todo find out why rust analyzer crashes
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
            let full_additional_http_status_codes_error_variants = generate_full_additional_http_status_codes_error_variants(
                common_middlewares_error_syn_variants.iter().collect(),
                additional_http_status_codes_error_variants.iter().collect()
            );
            let type_variants_from_request_response_syn_variants_partial = {
                let mut type_variants_from_request_response = std::vec::Vec::with_capacity(
                    common_error_syn_variants.len() +
                    2
                );
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
                full_additional_http_status_codes_error_variants
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
                quote::quote!{
                    #derive_debug_to_schema_token_stream
                    pub struct #operation_payload_upper_camel_case_token_stream {
                        pub #primary_key_field_ident: #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                    }
                }
            };
            // println!("{payload_token_stream}");
            let payload_with_serialize_deserialize_token_stream = {
                quote::quote!{
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
                    quote::quote!{
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
                    quote::quote!{
                        impl std::convert::TryFrom<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream> for #operation_payload_upper_camel_case_token_stream {
                            type Error = #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_upper_camel_case_token_stream;
                            fn try_from(value: #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                                match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value.#primary_key_field_ident) {
                                    Ok(value) => Ok(Self { #primary_key_field_ident: value }),
                                    Err(e) => Err(Self::Error::#not_uuid_token_upper_camel_case_stream {
                                        #not_uuid_token_snake_case_stream: e,
                                        #field_code_occurence_new_66343753_b4dc_4b64_b7a6_3f206033a0b1_token_stream,
                                    }),
                                }
                            }
                        }
                    }
                };
                quote::quote!{
                    #operation_payload_try_from_operation_payload_with_serialize_deserialize_error_named_token_stream
                    #impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream
                }
            };
            // println!("{impl_std_convert_try_from_operation_payload_with_serialize_deserialize_for_operation_payload_token_stream}");
            let impl_std_convert_from_operation_payload_for_operation_payload_with_serialize_deserialize_token_stream = {
                quote::quote!{
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
            quote::quote!{
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
                &code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                &derive_debug_thiserror_error_occurence_token_stream,
                &eo_display_token_stream,
                &eo_display_foreign_type_token_stream,
                &eo_display_with_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_token_stream,
                &derive_debug_serialize_deserialize_to_schema_token_stream,
                type_variants_from_request_response_syn_variants.clone(),//todo remove clone
                &proc_macro_name_upper_camel_case_ident_stringified,
                &operation,
            )
        };
        // println!("{try_operation_error_with_middleware_error_variants_token_stream}");
        let (
            http_request_token_stream,
            http_request_test_token_stream
         ) = {
            let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(&operation);
            let try_operation_error_named_token_stream = {
                let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(&operation);
                quote::quote!{
                    #derive_debug_thiserror_error_occurence_token_stream
                    pub enum #try_operation_error_named_upper_camel_case_token_stream {
                        #request_error_upper_camel_case_token_stream {
                            #eo_error_occurence_attribute_token_stream
                            #request_error_snake_case_token_stream: #try_operation_request_error_upper_camel_case_token_stream,
                            #code_occurence_snake_case_double_dot_space_error_occurence_lib_code_occurence_code_occurence_token_stream,
                        },
                        #http_request_error_named_serde_json_to_string_variant_token_stream,
                        #operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_declaration_token_stream,
                    }
                }
            };
            // println!("{try_operation_error_named_token_stream}");
            let http_request_token_stream = generate_try_operation_token_stream_new(
                &server_location_name_token_stream,
                &server_location_type_token_stream,
                &crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
                &quote::quote!{
                    //todo maybe for all cases use this? = remove this parameter and write it inside generate_try_operation_token_stream
                    let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(&#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)) {
                        Ok(value) => value,
                        Err(e) => {
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#serde_json_to_string_variant_initialization_token_stream);
                        }
                    };
                },
                &reqwest_client_new_token_stream,
                &commit_header_addition_token_stream,
                &content_type_application_json_header_addition_token_stream,
                &quote::quote!{
                    match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(value) {
                        Ok(value) => Ok(value),
                        Err(e) => Err(#try_operation_error_named_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client_one_initialization_token_stream )
                    }
                },
                &request_error_variant_initialization_token_stream,
                &table_name_stringified,
                &operation,
                &proc_macro_name_upper_camel_case_ident_stringified,
                type_variants_from_request_response_syn_variants,
                &desirable_status_code,
                &crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream,
            );
            let http_request_test_token_stream = {
                let test_content_token_stream = quote::quote!{
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
                        Err(e) => panic!("{e}")
                    }
                };
                proc_macro_helpers::naming_conventions::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            };
            (
                quote::quote!{
                    #try_operation_error_named_token_stream
                    #http_request_token_stream
                },
                http_request_test_token_stream
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
                            let query_part_stringified = format!("\" {primary_key_field_ident} = $1\"");//todo where
                            query_part_stringified.parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {query_part_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        quote::quote!{
                            query.push_str(&format!(#query_part_token_stream));
                        }
                    };
                    let handle_token_stream = {
                        let handle_stringified = format!("\"{delete_name_stringified} {from_name_stringified} {table_name_stringified} {where_name_stringified}\"");//todo where
                        handle_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {handle_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote!{
                        let mut query = format!(#handle_token_stream);
                        #additional_parameters_primary_key_modification_token_stream
                        query.push_str(&format!(#returning_primary_key_quotes_token_stream));
                        query
                    }
                };
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = quote::quote!{
                        query = #crate_server_postgres_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case_token_stream.#payload_snake_case_token_stream.#primary_key_field_ident, query);
                    };
                    quote::quote!{
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_name_token_stream);
                        #binded_query_modifications_token_stream
                        query
                    }
                };
                let from_log_and_return_error_token_stream = crate::from_log_and_return_error::from_log_and_return_error(
                    &try_operation_upper_camel_case_token_stream,
                    &error_log_call_token_stream,
                    &try_operation_response_variants_token_stream,
                );
                let acquire_pool_and_connection_token_stream = crate::acquire_pool_and_connection::acquire_pool_and_connection(
                    &from_log_and_return_error_token_stream,
                    &pg_connection_token_stream
                );
                quote::quote!{
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
                            value.try_get::<#sqlx_types_uuid_token_stream, #server_location_type_token_stream>(#primary_key_field_ident_quotes_token_stream)
                        } {
                            Ok(value) => #try_operation_response_variants_token_stream::#desirable_upper_camel_case_token_stream(#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(value)),
                            Err(e) => {
                                let error = #try_operation_upper_camel_case_token_stream::#operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server_initialization_token_stream;
                                #error_log_call_token_stream
                                return #try_operation_response_variants_token_stream::from(error);
                            }
                        },
                        Err(e) => {
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
            quote::quote!{
                #swagger_open_api_token_stream
                pub async fn #operation_snake_case_token_stream<'a>(
                    #app_state_name_token_stream: #axum_extract_state_token_stream<#app_state_path>,
                    #payload_extraction_result_snake_case_token_stream: Result<
                        #axum_json_token_stream<#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream>,
                        #axum_extract_rejection_json_rejection_token_stream,
                    >,
                ) -> #impl_axum_response_into_response_token_stream {
                    let #parameters_snake_case_token_stream = #operation_parameters_upper_camel_case_token_stream {
                        #payload_snake_case_token_stream: match #crate_server_routes_helpers_json_extractor_error_json_value_result_extractor_token_stream::<
                            #operation_payload_with_serialize_deserialize_upper_camel_case_token_stream,
                            #try_operation_response_variants_token_stream,
                        >::#try_extract_value_token_stream(#payload_extraction_result_snake_case_token_stream, &#app_state_name_token_stream)
                        {
                            Ok(value) => match #operation_payload_upper_camel_case_token_stream::try_from(value) {
                                Ok(value) => value,
                                Err(e) => {
                                    let error = #try_operation_upper_camel_case_token_stream::#operation_payload_try_from_operation_payload_with_serialize_deserialize_upper_camel_case_token_stream {
                                        #operation_payload_try_from_operation_payload_with_serialize_deserialize_snake_case_token_stream: e,
                                        #field_code_occurence_new_32b2a167_ab66_4ee6_8e59_3839fa83d830_token_stream,
                                    };
                                    #error_log_call_token_stream
                                    return #try_operation_response_variants_token_stream::from(error);
                                }
                            },
                            Err(err) => {
                                return err;
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
        let common_middlewares_error_syn_variants_from_impls = generate_common_middlewares_error_syn_variants_from_impls(
            common_middlewares_error_syn_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            additional_http_status_codes_error_variants.iter().collect::<std::vec::Vec<&(
                syn::Ident,
                proc_macro2::TokenStream,
                std::vec::Vec::<syn::Variant>
            )>>(),
            &operation,
            &proc_macro_name_upper_camel_case_ident_stringified,
        );
        // println!("{common_middlewares_error_syn_variants_from_impls}");
        (
            quote::quote!{
                #parameters_token_stream
                #try_operation_error_with_middleware_error_variants_token_stream
                #http_request_token_stream
                #route_handler_token_stream
                #common_middlewares_error_syn_variants_from_impls
            },
            http_request_test_token_stream
        )
    };
    // proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &delete_one_token_stream,
    //     &proc_macro_name_upper_camel_case_ident_stringified
    // );
    let emulate_crud_api_usage_test_token_stream = {
        let ident_emulate_crud_api_usage_test_snake_case_token_stream = {
            let ident_emulate_crud_api_usage_test_snake_case_stringified = format!("{ident_snake_case_stringified}_emulate_crud_api_usage_test");
            ident_emulate_crud_api_usage_test_snake_case_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_emulate_crud_api_usage_test_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        quote::quote! {
            #[test]
            fn #ident_emulate_crud_api_usage_test_snake_case_token_stream() {
                async fn find_out_if_it_works() {
                    let api_location = std::string::String::from("http://127.0.0.1:8080");
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
                    Err(e) => {
                        panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {e:#?}")
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
    //todo pub and private impl quote group
    let gen = quote::quote! {
        #common_token_stream

        #create_many_token_stream
        #create_one_token_stream
        #read_many_token_stream
        #read_one_token_stream
        #update_many_token_stream
        #update_one_token_stream
        #delete_many_token_stream
        #delete_one_token_stream
    };
    // if ident == "" {
    //     proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         &proc_macro_name_upper_camel_case,
    //         &gen,
    //         &proc_macro_name_upper_camel_case_ident_stringified
    //     );
    // }
    gen.into()
}

fn generate_std_vec_vec_syn_punctuated_punctuated(
    parts_vec: &[&str],
    proc_macro_name_upper_camel_case_ident_stringified: &str
) -> syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2> {
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

#[derive(
    Debug, 
    enum_extension::EnumExtension,
    strum_macros::EnumIter,
    PartialEq,
    Eq,
)]
enum SupportedAttributeType {
    // Bool,
    // Bytea,
    // Char,
    // Name,
    // Int8,
    // Int2,
    // Int4,
    // Text,
    // Oid,
    // Json,
    // JsonArray,
    // Point,
    // Lseg,
    // Path,
    // Box,
    // Polygon,
    // Line,
    // LineArray,
    // Cidr,
    // CidrArray,
    // Float4,
    // Float8,
    // Unknown,
    // Circle,
    // CircleArray,
    // Macaddr8,
    // Macaddr8Array,
    // Macaddr,
    // Inet,
    // BoolArray,
    // ByteaArray,
    // CharArray,
    // NameArray,
    // Int2Array,
    // Int4Array,
    // TextArray,
    // BpcharArray,
    // VarcharArray,
    // Int8Array,
    // PointArray,
    // LsegArray,
    // PathArray,
    // BoxArray,
    // Float4Array,
    // Float8Array,
    // PolygonArray,
    // OidArray,
    // MacaddrArray,
    // InetArray,
    // Bpchar,
    // Varchar,
    // Date,
    // Time,
    // Timestamp,
    // TimestampArray,
    // DateArray,
    // TimeArray,
    // Timestamptz,
    // TimestamptzArray,
    // Interval,
    // IntervalArray,
    // NumericArray,
    // Timetz,
    // TimetzArray,
    // Bit,
    // BitArray,
    // Varbit,
    // VarbitArray,
    // Numeric,
    // Record,
    // RecordArray,
    // Uuid,
    // UuidArray,
    // Jsonb,
    // JsonbArray,
    // Int4Range,
    // Int4RangeArray,
    // NumRange,
    // NumRangeArray,
    // TsRange,
    // TsRangeArray,
    // TstzRange,
    // TstzRangeArray,
    // DateRange,
    // DateRangeArray,
    // Int8Range,
    // Int8RangeArray,
    // Jsonpath,
    // JsonpathArray,
    // Money,
    // MoneyArray,

    // // https://www.postgresql.org/docs/9.3/datatype-pseudo.html
    // Void,

    // // A realized user-defined type. When a connection sees a DeclareXX variant it resolves
    // // into this one before passing it along to `accepts` or inside of `Value` objects.
    // Custom(Arc<PgCustomType>),

    // // From [`PgTypeInfo::with_name`]
    // DeclareWithName(UStr),

    // // NOTE: Do we want to bring back type declaration by ID? It's notoriously fragile but
    // //       someone may have a user for it
    // DeclareWithOid(Oid),

    /////////////////////////////
    Bool,
    Char,
    Smallint,
    Smallserial,
    Int2,
    Int,
    Serial, 
    Int4,
    Bigint,
    Bigserial, 
    Int8,
    Real, 
    Float4,
    DoublePrecision,
    Float8,
    Varchar,
    Charn, //CHAR(N) wtf????
    Text,
    Name,
    Bytea,
    Void,
    Interval,
    Int8range,
    Int4range,
    Tsrange,
    Tstzrange,
    Daterange,
    Numrange,
    Money,
    Ltree,
    Lquery,

    Numeric,

    Timestamptz,
    Timestamp,
    Date,
    Time,
    Timetz,
    Uuid,

    Inet,
    Cidr,

    Macaddr,

    Bit,
    Varbit,

    Json,
    Jsonb
}

impl std::fmt::Display for SupportedAttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "generate_postgresql_crud_bool"),
            Self::Char => write!(f, "generate_postgresql_crud_char"),
            Self::Smallint => write!(f, "generate_postgresql_crud_smallint"),
            Self::Smallserial => write!(f, "generate_postgresql_crud_smallserial"),
            Self::Int2 => write!(f, "generate_postgresql_crud_int2"),
            Self::Int => write!(f, "generate_postgresql_crud_int"),
            Self::Serial => write!(f, "generate_postgresql_crud_serial"), 
            Self::Int4 => write!(f, "generate_postgresql_crud_int4"),
            Self::Bigint => write!(f, "generate_postgresql_crud_bigint"),
            Self::Bigserial => write!(f, "generate_postgresql_crud_bigserial"), 
            Self::Int8 => write!(f, "generate_postgresql_crud_int8"),
            Self::Real => write!(f, "generate_postgresql_crud_real"), 
            Self::Float4 => write!(f, "generate_postgresql_crud_float4"),
            Self::DoublePrecision => write!(f, "generate_postgresql_crud_double_precision"),
            Self::Float8 => write!(f, "generate_postgresql_crud_float8"),
            Self::Varchar => write!(f, "generate_postgresql_crud_varchar"),
            Self::Charn => write!(f, "generate_postgresql_crud_charn"), //CHAR(N) wtf????
            Self::Text => write!(f, "generate_postgresql_crud_text"),
            Self::Name => write!(f, "generate_postgresql_crud_name"),
            Self::Bytea => write!(f, "generate_postgresql_crud_bytea"),
            Self::Void => write!(f, "generate_postgresql_crud_void"),
            Self::Interval => write!(f, "generate_postgresql_crud_interval"),
            Self::Int8range => write!(f, "generate_postgresql_crud_int8range"),
            Self::Int4range => write!(f, "generate_postgresql_crud_int4range"),
            Self::Tsrange => write!(f, "generate_postgresql_crud_tsrange"),
            Self::Tstzrange => write!(f, "generate_postgresql_crud_tstzrange"),
            Self::Daterange => write!(f, "generate_postgresql_crud_daterange"),
            Self::Numrange => write!(f, "generate_postgresql_crud_numrange"),
            Self::Money => write!(f, "generate_postgresql_crud_money"),
            Self::Ltree => write!(f, "generate_postgresql_crud_ltree"),
            Self::Lquery => write!(f, "generate_postgresql_crud_lquery"),

            Self::Numeric => write!(f, "generate_postgresql_crud_numeric"),

            Self::Timestamptz => write!(f, "generate_postgresql_crud_timestamptz"),
            Self::Timestamp => write!(f, "generate_postgresql_crud_timestamp"),
            Self::Date => write!(f, "generate_postgresql_crud_date"),
            Self::Time => write!(f, "generate_postgresql_crud_time"),
            Self::Timetz => write!(f, "generate_postgresql_crud_timetz"),
            Self::Uuid => write!(f, "generate_postgresql_crud_uuid"),

            Self::Inet => write!(f, "generate_postgresql_crud_inet"),
            Self::Cidr => write!(f, "generate_postgresql_crud_cidr"),

            Self::Macaddr => write!(f, "generate_postgresql_crud_macaddr"),

            Self::Bit => write!(f, "generate_postgresql_crud_bit"),
            Self::Varbit => write!(f, "generate_postgresql_crud_varbit"),

            Self::Json => write!(f, "generate_postgresql_crud_json"),
            Self::Jsonb => write!(f, "generate_postgresql_crud_jsonb"),
        }
    }
}

fn try_match_supported_attribute_type_with_supported_field_type(
    supported_attribute_type: &SupportedAttributeType,
    supported_field_type: &SupportedFieldType,
) -> bool {
    match (supported_attribute_type, supported_field_type) {
        (SupportedAttributeType::Bool, SupportedFieldType::StdPrimitiveBool) => true,
        (SupportedAttributeType::Char, SupportedFieldType::StdPrimitiveI8) => true,
        (SupportedAttributeType::Smallint, SupportedFieldType::StdPrimitiveI16) => true,
        (SupportedAttributeType::Smallserial, SupportedFieldType::StdPrimitiveI16) => true,
        (SupportedAttributeType::Int2, SupportedFieldType::StdPrimitiveI16) => true,
        (SupportedAttributeType::Int, SupportedFieldType::StdPrimitiveI32) => true,
        (SupportedAttributeType::Serial, SupportedFieldType::StdPrimitiveI16) => true,
        (SupportedAttributeType::Int4, SupportedFieldType::StdPrimitiveI16) => true,
        (SupportedAttributeType::Bigint, SupportedFieldType::StdPrimitiveI64) => true,
        (SupportedAttributeType::Bigserial, SupportedFieldType::StdPrimitiveI64) => true,
        (SupportedAttributeType::Int8, SupportedFieldType::StdPrimitiveI64) => true,
        (SupportedAttributeType::Real, SupportedFieldType::StdPrimitiveF32) => true,
        (SupportedAttributeType::Float4, SupportedFieldType::StdPrimitiveF32) => true,
        (SupportedAttributeType::DoublePrecision, SupportedFieldType::StdPrimitiveF64) => true,
        (SupportedAttributeType::Float8, SupportedFieldType::StdPrimitiveF64) => true,
        (SupportedAttributeType::Varchar, SupportedFieldType::StdPrimitiveStr) => true,
        (SupportedAttributeType::Varchar, SupportedFieldType::StdStringString) => true,
        (SupportedAttributeType::Charn, SupportedFieldType::StdPrimitiveStr) => true, //CHAR(N) wtf????
        (SupportedAttributeType::Charn, SupportedFieldType::StdStringString) => true, //CHAR(N) wtf????
        (SupportedAttributeType::Text, SupportedFieldType::StdPrimitiveStr) => true,
        (SupportedAttributeType::Text, SupportedFieldType::StdStringString) => true,
        (SupportedAttributeType::Name, SupportedFieldType::StdPrimitiveStr) => true,
        (SupportedAttributeType::Name, SupportedFieldType::StdStringString) => true,
        (SupportedAttributeType::Bytea, SupportedFieldType::StdPrimitiveArrayStdPrimitiveU8) => {
            true
        }
        (SupportedAttributeType::Bytea, SupportedFieldType::StdVecVecStdPrimitiveU8) => true,
        (SupportedAttributeType::Void, SupportedFieldType::StdPrimitiveUnit) => true,
        (SupportedAttributeType::Interval, SupportedFieldType::SqlxPostgresTypesPgInterval) => true,
        //
        // (SupportedAttributeType::Int8range, SupportedFieldType::) => true,
        // (SupportedAttributeType::Int4range, SupportedFieldType::) => true,
        // (SupportedAttributeType::Tsrange, SupportedFieldType::) => true,
        // (SupportedAttributeType::Tstzrange, SupportedFieldType::) => true,
        // (SupportedAttributeType::Daterange, SupportedFieldType::) => true,
        // (SupportedAttributeType::Numrange, SupportedFieldType::) => true,
        //
        // SqlxPostgresTypesPgRangeStdPrimitiveI32,
        // SqlxPostgresTypesPgRangeStdPrimitiveI64,
        // SqlxPostgresTypesPgRangeSqlxTypesDecimal,
        // SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
        // SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
        // SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
        // SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset,
        // SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
        // SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
        // SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
        // SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
        // SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
        //
        (SupportedAttributeType::Money, SupportedFieldType::SqlxPostgresTypesPgMoney) => true,
        (SupportedAttributeType::Ltree, SupportedFieldType::SqlxPostgresTypesPgLTree) => true,
        (SupportedAttributeType::Lquery, SupportedFieldType::SqlxPostgresTypesPgLQuery) => true,

        (SupportedAttributeType::Numeric, SupportedFieldType::SqlxTypesBigDecimal) => true,
        (SupportedAttributeType::Numeric, SupportedFieldType::SqlxTypesDecimal) => true,

        (
            SupportedAttributeType::Timestamptz,
            SupportedFieldType::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
        ) => true,
        (
            SupportedAttributeType::Timestamptz,
            SupportedFieldType::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
        ) => true,
        (SupportedAttributeType::Timestamptz, SupportedFieldType::SqlxTypesTimeOffsetDateTime) => {
            true
        }
        (SupportedAttributeType::Timestamp, SupportedFieldType::SqlxTypesChronoNaiveDateTime) => {
            true
        }
        (SupportedAttributeType::Timestamp, SupportedFieldType::SqlxTypesTimePrimitiveDateTime) => {
            true
        }
        (SupportedAttributeType::Date, SupportedFieldType::SqlxTypesChronoNaiveDate) => true,
        (SupportedAttributeType::Date, SupportedFieldType::SqlxTypesTimeDate) => true,
        (SupportedAttributeType::Time, SupportedFieldType::SqlxTypesChronoNaiveTime) => true,
        (SupportedAttributeType::Time, SupportedFieldType::SqlxTypesTimeTime) => true,
        // (SupportedAttributeType::Timetz, SupportedFieldType::) => true,//todo
        (SupportedAttributeType::Uuid, SupportedFieldType::SqlxTypesUuid) => true,

        (SupportedAttributeType::Inet, SupportedFieldType::SqlxTypesIpnetworkIpNetwork) => true,
        // (SupportedAttributeType::Inet, SupportedFieldType::IpAddr) => true,//todo
        (SupportedAttributeType::Cidr, SupportedFieldType::SqlxTypesIpnetworkIpNetwork) => true,
        // (SupportedAttributeType::Cidr, SupportedFieldType::IpAddr) => true,//todo
        (SupportedAttributeType::Macaddr, SupportedFieldType::SqlxTypesMacAddressMacAddress) => {
            true
        }
        (SupportedAttributeType::Bit, SupportedFieldType::SqlxTypesBitVecStdPrimitiveU32) => true, //maybe not correct
        (SupportedAttributeType::Varbit, SupportedFieldType::SqlxTypesBitVecStdPrimitiveU32) => {
            true
        } //maybe not correct
        // (SupportedAttributeType::Json, SupportedFieldType::) => true,//todo
        // (SupportedAttributeType::Jsonb, SupportedFieldType::) => true,//todo
        _ => false,
    }
}

impl std::str::FromStr for SupportedAttributeType {
    type Err = std::string::String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "generate_postgresql_crud_bool" => Ok(Self::Bool),
            "generate_postgresql_crud_char" => Ok(Self::Char),
            "generate_postgresql_crud_smallint" => Ok(Self::Smallint),
            "generate_postgresql_crud_smallserial" => Ok(Self::Smallserial),
            "generate_postgresql_crud_int2" => Ok(Self::Int2),
            "generate_postgresql_crud_int" => Ok(Self::Int),
            "generate_postgresql_crud_serial" => Ok(Self::Serial), 
            "generate_postgresql_crud_int4" => Ok(Self::Int4),
            "generate_postgresql_crud_bigint" => Ok(Self::Bigint),
            "generate_postgresql_crud_bigserial" => Ok(Self::Bigserial), 
            "generate_postgresql_crud_int8" => Ok(Self::Int8),
            "generate_postgresql_crud_real" => Ok(Self::Real), 
            "generate_postgresql_crud_float4" => Ok(Self::Float4),
            "generate_postgresql_crud_double_precision" => Ok(Self::DoublePrecision),
            "generate_postgresql_crud_float8" => Ok(Self::Float8),
            "generate_postgresql_crud_varchar" => Ok(Self::Varchar),
            "generate_postgresql_crud_charn" => Ok(Self::Charn), //CHAR(N) wtf????
            "generate_postgresql_crud_text" => Ok(Self::Text),
            "generate_postgresql_crud_name" => Ok(Self::Name),
            "generate_postgresql_crud_bytea" => Ok(Self::Bytea),
            "generate_postgresql_crud_void" => Ok(Self::Void),
            "generate_postgresql_crud_interval" => Ok(Self::Interval),
            "generate_postgresql_crud_int8range" => Ok(Self::Int8range),
            "generate_postgresql_crud_int4range" => Ok(Self::Int4range),
            "generate_postgresql_crud_tsrange" => Ok(Self::Tsrange),
            "generate_postgresql_crud_tstzrange" => Ok(Self::Tstzrange),
            "generate_postgresql_crud_daterange" => Ok(Self::Daterange),
            "generate_postgresql_crud_numrange" => Ok(Self::Numrange),
            "generate_postgresql_crud_money" => Ok(Self::Money),
            "generate_postgresql_crud_ltree" => Ok(Self::Ltree),
            "generate_postgresql_crud_lquery" => Ok(Self::Lquery),

            "generate_postgresql_crud_numeric" => Ok(Self::Numeric),

            "generate_postgresql_crud_timestamptz" => Ok(Self::Timestamptz),
            "generate_postgresql_crud_timestamp" => Ok(Self::Timestamp),
            "generate_postgresql_crud_date" => Ok(Self::Date),
            "generate_postgresql_crud_time" => Ok(Self::Time),
            "generate_postgresql_crud_timetz" => Ok(Self::Timetz),
            "generate_postgresql_crud_uuid" => Ok(Self::Uuid),

            "generate_postgresql_crud_inet" => Ok(Self::Inet),
            "generate_postgresql_crud_cidr" => Ok(Self::Cidr),

            "generate_postgresql_crud_macaddr" => Ok(Self::Macaddr),

            "generate_postgresql_crud_bit" => Ok(Self::Bit),
            "generate_postgresql_crud_varbit" => Ok(Self::Varbit),

            "generate_postgresql_crud_json" => Ok(Self::Json),
            "generate_postgresql_crud_jsonb" => Ok(Self::Jsonb),
            _ => Err(format!(
                "unsupported field attribute name: {value}, {:?}",
                Self::into_array().into_iter().map(|element|element.to_string()).collect::<std::vec::Vec<std::string::String>>()
            ))
        }
    }
}

// struct Field
//for now its better to check types manually to remove potential problems with token generation https://docs.rs/sqlx/0.7.2/sqlx/trait.Type.html
#[derive(
    Debug, 
    enum_extension::EnumExtension,
    strum_macros::EnumIter,
    PartialEq,
    Eq,
)]
enum SupportedFieldType {
    StdPrimitiveI16,
    StdPrimitiveStr,
    StdPrimitiveI64,
    StdPrimitiveI32,
    StdPrimitiveF64,
    StdPrimitiveF32,
    StdStringString,
    StdPrimitiveI8,
    StdPrimitiveBool,
    StdVecVecStdPrimitiveU8,
    StdPrimitiveArrayStdPrimitiveU8,
    StdPrimitiveUnit,
    //
    SqlxTypesDecimal,
    SqlxTypesBigDecimal,
    SqlxTypesTimeTime,
    SqlxTypesTimeDate,
    SqlxTypesChronoNaiveDate,
    SqlxTypesChronoNaiveDateTime,
    SqlxTypesChronoNaiveTime,
    SqlxTypesTimeOffsetDateTime,
    SqlxTypesTimePrimitiveDateTime,
    CoreTimeDuration,//todo maybe its std::time::Duration or core::time::Duration or both?
    SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset,
    SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    SqlxTypesUuid,
    SqlxTypesIpnetworkIpNetwork,
    SqlxTypesMacAddressMacAddress,
    SqlxPostgresTypesPgInterval,
    SqlxPostgresTypesPgMoney,
    SqlxPostgresTypesPgLQuery,
    SqlxPostgresTypesPgLTree,
    SqlxPostgresTypesOid,
    SqlxTypesBitVecStdPrimitiveU32,
    SqlxPostgresTypesTimeTzPgTimeTzSqlxTypesTimeTimeSqlxTypesTimeUtcOffset,
    SqlxPostgresTypesTimeTzPgTimeTzSqlxTypesChronoNaiveTimeSqlxTypesChronoFixedOffset,
    StdBoxedBoxBorrowStdPrimitiveStr,//std::boxed::Box<&std::primitive::str>
    StdBorrowCowAnonymousLifetimeStdPrimitiveStr,//std::borrow::Cow<'_, std::primitive::str>
    //
    SqlxPostgresTypesPgRangeStdPrimitiveI32,
    SqlxPostgresTypesPgRangeStdPrimitiveI64,
    SqlxPostgresTypesPgRangeSqlxTypesDecimal,
    SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
    SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset,
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
    SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
    SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,

    //todo this implementations

// impl Type<Postgres> for IpAddr // std::net::IpAddr
// where
//     IpNetwork: Type<Postgres>,
// impl<T, DB> Type<DB> for Option<T>
// where
//     T: Type<DB>,
//     DB: Database,

// impl<T, DB> Type<DB> for &T
// where
//     T: Type<DB> + ?Sized,
//     DB: Database,

// impl<DB> Type<DB> for Value
// where
//     Json<Value>: Type<DB>,
//     DB: Database,

// impl<DB> Type<DB> for RawValue
// where
//     Json<&'a RawValue>: for<'a> Type<DB>,
//     DB: Database,

// impl<T> Type<Postgres> for Vec<T, Global>
// where
//     T: PgHasArrayType,

// impl<T, const N: usize> Type<Postgres> for [T; N]
// where
//     T: PgHasArrayType,

// impl<T> Type<Postgres> for Json<T>

// impl<T> Type<Postgres> for [T]
// where
//     T: PgHasArrayType,

// impl<T1> Type<Postgres> for (T1,)

// impl<T1, T2> Type<Postgres> for (T1, T2)

// impl<T1, T2, T3> Type<Postgres> for (T1, T2, T3)

// impl<T1, T2, T3, T4> Type<Postgres> for (T1, T2, T3, T4)

// impl<T1, T2, T3, T4, T5> Type<Postgres> for (T1, T2, T3, T4, T5)

// impl<T1, T2, T3, T4, T5, T6> Type<Postgres> for (T1, T2, T3, T4, T5, T6)

// impl<T1, T2, T3, T4, T5, T6, T7> Type<Postgres> for (T1, T2, T3, T4, T5, T6, T7)

// impl<T1, T2, T3, T4, T5, T6, T7, T8> Type<Postgres> for (T1, T2, T3, T4, T5, T6, T7, T8)

// impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> Type<Postgres> for (T1, T2, T3, T4, T5, T6, T7, T8, T9)

    // Bool,
    // StdPrimitiveI8,
    // StdPrimitiveI16,
    // StdPrimitiveI32,
    // StdPrimitiveI64,
    // StdPrimitiveF32,
    // StdPrimitiveF64,
    // BorrowStdPrimitiveStr, 
    // StdStringString,
    // BorrowU8Array, 
    // StdVecVecU8,
    // Unit,
    // SqlxPostgresTypesPgInterval,
    // SqlxPostgresTypesPgRangeGeneric,
    // SqlxPostgresTypesPgMoney,
    // SqlxPostgresTypesPgLTree,
    // SqlxPostgresTypesPgLQuery,
    // BigdecimalBigDecimal,
    // RustDecimalDecimal,

    // ChronoDateTimeUtcGeneric,
    // ChronoDateTimeLocalGeneric,
    // ChronoNaiveDateTime,
    // ChronoNaiveDate,
    // ChronoNaiveTime,
    // ChronoPgTimeTzArray,//todo find out from what crate this type


    // TimePrimitiveDateTime,
    // TimeOffsetDateTime,
    // TimeDate,
    // TimeTime,
    // TimePgTimeTzArray,//todo find out from what crate this type


    // UuidUuid,


    // IpnetworkIpNetwork,
    // StdNetIpAddr,

    // MacAddressMacAddress,

    // BitVecBitVec,

    // Json,//tood full path
    // SerdeJsonValue,
    // BorrowSerdeJsonValueRawValue
}

impl std::fmt::Display for SupportedFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::StdPrimitiveI16 => write!(f, "std::primitive::i16"),
            Self::StdPrimitiveStr => write!(f, "std::primitive::str"),//todo borrow?
            Self::StdPrimitiveI64 => write!(f, "std::primitive::i64"),
            Self::StdPrimitiveI32 => write!(f, "std::primitive::i32"),
            Self::StdPrimitiveF64 => write!(f, "std::primitive::f64"),
            Self::StdPrimitiveF32 => write!(f, "std::primitive::f32"),
            Self::StdStringString => write!(f, "std::string::String"),
            Self::StdPrimitiveI8 => write!(f, "std::primitive::i8"),
            Self::StdPrimitiveBool => write!(f, "std::primitive::bool"),
            Self::StdVecVecStdPrimitiveU8 => write!(f, "std::vec::Vec<std::primitive::u8>"),
            Self::StdPrimitiveArrayStdPrimitiveU8 => write!(f, "[std::primitive::u8]"),
            Self::StdPrimitiveUnit => write!(f, "()"),

            Self::SqlxTypesDecimal => write!(f, "sqlx::types::Decimal"),
            Self::SqlxTypesBigDecimal => write!(f, "sqlx::types::BigDecimal"),
            Self::SqlxTypesTimeTime => write!(f, "sqlx::types::time::Time"),
            Self::SqlxTypesTimeDate => write!(f, "sqlx::types::time::Date"),
            Self::SqlxTypesChronoNaiveDate => write!(f, "sqlx::types::chrono::NaiveDate"),
            Self::SqlxTypesChronoNaiveDateTime => write!(f, "sqlx::types::chrono::NaiveDateTime"),
            Self::SqlxTypesChronoNaiveTime => write!(f, "sqlx::types::chrono::NaiveTime"),
            Self::SqlxTypesTimeOffsetDateTime => write!(f, "sqlx::types::time::OffsetDateTime"),
            Self::SqlxTypesTimePrimitiveDateTime => write!(f, "sqlx::types::time::PrimitiveDateTime"),
            Self::CoreTimeDuration => write!(f, "core::time::Duration"),//todo maybe its std::time::Duration or core::time::Duration or both?
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset => write!(f, "sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>"),
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal => write!(f, "sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>"),
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc => write!(f, "sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>"),
            Self::SqlxTypesUuid => write!(f, "sqlx::types::Uuid"),
            Self::SqlxTypesIpnetworkIpNetwork => write!(f, "sqlx::types::ipnetwork::IpNetwork"),
            Self::SqlxTypesMacAddressMacAddress => write!(f, "sqlx::types::mac_address::MacAddress"),
            Self::SqlxPostgresTypesPgInterval => write!(f, "sqlx_postgres::types::PgInterval"),
            Self::SqlxPostgresTypesPgMoney => write!(f, "sqlx_postgres::types::PgMoney"),
            Self::SqlxPostgresTypesPgLQuery => write!(f, "sqlx_postgres::types::PgLQuery"),
            Self::SqlxPostgresTypesPgLTree => write!(f, "sqlx_postgres::types::PgLTree"),
            Self::SqlxPostgresTypesOid => write!(f, "sqlx_postgres::types::Oid"),
            Self::SqlxTypesBitVecStdPrimitiveU32 => write!(f, "sqlx::types::BitVec<std::primitive::u32>"),
            Self::SqlxPostgresTypesTimeTzPgTimeTzSqlxTypesTimeTimeSqlxTypesTimeUtcOffset => write!(f, "sqlx_postgres::types::time_tz::PgTimeTz<sqlx::types::time::Time,sqlx::types::time::UtcOffset>"),
            Self::SqlxPostgresTypesTimeTzPgTimeTzSqlxTypesChronoNaiveTimeSqlxTypesChronoFixedOffset => write!(f, "sqlx_postgres::types::time_tz::PgTimeTz<sqlx::types::chrono::NaiveTime,sqlx::types::chrono::FixedOffset>"),
            Self::StdBoxedBoxBorrowStdPrimitiveStr => write!(f, "std::boxed::Box<&std::primitive::str>"),
            Self::StdBorrowCowAnonymousLifetimeStdPrimitiveStr => write!(f, "std::borrow::Cow<'_, std::primitive::str>"),

            Self::SqlxPostgresTypesPgRangeStdPrimitiveI32 => write!(f, "sqlx_postgres::types::PgRange<std::primitive::i32>"),
            Self::SqlxPostgresTypesPgRangeStdPrimitiveI64 => write!(f, "sqlx_postgres::types::PgRange<std::primitive::i64>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::Decimal>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::BigDecimal>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::TimeDate>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::chrono::NaiveDate>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::chrono::DateTime,sqlx::types::chrono::FixedOffset>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::chrono::DateTime,sqlx::types::ChronoLocal>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::chrono::DateTime,sqlx::types::chrono::Utc>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>"),
            Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime => write!(f, "sqlx_postgres::types::PgRange<sqlx::types::time::OffsetDateTime>"),

            // //todo some of types equal few ruest token streams not only one  https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html https://docs.rs/sqlx/0.7.2/sqlx/trait.Type.html
            // Self::Bool => write!(f, "std::primitive::bool"),
            // Self::StdPrimitiveI8 => write!(f, "std::primitive::i8"),
            // Self::StdPrimitiveI16 => write!(f, "std::primitive::i16"),
            // Self::StdPrimitiveI32 => write!(f, "std::primitive::i32"),
            // Self::StdPrimitiveI64 => write!(f, "std::primitive::i64"),
            // Self::StdPrimitiveF32 => write!(f, "std::primitive::f32"),
            // Self::StdPrimitiveF64 => write!(f, "std::primitive::f64"),
            // Self::BorrowStdPrimitiveStr => write!(f, "&std::primitive::str"), 
            // Self::StdStringString => write!(f, "std::string::String"),
            // Self::BorrowU8Array => write!(f, "&[std::primitive::i8]"), 
            // Self::StdVecVecU8 => write!(f, "std::vec::Vec<std::primitive::i8>"),
            // Self::Unit => write!(f, "()"),
            // Self::SqlxPostgresTypesPgInterval => write!(f, ""),
            // Self::SqlxPostgresTypesPgRangeGeneric => write!(f, ""),
            // Self::SqlxPostgresTypesPgMoney => write!(f, ""),
            // Self::SqlxPostgresTypesPgLTree => write!(f, ""),
            // Self::SqlxPostgresTypesPgLQuery => write!(f, ""),
            // Self::BigdecimalBigDecimal => write!(f, ""),
            // Self::RustDecimalDecimal => write!(f, ""),

            // Self::ChronoDateTimeUtcGeneric => write!(f, ""),
            // Self::ChronoDateTimeLocalGeneric => write!(f, ""),
            // Self::ChronoNaiveDateTime => write!(f, ""),
            // Self::ChronoNaiveDate => write!(f, ""),
            // Self::ChronoNaiveTime => write!(f, ""),
            // Self::ChronoPgTimeTzArray => write!(f, ""),//todo find out from what crate this type


            // Self::TimePrimitiveDateTime => write!(f, ""),
            // Self::TimeOffsetDateTime => write!(f, ""),
            // Self::TimeDate => write!(f, ""),
            // Self::TimeTime => write!(f, ""),
            // Self::TimePgTimeTzArray => write!(f, ""),//todo find out from what crate this type


            // Self::UuidUuid => write!(f, ""),


            // Self::IpnetworkIpNetwork => write!(f, ""),
            // Self::StdNetIpAddr => write!(f, ""),

            // Self::MacAddressMacAddress => write!(f, ""),

            // Self::BitVecBitVec => write!(f, ""),

            // Self::Json => write!(f, ""),//tood full path
            // Self::SerdeJsonValue => write!(f, ""),
            // Self::BorrowSerdeJsonValueRawValue => write!(f, "")
        }
    }
}

impl std::str::FromStr for SupportedFieldType {
    type Err = std::string::String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "std::primitive::i16" => Ok(Self::StdPrimitiveI16),
            "std::primitive::str" => Ok(Self::StdPrimitiveStr),//todo - maybe not allow str?
            "std::primitive::i64" => Ok(Self::StdPrimitiveI64),
            "std::primitive::i32" => Ok(Self::StdPrimitiveI32),
            "std::primitive::f64" => Ok(Self::StdPrimitiveF64),
            "std::primitive::f32" => Ok(Self::StdPrimitiveF32),
            "std::string::String" => Ok(Self::StdStringString),
            "std::primitive::i8" => Ok(Self::StdPrimitiveI8),
            "std::primitive::bool" => Ok(Self::StdPrimitiveBool),
            "std::vec::Vec<std::primitive::u8>" => Ok(Self::StdVecVecStdPrimitiveU8),
            "[std::primitive::u8]" => Ok(Self::StdPrimitiveArrayStdPrimitiveU8),
            "()" => Ok(Self::StdPrimitiveUnit),

            "sqlx::types::Decimal" => Ok(Self::SqlxTypesDecimal),
            "sqlx::types::BigDecimal" => Ok(Self::SqlxTypesBigDecimal),
            "sqlx::types::time::Time" => Ok(Self::SqlxTypesTimeTime),
            "sqlx::types::time::Date" => Ok(Self::SqlxTypesTimeDate),
            "sqlx::types::chrono::NaiveDate" => Ok(Self::SqlxTypesChronoNaiveDate),
            "sqlx::types::chrono::NaiveDateTime" => Ok(Self::SqlxTypesChronoNaiveDateTime),
            "sqlx::types::chrono::NaiveTime" => Ok(Self::SqlxTypesChronoNaiveTime),
            "sqlx::types::time::OffsetDateTime" => Ok(Self::SqlxTypesTimeOffsetDateTime),
            "sqlx::types::time::PrimitiveDateTime" => Ok(Self::SqlxTypesTimePrimitiveDateTime),
            "core::time::Duration" => Ok(Self::CoreTimeDuration),//todo maybe its std::time::Duration or core::time::Duration or both?
            "sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset),
            "sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoLocal),
            "sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>" => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc),
            "sqlx::types::Uuid" => Ok(Self::SqlxTypesUuid),
            "sqlx::types::ipnetwork::IpNetwork" => Ok(Self::SqlxTypesIpnetworkIpNetwork),
            "sqlx::types::mac_address::MacAddress" => Ok(Self::SqlxTypesMacAddressMacAddress),
            "sqlx_postgres::types::PgInterval" => Ok(Self::SqlxPostgresTypesPgInterval),
            "sqlx_postgres::types::PgMoney" => Ok(Self::SqlxPostgresTypesPgMoney),
            "sqlx_postgres::types::PgLQuery" => Ok(Self::SqlxPostgresTypesPgLQuery),
            "sqlx_postgres::types::PgLTree" => Ok(Self::SqlxPostgresTypesPgLTree),
            "sqlx_postgres::types::Oid" => Ok(Self::SqlxPostgresTypesOid),
            "sqlx::types::BitVec<std::primitive::u32>" => Ok(Self::SqlxTypesBitVecStdPrimitiveU32),
            "sqlx_postgres::types::time_tz::PgTimeTz<sqlx::types::time::Time,sqlx::types::time::UtcOffset>" => Ok(Self::SqlxPostgresTypesTimeTzPgTimeTzSqlxTypesTimeTimeSqlxTypesTimeUtcOffset),
            "sqlx_postgres::types::time_tz::PgTimeTz<sqlx::types::chrono::NaiveTime,sqlx::types::chrono::FixedOffset" => Ok(Self::SqlxPostgresTypesTimeTzPgTimeTzSqlxTypesChronoNaiveTimeSqlxTypesChronoFixedOffset),
            "std::boxed::Box<&std::primitive::str>" => Ok(Self::StdBoxedBoxBorrowStdPrimitiveStr),
            "std::borrow::Cow<'_, std::primitive::str>" => Ok(Self::StdBorrowCowAnonymousLifetimeStdPrimitiveStr),

            "sqlx_postgres::types::PgRange<std::primitive::i32>" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI32),
            "sqlx_postgres::types::PgRange<std::primitive::i64>" => Ok(Self::SqlxPostgresTypesPgRangeStdPrimitiveI64),
            "sqlx_postgres::types::PgRange<sqlx::types::Decimal>" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesDecimal),
            "sqlx_postgres::types::PgRange<sqlx::types::BigDecimal>" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal),
            "sqlx_postgres::types::PgRange<sqlx::types::TimeDate>" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeDate),
            "sqlx_postgres::types::PgRange<sqlx::types::chrono::NaiveDate>" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate),
            "sqlx_postgres::types::PgRange<sqlx::types::chrono::DateTime,sqlx::types::chrono::FixedOffset>" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoFixedOffset),
            "sqlx_postgres::types::PgRange<sqlx::types::chrono::DateTime,sqlx::types::ChronoLocal>" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal),
            "sqlx_postgres::types::PgRange<sqlx::types::chrono::DateTime,sqlx::types::chrono::Utc>" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc),
            "sqlx_postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime),
            "sqlx_postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime),
            "" => Ok(Self::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime),

            // "std::primitive::bool" => Self::Bool,
            // "std::primitive::i8" => Self::StdPrimitiveI8,
            // "std::primitive::i16" => Self::StdPrimitiveI16,
            // "std::primitive::i32" => Self::StdPrimitiveI32,
            // "std::primitive::i64" => Self::StdPrimitiveI64,
            // "std::primitive::f32" => Self::StdPrimitiveF32,
            // "std::primitive::f64" => Self::StdPrimitiveF64,
            // "&std::primitive::str" => Self::BorrowStdPrimitiveStr, 
            // "std::string::String" => Self::StdStringString,
            // "&[std::primitive::i8]" => Self::BorrowU8Array, 
            // "std::vec::Vec<std::primitive::i8>" => Self::StdVecVecU8,
            // "()" => Self::Unit,
            // "sqlx_postgres::types::PgInterval" => Self::SqlxPostgresTypesPgInterval,
            // "sqlx_postgres::types::PgRange<sqlx::types::BigDecimal>" => Self::SqlxPostgresTypesPgRangeGeneric,
            // "sqlx_postgres::types::PgRange<sqlx::types::time::Date>" => Self::SqlxPostgresTypesPgRangeGeneric,
            // "sqlx_postgres::types::PgRange<sqlx::types::chrono::DateTime<Tz>>" => Self::SqlxPostgresTypesPgRangeGeneric,//todo generic parameter
            // "sqlx_postgres::types::PgRange<sqlx::types::Decimal>" => Self::SqlxPostgresTypesPgRangeGeneric,
            // "sqlx_postgres::types::PgRange<sqlx::types::chrono::NaiveDate>" => Self::SqlxPostgresTypesPgRangeGeneric,
            // "sqlx_postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>" => Self::SqlxPostgresTypesPgRangeGeneric,
            // "sqlx_postgres::types::PgRange<sqlx::types::time::OffsetDateTime>" => Self::SqlxPostgresTypesPgRangeGeneric,
            // "sqlx_postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>" => Self::SqlxPostgresTypesPgRangeGeneric,
            // "sqlx_postgres::types::PgRange<std::primitive::i32>" => Self::SqlxPostgresTypesPgRangeGeneric,
            // "sqlx_postgres::types::PgRange<std::primitive::i64>" => Self::SqlxPostgresTypesPgRangeGeneric,
            // "sqlx_postgres::types::PgMoney" => Self::SqlxPostgresTypesPgMoney,
            // "sqlx_postgres::types::PgLTree" => Self::SqlxPostgresTypesPgLTree,
            // "sqlx_postgres::types::PgLQuery" => Self::SqlxPostgresTypesPgLQuery,
            // "" => Self::BigdecimalBigDecimal,
            // "" => Self::RustDecimalDecimal,

            // "" => Self::ChronoDateTimeUtcGeneric,
            // "" => Self::ChronoDateTimeLocalGeneric,
            // "" => Self::ChronoNaiveDateTime,
            // "" => Self::ChronoNaiveDate,
            // "" => Self::ChronoNaiveTime,
            // "" => Self::ChronoPgTimeTzArray,//todo find out from what crate this type


            // "" => Self::TimePrimitiveDateTime,
            // "" => Self::TimeOffsetDateTime,
            // "" => Self::TimeDate,
            // "" => Self::TimeTime,
            // "" => Self::TimePgTimeTzArray,//todo find out from what crate this type


            // "" => Self::UuidUuid,


            // "" => Self::IpnetworkIpNetwork,
            // "" => Self::StdNetIpAddr,

            // "" => Self::MacAddressMacAddress,

            // "" => Self::BitVecBitVec,

            // "" => Self::Json,//tood full path
            // "" => Self::SerdeJsonValue,
            // "" => Self::BorrowSerdeJsonValueRawValue,
            _ => Err(std::string::String::default()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct FieldNamedWrapperExcludingPrimaryKey {
    field: syn::Field,
    supported_attribute_type: SupportedAttributeType,
    supported_field_type: SupportedFieldType
}

fn generate_common_middlewares_error_syn_variants_from_impls(
    common_middlewares_error_syn_variants: std::vec::Vec::<&(
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec::<syn::Variant>
    )>,
    additional_http_status_codes_error_variants: std::vec::Vec::<&(
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec::<syn::Variant>
    )>,
    operation: &Operation,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let error_syn_variants = generate_full_additional_http_status_codes_error_variants(
        common_middlewares_error_syn_variants,
        additional_http_status_codes_error_variants
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
    quote::quote!{#(#value)*}
}

fn generate_not_unique_field_vec_upper_camel_stringified(field_ident: &syn::Ident) -> std::string::String {
    format!("NotUnique{}Vec",
        {
            use convert_case::Casing;
            field_ident.to_string().to_case(convert_case::Case::UpperCamel)
        }
    )
}

fn generate_not_unique_field_vec_snake_case_stringified(field_ident: &syn::Ident) -> std::string::String {
    format!("not_unique_{field_ident}_vec")
}

fn generate_self_fields_token_stream<'a>(
    fields: &[&'a syn::Field],
    proc_macro_name_upper_camel_case_ident_stringified: &'a str,
) -> std::vec::Vec<&'a syn::Ident> {
    fields.iter().map(|field|{
        field.ident.as_ref().unwrap_or_else(|| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                naming_constants::FIELD_IDENT_IS_NONE
            )
        })
    }).collect()   
}

fn generate_pub_field_ident_field_type_token_stream(
    element: &FieldNamedWrapperExcludingPrimaryKey,
    proc_macro_name_upper_camel_case_ident_stringified: &str
) -> proc_macro2::TokenStream {
    let field_ident = element.field.ident.as_ref()
        .unwrap_or_else(|| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                naming_constants::FIELD_IDENT_IS_NONE
            )
        });
    let field_type = &element.field.ty;
    quote::quote!{
        pub #field_ident: #field_type
    }
}

fn generate_field_ident_field_type_with_serialize_deserialize_token_stream(
    element: &FieldNamedWrapperExcludingPrimaryKey,
    proc_macro_name_upper_camel_case_ident_stringified: &str
) -> proc_macro2::TokenStream {
    let field_ident = element.field.ident.as_ref()
        .unwrap_or_else(|| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                naming_constants::FIELD_IDENT_IS_NONE
            )
        });
    let field_type = &element.field.ty;
    quote::quote!{
        pub #field_ident: #field_type //todo with serialize deserialize conversion
    }
}

fn generate_let_field_ident_value_field_ident_try_from_token_stream(
    element: &FieldNamedWrapperExcludingPrimaryKey,
    proc_macro_name_upper_camel_case_ident_stringified: &str
) -> proc_macro2::TokenStream {
    let field_ident = element.field.ident.as_ref()
        .unwrap_or_else(|| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                naming_constants::FIELD_IDENT_IS_NONE
            )
        });
    quote::quote!{
        let #field_ident = value.#field_ident;//todo with serialize deserialize
    }
}

fn generate_let_field_ident_value_field_ident_from_token_stream(
    element: &FieldNamedWrapperExcludingPrimaryKey,
    proc_macro_name_upper_camel_case_ident_stringified: &str
) -> proc_macro2::TokenStream {
    let field_ident = element.field.ident.as_ref()
        .unwrap_or_else(|| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                naming_constants::FIELD_IDENT_IS_NONE
            )
        });
    quote::quote!{
        let #field_ident = value.#field_ident;
    }
}


fn generate_http_request_many_token_stream(
    server_location_name_token_stream: &proc_macro2::TokenStream,
    server_location_type_token_stream: &proc_macro2::TokenStream,
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
    request_error_variant_initialization_token_stream: &proc_macro2::TokenStream,
    table_name_stringified: &str,
    operation: &Operation,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    type_variants_from_request_response_syn_variants: std::vec::Vec<&syn::Variant>,
    desirable_status_code: &proc_macro_helpers::status_code::StatusCode,
    desirable_type_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let parameters_snake_case_token_stream = proc_macro_helpers::naming_conventions::parameters_snake_case_token_stream();
    let payload_snake_case_token_stream = proc_macro_helpers::naming_conventions::payload_snake_case_token_stream();
    let operation_http_method_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&operation.http_method());
    let url_handle_token_stream = proc_macro_helpers::naming_conventions::UrlHandleSelfSnakeCaseTokenStream::url_handle_self_snake_case_token_stream(operation, table_name_stringified);
    let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(operation);
    let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
    let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
    let operation_payload_with_serialize_deserialize_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfPayloadWithSerializeDeserializeUpperCamelCaseTokenStream::self_payload_with_serialize_deserialize_upper_camel_case_token_stream(operation);
    let type_variants_from_request_response_syn_variants_len = type_variants_from_request_response_syn_variants.len();
    let code_occurence_snake_case_stringified = proc_macro_helpers::naming_conventions::code_occurence_snake_case_stringified();
    let code_occurence_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::code_occurence_upper_camel_case_stringified();
    let try_operation_response_variants_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(operation);
    let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(operation);
    let http_status_code_quote_token_stream = desirable_status_code.to_http_status_code_token_stream();
    let (
        unique_status_codes,
        unique_status_codes_len,
        unique_status_codes_len_minus_one
     ) = {
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
        let unique_status_codes = hashmap_unique_status_codes.into_keys().collect::<std::vec::Vec<proc_macro_helpers::status_code::StatusCode>>();
        (
            unique_status_codes,
            unique_status_codes_len,
            unique_status_codes_len_minus_one
        )
    };
    let desirable_enum_name = {
        let status_code_enum_name_stingified = format!("{try_operation_response_variants_upper_camel_case_token_stream}{desirable_status_code}");
        status_code_enum_name_stingified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {status_code_enum_name_stingified} {}",proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let status_code_enums_try_from = {
        let mut is_last_element_found = false;
        let desirable_status_code_case_token_stream = quote::quote! {
            match serde_json::from_str::<#desirable_enum_name>(&response_text) {
                Ok(value) => #try_operation_response_variants_upper_camel_case_token_stream::from(value),
                Err(e) => {
                    let e = #try_operation_request_error_upper_camel_case_token_stream::DeserializeResponse {
                        serde: e, 
                        status_code, 
                        headers, 
                        response_text,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                                line: 793, 
                                column: 17,
                            })
                        )
                    };
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#request_error_variant_initialization_token_stream);
                }
            }
        };
        let mut status_code_enums_try_from_variants = std::vec::Vec::with_capacity(unique_status_codes_len + 1);
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
                            let e = #try_operation_request_error_upper_camel_case_token_stream::UnexpectedStatusCode {
                                status_code, 
                                headers, 
                                response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(response_text), 
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(), 
                                    file!().to_string(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                                        line: 819,
                                        column: 17,
                                    })
                                )
                            };
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::#request_error_variant_initialization_token_stream);
                        }
                    });
                },
                false => {
                    if *desirable_status_code != status_code_attribute {
                        status_code_enums_try_from_variants.push(quote::quote! {
                            else if status_code == #http_status_code_token_stream {
                                match serde_json::from_str::<#try_operation_response_variants_desirable_attribute_token_stream>(&response_text) {
                                    Ok(value) => #try_operation_response_variants_upper_camel_case_token_stream::from(value),
                                    Err(e) => {
                                        let e = #try_operation_request_error_upper_camel_case_token_stream::DeserializeResponse {
                                            serde: e, 
                                            status_code, 
                                            headers, 
                                            response_text,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                                                file!().to_string(),
                                                line!(), 
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                                                    line: 825, 
                                                    column: 17,
                                                })
                                            )
                                        };
                                        return Err(#try_operation_error_named_upper_camel_case_token_stream::#request_error_variant_initialization_token_stream);
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
    quote::quote!{
        pub async fn #try_operation_snake_case_token_stream<'a>(
            #server_location_name_token_stream: #server_location_type_token_stream,
            #parameters_snake_case_token_stream: #operation_parameters_upper_camel_case_token_stream,
        ) -> Result<
            #std_vec_vec_crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream,
            #try_operation_error_named_upper_camel_case_token_stream,
        > {
            let #payload_snake_case_token_stream = match #serde_json_to_string_token_stream(&#operation_payload_with_serialize_deserialize_upper_camel_case_token_stream::from(#parameters_snake_case_token_stream.#payload_snake_case_token_stream)) {
                Ok(value) => value,
                Err(e) => {
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
                Err(e) => {
                    let e = #try_operation_request_error_upper_camel_case_token_stream::Reqwest {
                        reqwest: e, 
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(), 
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                                line: 880,
                                column: 13,
                            })
                        ),
                    };
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#request_error_variant_initialization_token_stream);
                }
            };
            let status_code = response.status();
            let headers = response.headers().clone();
            let response_text = match response.text().await {
                Ok(response_text) => response_text,
                Err(e) => {
                    let e = #try_operation_request_error_upper_camel_case_token_stream::FailedToGetResponseText {
                        reqwest: e, 
                        status_code, 
                        headers, 
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(), 
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                                line: 886,
                                column: 13,
                            })
                        )
                    };
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#request_error_variant_initialization_token_stream);
                }
            };
            let variants = #(#status_code_enums_try_from)*;
            match #desirable_type_token_stream::try_from(variants) {
                Ok(value) => {
                    let mut vec_values = std::vec::Vec::with_capacity(value.len());
                    let mut vec_errors = std::vec::Vec::with_capacity(value.len());
                    for element in value {
                        match #crate_server_postgres_uuid_wrapper_uuid_wrapper_token_stream::try_from(element) {
                            Ok(value) => {
                                vec_values.push(value);
                            }
                            Err(e) => {
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
                Err(e) => {
                    let e = #try_operation_request_error_upper_camel_case_token_stream::ExpectedType {
                        expected_type: e, 
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(), 
                            file!().to_string(),
                            line!(), 
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                                line: 892, 
                                column: 13,
                            })
                        )
                    };
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#request_error_variant_initialization_token_stream);
                }
            }
        }
    }
}

fn generate_try_operation_token_stream_new(
    server_location_name_token_stream: &proc_macro2::TokenStream,
    server_location_type_token_stream: &proc_macro2::TokenStream,
    return_result_ok_type_token_stream: &proc_macro2::TokenStream,
    payload_variable_initialization_token_stream: &proc_macro2::TokenStream,
    reqwest_client_new_token_stream: &proc_macro2::TokenStream,
    commit_header_addition_token_stream: &proc_macro2::TokenStream,
    content_type_application_json_header_addition_token_stream: &proc_macro2::TokenStream,
    ok_value_handle_token_stream: &proc_macro2::TokenStream,
    request_error_variant_initialization_token_stream: &proc_macro2::TokenStream,
    table_name_stringified: &str,
    operation: &Operation,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    type_variants_from_request_response_syn_variants: std::vec::Vec<&syn::Variant>,
    desirable_status_code: &proc_macro_helpers::status_code::StatusCode,
    desirable_type_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let parameters_snake_case_token_stream = proc_macro_helpers::naming_conventions::parameters_snake_case_token_stream();
    let payload_snake_case_token_stream = proc_macro_helpers::naming_conventions::payload_snake_case_token_stream();
    let try_operation_error_named_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfErrorNamedUpperCamelCaseTokenStream::try_self_error_named_upper_camel_case_token_stream(operation);
    let operation_parameters_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::SelfParametersUpperCamelCaseTokenStream::self_parameters_upper_camel_case_token_stream(operation);
    let try_operation_snake_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfSnakeCaseTokenStream::try_self_snake_case_token_stream(operation);
    let tvfrr_extraction_logic_token_stream = proc_macro_helpers::naming_conventions::TvfrrExtractionLogicTrySelfSnakeCaseTokenStream::tvfrr_extraction_logic_try_self_snake_case_token_stream(operation);
    let operation_http_method_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&operation.http_method());
    let url_handle_token_stream = proc_macro_helpers::naming_conventions::UrlHandleSelfSnakeCaseTokenStream::url_handle_self_snake_case_token_stream(operation, table_name_stringified);
    let field_code_occurence_new_bb5cfe29_9fb8_4b44_ab37_fa1788741b94_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_881ec246_9f27_4f0f_a43e_8b8f34990bf6_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_50089027_fdac_495d_b3c4_07daf290f498_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_e1b395e2_6a5a_4231_9bd8_2ff16a8fd405_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_9135de27_94c6_4f8d_a3b6_2d617411ce7f_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_fe7c33e3_9d55_434a_9253_e17442ff0b59_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        proc_macro_name_upper_camel_case_ident_stringified,
    );
    let operation_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(operation);
    let code_occurence_snake_case_stringified = proc_macro_helpers::naming_conventions::code_occurence_snake_case_stringified();
    let type_variants_from_request_response_syn_variants_len = type_variants_from_request_response_syn_variants.len();
    let code_occurence_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::code_occurence_upper_camel_case_stringified();
    let try_operation_response_variants_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfResponseVariantsUpperCamelCaseTokenStream::try_self_response_variants_upper_camel_case_token_stream(operation);
    let try_operation_request_error_upper_camel_case_token_stream = proc_macro_helpers::naming_conventions::TrySelfRequestErrorUpperCamelCaseTokenStream::try_self_request_error_upper_camel_case_token_stream(operation);
    let axum_http_status_code_quote_token_stream = desirable_status_code.to_axum_http_status_code_token_stream();
    let http_status_code_quote_token_stream = desirable_status_code.to_http_status_code_token_stream();
    let desirable_enum_name = {
        let status_code_enum_name_stingified = format!("{try_operation_response_variants_upper_camel_case_token_stream}{desirable_status_code}");
        status_code_enum_name_stingified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {status_code_enum_name_stingified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let reqwest_error_token_stream = quote::quote!{reqwest::Error};//todo reuse
    let tvfrr_extraction_logic_try_operation_snake_case_token_stream = {
        let tvfrr_extraction_logic_try_operation_snake_case_stringified = format!("tvfrr_extraction_logic_try_{operation_snake_case_stringified}");
        tvfrr_extraction_logic_try_operation_snake_case_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {tvfrr_extraction_logic_try_operation_snake_case_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let (
        unique_status_codes,
        unique_status_codes_len,
        unique_status_codes_len_minus_one
     ) = {
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
        let unique_status_codes = hashmap_unique_status_codes.into_keys().collect::<std::vec::Vec<proc_macro_helpers::status_code::StatusCode>>();
        (
            unique_status_codes,
            unique_status_codes_len,
            unique_status_codes_len_minus_one
        )
    };
    let api_request_unexpected_error_module_path_token_stream = quote::quote! { crate::common::api_request_unexpected_error };
    let status_code_enums_try_from = {
        let mut is_last_element_found = false;
        let field_code_occurence_new_c0b27ae4_b521_4d30_bc4e_7a142e105150_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            proc_macro_name_upper_camel_case_ident_stringified,
        );
        let field_code_occurence_new_45a6d1a7_60da_4839_9669_9c259bbb0ca0_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            proc_macro_name_upper_camel_case_ident_stringified,
        );
        let desirable_status_code_case_token_stream = quote::quote! {
            match serde_json::from_str::<#desirable_enum_name>(&response_text) {
                Ok(value) => #try_operation_response_variants_upper_camel_case_token_stream::from(value),
                Err(e) => {
                    let request_error = #try_operation_request_error_upper_camel_case_token_stream::DeserializeResponse {
                        serde: e, 
                        status_code, 
                        headers, 
                        response_text,
                        #field_code_occurence_new_c0b27ae4_b521_4d30_bc4e_7a142e105150_token_stream
                    };
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::RequestError {
                        request_error, 
                        #field_code_occurence_new_45a6d1a7_60da_4839_9669_9c259bbb0ca0_token_stream
                    });
                }
            }
        };
        let mut status_code_enums_try_from_variants = std::vec::Vec::with_capacity(unique_status_codes_len + 1);
        status_code_enums_try_from_variants.push(quote::quote! {
            if status_code == #http_status_code_quote_token_stream {
                #desirable_status_code_case_token_stream
            }
        });
        let field_code_occurence_new_d3a22bcc_9490_4bf7_a9cb_6c9fb2903eb1_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            proc_macro_name_upper_camel_case_ident_stringified,
        );
        let field_code_occurence_new_f864c70c_7901_4d99_805b_7f1cc0a42d68_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            proc_macro_name_upper_camel_case_ident_stringified,
        );
        let field_code_occurence_new_94dc9e5f_2847_4a75_9f6a_dd59051e2176_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            proc_macro_name_upper_camel_case_ident_stringified,
        );
        let field_code_occurence_new_59e3ec45_4f7b_47af_bf9a_304ca0bce1fb_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            proc_macro_name_upper_camel_case_ident_stringified,
        );
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
                            let request_error = #try_operation_request_error_upper_camel_case_token_stream::UnexpectedStatusCode {
                                status_code, 
                                headers, 
                                response_text_result: #api_request_unexpected_error_module_path_token_stream::ResponseTextResult::ResponseText(response_text), 
                                #field_code_occurence_new_94dc9e5f_2847_4a75_9f6a_dd59051e2176_token_stream
                            };
                            return Err(#try_operation_error_named_upper_camel_case_token_stream::RequestError {
                                request_error, 
                                #field_code_occurence_new_59e3ec45_4f7b_47af_bf9a_304ca0bce1fb_token_stream
                            });
                        }
                    });
                },
                false => {
                    if *desirable_status_code != status_code_attribute {
                        let field_code_occurence_new_a7ef266e_859d_419e_8bd8_51d2d98ede01_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                            file!(),
                            line!(),
                            column!(),
                            proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        let field_code_occurence_new_6e6b0f6d_00fb_4301_89e2_1d720c440a02_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                            file!(),
                            line!(),
                            column!(),
                            proc_macro_name_upper_camel_case_ident_stringified,
                        );
                        status_code_enums_try_from_variants.push(quote::quote! {
                            else if status_code == #http_status_code_token_stream {
                                match serde_json::from_str::<#try_operation_response_variants_desirable_attribute_token_stream>(&response_text) {
                                    Ok(value) => #try_operation_response_variants_upper_camel_case_token_stream::from(value),
                                    Err(e) => {
                                        let e = #try_operation_request_error_upper_camel_case_token_stream::DeserializeResponse {
                                            serde: e, 
                                            status_code, 
                                            headers, 
                                            response_text,
                                            #field_code_occurence_new_a7ef266e_859d_419e_8bd8_51d2d98ede01_token_stream
                                        };
                                        return Err(#try_operation_error_named_upper_camel_case_token_stream::#request_error_variant_initialization_token_stream);
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
    let field_code_occurence_new_22184e52_6750_4972_b86d_eb906e576cda_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_ceba18fc_5452_4c63_8db6_b60723c5b344_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_c8f5e68_4b48_4ea3_8a75_6f7fc99be861_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        proc_macro_name_upper_camel_case_ident_stringified,
    );
    quote::quote!{
        pub async fn #try_operation_snake_case_token_stream<'a>(
            #server_location_name_token_stream: #server_location_type_token_stream,
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
                Err(e) => {
                    let e = #try_operation_request_error_upper_camel_case_token_stream::Reqwest {
                        reqwest: e, 
                        #field_code_occurence_new_bb5cfe29_9fb8_4b44_ab37_fa1788741b94_token_stream,
                    };
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#request_error_variant_initialization_token_stream);
                }
            };
            let status_code = response.status();
            let headers = response.headers().clone();
            let response_text = match response.text().await {
                Ok(response_text) => response_text,
                Err(e) => {
                    let e = #try_operation_request_error_upper_camel_case_token_stream::FailedToGetResponseText {
                        reqwest: e, 
                        status_code, 
                        headers, 
                        #field_code_occurence_new_50089027_fdac_495d_b3c4_07daf290f498_token_stream
                    };
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#request_error_variant_initialization_token_stream);
                }
            };
            let variants = #(#status_code_enums_try_from)*;
            match #desirable_type_token_stream::try_from(variants) {
                Ok(value) => #ok_value_handle_token_stream, 
                Err(e) => {
                    let e = #try_operation_request_error_upper_camel_case_token_stream::ExpectedType {
                        expected_type: e, 
                        #field_code_occurence_new_9135de27_94c6_4f8d_a3b6_2d617411ce7f_token_stream
                    };
                    return Err(#try_operation_error_named_upper_camel_case_token_stream::#request_error_variant_initialization_token_stream);
                },
            }
        }
    }
}

fn generate_type_variants_from_request_response_syn_variants<'a>(
    type_variants_from_request_response_syn_variants_partial: std::vec::Vec<&'a syn::Variant>,
    full_additional_http_status_codes_error_variants: std::vec::Vec::<&'a (
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec::<syn::Variant>
    )>,
) -> std::vec::Vec<&'a syn::Variant> {
    let mut handle = std::vec::Vec::with_capacity(type_variants_from_request_response_syn_variants_partial.len() + full_additional_http_status_codes_error_variants.len());
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
    let mut value = std::vec::Vec::with_capacity(type_variants_from_request_response_syn_variants.len());
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
    let content_type_snake_case_token_stream = quote::quote!{content_type};
    let description_snake_case_token_stream = quote::quote!{description};
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
    let method_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&operation.http_method());
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
        quote::quote!{
            request_body(
                content = #content_type_token_stream, 
                #description_snake_case_token_stream = #request_body_description_token_stream, 
                #content_type_snake_case_token_stream = #application_json_quotes_token_stream
            )
        }
    };
    let path_snake_case_token_stream = proc_macro_helpers::naming_conventions::path_snake_case_token_stream();
    quote::quote!{
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

#[derive(
    proc_macro_assistants::ToSnakeCaseStringified,
)]
enum OperationHttpMethod {
    Post,
    Patch,
    Delete
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
//                 Err(e) => {
//                     panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {e:#?}")
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
    common_middlewares_error_syn_variants: std::vec::Vec::<&'a (
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec::<syn::Variant>
    )>,
    additional_http_status_codes_error_variants: std::vec::Vec::<&'a (
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec::<syn::Variant>
    )>,
) -> std::vec::Vec::<&'a (
    syn::Ident,
    proc_macro2::TokenStream,
    std::vec::Vec::<syn::Variant>
)> {
    let mut handle = std::vec::Vec::with_capacity(common_middlewares_error_syn_variants.len() + additional_http_status_codes_error_variants.len());
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
    std_string_string_syn_punctuated_punctuated: syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>,
) -> syn::Variant {
    let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
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
            Self::Asc => quote::quote!{Asc},
            Self::Desc => quote::quote!{Desc},
        }
    }
}