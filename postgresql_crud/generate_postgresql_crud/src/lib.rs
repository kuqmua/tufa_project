//todo decide where to do error log (maybe add in some places)
//todo generate route what will return columns of the table and their rust and postgersql types
//todo created at and updated at fields + created by + updated by
//todo attributes for activation generation crud methods(like generate create, update_one, delete_one)
//todo authorization for returning concrete error or just minimal info(user role)
//todo generate rules and roles
//todo maybe add unnest sql types?
//todo maybe add unnest to filter parameters if its array ?
//todo swagger ui https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs
//todo derive utoipa::ToSchema for what? original structs or with serialize deserialize?
//todo need to add utoipa::ToSchema annotation #[schema(value_type = YourToSchemaTraitImplStruct)] for all fields
//todo remove useless derives like useless serde::Serialize and Deserialize
//todo maybe generate compisite type for user defined type https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
//todo read again some interesting thoughts about sql as api https://habr.com/ru/companies/timeweb/articles/798937/
//todo reexport all crates what logic depends on (from crates.io) (use of undeclared crate or module `time`)
//todo add transaction isolation level (see postgresql docs)
//todo check on postgresql max length value of type
//todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
//todo postgresql json schema validation https://youtu.be/F6X60ln2VNc
//todo generate json schema from rust type https://docs.rs/schemars/latest/schemars/
//todo support read table length
//todo what is pub what is private
//todo header Retry-After logic

//todo postgresql json:
//* write json schema in postgresql
//* validate insert json field with json schema

#[proc_macro_attribute]
pub fn create_many_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn create_one_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_one_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_many_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_one_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_many_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_one_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_many_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn common_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}

#[proc_macro_attribute]
pub fn create_many_additional_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn create_one_additional_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_many_additional_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_one_additional_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_many_additional_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_one_additional_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_many_additional_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_one_additional_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn common_additional_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}

#[proc_macro_derive(GeneratePostgresqlCrud, attributes(generate_postgresql_crud_primary_key))]
pub fn generate_postgresql_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let generate_select_query_part_snake_case = naming::GenerateSelectQueryPartSnakeCase;
    let create_extension_if_not_exists_pg_jsonschema_upper_camel_case = naming::CreateExtensionIfNotExistsPgJsonschemaUpperCamelCase;
    let create_extension_if_not_exists_uuid_ossp_upper_camel_case = naming::CreateExtensionIfNotExistsUuidOsspUpperCamelCase;
    let prepare_postgresql_upper_camel_case = naming::PreparePostgresqlUpperCamelCase;
    let pool_snake_case = naming::PoolSnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let element_snake_case = naming::ElementSnakeCase;
    let no_fields_provided_upper_camel_case = naming::NoFieldsProvidedUpperCamelCase;
    let where_many_snake_case = naming::WhereManySnakeCase;
    let additional_parameters_snake_case = naming::AdditionalParametersSnakeCase;
    let postgresql_crud_snake_case = &naming::PostgresqlCrudSnakeCase;
    let value_upper_camel_case = naming::ValueUpperCamelCase;
    let from_snake_case = naming::FromSnakeCase;
    let create_query_part_snake_case = naming::CreateQueryPartSnakeCase;
    let create_query_bind_snake_case = naming::CreateQueryBindSnakeCase;
    let parameters_snake_case = naming::ParametersSnakeCase;
    let payload_snake_case = naming::PayloadSnakeCase;
    let select_snake_case = naming::SelectSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let debug_upper_camel_case = naming::DebugUpperCamelCase;
    let error_snake_case = naming::ErrorSnakeCase;
    let acc_snake_case = naming::AccSnakeCase;
    let query_part_snake_case = naming::QueryPartSnakeCase;
    let query_bind_snake_case = naming::QueryBindSnakeCase;
    let order_by_snake_case = naming::OrderBySnakeCase;
    let response_snake_case = naming::ResponseSnakeCase;
    let status_code_snake_case = naming::StatusCodeSnakeCase;
    let body_snake_case = naming::BodySnakeCase;
    let executor_snake_case = naming::ExecutorSnakeCase;
    let rows_snake_case = naming::RowsSnakeCase;
    let begin_snake_case = naming::BeginSnakeCase;
    let commit_snake_case = naming::CommitSnakeCase;
    let desirable_upper_camel_case = naming::DesirableUpperCamelCase;
    let request_snake_case = naming::RequestSnakeCase;
    let app_state_snake_case = naming::AppStateSnakeCase;
    let pool_connection_snake_case = naming::PoolConnectionSnakeCase;
    let body_bytes_snake_case = naming::BodyBytesSnakeCase;
    let url_snake_case = naming::UrlSnakeCase;
    let endpoint_location_snake_case = naming::EndpointLocationSnakeCase;
    let future_snake_case = naming::FutureSnakeCase;
    let by_snake_case = naming::BySnakeCase;
    let prefix_snake_case = naming::PrefixSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
    let expected_response_snake_case = naming::ExpectedResponseSnakeCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let columns_snake_case = naming::ColumnsSnakeCase;
    let order_snake_case = naming::OrderSnakeCase;
    let order_by_upper_camel_case = naming::OrderByUpperCamelCase;
    let query_string_snake_case = naming::QueryStringSnakeCase;
    let binded_query_snake_case = naming::BindedQuerySnakeCase;
    let rollback_snake_case = naming::RollbackSnakeCase;
    let table_name_snake_case = naming::TableNameSnakeCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    let into_serialize_deserialize_version_snake_case = naming::IntoSerializeDeserializeVersionSnakeCase;
    let primary_key_snake_case = naming::PrimaryKeySnakeCase;
    let pagination_snake_case = naming::PaginationSnakeCase;
    let test_cases_snake_case = naming::TestCasesSnakeCase;
    let default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementUpperCamelCase;
    let default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;
    let error_0_token_stream = token_patterns::Error0;
    let error_1_token_stream = token_patterns::Error1;
    let error_2_token_stream = token_patterns::Error2;
    let error_3_token_stream = token_patterns::Error3;
    let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
    let derive_debug_this_error_error_occurence = token_patterns::DeriveDebugThisErrorErrorOccurence;
    let sqlx_acquire = token_patterns::SqlxAcquire;
    let serde_serialize = token_patterns::SerdeSerialize;
    let serde_deserialize = token_patterns::SerdeDeserialize;
    let derive_debug = token_patterns::DeriveDebug;
    let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
    let derive_debug_serde_serialize_serde_deserialize = token_patterns::DeriveDebugSerdeSerializeSerdeDeserialize;
    let derive_debug_clone_copy = token_patterns::DeriveDebugCloneCopy;
    let ref_std_primitive_str = token_patterns::RefStdPrimitiveStr;
    let field_attribute_serde_skip_serializing_if_option_is_none_token_stream = token_patterns::FieldAttributeSerdeSkipSerializingIfOptionIsNone;
    let sqlx_row = token_patterns::SqlxRow;
    let std_string_string = token_patterns::StdStringString;
    let postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
    // let postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_snake_case_stringified = naming::ToTokensToSnakeCaseStringified::case(&ident);
    #[derive(Debug, Clone)]
    struct SynFieldWrapper {
        syn_field: syn::Field,
        field_ident: syn::Ident,
    }
    let (primary_key_field, fields, fields_without_primary_key) = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            let mut option_primary_key_field: std::option::Option<SynFieldWrapper> = None;
            let mut fields = vec![];
            let mut fields_without_primary_key = vec![];
            for element in &fields_named.named {
                let field_ident = element.ident.clone().unwrap();
                let field_ident_len = field_ident.to_string().len();
                let max_postgresql_column_length = 63;
                //todo write runtime check
                assert!((field_ident_len <= max_postgresql_column_length), "Postgresql truncates column names to {max_postgresql_column_length} characters, this is more: {field_ident} ({field_ident_len} characters)");
                fields.push(SynFieldWrapper {
                    syn_field: element.clone(),
                    field_ident: field_ident.clone(),
                });
                let mut is_primary_key = false;
                {
                    for attr in &element.attrs {
                        if attr.path().segments.len() == 1 {
                            let first_segment_ident = &attr.path().segments.first().expect("no first value in punctuated").ident;
                            let generate_postgresql_crud_primary_key_snake_case_stringified = naming::GeneratePostgresqlCrudPrimaryKeySnakeCase.to_string();
                            if first_segment_ident == &generate_postgresql_crud_primary_key_snake_case_stringified {
                                if option_primary_key_field.is_some() {
                                    panic!("two or more supported {generate_postgresql_crud_primary_key_snake_case_stringified} attributes!");
                                } else {
                                    option_primary_key_field = Some(SynFieldWrapper {
                                        syn_field: element.clone(),
                                        field_ident: field_ident.clone(),
                                    });
                                    is_primary_key = true;
                                }
                            }
                        }
                    }
                }
                if !is_primary_key {
                    fields_without_primary_key.push(SynFieldWrapper {
                        syn_field: element.clone(),
                        field_ident: field_ident.clone(),
                    });
                }
            }
            // assert!((fields.len() <= 100), "explicitly not supporting number of columns more than 100 so its less possibility to cause stack overflow or build process exit");
            (option_primary_key_field.unwrap_or_else(|| panic!("primary_key_field is None")), fields, fields_without_primary_key)
        } else {
            panic!("supports only syn::Fields::Named");
        }
    } else {
        panic!("does work only on structs!");
    };
    let fields_len = fields.len();
    let fields_len_without_primary_key = fields_without_primary_key.len();
    let primary_key_field_type = &primary_key_field.syn_field.ty;
    let generate_as_postgresql_type_token_stream = |field_type: &dyn quote::ToTokens| {
        quote::quote! {<#field_type as postgresql_crud::PostgresqlType>::}
    };
    let primary_key_field_type_as_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&primary_key_field_type);
    let generate_as_postgresql_type_tokens_token_stream = |field_type: &dyn quote::ToTokens, tokens: &dyn quote::ToTokens| {
        let as_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&field_type);
        quote::quote! {#as_postgresql_type_token_stream #tokens}
    };
    let generate_as_postgresql_type_table_type_declaration_token_stream = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_token_stream(
            &field_type,
            &naming::TableTypeDeclarationUpperCamelCase
        )
    };
    let primary_key_field_type_as_postgresql_type_table_type_declaration_token_stream = generate_as_postgresql_type_table_type_declaration_token_stream(&primary_key_field_type);
    let generate_as_postgresql_type_create_token_stream = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_token_stream(
            &field_type,
            &naming::CreateUpperCamelCase
        )
    };
    let generate_as_postgresql_type_select_token_stream = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_token_stream(
            &field_type,
            &naming::SelectUpperCamelCase
        )
    };
    let primary_key_field_type_as_postgresql_type_select_token_stream = generate_as_postgresql_type_select_token_stream(&primary_key_field_type);
    let generate_as_postgresql_type_where_element_token_stream = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_token_stream(
            &field_type,
            &naming::WhereElementUpperCamelCase
        )
    };
    let primary_key_field_type_as_postgresql_type_where_element_token_stream = generate_as_postgresql_type_where_element_token_stream(&primary_key_field_type);
    let generate_as_postgresql_type_read_token_stream = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_token_stream(
            &field_type,
            &naming::ReadUpperCamelCase
        )
    };
    // let primary_key_field_type_as_postgresql_type_read_token_stream = generate_as_postgresql_type_read_token_stream(&primary_key_field_type);
    let generate_as_postgresql_type_update_token_stream = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_token_stream(
            &field_type,
            &naming::UpdateUpperCamelCase
        )
    };
    let primary_key_field_type_as_postgresql_type_update_token_stream = generate_as_postgresql_type_update_token_stream(&primary_key_field_type);
    let primary_key_field_type_as_primary_key_upper_camel_case = quote::quote! {
        <#primary_key_field_type as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey
    };
    let std_vec_vec_primary_key_field_type_read_token_stream = quote::quote! {std::vec::Vec::<#primary_key_field_type_as_primary_key_upper_camel_case>};
    let primary_key_field_ident = &primary_key_field.field_ident;
    let primary_key_field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&primary_key_field_ident);
    let primary_key_field_type_update_token_stream = &naming::parameter::SelfUpdateUpperCamelCase::from_type_last_segment(&primary_key_field_type);
    let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
    enum ShouldAddBorrow {
        True,
        False
    }
    impl quote::ToTokens for ShouldAddBorrow {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match &self {
                Self::True => quote::quote! {&}.to_tokens(tokens),
                Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
            }
        }
    }
    let generate_select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream = |should_add_borrow: &ShouldAddBorrow|{
        quote::quote! {#select_snake_case: #should_add_borrow postgresql_crud::NotEmptyUniqueEnumVec<#ident_select_upper_camel_case>}
    };
    let select_borrow_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream = generate_select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream(&ShouldAddBorrow::True);
    let select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream = generate_select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream(&ShouldAddBorrow::False);
    let pub_select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream = {
        quote::quote! {pub #select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream}
    };
    let generate_fields_named_with_comma_token_stream = |function: &dyn Fn(&SynFieldWrapper) -> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_token_stream = fields.iter().map(function);
        quote::quote! {#(#fields_token_stream),*}
    };
    let generate_fields_named_without_comma_token_stream = |function: &dyn Fn(&SynFieldWrapper) -> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_token_stream = fields.iter().map(function);
        quote::quote! {#(#fields_token_stream)*}
    };
    let generate_fields_named_without_primary_key_with_comma_token_stream = |function: &dyn Fn(&SynFieldWrapper) -> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_token_stream = fields_without_primary_key.iter().map(function);
        quote::quote! {#(#fields_token_stream),*}
    };
    let generate_fields_named_without_primary_key_without_comma_token_stream = |function: &dyn Fn(&SynFieldWrapper) -> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_token_stream = fields_without_primary_key.iter().map(function);
        quote::quote! {#(#fields_token_stream)*}
    };
    let impl_ident_token_stream = {
        let ident_prepare_postgresql_error_named_upper_camel_case = naming::parameter::SelfPreparePostgresqlErrorNamedUpperCamelCase::from_tokens(&ident);
        let ident_prepare_postgresql_error_named_token_stream = quote::quote!{
            #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum #ident_prepare_postgresql_error_named_upper_camel_case {
                #create_extension_if_not_exists_pg_jsonschema_upper_camel_case {
                    #[eo_to_std_string_string]
                    error: sqlx::Error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                #create_extension_if_not_exists_uuid_ossp_upper_camel_case {
                    #[eo_to_std_string_string]
                    error: sqlx::Error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                #prepare_postgresql_upper_camel_case {
                    #[eo_to_std_string_string]
                    error: sqlx::Error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
            }
        };
        let pub_fn_table_token_stream = {
            let ident_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_snake_case_stringified);
            quote::quote! {
                pub fn #table_name_snake_case() -> &'static str {
                    #ident_snake_case_double_quotes_token_stream
                }
            }
        };
        let fn_primary_key_token_stream = {
            let primary_key_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&primary_key_field_ident);
            quote::quote!{
                fn #primary_key_snake_case() -> &'static std::primitive::str {
                    #primary_key_field_ident_double_quotes_token_stream
                }
            }
        };
        let pub_async_fn_prepare_postgresql_token_stream = {
            let prepare_postgresql_double_quotes_token_stream = {
                let acc = {
                    let mut acc = std::string::String::new();
                    for _ in &fields {
                        acc.push_str("{},");
                    }
                    let _: Option<char> = acc.pop();
                    acc
                };
                generate_quotes::double_quotes_token_stream(&format!("create table if not exists {ident_snake_case_stringified} ({acc})"))
            };
            let serde_json_to_string_schemars_schema_for_generic_unwrap_token_stream = {
                let generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream = |field_type: &syn::Type, field_ident: &syn::Ident, is_primary_key: std::primitive::bool| {
                    let is_primary_key_token_stream: &dyn quote::ToTokens = if is_primary_key { &naming::TrueSnakeCase } else { &naming::FalseSnakeCase };
                    let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                    let field_type_postgresql_type_table_type_declaration_token_stream = generate_as_postgresql_type_table_type_declaration_token_stream(&field_type);
                    quote::quote! {
                        #field_type_postgresql_type_table_type_declaration_token_stream::create_table_column_query_part(&#field_ident_double_quotes_token_stream, #is_primary_key_token_stream)
                    }
                };
                let mut acc = vec![generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream(&primary_key_field_type, &primary_key_field.field_ident, true)];
                for element in &fields_without_primary_key {
                    acc.push(generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream(&element.syn_field.ty, &element.field_ident, false));
                }
                acc
            };
            quote::quote! {
                pub async fn prepare_postgresql(#pool_snake_case: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #ident_prepare_postgresql_error_named_upper_camel_case> {
                    let create_extension_if_not_exists_pg_jsonschema_query_stringified = "create extension if not exists pg_jsonschema";
                    println!("{create_extension_if_not_exists_pg_jsonschema_query_stringified}");
                    if let Err(error) = sqlx::query(create_extension_if_not_exists_pg_jsonschema_query_stringified).execute(#pool_snake_case).await {
                        return Err(#ident_prepare_postgresql_error_named_upper_camel_case::#create_extension_if_not_exists_pg_jsonschema_upper_camel_case {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!()
                        });
                    }
                    let create_extension_if_not_exists_uuid_ossp_query_stringified = "create extension if not exists \"uuid-ossp\"";
                    println!("{create_extension_if_not_exists_uuid_ossp_query_stringified}");
                    if let Err(error) = sqlx::query(create_extension_if_not_exists_uuid_ossp_query_stringified).execute(#pool_snake_case).await {
                        return Err(#ident_prepare_postgresql_error_named_upper_camel_case::#create_extension_if_not_exists_uuid_ossp_upper_camel_case {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!()
                        });
                    }
                    let prepare_postgresql_query_stringified = format!(
                        #prepare_postgresql_double_quotes_token_stream,
                        #(#serde_json_to_string_schemars_schema_for_generic_unwrap_token_stream),*
                    );
                    println!("{prepare_postgresql_query_stringified}");
                    if let Err(error) = sqlx::query(&prepare_postgresql_query_stringified).execute(#pool_snake_case).await {
                        return Err(#ident_prepare_postgresql_error_named_upper_camel_case::#prepare_postgresql_upper_camel_case {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!()
                        });
                    }
                    Ok(())
                }
            }
        };
        let pub_fn_allow_methods_token_stream = {
            let http_method_token_stream = quote::quote! {http::Method};
            quote::quote! {
                pub fn allow_methods() -> [#http_method_token_stream;4] {[
                    #http_method_token_stream::GET,
                    #http_method_token_stream::POST,
                    #http_method_token_stream::PATCH,
                    #http_method_token_stream::DELETE
                ]}
            }
        };
        let fn_generate_select_query_part_token_stream = {
            let variants_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper|{
                let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                let initialization_token_stream = {
                    let field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                    let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(
                        &element.syn_field.ty
                    );
                    quote::quote! {
                        => #as_postgresql_crud_postgresql_type_postgresql_type_token_stream select_query_part(
                            #value_snake_case,
                            #field_ident_string_double_quotes_token_stream
                        )
                    }
                };
                quote::quote! {#ident_select_upper_camel_case::#field_ident_upper_camel_case_token_stream(value) #initialization_token_stream}
            });
            quote::quote! {
                fn #generate_select_query_part_snake_case(#select_borrow_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream) -> #std_string_string {
                    let mut #value_snake_case = #std_string_string::default();
                    for #element_snake_case in #select_snake_case.to_vec() {
                        #value_snake_case.push_str(&match #element_snake_case {
                            #variants_token_stream
                        });
                        #value_snake_case.push_str(",");
                    }
                    let _: std::option::Option<char> = #value_snake_case.pop();
                    #value_snake_case
                }
            }
        };
        quote::quote! {
            #ident_prepare_postgresql_error_named_token_stream
            impl #ident {
                #pub_fn_table_token_stream
                #fn_primary_key_token_stream
                #pub_async_fn_prepare_postgresql_token_stream
                #pub_fn_allow_methods_token_stream
                #fn_generate_select_query_part_token_stream
            }
        }
    };
    let ident_table_name_call_token_stream = quote::quote!{#ident::#table_name_snake_case()};
    let generate_select_query_part_parameters_payload_select_call_token_stream = quote::quote!{
        #ident::#generate_select_query_part_snake_case(&#parameters_snake_case.#payload_snake_case.#select_snake_case)
    };
    let eprintln_error_token_stream = quote::quote! {eprintln!("{error}");};
    let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
    let generate_postgresql_crud_value_declaration_token_stream = |content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {#postgresql_crud_snake_case::#value_upper_camel_case<#content_token_stream>}
    };
    let generate_postgresql_crud_value_initialization_token_stream = |content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {#postgresql_crud_snake_case::#value_upper_camel_case{#value_snake_case: #content_token_stream}}
    };
    let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream = |ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
        postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
            &ident,
            &proc_macro2::TokenStream::new(),
            &content_token_stream
        )
    };
    #[derive(Debug)]
    struct SynVariantWrapper {
        variant: syn::Variant,
        status_code: std::option::Option<macros_helpers::status_code::StatusCode>,
    }
    impl SynVariantWrapper {
        const fn get_syn_variant(&self) -> &syn::Variant {
            &self.variant
        }
        const fn get_option_status_code(&self) -> &std::option::Option<macros_helpers::status_code::StatusCode> {
            &self.status_code
        }
    }
    let new_syn_variant_wrapper = |
        variant_name: &dyn std::fmt::Display,
        status_code: std::option::Option<macros_helpers::status_code::StatusCode>,
        fields: std::vec::Vec<(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute, &dyn std::fmt::Display, syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>)>
    | -> SynVariantWrapper {
        SynVariantWrapper {
            variant: syn::Variant {
                attrs: status_code.as_ref().map_or_else(|| vec![], |value| vec![syn::Attribute {
                    pound_token: syn::token::Pound { spans: [proc_macro2::Span::call_site()] },
                    style: syn::AttrStyle::Outer,
                    bracket_token: syn::token::Bracket::default(),
                    meta: syn::Meta::Path(syn::Path {
                        leading_colon: None,
                        segments: {
                            let mut handle = syn::punctuated::Punctuated::new();
                            handle.push(syn::PathSegment {
                                ident: proc_macro2::Ident::new(&naming::AsRefStrToSnakeCaseStringified::case(value), proc_macro2::Span::call_site()),
                                arguments: syn::PathArguments::None,
                            });
                            handle
                        },
                    }),
                }]),
                ident: syn::Ident::new(&variant_name.to_string(), proc_macro2::Span::call_site()),
                fields: syn::Fields::Named(syn::FieldsNamed {
                    brace_token: syn::token::Brace::default(),
                    named: {
                        let mut handle = fields.into_iter().fold(syn::punctuated::Punctuated::new(), |mut acc, element| {
                            acc.push_value(syn::Field {
                                attrs: vec![syn::Attribute {
                                    pound_token: syn::token::Pound { spans: [proc_macro2::Span::call_site()] },
                                    style: syn::AttrStyle::Outer,
                                    bracket_token: syn::token::Bracket::default(),
                                    meta: syn::Meta::Path(syn::Path {
                                        leading_colon: None,
                                        segments: {
                                            let mut handle = syn::punctuated::Punctuated::new();
                                            handle.push(syn::PathSegment {
                                                ident: proc_macro2::Ident::new(macros_helpers::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&element.0), proc_macro2::Span::call_site()),
                                                arguments: syn::PathArguments::None,
                                            });
                                            handle
                                        },
                                    }),
                                }],
                                vis: syn::Visibility::Inherited,
                                mutability: syn::FieldMutability::None,
                                ident: Some(syn::Ident::new(&element.1.to_string(), proc_macro2::Span::call_site())),
                                colon_token: Some(syn::token::Colon { spans: [proc_macro2::Span::call_site()] }),
                                ty: syn::Type::Path(syn::TypePath {
                                    qself: None,
                                    path: syn::Path { leading_colon: None, segments: element.2 },
                                }),
                            });
                            acc.push_punct(syn::token::Comma { spans: [proc_macro2::Span::call_site()] });
                            acc
                        });
                        handle.push_value(macros_helpers::code_occurence_syn_field::code_occurence_syn_field());
                        handle
                    },
                }),
                discriminant: None,
            },
            status_code,
        }
    };
    let query_part_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::QueryPartUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
            &naming::ErrorSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&postgresql_crud_snake_case.to_string(), &query_part_error_named_upper_camel_case.to_string()]),
        )],
    );
    #[derive(Debug, Clone, Copy, naming::AsRefStrEnumWithUnitFieldsToUpperCamelCaseStringified, naming::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified)]
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
        const fn http_method(self) -> OperationHttpMethod {
            match self {
                Self::CreateMany | Self::CreateOne | Self::ReadMany | Self::ReadOne => OperationHttpMethod::Post,
                Self::UpdateMany | Self::UpdateOne => OperationHttpMethod::Patch,
                Self::DeleteMany | Self::DeleteOne => OperationHttpMethod::Delete,
            }
        }
        const fn desirable_status_code(self) -> macros_helpers::status_code::StatusCode {
            match self {
                Self::CreateMany | Self::CreateOne => macros_helpers::status_code::StatusCode::Created201,
                Self::ReadMany | Self::ReadOne | Self::UpdateMany | Self::UpdateOne | Self::DeleteMany | Self::DeleteOne => macros_helpers::status_code::StatusCode::Ok200,
            }
        }
        const fn generate_postgresql_crud_attribute_additional_error_variants(self) -> GeneratePostgresqlCrudAttribute {
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
        const fn generate_postgresql_crud_attribute_additional_logic(self) -> GeneratePostgresqlCrudAttribute {
            match self {
                Self::CreateMany => GeneratePostgresqlCrudAttribute::CreateManyAdditionalLogic,
                Self::CreateOne => GeneratePostgresqlCrudAttribute::CreateOneAdditionalLogic,
                Self::ReadMany => GeneratePostgresqlCrudAttribute::ReadManyAdditionalLogic,
                Self::ReadOne => GeneratePostgresqlCrudAttribute::ReadOneAdditionalLogic,
                Self::UpdateMany => GeneratePostgresqlCrudAttribute::UpdateManyAdditionalLogic,
                Self::UpdateOne => GeneratePostgresqlCrudAttribute::UpdateOneAdditionalLogic,
                Self::DeleteMany => GeneratePostgresqlCrudAttribute::DeleteManyAdditionalLogic,
                Self::DeleteOne => GeneratePostgresqlCrudAttribute::DeleteOneAdditionalLogic,
            }
        }
        fn operation_error_named_with_serialize_deserialize_snake_case(&self) -> naming::parameter::SelfErrorNamedWithSerializeDeserializeSnakeCase {
            naming::parameter::SelfErrorNamedWithSerializeDeserializeSnakeCase::from_display(self)
        }
        fn snake_case_stringified(&self) -> std::string::String {
            naming::AsRefStrToSnakeCaseStringified::case(&self.to_string())
        }
        fn snake_case_token_stream(&self) -> proc_macro2::TokenStream {
            naming::AsRefStrToSnakeCaseTokenStream::case_or_panic(&self.to_string())
        }
        fn operation_payload_example_snake_case(&self) -> impl naming::StdFmtDisplayPlusQuoteToTokens {
            naming::parameter::SelfPayloadExampleSnakeCase::from_display(&self)
        }
    }
    impl std::fmt::Display for Operation {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                Self::CreateMany => write!(formatter, "CreateMany"),
                Self::CreateOne => write!(formatter, "CreateOne"),
                Self::ReadMany => write!(formatter, "ReadMany"),
                Self::ReadOne => write!(formatter, "ReadOne"),
                Self::UpdateMany => write!(formatter, "UpdateMany"),
                Self::UpdateOne => write!(formatter, "UpdateOne"),
                Self::DeleteMany => write!(formatter, "DeleteMany"),
                Self::DeleteOne => write!(formatter, "DeleteOne"),
            }
        }
    }
    #[derive(naming::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified)]
    enum OperationHttpMethod {
        Post,
        Patch,
        Delete,
    }
    let generate_ident_operation_error_named_upper_camel_case = |operation: &Operation|{
        format!("{ident}{operation}ErrorNamed")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_ident_operation_response_variants_upper_camel_case = |operation: &Operation|{
        format!("{ident}{operation}ResponseVariants")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_initialization_token_stream = |syn_variant_wrapper: &SynVariantWrapper, file: &'static str, line: std::primitive::u32, column: std::primitive::u32| -> proc_macro2::TokenStream {
        let variant_ident = &syn_variant_wrapper.variant.ident;
        let fields_token_stream = if let syn::Fields::Named(value) = &syn_variant_wrapper.variant.fields {
            value.named.iter().enumerate().map(|(index, element)| {
                let field_ident = &element.ident;
                if *field_ident.as_ref().unwrap_or_else(|| panic!("{}", naming::FIELD_IDENT_IS_NONE)) == naming::CodeOccurenceSnakeCase.to_string() {
                    macros_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(file, line, column)
                } else {
                    let error_increment_snake_case = naming::parameter::ErrorSelfSnakeCase::from_display(&index);
                    quote::quote! {#field_ident: #error_increment_snake_case}
                }
            })
        } else {
            panic!("syn::Fields::Named(value) != &self.variant.fields {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE);
        };
        quote::quote! {
            #variant_ident {
                #(#fields_token_stream),*
            }
        }
    };
    let wrap_into_axum_response_token_stream = |axum_json_content_token_stream: &dyn quote::ToTokens, status_code_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            let mut response = axum::response::IntoResponse::into_response(
                axum::Json(#axum_json_content_token_stream)
            );
            *response.status_mut() = #status_code_token_stream;
            return response;
        }
    };
    let generate_operation_error_initialization_eprintln_response_creation_token_stream = |operation: &Operation, syn_variant_wrapper: &SynVariantWrapper, file: &'static str, line: std::primitive::u32, column: std::primitive::u32| {
        let ident_operation_error_named_upper_camel_case = generate_ident_operation_error_named_upper_camel_case(&operation);
        let ident_operation_response_variants_upper_camel_case = generate_ident_operation_response_variants_upper_camel_case(&operation);
        let syn_variant_initialization_token_stream = generate_initialization_token_stream(syn_variant_wrapper, file, line, column);
        let status_code_token_stream = syn_variant_wrapper.get_option_status_code().unwrap_or_else(|| panic!("option_status_code is None")).to_axum_http_status_code_token_stream();
        let wraped_into_axum_response_token_stream = wrap_into_axum_response_token_stream(&quote::quote! {#ident_operation_response_variants_upper_camel_case::#from_snake_case(#error_snake_case)}, &status_code_token_stream);
        quote::quote! {
            let #error_snake_case = #ident_operation_error_named_upper_camel_case::#syn_variant_initialization_token_stream;
            #eprintln_error_token_stream
            #wraped_into_axum_response_token_stream
        }
    };
    let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
    let ident_create_token_stream = {
        let ident_create_token_stream = {
            let pub_field_ident_field_type_fields_named_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let element_syn_field_ty_as_postgresql_type_create_token_stream = generate_as_postgresql_type_create_token_stream(&element.syn_field.ty);
                quote::quote! {
                    pub #field_ident: #element_syn_field_ty_as_postgresql_type_create_token_stream
                }
            });
            quote::quote! {
                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
                pub struct #ident_create_upper_camel_case {
                    #pub_field_ident_field_type_fields_named_without_primary_key_token_stream
                }
            }
        };
        let impl_ident_create_token_stream = {
            let primary_key_field_type_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = {
                let primary_key_field_type_as_postgresql_type_create_token_stream = generate_as_postgresql_type_create_token_stream(&primary_key_field_type);
                quote::quote!{
                    <
                        #primary_key_field_type_as_postgresql_type_create_token_stream as #postgresql_crud_snake_case::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case
                    >::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case()
                }
            };
            let fn_create_query_part_token_stream = {
                let generate_match_as_postgresql_crud_postgresql_type_postgresql_type_create_query_part_token_stream = |
                    field_type: &syn::Type,
                    content_token_stream: &dyn quote::ToTokens
                |{
                    let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&field_type);
                    quote::quote! {
                        match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #create_query_part_snake_case(
                            &#content_token_stream,
                            #increment_snake_case
                        ) {
                            Ok(#value_snake_case) => {
                                #acc_snake_case.push_str(&format!("{value},"));
                            }
                            Err(#error_0_token_stream) => {
                                return Err(#error_0_token_stream);
                            }
                        }
                    }
                };
                let primary_key_content_token_stream = generate_match_as_postgresql_crud_postgresql_type_postgresql_type_create_query_part_token_stream(
                    &primary_key_field_type,
                    &primary_key_field_type_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                );
                let column_increments_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper|{
                    generate_match_as_postgresql_crud_postgresql_type_postgresql_type_create_query_part_token_stream(
                        &element.syn_field.ty,
                        &{
                            let element_field_ident = &element.field_ident;
                            quote::quote!{self.#element_field_ident}
                        }
                    )
                });
                quote::quote!{
                    fn #create_query_part_snake_case(&self, #increment_snake_case: &mut std::primitive::u64) -> Result<#std_string_string, postgresql_crud::#query_part_error_named_upper_camel_case> {
                        let mut #acc_snake_case = std::string::String::default();
                        #primary_key_content_token_stream
                        #column_increments_token_stream
                        let _: Option<char> = #acc_snake_case.pop();
                        Ok(#acc_snake_case)
                    }
                }
            };
            let fn_create_query_bind_token_stream = {
                let generate_query_as_postgresql_crud_postgresql_type_postgresql_type_create_query_bind_token_stream = |
                    field_type: &syn::Type,
                    content_token_stream: &dyn quote::ToTokens
                |{
                    let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&field_type);
                    quote::quote! {
                        #query_snake_case = #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #create_query_bind_snake_case(
                            #content_token_stream,
                            #query_snake_case
                        );
                    }
                };
                let primary_key_content_token_stream = generate_query_as_postgresql_crud_postgresql_type_postgresql_type_create_query_bind_token_stream(
                    &primary_key_field_type,
                    &primary_key_field_type_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                );
                let binded_query_modifications_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper|{
                    generate_query_as_postgresql_crud_postgresql_type_postgresql_type_create_query_bind_token_stream(
                        &element.syn_field.ty,
                        &{
                            let field_ident = &element.field_ident;
                            quote::quote! {self.#field_ident}
                        }
                    )
                });
                quote::quote!{
                    fn #create_query_bind_snake_case(self, mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                        #primary_key_content_token_stream
                        #binded_query_modifications_token_stream
                        #query_snake_case
                    }
                }
            };
            quote::quote!{
                impl #ident_create_upper_camel_case {
                    #fn_create_query_part_token_stream
                    #fn_create_query_bind_token_stream
                }
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
            &ident_create_upper_camel_case,
            &{
                let fields_initialiation_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper|{
                    let field_ident = &element.field_ident;
                    quote::quote! {#field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}
                });
                quote::quote! {
                    Self{#fields_initialiation_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream}
                }
            }
        );
        quote::quote! {
            #ident_create_token_stream
            #impl_ident_create_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream 
        }
    };
    let ident_where_many_upper_camel_case = naming::parameter::SelfWhereManyUpperCamelCase::from_tokens(&ident);
    let ident_where_many_try_new_error_named_upper_camel_case = naming::parameter::SelfWhereManyTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
    let ident_where_many_token_stream = {
        let fields_declaration_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
            let field_ident = &element.field_ident;
            let element_syn_field_ty_as_postgresql_type_where_element_token_stream = generate_as_postgresql_type_where_element_token_stream(&element.syn_field.ty);
            quote::quote! {#field_ident: std::option::Option<postgresql_crud::PostgresqlTypeWhere<#element_syn_field_ty_as_postgresql_type_where_element_token_stream>>}
        });
        let ident_where_many_token_stream = quote::quote! {
            #[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
            pub struct #ident_where_many_upper_camel_case {
                #fields_declaration_token_stream
            }
        };
        let ident_where_many_try_new_error_named_token_stream = {
            quote::quote!{
                #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #ident_where_many_try_new_error_named_upper_camel_case {
                    #no_fields_provided_upper_camel_case {
                        #[eo_to_std_string_string]
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
            }
        };
        let impl_ident_where_many_token_stream = {
            let pub_fn_try_new_token_stream = {
                let none_token_stream = generate_fields_named_with_comma_token_stream(&|_: &SynFieldWrapper| -> proc_macro2::TokenStream {
                    quote::quote! {None}
                });
                let generate_fields_token_stream = |should_add_borrow: ShouldAddBorrow|{
                    generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                        let field_ident = &element.field_ident;
                        quote::quote! {#should_add_borrow #field_ident}
                    })
                };
                let fields_token_stream = generate_fields_token_stream(ShouldAddBorrow::True);
                let fields_inialization_token_stream = generate_fields_token_stream(ShouldAddBorrow::False);
                quote::quote!{
                    pub fn try_new(#fields_declaration_token_stream) -> Result<#ident_where_many_upper_camel_case, #ident_where_many_try_new_error_named_upper_camel_case> {
                        if let (#none_token_stream) = (#fields_token_stream) {
                            return Err(#ident_where_many_try_new_error_named_upper_camel_case::#no_fields_provided_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        Ok(Self {#fields_inialization_token_stream})
                    }
                }
            };
            quote::quote!{
                impl #ident_where_many_upper_camel_case {
                    #pub_fn_try_new_token_stream
                }
            }
        };
        let impl_serde_deserialize_for_ident_where_many_token_stream = postgresql_crud_macros_common::generate_impl_serde_deserialize_for_struct_token_stream(
            &ident_where_many_upper_camel_case,
            fields.iter().map(|element|(&element.field_ident, &element.syn_field.ty)).collect::<std::vec::Vec<(&syn::Ident, &syn::Type)>>(),
            fields_len,
            &|_: &syn::Ident, syn_type: &syn::Type|{
                let syn_type_as_postgresql_type_where_element_token_stream = generate_as_postgresql_type_where_element_token_stream(&syn_type);
                quote::quote!{std::option::Option<postgresql_crud::PostgresqlTypeWhere<#syn_type_as_postgresql_type_where_element_token_stream>>}
            },
        );
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_many_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
            &ident_where_many_upper_camel_case,
            &{
                let fields_token_stream = generate_fields_named_without_comma_token_stream(&|element: &SynFieldWrapper|{
                    let field_ident = &element.field_ident;
                    quote::quote! {
                        #field_ident: Some(
                            #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                        ),
                    }
                });
                quote::quote! {Self{#fields_token_stream}}
            }
        );
        quote::quote! {
            #ident_where_many_token_stream
            #ident_where_many_try_new_error_named_token_stream
            #impl_ident_where_many_token_stream
            #impl_serde_deserialize_for_ident_where_many_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_many_token_stream
        }
    };
    let std_option_option_ident_where_many_upper_camel_case = naming::parameter::StdOptionOptionSelfWhereManyUpperCamelCase::from_tokens(&ident);
    let std_option_option_ident_where_many_token_stream = {
        let std_option_option_ident_where_many_token_stream = {
            quote::quote! {
                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
                pub struct #std_option_option_ident_where_many_upper_camel_case(pub std::option::Option<#ident_where_many_upper_camel_case>);
            }
        };
        let impl_postgresql_type_where_filter_for_std_option_option_ident_where_many_token_stream = postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
            &quote::quote! {<'a>},
            &std_option_option_ident_where_many_upper_camel_case,
            &proc_macro2::TokenStream::new(),
            &postgresql_crud_macros_common::IncrementParameterUnderscore::False,
            &postgresql_crud_macros_common::ColumnParameterUnderscore::True,
            &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::True,
            &{
                let additional_parameters_modification_token_stream = fields.iter().enumerate().map(|(index, element)| {
                    let field_ident = &element.field_ident;
                    let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                    let maybe_is_first_push_to_additional_parameters_already_happend_true_token_stream = if index == fields_len_without_primary_key {
                        proc_macro2::TokenStream::new()
                    }
                    else {
                        quote::quote!{is_first_push_to_additional_parameters_already_happend = true;}
                    };
                    quote::quote! {
                        if let Some(#value_snake_case) = &#value_snake_case.#field_ident {
                            match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                                #value_snake_case,
                                increment,
                                &#field_ident_double_quotes_token_stream,
                                is_first_push_to_additional_parameters_already_happend,
                            ) {
                                Ok(#value_snake_case) => {
                                    #additional_parameters_snake_case.push_str(&#value_snake_case);
                                    #maybe_is_first_push_to_additional_parameters_already_happend_true_token_stream
                                }
                                Err(#error_0_token_stream) => {
                                    return Err(#error_0_token_stream);
                                }
                            }
                        }
                    }
                });
                quote::quote!{
                    Ok(match &self.0 {
                        Some(value) => {
                            let mut #additional_parameters_snake_case = #std_string_string::from("where");
                            let mut is_first_push_to_additional_parameters_already_happend = false;
                            #(#additional_parameters_modification_token_stream)*
                            #additional_parameters_snake_case
                        },
                        None => #std_string_string::default()
                    })
                }
            },
            &postgresql_crud_macros_common::IsQueryBindMutable::True,
            &{
                let binded_query_modifications_token_stream = generate_fields_named_without_comma_token_stream(&|element: &SynFieldWrapper|{
                    let field_ident = &element.field_ident;
                    quote::quote! {
                        if let Some(#value_snake_case) = #value_snake_case.#field_ident {
                            #query_snake_case = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(#value_snake_case, #query_snake_case);
                        }
                    }
                });
                quote::quote! {
                    if let Some(#value_snake_case) = self.0 {
                        #binded_query_modifications_token_stream
                    }
                    #query_snake_case
                }
            },
            &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
        );
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_std_option_option_ident_where_many_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
            &std_option_option_ident_where_many_upper_camel_case,
            &quote::quote!{Self(Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))}
        );
        quote::quote! {
            #std_option_option_ident_where_many_token_stream
            #impl_postgresql_type_where_filter_for_std_option_option_ident_where_many_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_std_option_option_ident_where_many_token_stream
        }
    };
    let pub_where_many_std_option_option_ident_where_many_token_stream = quote::quote!{pub #where_many_snake_case: #std_option_option_ident_where_many_upper_camel_case};
    let where_many_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = quote::quote!{
        #where_many_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
    };
    enum ReadManyOrDeleteMany {
        ReadMany,
        DeleteMany
    }
    impl std::convert::From<&ReadManyOrDeleteMany> for Operation {
        fn from(value: &ReadManyOrDeleteMany) -> Self {
            match &value {
                ReadManyOrDeleteMany::ReadMany => Self::ReadMany,
                ReadManyOrDeleteMany::DeleteMany => Self::DeleteMany,
            }
        }
    }
    let generate_read_or_delete_many_additional_paramaters_initialization_token_stream = |read_many_or_delete_many: &ReadManyOrDeleteMany|{
        let maybe_mut_token_stream: &dyn quote::ToTokens = match &read_many_or_delete_many {
            ReadManyOrDeleteMany::ReadMany => &naming::MutSnakeCase,
            ReadManyOrDeleteMany::DeleteMany => &proc_macro2::TokenStream::new(),
        };
        let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
            &Operation::from(read_many_or_delete_many),
            &query_part_syn_variant_wrapper,
            file!(),
            line!(),
            column!()
        );
        quote::quote!{
            let #maybe_mut_token_stream #additional_parameters_snake_case = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                &#parameters_snake_case.#payload_snake_case.#where_many_snake_case,
                &mut #increment_snake_case,
                &"",//useless
                false//useless
            ) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_0_token_stream) => {
                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
            };
        }
    };
    let query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_token_stream = quote::quote!{
        #query_snake_case = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(#parameters_snake_case.#payload_snake_case.#where_many_snake_case, #query_snake_case);
    };
    let try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_snake_case = naming::parameter::TryFromSqlxPostgresPgRowWithNotEmptyUniqueEnumVecSelfSelectSnakeCase::from_display(&ident);
    let sqlx_error_syn_punctuated_punctuated = macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["sqlx", "Error"]);
    let macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string = macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString;
    let macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize = macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize;
    let postgresql_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::PostgresqlUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![(macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &naming::PostgresqlSnakeCase, sqlx_error_syn_punctuated_punctuated.clone())],
    );
    enum ReadManyOrReadOne {
        ReadMany,
        ReadOne
    }
    impl std::convert::From<&ReadManyOrReadOne> for Operation {
        fn from(value: &ReadManyOrReadOne) -> Self {
            match &value {
                ReadManyOrReadOne::ReadMany => Self::ReadMany,
                ReadManyOrReadOne::ReadOne => Self::ReadOne,
            }
        }
    }
    let generate_match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream = |read_many_or_read_one: &ReadManyOrReadOne|{
        let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
            &Operation::from(read_many_or_read_one),
            &postgresql_syn_variant_wrapper,
            file!(),
            line!(),
            column!()
        );
        quote::quote! {
            match #ident_read_upper_camel_case::#try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_snake_case(
                #value_snake_case,
                &#parameters_snake_case.#payload_snake_case.#select_snake_case
            ) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_0_token_stream) => {
                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
            }
        }
    };
    let select_token_stream = {
        let ident_select_token_stream = {
            let variants = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper|{
                let serialize_deserialize_ident_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                let element_syn_field_ty_as_postgresql_type_select_token_stream = generate_as_postgresql_type_select_token_stream(&element.syn_field.ty);
                quote::quote! {
                    #[serde(rename(serialize = #serialize_deserialize_ident_token_stream, deserialize = #serialize_deserialize_ident_token_stream))]
                    #field_ident_upper_camel_case_token_stream(#element_syn_field_ty_as_postgresql_type_select_token_stream)
                }
            });
            quote::quote! {
                #[derive(
                    #debug_upper_camel_case,
                    #serde_serialize,
                    #serde_deserialize,
                    PartialEq,
                    Clone,
                )]
                pub enum #ident_select_upper_camel_case {
                    #variants
                }
            }
        };
        let impl_std_fmt_display_for_ident_select_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(
            &proc_macro2::TokenStream::new(),
            &ident_select_upper_camel_case,
            &proc_macro2::TokenStream::new(),
            &quote::quote! {write!(formatter, "{}", serde_json::to_string(&self).unwrap_or_else(|e|format!("cannot serialize into json: {e:?}")))},
        );
        let impl_error_occurence_lib_to_std_string_string_for_ident_select_token_stream =
            macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_select_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self}")});
        let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
            &ident_select_upper_camel_case,
            &{
                let elements_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper|{
                    let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                    quote::quote! {
                        #ident_select_upper_camel_case::#field_ident_upper_camel_case_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                    }
                });
                quote::quote! {vec![#elements_token_stream]}
            }
        );
        quote::quote! {
            #ident_select_token_stream
            #impl_std_fmt_display_for_ident_select_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_select_token_stream
            #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream
        }
    };
    let select_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = quote::quote!{
        #select_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
    };
    let ident_read_token_stream = {
        let ident_read_token_stream = {
            let field_option_primary_key_token_stream = {
                let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(&generate_as_postgresql_type_read_token_stream(&primary_key_field_type));
                quote::quote! {
                    #field_attribute_serde_skip_serializing_if_option_is_none_token_stream
                    pub #primary_key_field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream>
                }
            };
            let fields_options_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                let field_vis = &element.syn_field.vis;
                let field_ident = &element.field_ident;
                let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(&generate_as_postgresql_type_read_token_stream(&element.syn_field.ty));
                quote::quote! {
                    #field_attribute_serde_skip_serializing_if_option_is_none_token_stream
                    #field_vis #field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream>
                }
            });
            quote::quote! {
                #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
                pub struct #ident_read_upper_camel_case {
                    #field_option_primary_key_token_stream,
                    #fields_options_without_primary_key_token_stream
                }
            }
        };
        let impl_ident_read_token_stream = {
            let fn_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream = {
                let declaration_primary_key_token_stream = {
                    let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(&primary_key_field_type_as_primary_key_upper_camel_case);
                    quote::quote! {
                        let mut #primary_key_field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream> = None;
                    }
                };
                let declaration_without_primary_key_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper|{
                    let field_ident = &element.field_ident;
                    let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(
                        &generate_as_postgresql_type_read_token_stream(&element.syn_field.ty)
                    );
                    quote::quote! {
                        let mut #field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream> = None;
                    }
                });
                let assignment_variant_primary_key_token_stream = {
                    let primary_key_field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&primary_key_field_ident);
                    let postgresql_crud_value_initialization_token_stream = generate_postgresql_crud_value_initialization_token_stream(&value_snake_case);
                    quote::quote! {
                        #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(_) => match sqlx::Row::try_get::<
                            #primary_key_field_type_as_primary_key_upper_camel_case,
                            #ref_std_primitive_str
                        >(
                            &#value_snake_case,
                            #primary_key_field_ident_string_double_quotes_token_stream
                        ) {
                            Ok(#value_snake_case) => {
                                #primary_key_field_ident = Some(#postgresql_crud_value_initialization_token_stream);
                            },
                            Err(#error_0_token_stream) => {
                                return Err(#error_0_token_stream);
                            }
                        }
                    }
                };
                let assignment_variants_without_primary_key_token_stream = fields_without_primary_key
                    .iter()
                    .map(|element| {
                        let field_ident = &element.field_ident;
                        let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                        let field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                        let postgresql_crud_value_initialization_token_stream = generate_postgresql_crud_value_initialization_token_stream(&value_snake_case);
                        let element_syn_field_ty_as_postgresql_type_read_token_stream = generate_as_postgresql_type_read_token_stream(&element.syn_field.ty);
                        quote::quote! {
                            #ident_select_upper_camel_case::#field_ident_upper_camel_case_token_stream(_) => match sqlx::Row::try_get::<
                                #element_syn_field_ty_as_postgresql_type_read_token_stream,
                                #ref_std_primitive_str
                            >(
                                &#value_snake_case,
                                #field_ident_string_double_quotes_token_stream
                            ) {
                                Ok(#value_snake_case) => {
                                    #field_ident = Some(#postgresql_crud_value_initialization_token_stream);
                                },
                                Err(#error_0_token_stream) => {
                                    return Err(#error_0_token_stream);
                                }
                            }
                        }
                    })
                    .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                let fields_initiation_token_stream = &fields.iter()
                .map(|element| element.syn_field.ident.as_ref().unwrap_or_else(|| panic!("{}", naming::FIELD_IDENT_IS_NONE)))
                .collect::<std::vec::Vec<&syn::Ident>>();
                quote::quote! {
                    fn #try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_snake_case(
                        #value_snake_case: sqlx::postgres::PgRow,
                        #select_borrow_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream
                    ) -> Result<Self, sqlx::Error> {
                        #declaration_primary_key_token_stream
                        #declaration_without_primary_key_token_stream
                        for #element_snake_case in #select_snake_case.to_vec() {
                            match #element_snake_case {
                                #assignment_variant_primary_key_token_stream,
                                #(#assignment_variants_without_primary_key_token_stream),*
                            }
                        }
                        Ok(Self {#(#fields_initiation_token_stream),*})
                    }
                }
            };
            quote::quote! {
                impl #ident_read_upper_camel_case {
                    #fn_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream
                }
            }
        };
        quote::quote! {
            #ident_read_token_stream
            #impl_ident_read_token_stream
        }
    };
    let generate_ident_try_operation_error_named_upper_camel_case = |operation: &Operation|{
        format!("{ident}Try{operation}ErrorNamed")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case = |operation: &Operation|{
        format!("{ident}{operation}ErrorNamedWithSerializeDeserialize")
        .parse::<proc_macro2::TokenStream>().unwrap()
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
        CreateManyAdditionalLogic,
        CreateOneAdditionalLogic,
        ReadManyAdditionalLogic,
        ReadOneAdditionalLogic,
        UpdateManyAdditionalLogic,
        UpdateOneAdditionalLogic,
        DeleteManyAdditionalLogic,
        DeleteOneAdditionalLogic,
        CommonAdditionalLogic,
    }
    impl GeneratePostgresqlCrudAttribute {
        fn generate_path_to_attribute(self) -> std::string::String {
            let value = match self {
                Self::CreateManyAdditionalErrorVariants => naming::CreateManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::CreateOneAdditionalErrorVariants => naming::CreateOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::ReadManyAdditionalErrorVariants => naming::ReadManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::ReadOneAdditionalErrorVariants => naming::ReadOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::UpdateManyAdditionalErrorVariants => naming::UpdateManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::UpdateOneAdditionalErrorVariants => naming::UpdateOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::DeleteManyAdditionalErrorVariants => naming::DeleteManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::DeleteOneAdditionalErrorVariants => naming::DeleteOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::CommonAdditionalErrorVariants => naming::CommonAdditionalErrorVariantsSnakeCase.to_string(),
                Self::CreateManyAdditionalLogic => naming::CreateManyAdditionalLogicSnakeCase.to_string(),
                Self::CreateOneAdditionalLogic => naming::CreateOneAdditionalLogicSnakeCase.to_string(),
                Self::ReadManyAdditionalLogic => naming::ReadManyAdditionalLogicSnakeCase.to_string(),
                Self::ReadOneAdditionalLogic => naming::ReadOneAdditionalLogicSnakeCase.to_string(),
                Self::UpdateManyAdditionalLogic => naming::UpdateManyAdditionalLogicSnakeCase.to_string(),
                Self::UpdateOneAdditionalLogic => naming::UpdateOneAdditionalLogicSnakeCase.to_string(),
                Self::DeleteManyAdditionalLogic => naming::DeleteManyAdditionalLogicSnakeCase.to_string(),
                Self::DeleteOneAdditionalLogic => naming::DeleteOneAdditionalLogicSnakeCase.to_string(),
                Self::CommonAdditionalLogic => naming::CommonAdditionalLogicSnakeCase.to_string(),
            };
            format!("{}::{value}", naming::PostgresqlCrudSnakeCase)
        }
    }
    let postgresql_crud_order_by_token_stream = quote::quote! {#postgresql_crud_snake_case::#order_by_upper_camel_case};
    let postgresql_crud_order_token_stream = quote::quote! {#postgresql_crud_snake_case::Order};
    let ident_column_read_permission_token_stream = {
        let ident_column_read_permission_upper_camel_case = naming::parameter::SelfColumnReadPermissionUpperCamelCase::from_display(&ident);
        let fields_permission_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper|{
            let field_ident = &element.field_ident;
            //todo permissions for json
            quote::quote! {
                #field_ident: std::primitive::bool
            }
        });
        quote::quote! {
            #derive_debug_clone_copy
            pub struct #ident_column_read_permission_upper_camel_case {
                #fields_permission_token_stream
            }
        }
    };
    let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
    let ident_update_try_new_error_named_upper_camel_case = naming::parameter::SelfUpdateTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
    let update_query_part_primary_key_snake_case = naming::UpdateQueryPartPrimaryKeySnakeCase;
    let ident_update_token_stream = {
        let generate_option_value_field_type_as_postgresql_type_update_token_stream = |syn_type: &syn::Type|{
            let path_value_token_stream = {
                let value = format!("{}::{}", naming::PostgresqlCrudSnakeCase, naming::ValueUpperCamelCase);
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let syn_type_as_postgresql_type_update_token_stream = generate_as_postgresql_type_update_token_stream(&syn_type);
            quote::quote! {std::option::Option<#path_value_token_stream<#syn_type_as_postgresql_type_update_token_stream>>}
        };
        let fields_declaration_token_stream = {
            let fields_named_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                let field_ident = &element.field_ident;
                let generate_option_value_field_type_as_postgresql_type_update_token_stream = generate_option_value_field_type_as_postgresql_type_update_token_stream(&element.syn_field.ty);
                quote::quote! {
                    #field_ident: #generate_option_value_field_type_as_postgresql_type_update_token_stream
                }
            });
            quote::quote! {
                #primary_key_field_ident: #primary_key_field_type_update_token_stream,
                #fields_named_without_primary_key_token_stream
            }
        };
        let ident_update_token_stream = quote::quote! {
            #[derive(Debug, serde::Serialize, utoipa::ToSchema)]
            pub struct #ident_update_upper_camel_case {
                #fields_declaration_token_stream
            }
        };
        let ident_update_try_new_error_named_token_stream = {
            quote::quote!{
                #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #ident_update_try_new_error_named_upper_camel_case {
                    #no_fields_provided_upper_camel_case {
                        #[eo_to_std_string_string]
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
            }
        };
        let impl_ident_update_token_stream = {
            let pub_fn_try_new_token_stream = {
                let (
                    left_token_stream,
                    right_token_stream
                ) = {
                    let maybe_wrap_into_braces_handle_token_stream = |content_token_stream: &dyn quote::ToTokens|{
                        postgresql_crud_macros_common::maybe_wrap_into_braces_token_stream(content_token_stream, fields_len_without_primary_key > 1)
                    };
                    (
                        maybe_wrap_into_braces_handle_token_stream(&generate_fields_named_without_primary_key_with_comma_token_stream(&|_: &SynFieldWrapper| -> proc_macro2::TokenStream {
                            quote::quote! {None}
                        })),
                        maybe_wrap_into_braces_handle_token_stream(&generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                            let field_ident = &element.field_ident;
                            quote::quote! {&#field_ident}
                        }))
                    )
                };
                let fields_inialization_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                    let field_ident = &element.field_ident;
                    quote::quote! {#field_ident}
                });
                quote::quote!{
                    pub fn try_new(
                        #fields_declaration_token_stream
                    ) -> Result<#ident_update_upper_camel_case, #ident_update_try_new_error_named_upper_camel_case> {
                        if let #left_token_stream = #right_token_stream {
                            return Err(#ident_update_try_new_error_named_upper_camel_case::#no_fields_provided_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        Ok(Self {#fields_inialization_token_stream})
                    }
                }
            };
            let update_query_part_primary_key_token_stream = {
                quote::quote!{
                    fn #update_query_part_primary_key_snake_case(&self, #increment_snake_case: &mut std::primitive::u64) -> Result<#std_string_string, #postgresql_crud_snake_case::#query_part_error_named_upper_camel_case> {
                        match #primary_key_field_type_as_postgresql_type_token_stream #update_query_part_snake_case(
                            &self.#primary_key_field_ident,
                            &"",
                            &#ident::#primary_key_snake_case(),
                            &"",
                            #increment_snake_case,
                        ) {
                            Ok(value_snake_case) => Ok(value_snake_case),
                            Err(#error_0_token_stream) => Err(#error_0_token_stream)
                        }
                    }
                }
            };
            let update_query_part_fields_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper|{
                let field_ident = &element.field_ident;
                let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                let update_query_part_field_ident_snake_case = naming::parameter::UpdateQueryPartSelfSnakeCase::from_tokens(&field_ident);
                let field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
                quote::quote!{
                    fn #update_query_part_field_ident_snake_case(
                        #value_snake_case: &postgresql_crud::Value<#field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_upper_camel_case>,
                        #increment_snake_case: &mut std::primitive::u64
                    ) -> Result<#std_string_string, #postgresql_crud_snake_case::#query_part_error_named_upper_camel_case> {
                        match #field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_part_snake_case(
                            &#value_snake_case.#value_snake_case,
                            &#field_ident_double_quotes_token_stream,
                            &#field_ident_double_quotes_token_stream,
                            &"",
                            #increment_snake_case
                        ) {
                            Ok(#value_snake_case) => Ok(#value_snake_case),
                            Err(#error_0_token_stream) => Err(#error_0_token_stream),
                        }
                    }
                }
            });
            quote::quote!{
                impl #ident_update_upper_camel_case {
                    #pub_fn_try_new_token_stream
                    #update_query_part_primary_key_token_stream
                    #update_query_part_fields_token_stream
                }
            }
        };
        let impl_serde_deserialize_for_ident_update_token_stream = postgresql_crud_macros_common::generate_impl_serde_deserialize_for_struct_token_stream(
            &ident_update_upper_camel_case,
            fields.iter().map(|element|(&element.field_ident, &element.syn_field.ty)).collect::<std::vec::Vec<(&syn::Ident, &syn::Type)>>(),
            fields_len,
            &|syn_ident: &syn::Ident, syn_type: &syn::Type|{
                if syn_ident == primary_key_field_ident {
                    quote::quote!{#primary_key_field_type_update_token_stream}
                }
                else {
                    generate_option_value_field_type_as_postgresql_type_update_token_stream(&syn_type)
                }
            },
        );
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
            &ident_update_upper_camel_case,
            &{
                let primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                    quote::quote! {
                        #primary_key_field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                    }
                };
                let fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper|{
                    let field_ident = &element.field_ident;
                    quote::quote! {
                        #field_ident: Some(postgresql_crud::Value{
                            #value_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                        })
                    }
                });
                quote::quote! {Self{
                    #primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream,
                    #fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream
                }}
            }
        );
        quote::quote!{
            #ident_update_token_stream
            #ident_update_try_new_error_named_token_stream
            #impl_ident_update_token_stream
            #impl_serde_deserialize_for_ident_update_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream
        }
    };
    let generate_match_update_query_part_primary_key_token_stream = |operation: &Operation, content_token_stream: &dyn quote::ToTokens|{
        let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
        quote::quote!{
            match #content_token_stream.#update_query_part_primary_key_snake_case(&mut #increment_snake_case) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_0_token_stream) => {
                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
            }
        }
    };
    let use_postgresql_crud_try_stream_ext_token_stream = quote::quote! {use #postgresql_crud_snake_case::TryStreamExt};
    let std_string_string_syn_punctuated_punctuated = macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["std", "string", "String"]);
    let row_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::RowAndRollbackUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &naming::RowSnakeCase, sqlx_error_syn_punctuated_punctuated.clone()),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &rollback_snake_case, sqlx_error_syn_punctuated_punctuated.clone()),
        ],
    );
    let sqlx_query_sqlx_postgres_token_stream = quote::quote! {sqlx::query::<sqlx::Postgres>};
    let (postgresql_crud_postgresql_type_where_filter_query_part_token_stream, postgresql_crud_postgresql_type_where_filter_query_bind_token_stream) = {
        let postgresql_crud_postgresql_type_where_filter_token_stream = quote::quote! {#postgresql_crud_snake_case::PostgresqlTypeWhereFilter::};
        (
            quote::quote! {#postgresql_crud_postgresql_type_where_filter_token_stream #query_part_snake_case},
            quote::quote! {#postgresql_crud_postgresql_type_where_filter_token_stream #query_bind_snake_case},
        )
    };

    let std_vec_vec_struct_options_ident_token_stream = quote::quote! {std::vec::Vec::<#ident_read_upper_camel_case>};
    let not_unique_field_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NotUniqueFieldUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize.clone(),
            &naming::NotUniqueFieldSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&ident_select_upper_camel_case.to_string()]),
        )],
    );
    let serde_json_to_string_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::SerdeJsonToStringUpperCamelCase,
        None,
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
            &naming::SerdeJsonToStringSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
        )],
    );
    let failed_to_get_response_text_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::FailedToGetResponseTextUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &status_code_snake_case,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["http", "StatusCode"]),
            ),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &naming::HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "header", "HeaderMap"]),
            ),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &naming::ReqwestSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "Error"]),
            ),
        ],
    );
    let deserialize_response_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::DeserializeResponseUpperCamelCase,
        None,
        vec![
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &status_code_snake_case,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["http", "StatusCode"]),
            ),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &naming::HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "header", "HeaderMap"]),
            ),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize.clone(),
                &naming::ResponseTextSnakeCase,
                std_string_string_syn_punctuated_punctuated,
            ),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &naming::SerdeSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
            ),
        ],
    );
    let reqwest_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::ReqwestUpperCamelCase,
        None,
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
            &naming::ReqwestSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "Error"]),
        )],
    );
    let check_body_size_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::CheckBodySizeUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
            &naming::CheckBodySizeSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&postgresql_crud_snake_case.to_string(), "check_body_size", &naming::CheckBodySizeErrorNamedUpperCamelCase.to_string()]),
        )],
    );
    let serde_json_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::SerdeJsonUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
            &naming::SerdeJsonSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
        )],
    );
    let header_content_type_application_json_not_found_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::HeaderContentTypeApplicationJsonNotFoundUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        std::vec::Vec::<(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute, &'static dyn std::fmt::Display, syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>)>::default(),
    );
    let common_http_request_syn_variants = {
        vec![
            serde_json_to_string_syn_variant_wrapper.get_syn_variant().clone(),
            failed_to_get_response_text_syn_variant_wrapper.get_syn_variant().clone(),
            deserialize_response_syn_variant_wrapper.get_syn_variant().clone(),
            reqwest_syn_variant_wrapper.get_syn_variant().clone(),
        ]
    };
    let generate_additional_error_variants = |syn_derive_input: &syn::DeriveInput, generate_postgresql_crud_attribute: GeneratePostgresqlCrudAttribute| -> std::vec::Vec<syn::Variant> {
        let generate_postgresql_crud_attribute_stringified = generate_postgresql_crud_attribute.to_string();
        let common_additional_error_variants_attribute_token_stream = macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(&syn_derive_input.attrs, &generate_postgresql_crud_attribute.generate_path_to_attribute());
        let value: syn::DeriveInput = syn::parse((*common_additional_error_variants_attribute_token_stream).clone().into()).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
        let value_ident_stringified = value.ident.to_string();
        assert!(value_ident_stringified == generate_postgresql_crud_attribute_stringified, "{value_ident_stringified} is not equal to {generate_postgresql_crud_attribute_stringified}");
        let variants = if let syn::Data::Enum(data_enum) = value.data {
            data_enum.variants
        } else {
            panic!("value.data is not syn::Data::Enum");
        };
        variants.into_iter().collect()
    };
    let common_additional_error_variants = generate_additional_error_variants(&syn_derive_input, GeneratePostgresqlCrudAttribute::CommonAdditionalErrorVariants);
    let common_route_syn_variants = {
        let common_additional_error_variants_vec = common_additional_error_variants.iter().collect::<std::vec::Vec<&syn::Variant>>();
        let mut value = vec![];
        value.push(check_body_size_syn_variant_wrapper.get_syn_variant());
        value.push(postgresql_syn_variant_wrapper.get_syn_variant());
        value.push(serde_json_syn_variant_wrapper.get_syn_variant());
        value.push(header_content_type_application_json_not_found_syn_variant_wrapper.get_syn_variant());
        for element in common_additional_error_variants_vec {
            value.push(element);
        }
        value
    };
    let common_route_with_row_and_rollback_syn_variants = {
        let mut value = vec![];
        for element in &common_route_syn_variants {
            value.push(*element);
        }
        value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
        value
    };
    let common_additional_logic_token_stream = macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(&syn_derive_input.attrs, &GeneratePostgresqlCrudAttribute::CommonAdditionalLogic.generate_path_to_attribute());
    let generate_pub_handle_token_stream = |is_pub: bool| if is_pub { quote::quote! {pub} } else { proc_macro2::TokenStream::new() };
    let generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = |primary_key_type_token_stream: &dyn quote::ToTokens| {
        let is_pub = true;
        let pub_handle_token_stream = generate_pub_handle_token_stream(is_pub);
        quote::quote! {#pub_handle_token_stream #primary_key_field_ident: #primary_key_type_token_stream}
    };
    let generate_match_postgres_transaction_rollback_await_token_stream = |
        operation: &Operation,
        postgresql_file: &'static str,
        postgresql_line: std::primitive::u32,
        postgresql_column: std::primitive::u32,
        row_and_rollback_file: &'static str,
        row_and_rollback_line: std::primitive::u32,
        row_and_rollback_column: std::primitive::u32
    | {
        let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, postgresql_file, postgresql_line, postgresql_column);
        let row_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream =
            generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &row_and_rollback_syn_variant_wrapper, row_and_rollback_file, row_and_rollback_line, row_and_rollback_column);
        quote::quote! {
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
    let generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream = |
        operation: &Operation,
        postgresql_file: &'static str,
        postgresql_line: std::primitive::u32,
        postgresql_column: std::primitive::u32,
        row_and_rollback_file: &'static str,
        row_and_rollback_line: std::primitive::u32,
        row_and_rollback_column: std::primitive::u32
    | {
        let match_postgres_transaction_rollback_await_token_stream = generate_match_postgres_transaction_rollback_await_token_stream(operation, postgresql_file, postgresql_line, postgresql_column, row_and_rollback_file, row_and_rollback_line, row_and_rollback_column);
        quote::quote! {
            drop(#rows_snake_case);
            #match_postgres_transaction_rollback_await_token_stream
        }
    };
    let wrap_into_value_token_stream = |content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            let #value_snake_case = {
                #content_token_stream
            };
        }
    };
    let generate_fetch_token_stream = |value_handle_token_stream: &dyn quote::ToTokens, try_next_error_initialization_token_stream: &dyn quote::ToTokens| {
        wrap_into_value_token_stream(&quote::quote! {
            let mut #rows_snake_case = #binded_query_snake_case.fetch(#executor_snake_case.as_mut());
            let mut #acc_snake_case = std::vec::Vec::new();
            while let Some(#value_snake_case) = match {
                #use_postgresql_crud_try_stream_ext_token_stream;
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
    let generate_fetch_one_token_stream = |value_handle_token_stream: &dyn quote::ToTokens, fetch_one_error_initialization_token_stream: &dyn quote::ToTokens| {
        wrap_into_value_token_stream(&quote::quote! {
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
    let generate_sqlx_row_try_get_primary_key_token_stream = |sqlx_row_try_get_type_token_stream: &dyn quote::ToTokens, ok_token_stream: &dyn quote::ToTokens, err_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            match #sqlx_row::try_get::<
                #sqlx_row_try_get_type_token_stream,
                #ref_std_primitive_str
            >(&#value_snake_case, &#ident::#primary_key_snake_case()) {
                Ok(#value_snake_case) => #ok_token_stream,
                Err(#error_0_token_stream) => {
                    #err_token_stream
                }
            }
        }
    };
    let wrap_content_into_postgresql_transaction_begin_commit_value_token_stream = |operation: &Operation, content_token_stream: &dyn quote::ToTokens| {
        let postgres_transaction_begin_token_stream = {
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let mut #executor_snake_case = match #sqlx_acquire::#begin_snake_case(#executor_snake_case).await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                };
            }
        };
        let postgres_transaction_commit_token_stream = {
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
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
    let generate_error_occurence_variant_token_stream = |error_variant: &syn::Variant| -> proc_macro2::TokenStream {
        let variant_ident = &error_variant.ident;
        let syn::Fields::Named(fields_named) = &error_variant.fields else {
            panic!("expected fields would be named");
        };
        let fields_mapped_into_token_stream = fields_named.named.iter().map(|field| {
            let field_ident = field.ident.as_ref().unwrap_or_else(|| panic!("{}", naming::FIELD_IDENT_IS_NONE));
            let error_occurence_attribute = if *field_ident == *naming::CodeOccurenceSnakeCase.to_string() {
                proc_macro2::TokenStream::new()
            } else {
                let mut error_occurence_attribute: Option<macros_helpers::error_occurence::ErrorOccurenceFieldAttribute> = None;
                for element in &field.attrs {
                    if element.path().segments.len() == 1 {
                        let segment = element.path().segments.first().unwrap_or_else(|| panic!("element.path().segments.get(0) is None"));
                        if let Ok(value) = {
                            use std::str::FromStr;
                            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::from_str(&segment.ident.to_string())
                        } {
                            match error_occurence_attribute {
                                Some(value) => panic!("duplicated attributes ({}) are not supported", macros_helpers::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&value)),
                                None => {
                                    error_occurence_attribute = Some(value);
                                }
                            }
                        }
                    }
                }
                error_occurence_attribute.map_or_else(|| panic!("{variant_ident} no supported attribute"), |value| value.to_attribute_view_token_stream())
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
    let generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream = |
        operation: &Operation,
        desirable_type_token_stream: &dyn quote::ToTokens,
        type_variants_from_request_response_syn_variants: &std::vec::Vec<syn::Variant>
    | -> proc_macro2::TokenStream {
        let ident_operation_response_variants_upper_camel_case = generate_ident_operation_response_variants_upper_camel_case(&operation);
        let ident_try_operation_logic_response_variants_token_stream = {
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(|element| macros_helpers::error_occurence::generate_serialize_deserialize_version_of_named_syn_variant(element));
            quote::quote! {
                #derive_debug_serde_serialize_serde_deserialize
                pub enum #ident_operation_response_variants_upper_camel_case {
                    #desirable_upper_camel_case(#desirable_type_token_stream),
                    #(#variants_token_stream),*
                }
            }
        };
        let ident_operation_error_named_upper_camel_case = generate_ident_operation_error_named_upper_camel_case(&operation);
        let impl_std_convert_from_ident_operation_error_named_for_ident_operation_response_variants_token_stream = {
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(|element| {
                let variant_ident = &element.ident;
                let syn::Fields::Named(fields_named) = &element.fields else {
                    panic!("expected fields would be named");
                };
                let fields_mapped_into_token_stream = {
                    let fields_token_stream = fields_named.named.iter().map(|field| &field.ident);
                    quote::quote! {#(#fields_token_stream),*}
                };
                let ident_operation_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case(&operation);
                quote::quote! {
                    #ident_operation_error_named_with_serialize_deserialize_upper_camel_case::#variant_ident {
                        #fields_mapped_into_token_stream
                    } => Self::#variant_ident {
                        #fields_mapped_into_token_stream
                    }
                }
            });
            macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                &ident_operation_error_named_upper_camel_case,
                &ident_operation_response_variants_upper_camel_case,
                &quote::quote! {
                    match #value_snake_case.#into_serialize_deserialize_version_snake_case() {
                        #(#variants_token_stream),*
                    }
                },
            )
        };
        let ident_operation_error_named_token_stream = {
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(generate_error_occurence_variant_token_stream);
            quote::quote! {
                #derive_debug_this_error_error_occurence
                pub enum #ident_operation_error_named_upper_camel_case {
                    #(#variants_token_stream),*
                }
            }
        };
        quote::quote! {
            #ident_try_operation_logic_response_variants_token_stream
            #impl_std_convert_from_ident_operation_error_named_for_ident_operation_response_variants_token_stream
            #ident_operation_error_named_token_stream
        }
    };
    let generate_ident_operation_payload_upper_camel_case = |operation: &Operation|{
        match &operation {
            Operation::CreateOne => quote::quote!{#ident_create_upper_camel_case},
            Operation::UpdateOne => quote::quote!{#ident_update_upper_camel_case},
            Operation::CreateMany |
            Operation::ReadMany |
            Operation::ReadOne |
            Operation::UpdateMany |
            Operation::DeleteMany |
            Operation::DeleteOne => format!("{ident}{operation}{}", naming::PayloadUpperCamelCase).parse::<proc_macro2::TokenStream>().unwrap()
        }
    };
    let generate_ident_operation_parameters_upper_camel_case = |operation: &Operation|{
        format!("{ident}{operation}Parameters")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_parameters_pattern_token_stream = |operation: &Operation, payload_token_stream: proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let parameters_token_stream = {
            let ident_operation_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&operation);
            let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
            quote::quote! {
                #derive_debug
                pub struct #ident_operation_parameters_upper_camel_case {
                    pub #payload_snake_case: #ident_operation_payload_upper_camel_case,
                }
            }
        };
        quote::quote! {
            #payload_token_stream
            #parameters_token_stream
        }
    };
    let generate_parameters_payload_and_default_token_stream = |
        operation: &Operation,
        declaration_token_stream: &dyn quote::ToTokens,
        default_init_content_token_stream: &dyn quote::ToTokens
    |{
        let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
        let ident_operation_payload_token_stream = {
            quote::quote! {
                #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
                pub struct #ident_operation_payload_upper_camel_case #declaration_token_stream
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
            &ident_operation_payload_upper_camel_case,
            &quote::quote! {Self #default_init_content_token_stream}
        );
        quote::quote! {
            #ident_operation_payload_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
        }
    };
    let generate_type_variants_from_request_response_syn_variants = |syn_variants: &std::vec::Vec<&syn::Variant>, operation: &Operation| -> std::vec::Vec<syn::Variant> {
        let mut type_variants_from_request_response_syn_variants = std::vec::Vec::new();
        for element in syn_variants {
            type_variants_from_request_response_syn_variants.push((*element).clone());
        }
        let operation_additional_error_variants = generate_additional_error_variants(&syn_derive_input, operation.generate_postgresql_crud_attribute_additional_error_variants());
        for element in operation_additional_error_variants {
            type_variants_from_request_response_syn_variants.push(element.clone());
        }
        type_variants_from_request_response_syn_variants
    };
    let generate_ident_try_operation_error_named_token_stream = |operation: &Operation, common_http_request_syn_variants: &std::vec::Vec<syn::Variant>| -> proc_macro2::TokenStream {
        let ident_try_operation_error_named_upper_camel_case = generate_ident_try_operation_error_named_upper_camel_case(&operation);
        let syn_variants = {
            let mut value = vec![];
            for element in common_http_request_syn_variants {
                value.push(element.clone());
            }
            value.push({
                let ident_operation_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case(&operation);
                new_syn_variant_wrapper(
                    &ident_operation_error_named_with_serialize_deserialize_upper_camel_case,
                    None,
                    vec![(
                        macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                        &operation.operation_error_named_with_serialize_deserialize_snake_case(),
                        macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&ident_operation_error_named_with_serialize_deserialize_upper_camel_case.to_string()]),
                    )],
                )
                .get_syn_variant()
                .clone()
            });
            value
        };
        let variants_token_stream = syn_variants.iter().map(generate_error_occurence_variant_token_stream);
        quote::quote! {
            #derive_debug_thiserror_error_occurence
            pub enum #ident_try_operation_error_named_upper_camel_case {
                #(#variants_token_stream),*
            }
        }
    };
    let std_sync_arc_combination_of_app_state_logic_traits_token_stream = quote::quote!{std::sync::Arc<dyn #postgresql_crud_snake_case::CombinationOfAppStateLogicTraits>};
    let generate_operation_token_stream = |
        operation: &Operation,
        common_additional_logic_token_stream: &dyn quote::ToTokens,
        parameters_logic_token_stream: &dyn quote::ToTokens,
        expected_updated_primary_keys_token_stream: &dyn quote::ToTokens,
        query_string_token_stream: &dyn quote::ToTokens,
        binded_query_token_stream: &dyn quote::ToTokens,
        postgresql_logic_token_stream: &dyn quote::ToTokens
    | -> proc_macro2::TokenStream {
        let operation_snake_case_token_stream = operation.snake_case_token_stream();
        let request_parts_preparation_token_stream = {
            let header_content_type_application_json_not_found_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = &generate_operation_error_initialization_eprintln_response_creation_token_stream(
                operation,
                &header_content_type_application_json_not_found_syn_variant_wrapper,
                file!(),
                line!(),
                column!()
            );
            let check_body_size_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = &generate_operation_error_initialization_eprintln_response_creation_token_stream(
                operation,
                &check_body_size_syn_variant_wrapper,
                file!(),
                line!(),
                column!()
            );
            quote::quote! {
                let (parts, #body_snake_case) = #request_snake_case.into_parts();
                let headers = parts.headers;
                if !matches!(
                    headers.get(axum::http::header::CONTENT_TYPE),
                    Some(value) if value == axum::http::header::HeaderValue::from_static("application/json")
                ) {
                    #header_content_type_application_json_not_found_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                }
                let body_bytes = match #postgresql_crud_snake_case::check_body_size::check_body_size(#body_snake_case, *#app_state_snake_case.get_maximum_size_of_http_body_in_bytes()).await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        #check_body_size_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                    }
                };
            }
        };
        let additional_validators_token_stream = {
            let operation_additional_logic_token_stream = macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(&syn_derive_input.attrs, &operation.generate_postgresql_crud_attribute_additional_logic().generate_path_to_attribute());
            quote::quote! {
                #common_additional_logic_token_stream
                #operation_additional_logic_token_stream
            }
        };
        let acquire_pool_and_connection_token_stream = {
            let postgresql_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let mut #pool_connection_snake_case = match #app_state_snake_case.get_postgres_pool().acquire().await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                    }
                };
                let #executor_snake_case = match sqlx::Acquire::acquire(&mut #pool_connection_snake_case).await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                    }
                };
            }
        };
        let wraped_into_axum_response_token_stream = wrap_into_axum_response_token_stream(
            &{
                let ident_operation_response_variants_upper_camel_case = generate_ident_operation_response_variants_upper_camel_case(&operation);
                quote::quote! {#ident_operation_response_variants_upper_camel_case::#desirable_upper_camel_case(#value_snake_case)}
            },
            &operation.desirable_status_code().to_axum_http_status_code_token_stream(),
        );
        quote::quote! {
            impl #ident {
                pub async fn #operation_snake_case_token_stream(
                    #app_state_snake_case: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_token_stream>,
                    #request_snake_case: axum::extract::Request,
                ) -> axum::response::Response {
                    #request_parts_preparation_token_stream
                    #additional_validators_token_stream
                    #parameters_logic_token_stream
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
                    #wraped_into_axum_response_token_stream
                }
            }
        }
    };
    let generate_parameters_logic_token_stream = |operation: &Operation, operation_payload_with_serialize_deserialize_check_token_stream: &dyn quote::ToTokens| -> proc_macro2::TokenStream {
        let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
        let serde_json_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &serde_json_syn_variant_wrapper, file!(), line!(), column!());
        let ident_operation_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&operation);
        //todo in case of large type there is a stackoverflow. for example it was a 3.5md json file generated by create_many_payload_example. 3400 fields = success. 16000 = stackoverflow
        quote::quote! {
            let #parameters_snake_case = #ident_operation_parameters_upper_camel_case {
                #payload_snake_case: match serde_json::from_slice::<#ident_operation_payload_upper_camel_case>(
                    &#body_bytes_snake_case,
                ) {
                    Ok(#value_snake_case) => {
                        let #value_snake_case = #ident_operation_payload_upper_camel_case::#from_snake_case(#value_snake_case);
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
    let generate_try_operation_token_stream = |
        operation: &Operation,
        type_variants_from_request_response_syn_variants: &[syn::Variant],
        result_ok_type_token_stream: &dyn quote::ToTokens,
        payload_check_token_stream: &dyn quote::ToTokens,
        desirable_from_or_try_from_desirable_with_serialize_deserialize_token_stream: &dyn quote::ToTokens
    | -> proc_macro2::TokenStream {
        let try_operation_snake_case = naming::parameter::TrySelfSnakeCase::from_display(operation);
        let ident_try_operation_error_named_upper_camel_case = generate_ident_try_operation_error_named_upper_camel_case(&operation);
        let ident_operation_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&operation);
        let payload_token_stream = {
            let serde_json_to_string_syn_variant_initialization_token_stream = generate_initialization_token_stream(&serde_json_to_string_syn_variant_wrapper, file!(), line!(), column!());
            let operation_payload_with_serialize_deserialize_initialization_token_stream = {
                let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
                quote::quote! {#ident_operation_payload_upper_camel_case::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
            };
            quote::quote! {
                let #payload_snake_case = {
                    #payload_check_token_stream
                    let #value_snake_case = #operation_payload_with_serialize_deserialize_initialization_token_stream;
                    match serde_json::to_string(&#value_snake_case) {
                        Ok(#value_snake_case) => #value_snake_case,
                        Err(#error_0_token_stream) => {
                            return Err(#ident_try_operation_error_named_upper_camel_case::#serde_json_to_string_syn_variant_initialization_token_stream);
                        }
                    }
                };
            }
        };
        let url_token_stream = {
            let url_handle_token_stream = naming::UrlHandleSelfSnakeCaseTokenStream::url_handle_self_snake_case_token_stream(operation, &ident_snake_case_stringified);
            quote::quote! {
                let #url_snake_case = format!(
                    #url_handle_token_stream,
                    #endpoint_location_snake_case,
                );
            }
        };
        let future_token_stream = {
            let operation_http_method_snake_case_token_stream = naming::AsRefStrToSnakeCaseTokenStream::case_or_panic(&operation.http_method());
            let commit_header_addition_token_stream = quote::quote! {
                .header(
                    &#postgresql_crud_snake_case::CommitSnakeCase.to_string(),//todo remove it
                    git_info::PROJECT_GIT_INFO.commit,
                )
            };
            let application_json_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&"application/json");
            let content_type_application_json_header_addition_token_stream = quote::quote! {
                .header(reqwest::header::CONTENT_TYPE, #application_json_double_quotes_token_stream)
            };
            quote::quote! {
                let #future_snake_case = reqwest::Client::new()
                    .#operation_http_method_snake_case_token_stream(&#url_snake_case)
                    #commit_header_addition_token_stream
                    #content_type_application_json_header_addition_token_stream
                    .#body_snake_case(#payload_snake_case)
                    .send();
            }
        };
        let response_token_stream = {
            let reqwest_syn_variant_initialization_token_stream = generate_initialization_token_stream(&reqwest_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #response_snake_case = match #future_snake_case.await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        return Err(#ident_try_operation_error_named_upper_camel_case::#reqwest_syn_variant_initialization_token_stream);
                    }
                };
            }
        };
        let status_code_token_stream = quote::quote! {
            let #error_0_token_stream = #response_snake_case.status();
        };
        let headers_token_stream = quote::quote! {
            let #error_1_token_stream = #response_snake_case.headers().clone();
        };
        let response_text_token_stream = {
            let failed_to_get_response_text_syn_variant_initialization_token_stream = generate_initialization_token_stream(&failed_to_get_response_text_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #error_2_token_stream = match #response_snake_case.text().await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_2_token_stream) => {
                        return Err(#ident_try_operation_error_named_upper_camel_case::#failed_to_get_response_text_syn_variant_initialization_token_stream);
                    }
                };
            }
        };
        let ident_operation_response_variants_upper_camel_case = generate_ident_operation_response_variants_upper_camel_case(&operation);
        let expected_response_token_stream = {
            let deserialize_response_syn_variant_initialization_token_stream = generate_initialization_token_stream(&deserialize_response_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #expected_response_snake_case = match serde_json::from_str::<#ident_operation_response_variants_upper_camel_case>(&#error_2_token_stream) {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_3_token_stream) => {
                        return Err(#ident_try_operation_error_named_upper_camel_case::#deserialize_response_syn_variant_initialization_token_stream);
                    }
                };
            }
        };
        let try_operation_logic_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case(&operation);
        let operation_error_named_with_serialize_deserialize_snake_case = &operation.operation_error_named_with_serialize_deserialize_snake_case();
        let try_operation_logic_error_named_with_serialize_deserialize_token_stream = {
            let try_operation_logic_response_variants_to_try_operation_logic_error_named_with_serialize_deserialize = type_variants_from_request_response_syn_variants.iter().map(|element| {
                let variant_ident = &element.ident;
                let fields_idents_token_stream = if let syn::Fields::Named(fields_named) = &element.fields {
                    let fields_idents = fields_named.named.iter().map(|field| &field.ident);
                    quote::quote! {#(#fields_idents),*}
                } else {
                    panic!("expected fields would be named");
                };
                quote::quote! {
                    #ident_operation_response_variants_upper_camel_case::#variant_ident {
                        #fields_idents_token_stream
                    } => #try_operation_logic_error_named_with_serialize_deserialize_upper_camel_case::#variant_ident { #fields_idents_token_stream }
                }
            });
            quote::quote! {
                let #operation_error_named_with_serialize_deserialize_snake_case = match #expected_response_snake_case {
                    #ident_operation_response_variants_upper_camel_case::#desirable_upper_camel_case(#value_snake_case) => {
                        let #value_snake_case = #desirable_from_or_try_from_desirable_with_serialize_deserialize_token_stream;
                        return Ok(#value_snake_case);
                    },
                    #(#try_operation_logic_response_variants_to_try_operation_logic_error_named_with_serialize_deserialize),*
                };
            }
        };
        let return_error_token_stream = {
            let field_code_occurence_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_token_stream = macros_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(file!(), line!(), column!());
            quote::quote! {
                Err(#ident_try_operation_error_named_upper_camel_case::#try_operation_logic_error_named_with_serialize_deserialize_upper_camel_case {
                    #operation_error_named_with_serialize_deserialize_snake_case,
                    #field_code_occurence_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_token_stream,
                })
            }
        };
        quote::quote! {
            impl #ident {
                pub async fn #try_operation_snake_case(
                    #endpoint_location_snake_case: #ref_std_primitive_str,
                    #parameters_snake_case: #ident_operation_parameters_upper_camel_case
                ) -> Result<#result_ok_type_token_stream, #ident_try_operation_error_named_upper_camel_case> {
                    #payload_token_stream
                    #url_token_stream
                    #future_token_stream
                    #response_token_stream
                    #status_code_token_stream
                    #headers_token_stream
                    #response_text_token_stream
                    #expected_response_token_stream
                    #try_operation_logic_error_named_with_serialize_deserialize_token_stream
                    #return_error_token_stream
                }
            }
        }
    };
    enum CreateManyOrUpdateManyOrDeleteMany {
        CreateMany,
        UpdateMany,
        DeleteMany
    }
    impl std::convert::From<&CreateManyOrUpdateManyOrDeleteMany> for Operation {
        fn from(value: &CreateManyOrUpdateManyOrDeleteMany) -> Self {
            match &value {
                CreateManyOrUpdateManyOrDeleteMany::CreateMany => Self::CreateMany,
                CreateManyOrUpdateManyOrDeleteMany::UpdateMany => Self::UpdateMany,
                CreateManyOrUpdateManyOrDeleteMany::DeleteMany => Self::DeleteMany,
            }
        }
    }
    let generate_create_update_delete_many_fetch_token_stream = |create_many_or_update_many_or_delete_many: &CreateManyOrUpdateManyOrDeleteMany| {
        let current_operation = match &create_many_or_update_many_or_delete_many {
            CreateManyOrUpdateManyOrDeleteMany::CreateMany => Operation::CreateMany,
            CreateManyOrUpdateManyOrDeleteMany::UpdateMany => Operation::UpdateMany,
            CreateManyOrUpdateManyOrDeleteMany::DeleteMany => Operation::DeleteMany,
        };
        generate_fetch_token_stream(
            &generate_sqlx_row_try_get_primary_key_token_stream(
                &primary_key_field_type_as_primary_key_upper_camel_case,
                &quote::quote! {Some(#value_snake_case)},
                &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
            ),
            &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
        )
    };
    enum CreateOneOrUpdateOneOrDeleteOne {
        CreateOne,
        UpdateOne,
        DeleteOne
    }
    impl std::convert::From<&CreateOneOrUpdateOneOrDeleteOne> for Operation {
        fn from(value: &CreateOneOrUpdateOneOrDeleteOne) -> Self {
            match &value {
                CreateOneOrUpdateOneOrDeleteOne::CreateOne => Self::CreateOne,
                CreateOneOrUpdateOneOrDeleteOne::UpdateOne => Self::UpdateOne,
                CreateOneOrUpdateOneOrDeleteOne::DeleteOne => Self::DeleteOne,
            }
        }
    }
    let generate_create_update_delete_one_fetch_token_stream = |create_one_or_update_one_or_delete_one: &CreateOneOrUpdateOneOrDeleteOne| {
        let current_operation = match &create_one_or_update_one_or_delete_one {
            CreateOneOrUpdateOneOrDeleteOne::CreateOne => Operation::CreateOne,
            CreateOneOrUpdateOneOrDeleteOne::UpdateOne => Operation::UpdateOne,
            CreateOneOrUpdateOneOrDeleteOne::DeleteOne => Operation::DeleteOne,
        };
        generate_fetch_one_token_stream(
            &generate_sqlx_row_try_get_primary_key_token_stream(
                &quote::quote! {#primary_key_field_type_as_primary_key_upper_camel_case},
                &value_snake_case,
                &generate_match_postgres_transaction_rollback_await_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
            ),
            &generate_match_postgres_transaction_rollback_await_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
        )
    };
    let generate_operation_payload_example_token_stream = |operation: &Operation| {
        let operation_payload_example_snake_case = operation.operation_payload_example_snake_case();
        let wraped_into_axum_response_token_stream = wrap_into_axum_response_token_stream(
            &{
                let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
                quote::quote! {<#ident_operation_payload_upper_camel_case as postgresql_crud::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case>::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case()}
            },
            &quote::quote! {axum::http::StatusCode::OK},
        );
        quote::quote! {
            impl #ident {
                pub async fn #operation_payload_example_snake_case() -> axum::response::Response {
                    #wraped_into_axum_response_token_stream
                }
            }
        }
    };
    let increment_initialization_token_stream = quote::quote! {let mut #increment_snake_case: std::primitive::u64 = 0;};
    let column_names = {
        let mut value = fields.iter().fold(std::string::String::default(), |mut acc, element| {
            acc.push_str(&format!("{}", &element.field_ident));
            acc.push(',');
            acc
        });
        let _: Option<char> = value.pop();
        value
    };
    let column_names_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&column_names);
    let create_many_token_stream = {
        let operation = Operation::CreateMany;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_parameters_payload_and_default_token_stream(
                &operation,
                &quote::quote!{(pub std::vec::Vec<#ident_create_upper_camel_case>);},
                &quote::quote!{(vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])},
            )
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream = generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(
                &operation,
                &std_vec_vec_primary_key_field_type_read_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            let operation_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {#postgresql_crud_snake_case::generate_create_many_query_string(
                        &#ident_table_name_call_token_stream,
                        #column_names_double_quotes_token_stream,
                        {
                            #increment_initialization_token_stream
                            let mut #acc_snake_case = #std_string_string::default();
                            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                match #element_snake_case.#create_query_part_snake_case(&mut #increment_snake_case) {
                                    Ok(#value_snake_case) => {
                                        #acc_snake_case.push_str(&format!("({value}),"));
                                    },
                                    Err(#error_0_token_stream) => {
                                        #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                            let _: Option<char> = #acc_snake_case.pop();
                            #acc_snake_case
                        },
                        &#ident::#primary_key_snake_case()
                    )}
                };
                let binded_query_token_stream = quote::quote! {
                    let mut #query_snake_case = sqlx::query::<sqlx::Postgres>(&#query_string_snake_case);
                    for #element_snake_case in #parameters_snake_case.#payload_snake_case.0 {
                        #query_snake_case = #element_snake_case.#create_query_bind_snake_case(#query_snake_case);
                    }
                    #query_snake_case
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_many_fetch_token_stream(&CreateManyOrUpdateManyOrDeleteMany::CreateMany)
                );
                generate_operation_token_stream(
                    &operation,
                    &common_additional_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
                #operation_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_field_type_read_token_stream,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let operation_payload_example_token_stream = generate_operation_payload_example_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
            #operation_payload_example_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"create_many",
    //     &create_many_token_stream,
    // );
    let create_one_token_stream = {
        let operation = Operation::CreateOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = vec![];
                for element in &common_route_with_row_and_rollback_syn_variants {
                    value.push(*element);
                }
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            proc_macro2::TokenStream::new()
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(
                    &operation,
                    &primary_key_field_type_as_primary_key_upper_camel_case,
                    &type_variants_from_request_response_syn_variants,
                );
            let operation_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {
                        #postgresql_crud_snake_case::generate_create_one_query_string(
                            &#ident_table_name_call_token_stream,
                            #column_names_double_quotes_token_stream,
                            match #parameters_snake_case.#payload_snake_case.#create_query_part_snake_case(&mut 0) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_0_token_stream) => {
                                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            },
                            &#ident::#primary_key_snake_case()
                        )
                    }
                };
                let binded_query_token_stream = quote::quote! {
                    let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                    #query_snake_case = #parameters_snake_case.#payload_snake_case.#create_query_bind_snake_case(#query_snake_case);
                    #query_snake_case
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_one_fetch_token_stream(&CreateOneOrUpdateOneOrDeleteOne::CreateOne)
                );
                generate_operation_token_stream(
                    &operation,
                    &common_additional_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
                #operation_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_as_primary_key_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let operation_payload_example_token_stream = generate_operation_payload_example_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
            #operation_payload_example_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"create_one",
    //     &create_one_token_stream,
    // );
    let read_many_token_stream = {
        let operation = Operation::ReadMany;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value.push(not_unique_field_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_parameters_payload_and_default_token_stream(
                &operation,
                &quote::quote!{{
                    #pub_where_many_std_option_option_ident_where_many_token_stream,
                    #pub_select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream,
                    pub #order_by_snake_case: #postgresql_crud_order_by_token_stream<#ident_select_upper_camel_case>,
                    pub #pagination_snake_case: postgresql_crud::PaginationStartsWithZero,
                }},
                &quote::quote!{{
                    #where_many_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                    #select_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                    #order_by_snake_case: postgresql_crud::OrderBy {
                        #column_snake_case: #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(
                            #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                        ),
                        #order_snake_case: Some(
                            #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                        ),
                    },
                    #pagination_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                }},
            )
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream = generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(
                &operation,
                &std_vec_vec_struct_options_ident_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            let operation_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let additional_paramaters_initialization_token_stream = generate_read_or_delete_many_additional_paramaters_initialization_token_stream(&ReadManyOrDeleteMany::ReadMany);
                    let additional_parameters_order_by_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}{order_snake_case} {by_snake_case} {{}} {{}}"));
                    let prefix_to_additional_parameters_token_stream = quote::quote! {
                        let #prefix_snake_case = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                    };
                    let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    let order_by_column_match_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper|{
                        let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                        quote::quote! {
                            #ident_select_upper_camel_case::#field_ident_upper_camel_case(_) => #field_ident_double_quotes_token_stream
                        }
                    });
                    quote::quote! {#postgresql_crud_snake_case::generate_read_many_query_string(
                        &#ident_table_name_call_token_stream,
                        #generate_select_query_part_parameters_payload_select_call_token_stream,
                        {
                            #increment_initialization_token_stream
                            #additional_paramaters_initialization_token_stream
                            {
                                #prefix_to_additional_parameters_token_stream
                                let #value_snake_case = &#parameters_snake_case.#payload_snake_case.#order_by_snake_case;
                                let #order_snake_case = match &#value_snake_case.#order_snake_case {
                                    Some(#value_snake_case) => #value_snake_case.to_snake_case_stringified(),
                                    None => #postgresql_crud_order_token_stream::default().to_snake_case_stringified(),
                                };
                                #additional_parameters_snake_case.push_str(&format!(
                                    #additional_parameters_order_by_handle_token_stream,
                                    #prefix_snake_case,
                                    &match &#value_snake_case.#column_snake_case {
                                        #order_by_column_match_token_stream
                                    },
                                    #order_snake_case,
                                ));
                            }
                            {
                                #prefix_to_additional_parameters_token_stream
                                let #value_snake_case = match #postgresql_crud_postgresql_type_where_filter_query_part_token_stream(
                                    &#parameters_snake_case.#payload_snake_case.pagination,
                                    &mut #increment_snake_case,
                                    &"",
                                    std::primitive::bool::default()
                                ) {
                                    Ok(#value_snake_case) => #value_snake_case,
                                    Err(#error_0_token_stream) => {
                                        #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    },
                                };
                                #additional_parameters_snake_case.push_str(&format!(
                                    "{}{}",
                                    #prefix_snake_case,
                                    #value_snake_case
                                ));
                            }
                            #additional_parameters_snake_case
                        }
                    )}
                };
                let binded_query_token_stream = quote::quote! {
                    let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                    #query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_token_stream
                    #query_snake_case = #postgresql_crud_postgresql_type_where_filter_query_bind_token_stream(
                        #parameters_snake_case.#payload_snake_case.pagination,
                        #query_snake_case,
                    );
                    #query_snake_case
                };
                let postgresql_logic_token_stream = {
                    let fetch_token_stream = generate_fetch_token_stream(
                        &{
                            let match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream = generate_match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream(
                                &ReadManyOrReadOne::ReadMany
                            );
                            quote::quote! {Some(#match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream)}
                        },
                        &generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!()),
                    );
                    quote::quote! {
                        #fetch_token_stream
                        #value_snake_case
                    }
                };
                generate_operation_token_stream(
                    &operation,
                    &common_additional_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
                #operation_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(not_unique_field_syn_variant_wrapper.get_syn_variant().clone());
                value
            });
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_struct_options_ident_token_stream,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {
                    #value_snake_case
                    .into_iter()
                    .fold(std::vec::Vec::new(), |mut #acc_snake_case, #element_snake_case| {
                        #acc_snake_case.push(#element_snake_case);
                        #acc_snake_case
                    })
                },
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let operation_payload_example_token_stream = generate_operation_payload_example_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
            #operation_payload_example_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"read_many",
    //     &read_many_token_stream,
    // );
    let read_one_token_stream = {
        let operation = Operation::ReadOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(not_unique_field_syn_variant_wrapper.get_syn_variant());
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_parameters_payload_and_default_token_stream(
                &operation,
                &{
                    let pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream(
                        &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&primary_key_field_type)
                    );
                    quote::quote!{{
                        #pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream,
                        #pub_select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream,
                    }}
                },
                &quote::quote!{{
                    #primary_key_field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                    #select_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                }},
            )
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(
                    &operation,
                    &ident_read_upper_camel_case,
                    &type_variants_from_request_response_syn_variants,
                );
            let operation_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {#postgresql_crud_snake_case::generate_read_one_query_string(
                        &#ident_table_name_call_token_stream,
                        #generate_select_query_part_parameters_payload_select_call_token_stream,
                        match #postgresql_crud_postgresql_type_where_filter_query_part_token_stream(
                            &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            &mut 0,
                            &#ident::#primary_key_snake_case(),
                            false
                        ) {
                            Ok(#value_snake_case) => #value_snake_case,
                            Err(#error_0_token_stream) => {
                                #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            }
                        }
                    )}
                };
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = quote::quote! {
                        let #query_snake_case = #postgresql_crud_postgresql_type_where_filter_query_bind_token_stream(#parameters_snake_case.#payload_snake_case.#primary_key_field_ident, #query_snake_case);
                    };
                    quote::quote! {
                        let #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_modifications_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = {
                    let fetch_one_token_stream = generate_fetch_one_token_stream(
                        &generate_match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream(&ReadManyOrReadOne::ReadOne),
                        &generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!()),
                    );
                    quote::quote! {
                        #fetch_one_token_stream
                        #value_snake_case
                    }
                };
                generate_operation_token_stream(
                    &operation,
                    &common_additional_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
                #operation_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(not_unique_field_syn_variant_wrapper.get_syn_variant().clone());
                value
            });
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &ident_read_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {#value_snake_case},
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let operation_payload_example_token_stream = generate_operation_payload_example_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
            
            #operation_payload_example_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"read_one",
    //     &read_one_token_stream,
    // );
    //todo update not only with array of objects with ids but with WHERE and one object
    let update_many_token_stream = {
        let operation = Operation::UpdateMany;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            {
                let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
                let std_vec_vec_ident_update_token_stream = quote::quote!{std::vec::Vec<#ident_update_upper_camel_case>};
                let ident_operation_payload_vec_token_stream = quote::quote! {
                    #[derive(Debug, serde::Serialize, utoipa::ToSchema)]
                    pub struct #ident_operation_payload_upper_camel_case(#std_vec_vec_ident_update_token_stream);
                };
                let ident_operation_payload_try_new_error_named_upper_camel_case = format!("{ident}{operation}PayloadTryNewErrorNamed").parse::<proc_macro2::TokenStream>().unwrap();
                let not_unique_primary_key_upper_camel_case = naming::NotUniquePrimaryKeyUpperCamelCase;
                let not_unique_primary_key_snake_case = naming::NotUniquePrimaryKeySnakeCase;
                let ident_operation_payload_try_new_error_named_token_stream = {
                    let primary_key_field_type_update_token_stream = naming::parameter::SelfUpdateUpperCamelCase::from_type_last_segment(&primary_key_field_type);
                    quote::quote!{
                        #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #ident_operation_payload_try_new_error_named_upper_camel_case {
                            #not_unique_primary_key_upper_camel_case {
                                #[eo_to_std_string_string]
                                #not_unique_primary_key_snake_case: #primary_key_field_type_update_token_stream,
                                #[eo_to_std_string_string]
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                            }
                            
                        }
                    }
                };
                let impl_ident_operation_payload_vec_token_stream = {
                    let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
                    quote::quote! {
                        impl #ident_operation_payload_upper_camel_case {
                            fn try_new(#value_snake_case: #std_vec_vec_ident_update_token_stream) -> Result<Self, #ident_operation_payload_try_new_error_named_upper_camel_case> {
                                let mut #acc_snake_case = std::vec::Vec::new();
                                for #element_snake_case in &#value_snake_case {
                                    if !#acc_snake_case.contains(&&#element_snake_case.#primary_key_field_ident) {
                                        #acc_snake_case.push(&#element_snake_case.#primary_key_field_ident);
                                    }
                                    else {
                                        return Err(#ident_operation_payload_try_new_error_named_upper_camel_case::#not_unique_primary_key_upper_camel_case {
                                            #not_unique_primary_key_snake_case: #element_snake_case.#primary_key_field_ident.clone(),
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        });
                                    }
                                }
                                Ok(Self(#value_snake_case))
                            }
                        }
                    }
                };
                let impl_serde_deserialize_for_ident_update_many_payload_token_stream = {
                    let tuple_struct_ident_operation_payload_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                        &format!("tuple struct {ident_operation_payload_upper_camel_case}")
                    );
                    let tuple_struct_ident_operation_payload_with_1_element_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                        &format!("tuple struct {ident_operation_payload_upper_camel_case} with 1 element")
                    );
                    let match_ident_update_many_payload_try_new_field0_token_stream = postgresql_crud_macros_common::generate_match_try_new_in_deserialize_token_stream(
                        &ident_operation_payload_upper_camel_case,
                        &quote::quote!{__field0}
                    );
                    let ident_operation_payload_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_operation_payload_upper_camel_case);
                    quote::quote! {
                        const _: () = {
                            #[allow(unused_extern_crates, clippy::useless_attribute)]
                            extern crate serde as _serde;
                            #[automatically_derived]
                            impl<'de> _serde::Deserialize<'de> for #ident_operation_payload_upper_camel_case {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    #[doc(hidden)]
                                    struct __Visitor<'de> {
                                        marker: _serde::__private::PhantomData<#ident_operation_payload_upper_camel_case>,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    #[automatically_derived]
                                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = #ident_operation_payload_upper_camel_case;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private::Formatter<'_>,
                                        ) -> _serde::__private::fmt::Result {
                                            _serde::__private::Formatter::write_str(
                                                __formatter,
                                                #tuple_struct_ident_operation_payload_double_quotes_token_stream,
                                            )
                                        }
                                        #[inline]
                                        fn visit_newtype_struct<__E>(
                                            self,
                                            __e: __E,
                                        ) -> _serde::__private::Result<Self::Value, __E::Error>
                                        where
                                            __E: _serde::Deserializer<'de>,
                                        {
                                            let __field0: #std_vec_vec_ident_update_token_stream = <#std_vec_vec_ident_update_token_stream as _serde::Deserialize>::deserialize(__e)?;
                                            #match_ident_update_many_payload_try_new_field0_token_stream
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(
                                            self,
                                            mut __seq: __A,
                                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::SeqAccess<'de>,
                                        {
                                            let __field0 = match _serde::de::SeqAccess::next_element::<#std_vec_vec_ident_update_token_stream>(&mut __seq)? {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde::__private::Err(
                                                        _serde::de::Error::invalid_length(
                                                            0usize,
                                                            &#tuple_struct_ident_operation_payload_with_1_element_double_quotes_token_stream,
                                                        ),
                                                    );
                                                }
                                            };
                                            #match_ident_update_many_payload_try_new_field0_token_stream
                                        }
                                    }
                                    _serde::Deserializer::deserialize_newtype_struct(
                                        __deserializer,
                                        #ident_operation_payload_double_quotes_token_stream,
                                        __Visitor {
                                            marker: _serde::__private::PhantomData::<#ident_operation_payload_upper_camel_case>,
                                            lifetime: _serde::__private::PhantomData,
                                        },
                                    )
                                }
                            }
                        };
                    }
                };
                let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
                    &ident_operation_payload_upper_camel_case,
                    &quote::quote! {
                        Self(vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])
                    },
                );
                quote::quote! {
                    #ident_operation_payload_vec_token_stream
                    #ident_operation_payload_try_new_error_named_token_stream
                    #impl_ident_operation_payload_vec_token_stream
                    #impl_serde_deserialize_for_ident_update_many_payload_token_stream
                    #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
                }
            }
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream = generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(
                &operation,
                &std_vec_vec_primary_key_field_type_read_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            let operation_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                    &operation,
                    &proc_macro2::TokenStream::new()
                );
                let query_string_token_stream = {
                    let match_update_query_part_primary_key_token_stream = generate_match_update_query_part_primary_key_token_stream(&operation, &element_snake_case);
                    let fields_named_without_primary_key_update_assignment_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper|{
                        let field_ident = &element.field_ident;
                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                        let is_field_ident_update_exists_snake_case = naming::parameter::IsSelfUpdateExistSnakeCase::from_tokens(&field_ident);
                        let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                        let update_query_part_field_ident_snake_case = naming::parameter::UpdateQueryPartSelfSnakeCase::from_tokens(&field_ident);
                        let generate_when_column_id_then_value_update_many_query_part_snake_case = naming::GenerateWhenColumnIdThenValueUpdateManyQueryPartSnakeCase;
                        let update_query_part_primary_key_snake_case = naming::UpdateQueryPartPrimaryKeySnakeCase;
                        quote::quote! {
                            {
                                let mut #is_field_ident_update_exists_snake_case = false;
                                for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                    if #element_snake_case.#field_ident.is_some() {
                                        #is_field_ident_update_exists_snake_case = true;
                                        break;
                                    }
                                }
                                if #is_field_ident_update_exists_snake_case {
                                    #acc_snake_case.push_str(&
                                        postgresql_crud::generate_column_equals_case_acc_else_column_end_comma_update_many_query_part(
                                            &#field_ident_double_quotes_token_stream,
                                            {
                                                let mut #acc_snake_case = #std_string_string::default();
                                                for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                                    if let Some(#value_snake_case) = &#element_snake_case.#field_ident {
                                                        #acc_snake_case.push_str(&#postgresql_crud_snake_case::#generate_when_column_id_then_value_update_many_query_part_snake_case(
                                                            &#ident::#primary_key_snake_case(),
                                                            match #element_snake_case.#update_query_part_primary_key_snake_case(&mut #increment_snake_case) {
                                                                Ok(#value_snake_case) => #value_snake_case,
                                                                Err(#error_0_token_stream) => {
                                                                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                                }
                                                            },
                                                            match #ident_update_upper_camel_case::#update_query_part_field_ident_snake_case(&#value_snake_case, &mut #increment_snake_case) {
                                                                Ok(#value_snake_case) => #value_snake_case,
                                                                Err(#error_0_token_stream) => {
                                                                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                                }
                                                            },
                                                        ));
                                                    }
                                                }
                                                #acc_snake_case
                                            }
                                        )
                                    );
                                }
                            }
                        }
                    });
                    quote::quote! {
                        {
                            #increment_initialization_token_stream
                            let elements = {
                                let mut #acc_snake_case = #std_string_string::default();
                                #fields_named_without_primary_key_update_assignment_token_stream
                                let _: Option<char> = #acc_snake_case.pop();
                                #acc_snake_case
                            };
                            let primary_keys = {
                                let mut #acc_snake_case = #std_string_string::default();
                                for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                    #acc_snake_case.push_str(&format!("{},", #match_update_query_part_primary_key_token_stream));
                                }
                                let _: Option<char> = #acc_snake_case.pop();
                                #acc_snake_case
                            };
                            postgresql_crud::generate_update_many_query_string(
                                &#ident_table_name_call_token_stream,
                                elements,
                                &#ident::#primary_key_snake_case(),
                                primary_keys
                            )
                        }
                    }
                };
                let binded_query_token_stream = {
                    let fields_named_without_primary_key_update_assignment_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper|{
                        let field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
                        quote::quote! {
                            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                if let Some(#value_snake_case) = &#element_snake_case.#field_ident {
                                    #query_snake_case = #query_snake_case.bind(#element_snake_case.#primary_key_field_ident.clone());
                                    #query_snake_case = #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_bind_snake_case(
                                        #value_snake_case.#value_snake_case.clone(),
                                        #query_snake_case,
                                    );
                                }
                            }
                        }
                    });
                    let primary_key_update_assignment_token_stream = quote::quote! {
                        for #element_snake_case in #parameters_snake_case.#payload_snake_case.0 {
                            #query_snake_case = #primary_key_field_type_as_postgresql_type_token_stream #update_query_bind_snake_case(
                                #element_snake_case.#primary_key_field_ident,
                                #query_snake_case,
                            );
                        }
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #fields_named_without_primary_key_update_assignment_token_stream
                        #primary_key_update_assignment_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation, 
                    &generate_create_update_delete_many_fetch_token_stream(&CreateManyOrUpdateManyOrDeleteMany::UpdateMany)
                );
                generate_operation_token_stream(
                    &operation,
                    &common_additional_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
                #operation_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants.clone());
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_field_type_read_token_stream,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let operation_payload_example_token_stream = generate_operation_payload_example_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
            #operation_payload_example_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"update_many",
    //     &update_many_token_stream,
    // );
    let update_one_token_stream = {
        let operation = Operation::UpdateOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            proc_macro2::TokenStream::new()
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(
                    &operation,
                    &primary_key_field_type_as_primary_key_upper_camel_case,
                    &type_variants_from_request_response_syn_variants,
                );
            let operation_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(
                    &operation,
                    &proc_macro2::TokenStream::new()
                );
                let query_string_token_stream = {
                    let additional_parameters_modification_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper|{
                        let field_ident = &element.field_ident;
                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                        let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                        let generate_column_queals_value_comma_update_one_query_part_snake_case = naming::GenerateColumnQuealsValueCommaUpdateOneQueryPartSnakeCase;
                        let update_query_part_field_ident_snake_case = naming::parameter::UpdateQueryPartSelfSnakeCase::from_tokens(&field_ident);
                        quote::quote! {
                            if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                #acc_snake_case.push_str(&#postgresql_crud_snake_case::#generate_column_queals_value_comma_update_one_query_part_snake_case(
                                    #field_ident_double_quotes_token_stream,
                                    match #ident_update_upper_camel_case::#update_query_part_field_ident_snake_case(&#value_snake_case, &mut #increment_snake_case) {
                                        Ok(#value_snake_case) => #value_snake_case,
                                        Err(#error_0_token_stream) => {
                                            #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        }
                                    }
                                ));
                            }
                        }
                    });
                    let primary_key_query_part_snake_case = naming::PrimaryKeyQueryPartSnakeCase;
                    let additional_parameters_primary_key_modification_token_stream = generate_match_update_query_part_primary_key_token_stream(
                        &operation,
                        &quote::quote!{#parameters_snake_case.#payload_snake_case}
                    );
                    quote::quote! {
                        {
                            #increment_initialization_token_stream
                            let #columns_snake_case = {
                                let mut #acc_snake_case = #std_string_string::default();
                                #additional_parameters_modification_token_stream
                                let _: Option<char> = #acc_snake_case.pop();
                                #acc_snake_case
                            };
                            let #primary_key_query_part_snake_case = #additional_parameters_primary_key_modification_token_stream;
                            #postgresql_crud_snake_case::generate_update_one_query_string(
                                &#ident_table_name_call_token_stream,
                                #columns_snake_case,
                                &#ident::#primary_key_snake_case(),
                                #primary_key_query_part_snake_case
                            )
                        }
                    }
                };
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper|{
                        let field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
                        quote::quote! {
                            if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                #query_snake_case = #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_bind_snake_case(
                                    #value_snake_case.#value_snake_case,
                                    #query_snake_case,
                                );
                            }
                        }
                    });
                    let binded_query_primary_key_modification_token_stream = quote::quote! {
                        #query_snake_case = #primary_key_field_type_as_postgresql_type_token_stream #update_query_bind_snake_case(
                            #parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            #query_snake_case,
                        );
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_modifications_token_stream
                        #binded_query_primary_key_modification_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_one_fetch_token_stream(&CreateOneOrUpdateOneOrDeleteOne::UpdateOne)
                );
                generate_operation_token_stream(
                    &operation,
                    &common_additional_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
                #operation_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_as_primary_key_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let operation_payload_example_token_stream = generate_operation_payload_example_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
            #operation_payload_example_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"update_one",
    //     &update_one_token_stream,
    // );
    let delete_many_token_stream = {
        let operation = Operation::DeleteMany;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_parameters_payload_and_default_token_stream(
                &operation,
                &quote::quote!{{#pub_where_many_std_option_option_ident_where_many_token_stream}},
                &quote::quote!{{#where_many_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}},
            )
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(
                    &operation,
                    &std_vec_vec_primary_key_field_type_read_token_stream,
                    &type_variants_from_request_response_syn_variants,
                );
            let operation_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let additional_paramaters_initialization_token_stream = generate_read_or_delete_many_additional_paramaters_initialization_token_stream(&ReadManyOrDeleteMany::DeleteMany);
                    quote::quote! {#postgresql_crud_snake_case::generate_delete_many_query_string(
                        &#ident_table_name_call_token_stream,
                        {
                            #increment_initialization_token_stream
                            #additional_paramaters_initialization_token_stream
                            #additional_parameters_snake_case
                        },
                        &#ident::#primary_key_snake_case(),
                    )}
                };
                let binded_query_token_stream = quote::quote! {
                    let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                    #query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_token_stream
                    #query_snake_case
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_many_fetch_token_stream(&CreateManyOrUpdateManyOrDeleteMany::DeleteMany)
                );
                generate_operation_token_stream(
                    &operation,
                    &common_additional_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
                #operation_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_field_type_read_token_stream,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let operation_payload_example_token_stream = generate_operation_payload_example_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
            #operation_payload_example_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"delete_many",
    //     &delete_many_token_stream,
    // );
    let delete_one_token_stream = {
        let operation = Operation::DeleteOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(&common_route_with_row_and_rollback_syn_variants, &operation);
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_parameters_payload_and_default_token_stream(
                &operation,
                &{
                    let content_token_stream = generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream(
                        &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&primary_key_field_type)
                    );
                    quote::quote!{{#content_token_stream}}
                },
                &{
                    let primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                        quote::quote! {
                            #primary_key_field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                        }
                    };
                    quote::quote!{{#primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream}}
                },
            )
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(
                    &operation,
                    &primary_key_field_type_as_primary_key_upper_camel_case,
                    &type_variants_from_request_response_syn_variants,
                );
            let operation_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = quote::quote! {#postgresql_crud_snake_case::generate_delete_one_query_string(
                    &#ident_table_name_call_token_stream,
                    &#ident::#primary_key_snake_case(),
                )};
                let binded_query_token_stream = {
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #query_snake_case = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(
                            #parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            #query_snake_case
                        );
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_one_fetch_token_stream(&CreateOneOrUpdateOneOrDeleteOne::DeleteOne)
                );
                generate_operation_token_stream(
                    &operation,
                    &common_additional_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
                #operation_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_as_primary_key_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let operation_payload_example_token_stream = generate_operation_payload_example_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
            #operation_payload_example_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"delete_one",
    //     &delete_one_token_stream,
    // );
    let routes_token_stream = {
        let generate_slash_route_double_quotes_token_stream = |value: &dyn std::fmt::Display|{
            generate_quotes::double_quotes_token_stream(&format!("/{value}"))
        };
        let create_many = Operation::CreateMany;
        let create_one = Operation::CreateOne;
        let read_many = Operation::ReadMany;
        let read_one = Operation::ReadOne;
        let update_many = Operation::UpdateMany;
        let update_one = Operation::UpdateOne;
        let delete_many = Operation::DeleteMany;
        let delete_one = Operation::DeleteOne;
        let slash_create_many_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&create_many.snake_case_stringified());
        let slash_create_one_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&create_one.snake_case_stringified());
        let slash_read_many_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&read_many.snake_case_stringified());
        let slash_read_one_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&read_one.snake_case_stringified());
        let slash_update_many_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&update_many.snake_case_stringified());
        let slash_update_one_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&update_one.snake_case_stringified());
        let slash_delete_many_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&delete_many.snake_case_stringified());
        let slash_delete_one_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&delete_one.snake_case_stringified());
        let create_many_snake_case_token_stream = create_many.snake_case_token_stream();
        let create_one_snake_case_token_stream = create_one.snake_case_token_stream();
        let read_many_snake_case_token_stream = read_many.snake_case_token_stream();
        let read_one_snake_case_token_stream = read_one.snake_case_token_stream();
        let update_many_snake_case_token_stream = update_many.snake_case_token_stream();
        let update_one_snake_case_token_stream = update_one.snake_case_token_stream();
        let delete_many_snake_case_token_stream = delete_many.snake_case_token_stream();
        let delete_one_snake_case_token_stream = delete_one.snake_case_token_stream();
        let create_many_payload_example_snake_case = create_many.operation_payload_example_snake_case();
        let create_one_payload_example_snake_case = create_one.operation_payload_example_snake_case();
        let read_many_payload_example_snake_case = read_many.operation_payload_example_snake_case();
        let read_one_payload_example_snake_case = read_one.operation_payload_example_snake_case();
        let update_many_payload_example_snake_case = update_many.operation_payload_example_snake_case();
        let update_one_payload_example_snake_case = update_one.operation_payload_example_snake_case();
        let delete_many_payload_example_snake_case = delete_many.operation_payload_example_snake_case();
        let delete_one_payload_example_snake_case = delete_one.operation_payload_example_snake_case();
        let slash_create_many_example_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&create_many_payload_example_snake_case);
        let slash_create_one_example_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&create_one_payload_example_snake_case);
        let slash_read_many_example_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&read_many_payload_example_snake_case);
        let slash_read_one_example_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&read_one_payload_example_snake_case);
        let slash_update_many_example_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&update_many_payload_example_snake_case);
        let slash_update_one_example_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&update_one_payload_example_snake_case);
        let slash_delete_many_example_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&delete_many_payload_example_snake_case);
        let slash_delete_one_example_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&delete_one_payload_example_snake_case);
        quote::quote!{
            impl #ident {
                pub fn routes(#app_state_snake_case: #std_sync_arc_combination_of_app_state_logic_traits_token_stream) -> axum::Router {
                    axum::Router::new().nest(
                        &format!("/{}",#ident::#table_name_snake_case()),
                        axum::Router::new()
                        .route(#slash_create_many_double_quotes_token_stream, axum::routing::post(#ident::#create_many_snake_case_token_stream))
                        .route(#slash_create_many_example_double_quotes_token_stream, axum::routing::get(#ident::#create_many_payload_example_snake_case))
                        .route(#slash_create_one_double_quotes_token_stream, axum::routing::post(#ident::#create_one_snake_case_token_stream))
                        .route(#slash_create_one_example_double_quotes_token_stream, axum::routing::get(#ident::#create_one_payload_example_snake_case))
                        .route(#slash_read_many_double_quotes_token_stream, axum::routing::post(#ident::#read_many_snake_case_token_stream))
                        .route(#slash_read_many_example_double_quotes_token_stream, axum::routing::get(#ident::#read_many_payload_example_snake_case))
                        .route(#slash_read_one_double_quotes_token_stream, axum::routing::post(#ident::#read_one_snake_case_token_stream))
                        .route(#slash_read_one_example_double_quotes_token_stream, axum::routing::get(#ident::#read_one_payload_example_snake_case))
                        .route(#slash_update_many_double_quotes_token_stream, axum::routing::patch(#ident::#update_many_snake_case_token_stream))
                        .route(#slash_update_many_example_double_quotes_token_stream, axum::routing::get(#ident::#update_many_payload_example_snake_case))
                        .route(#slash_update_one_double_quotes_token_stream, axum::routing::patch(#ident::#update_one_snake_case_token_stream))
                        .route(#slash_update_one_example_double_quotes_token_stream, axum::routing::get(#ident::#update_one_payload_example_snake_case))
                        .route(#slash_delete_many_double_quotes_token_stream, axum::routing::delete(#ident::#delete_many_snake_case_token_stream))
                        .route(#slash_delete_many_example_double_quotes_token_stream, axum::routing::get(#ident::#delete_many_payload_example_snake_case))
                        .route(#slash_delete_one_double_quotes_token_stream, axum::routing::delete(#ident::#delete_one_snake_case_token_stream))
                        .route(#slash_delete_one_example_double_quotes_token_stream, axum::routing::get(#ident::#delete_one_payload_example_snake_case))
                        // .layer(tower_http::cors::CorsLayer::new().allow_methods(#ident::allow_methods()))
                        .with_state(#app_state_snake_case)
                    )
                }
            }
        }
    };
    let ident_tests_token_stream = {
        let ident_tests_snake_case = naming::parameter::SelfTestsSnakeCase::from_display(&ident);
        let drop_table_if_exists_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
            &format!("drop table if exists {}", naming::DisplayToSnakeCaseStringified::case(&ident))
        );
        let ident_create_many_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::CreateMany);
        let ident_create_many_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&Operation::CreateMany);
        let ident_create_one_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::CreateOne);
        let ident_read_many_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::ReadMany);
        let ident_read_many_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&Operation::ReadMany);
        let ident_read_one_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::ReadOne);
        let ident_read_one_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&Operation::ReadOne);
        let ident_update_many_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::UpdateMany);
        let ident_update_many_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&Operation::UpdateMany);
        let ident_update_one_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::UpdateOne);
        let ident_delete_many_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::DeleteMany);
        let ident_delete_many_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&Operation::DeleteMany);
        let ident_delete_one_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::DeleteOne);
        let ident_delete_one_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&Operation::DeleteOne);
        let ident_try_read_one_error_named_upper_camel_case = generate_ident_try_operation_error_named_upper_camel_case(&Operation::ReadOne);
        let ident_read_one_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case(&Operation::ReadOne);
        let std_option_option_ident_where_many_upper_camel_case = naming::parameter::StdOptionOptionSelfWhereManyUpperCamelCase::from_tokens(&ident);
        let ident_create_default_fields_initialization_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
            let field_ident = &element.field_ident;
            let field_type_as_postgresql_type_create_token_stream = generate_as_postgresql_type_create_token_stream(&element.syn_field.ty);
            quote::quote! {
                #field_ident: <#field_type_as_postgresql_type_create_token_stream as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()
            }
        });
        let fields_none_initialization_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
            let field_ident = &element.field_ident;
            quote::quote! {#field_ident: None}
        });
        let new_or_try_new_unwraped_for_test_snake_case = naming::NewOrTryNewUnwrapedForTestSnakeCase;
        //todo instead of first dropping table - check if its not exists. if exists test must fail
        let update_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
            let field_ident = &element.field_ident;
            let field_type = &element.syn_field.ty;
            let field_type_as_postgresql_type_update_token_stream = generate_as_postgresql_type_update_token_stream(
                &field_type
            );
            let fields_without_primary_key_and_field_ident = {
                let mut acc = vec![];
                for field in &fields {
                    if &field.field_ident != primary_key_field_ident && &field.field_ident != field_ident {
                        acc.push(field);
                    }
                }
                acc
            };
            let ident_update_try_new_without_primary_key_token_stream = fields_without_primary_key.iter().fold(vec![], |mut acc, element| {
                if &element.field_ident == field_ident {
                    acc.push(quote::quote!{some_value_update.clone()});
                }
                else {
                    acc.push(quote::quote!{None});
                }
                acc
            });
            let field_ident_nones_token_stream = fields_without_primary_key_and_field_ident.iter().fold(vec![], |mut acc, element| {
                let field_ident = &element.field_ident;
                acc.push(quote::quote!{#field_ident: None});
                acc
            });
            let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
            let field_type_as_postgresql_type_select_token_stream = generate_as_postgresql_type_select_token_stream(&field_type);
            let field_type_as_postgresql_type_read_token_stream = generate_as_postgresql_type_read_token_stream(&field_type);
            let try_update_many_result_different_for_field_ident_field_type_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!("try_update_many result different for {field_ident}: {}", quote::quote!{#field_type})
            );
            let try_read_many_result_different_after_try_update_many_for_field_ident_field_type_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!("try_read_many result different after try_update_many for {field_ident}: {}", quote::quote!{#field_type})
            );
            let try_update_one_result_different_for_field_ident_field_type_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!("try_update_one result different for {field_ident}: {}", quote::quote!{#field_type})
            );
            let try_read_one_result_different_after_try_update_one_for_field_ident_field_type_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                &format!("try_read_one result different after try_update_one for {field_ident}: {}", quote::quote!{#field_type})
            );
            let postgresql_type_test_cases_upper_camel_case = naming::PostgresqlTypeTestCasesUpperCamelCase;
            quote::quote! {
                for #element_snake_case in <#field_type as postgresql_crud::tests::#postgresql_type_test_cases_upper_camel_case>:: #test_cases_snake_case() {
                    let some_value_update = Some(postgresql_crud::Value {
                        value: #field_type_as_postgresql_type_update_token_stream::#new_or_try_new_unwraped_for_test_snake_case(#element_snake_case.clone()),
                    });
                    let vec_of_primary_keys_returned_from_update_many = {
                        let mut value = super::#ident::try_update_many(
                            &url,
                            super::#ident_update_many_parameters_upper_camel_case {
                                payload: super::#ident_update_many_payload_upper_camel_case::try_new(vec![
                                    super::#ident_update_upper_camel_case::try_new(
                                        #primary_key_field_type_as_postgresql_type_update_token_stream::from(primary_key_read_returned_from_create_many1.clone()),
                                        #(#ident_update_try_new_without_primary_key_token_stream),*
                                    ).unwrap(),
                                    super::#ident_update_upper_camel_case::try_new(
                                        #primary_key_field_type_as_postgresql_type_update_token_stream::from(primary_key_read_returned_from_create_many2.clone()),
                                        #(#ident_update_try_new_without_primary_key_token_stream),*
                                    ).unwrap(),
                                ])
                                .unwrap(),
                            },
                        )
                        .await
                        .unwrap();
                        value.sort();
                        value
                    };
                    assert_eq!(
                        {
                            let mut value = vec![
                                primary_key_read_returned_from_create_many1.clone(),
                                primary_key_read_returned_from_create_many2.clone()
                            ];
                            value.sort();
                            value
                        },
                        vec_of_primary_keys_returned_from_update_many,
                        #try_update_many_result_different_for_field_ident_field_type_double_quotes_token_stream
                    );
                    let select_primary_key_field_ident = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![
                        super::#ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(
                            <
                                #primary_key_field_type_as_postgresql_type_select_token_stream as #postgresql_crud_snake_case::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case
                            >::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case()
                        ),
                        super::#ident_select_upper_camel_case::#field_ident_upper_camel_case_token_stream(
                            <
                                #field_type_as_postgresql_type_select_token_stream as #postgresql_crud_snake_case::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case
                            >::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case()
                        ),
                    ])
                    .unwrap();
                    let vec_of_ident_read_returned_from_read_many = vec_of_ident_read_with_primary_key_sort_by_primary_key(super::#ident::try_read_many(
                            &url,
                            super::#ident_read_many_parameters_upper_camel_case {
                                payload: super::#ident_read_many_payload_upper_camel_case {
                                    where_many: where_many_1_and_2_primary_keys.clone(),
                                    select: select_primary_key_field_ident.clone(),
                                    order_by: postgresql_crud::OrderBy {
                                        column: super::#ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream),
                                        order: Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream),
                                    },
                                    pagination: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                },
                            },
                        )
                        .await
                        .unwrap()
                    );
                    let some_value_field_ident_read = Some(postgresql_crud::Value {
                        value: #field_type_as_postgresql_type_read_token_stream::#new_or_try_new_unwraped_for_test_snake_case(#element_snake_case),
                    });
                    assert_eq!(
                        vec_of_ident_read_with_primary_key_sort_by_primary_key(vec![
                            super::#ident_read_upper_camel_case {
                                #primary_key_field_ident: some_value_primary_key_read_returned_from_create_many1.clone(),
                                #field_ident: some_value_field_ident_read.clone(),
                                #(#field_ident_nones_token_stream),*
                            },
                            super::#ident_read_upper_camel_case {
                                #primary_key_field_ident: some_value_primary_key_read_returned_from_create_many2.clone(),
                                #field_ident: some_value_field_ident_read.clone(),
                                #(#field_ident_nones_token_stream),*
                            }
                        ]),
                        vec_of_ident_read_returned_from_read_many,
                        #try_read_many_result_different_after_try_update_many_for_field_ident_field_type_double_quotes_token_stream
                    );
                    let primary_key_returned_from_update_one = super::#ident::try_update_one(
                        &url,
                        super::#ident_update_one_parameters_upper_camel_case {
                            payload: super::#ident_update_upper_camel_case::try_new(
                                #primary_key_field_type_as_postgresql_type_update_token_stream::from(primary_key_read_returned_from_create_one.clone()),
                                #(#ident_update_try_new_without_primary_key_token_stream),*
                            ).unwrap(),
                        },
                    )
                    .await
                    .unwrap();
                    assert_eq!(
                        primary_key_read_returned_from_create_one.clone(),
                        primary_key_returned_from_update_one,
                        #try_update_one_result_different_for_field_ident_field_type_double_quotes_token_stream
                    );
                    let ident_read_returned_from_read_one = super::#ident::try_read_one(
                        &url,
                        super::#ident_read_one_parameters_upper_camel_case {
                            payload: super::#ident_read_one_payload_upper_camel_case {
                                #primary_key_field_ident: primary_key_read_returned_from_create_one.clone(),
                                select: select_primary_key_field_ident.clone(),
                            },
                        },
                    )
                    .await
                    .unwrap();
                    assert_eq!(
                        super::#ident_read_upper_camel_case {
                            #primary_key_field_ident: some_value_primary_key_read_returned_from_create_one.clone(),
                            #field_ident: some_value_field_ident_read.clone(),
                            #(#field_ident_nones_token_stream),*
                        },
                        ident_read_returned_from_read_one,
                        #try_read_one_result_different_after_try_update_one_for_field_ident_field_type_double_quotes_token_stream
                    );
                }
            }
        });
        quote::quote! {
            #[cfg(test)]
            mod #ident_tests_snake_case {
                #[test]
                fn test_size_of() {
                    assert_eq!(std::mem::size_of::<super::#ident>(), 0);
                }
                #[test]
                fn test_crud() {
                    std::thread::Builder::new()
                        .stack_size(16 * 1024 * 1024)
                        .spawn(|| {
                            tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().unwrap().block_on(async {
                                static CONFIG: std::sync::OnceLock<crate::repositories_types::server::config::Config> = std::sync::OnceLock::new();
                                let config = CONFIG.get_or_init(|| crate::repositories_types::server::config::Config::try_from_env().unwrap());
                                let postgres_pool = sqlx::postgres::PgPoolOptions::new().connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&config))).await.unwrap();
                                let url = format!("http://{}", app_state::GetServiceSocketAddress::get_service_socket_address(&config));
                                async fn drop_table_if_exists(postgres_pool: &sqlx::Pool<sqlx::Postgres>) {
                                    let query = #drop_table_if_exists_ident_double_quotes_token_stream;
                                    println!("{query}");
                                    let _unused = sqlx::query(query).execute(postgres_pool).await.unwrap();
                                }
                                drop_table_if_exists(&postgres_pool).await;
                                let postgres_pool_for_tokio_spawn_sync_move = postgres_pool.clone();
                                let _unused = tokio::spawn(async move {
                                    super::#ident::prepare_postgresql(&postgres_pool_for_tokio_spawn_sync_move).await.unwrap();
                                    let app_state = std::sync::Arc::new(crate::repositories_types::server::routes::app_state::AppState {
                                        postgres_pool: postgres_pool_for_tokio_spawn_sync_move.clone(),
                                        config: &config,
                                        project_git_info: &git_info::PROJECT_GIT_INFO,
                                    });
                                    axum::serve(
                                        tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&config)).await.unwrap(),
                                        axum::Router::new().merge(super::#ident::routes(std::sync::Arc::<crate::repositories_types::server::routes::app_state::AppState<'_>>::clone(&app_state))).into_make_service(),
                                    )
                                    .await
                                    .unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
                                });
                                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                                let ident_create_default = super::#ident_create_upper_camel_case {
                                    #ident_create_default_fields_initialization_without_primary_key_token_stream
                                };
                                let vec_of_primary_keys_returned_from_create_many = super::#ident::try_create_many(
                                    &url,
                                    super::#ident_create_many_parameters_upper_camel_case {
                                        payload: super::#ident_create_many_payload_upper_camel_case(vec![ident_create_default.clone(), ident_create_default.clone()]),
                                    },
                                )
                                .await
                                .unwrap();
                                assert_eq!(2, vec_of_primary_keys_returned_from_create_many.len(), "try_create_many result different");
                                let (
                                    primary_key_read_returned_from_create_many1,
                                    primary_key_read_returned_from_create_many2
                                ) = {
                                    let mut iter = vec_of_primary_keys_returned_from_create_many.into_iter();
                                    (
                                        iter.next().unwrap(),
                                        iter.next().unwrap()
                                    )
                                };
                                let select_primary_key = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![
                                    super::#ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(
                                        #primary_key_field_type_as_postgresql_type_select_token_stream::default(),
                                    )
                                ])
                                .unwrap();
                                let where_many_1_and_2_primary_keys = super::#std_option_option_ident_where_many_upper_camel_case(Some(super::#ident_where_many_upper_camel_case {
                                    #primary_key_field_ident: Some(postgresql_crud::PostgresqlTypeWhere::try_new(
                                        postgresql_crud::LogicalOperator::Or,
                                        vec![
                                            #primary_key_field_type_as_postgresql_type_where_element_token_stream::Equal(postgresql_crud::where_element_filters::PostgresqlTypeWhereElementEqual {
                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                value: #primary_key_field_type_as_postgresql_type_table_type_declaration_token_stream::from(primary_key_read_returned_from_create_many1.clone())
                                            }),
                                            #primary_key_field_type_as_postgresql_type_where_element_token_stream::Equal(postgresql_crud::where_element_filters::PostgresqlTypeWhereElementEqual {
                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                value: #primary_key_field_type_as_postgresql_type_table_type_declaration_token_stream::from(primary_key_read_returned_from_create_many2.clone())
                                            })
                                        ]
                                    ).unwrap()),
                                    #fields_none_initialization_token_stream
                                }));
                                let vec_of_ident_read_with_primary_key_sort_by_primary_key = |mut vec: std::vec::Vec<super::#ident_read_upper_camel_case>| -> std::vec::Vec<super::#ident_read_upper_camel_case> {
                                    vec.sort_by_key(|element| element.#primary_key_field_ident.clone().unwrap().value);
                                    vec
                                };
                                let vec_of_ident_read_returned_from_read_many = vec_of_ident_read_with_primary_key_sort_by_primary_key(
                                    super::#ident::try_read_many(
                                        &url,
                                        super::#ident_read_many_parameters_upper_camel_case {
                                            payload: super::#ident_read_many_payload_upper_camel_case {
                                                where_many: where_many_1_and_2_primary_keys.clone(),
                                                select: select_primary_key.clone(),
                                                order_by: postgresql_crud::OrderBy {
                                                    column: super::#ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream),
                                                    order: Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream),
                                                },
                                                pagination: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                            },
                                        },
                                    )
                                    .await
                                    .unwrap()
                                );
                                let some_value_primary_key_read_returned_from_create_many1 = Some(postgresql_crud::Value { value: primary_key_read_returned_from_create_many1.clone() });
                                let some_value_primary_key_read_returned_from_create_many2 = Some(postgresql_crud::Value { value: primary_key_read_returned_from_create_many2.clone() });
                                assert_eq!(
                                    vec_of_ident_read_with_primary_key_sort_by_primary_key(vec![
                                        super::#ident_read_upper_camel_case {
                                            #primary_key_field_ident: some_value_primary_key_read_returned_from_create_many1.clone(),
                                            #fields_none_initialization_token_stream
                                        },
                                        super::#ident_read_upper_camel_case {
                                            #primary_key_field_ident: some_value_primary_key_read_returned_from_create_many2.clone(),
                                            #fields_none_initialization_token_stream
                                        }
                                    ]),
                                    vec_of_ident_read_returned_from_read_many,
                                    "try_read_many result different after try_create_many"
                                );
                                let primary_key_returned_from_create_one = super::#ident::try_create_one(
                                    &url,
                                    super::#ident_create_one_parameters_upper_camel_case {
                                        payload: ident_create_default
                                    }
                                ).await.unwrap();
                                let primary_key_read_returned_from_create_one = primary_key_returned_from_create_one;
                                let ident_read_returned_from_read_one = super::#ident::try_read_one(
                                    &url,
                                    super::#ident_read_one_parameters_upper_camel_case {
                                        payload: super::#ident_read_one_payload_upper_camel_case {
                                            #primary_key_field_ident: primary_key_read_returned_from_create_one.clone(),
                                            select: select_primary_key.clone(),
                                        },
                                    },
                                )
                                .await
                                .unwrap();
                                let some_value_primary_key_read_returned_from_create_one = Some(postgresql_crud::Value { value: primary_key_read_returned_from_create_one.clone() });
                                assert_eq!(
                                    super::#ident_read_upper_camel_case {
                                        #primary_key_field_ident: some_value_primary_key_read_returned_from_create_one.clone(),
                                        #fields_none_initialization_token_stream
                                    },
                                    ident_read_returned_from_read_one,
                                    "try_read_one result different after try_create_one"
                                );
                                #update_token_stream
                                let vec_of_primary_keys_returned_from_delete_many = {
                                    let mut value = super::#ident::try_delete_many(
                                        &url,
                                        super::#ident_delete_many_parameters_upper_camel_case {
                                            payload: super::#ident_delete_many_payload_upper_camel_case { where_many: where_many_1_and_2_primary_keys.clone() },
                                        },
                                    )
                                    .await
                                    .unwrap();
                                    value.sort();
                                    value
                                };
                                assert_eq!(
                                    {
                                        let mut value = vec![
                                            primary_key_read_returned_from_create_many1.clone(),
                                            primary_key_read_returned_from_create_many2.clone()
                                        ];
                                        value.sort();
                                        value
                                    },
                                    vec_of_primary_keys_returned_from_delete_many,
                                    "try_delete_many result different"
                                );
                                let vec_of_ident_read_returned_from_read_many = super::#ident::try_read_many(
                                    &url,
                                    super::#ident_read_many_parameters_upper_camel_case {
                                        payload: super::#ident_read_many_payload_upper_camel_case {
                                            where_many: where_many_1_and_2_primary_keys.clone(),
                                            select: select_primary_key.clone(),
                                            order_by: postgresql_crud::OrderBy {
                                                column: super::#ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream),
                                                order: Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream),
                                            },
                                            pagination: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                                        },
                                    },
                                )
                                .await
                                .unwrap();
                                assert_eq!(
                                    std::vec::Vec::<super::#ident_read_upper_camel_case>::default(),
                                    vec_of_ident_read_returned_from_read_many,
                                    "try_read_many result different after try_delete_many"
                                );
                                let primary_key_returned_from_delete_one = super::#ident::try_delete_one(
                                    &url,
                                    super::#ident_delete_one_parameters_upper_camel_case {
                                        payload: super::#ident_delete_one_payload_upper_camel_case { #primary_key_field_ident: primary_key_read_returned_from_create_one.clone() },
                                    },
                                )
                                .await
                                .unwrap();
                                assert_eq!(primary_key_read_returned_from_create_one.clone(), primary_key_returned_from_delete_one, "try_delete_one result different");
                                if let Err(error) = super::#ident::try_read_one(
                                    &url,
                                    super::#ident_read_one_parameters_upper_camel_case {
                                        payload: super::#ident_read_one_payload_upper_camel_case {
                                            #primary_key_field_ident: primary_key_read_returned_from_create_one.clone(),
                                            select: select_primary_key.clone(),
                                        },
                                    },
                                ).await &&
                                let super::#ident_try_read_one_error_named_upper_camel_case::#ident_read_one_error_named_with_serialize_deserialize_upper_camel_case {
                                    read_one_error_named_with_serialize_deserialize,
                                    code_occurence: _,
                                } = &error &&
                                let super::#ident_read_one_error_named_with_serialize_deserialize_upper_camel_case::Postgresql { postgresql, code_occurence: _ } = &read_one_error_named_with_serialize_deserialize &&
                                "no rows returned by a query that expected to return at least one row".to_string() != *postgresql {
                                    panic!("try_read_one result different after try_delete_one");
                                }
                                drop_table_if_exists(&postgres_pool).await;
                            });
                        })
                        .unwrap()
                        .join()
                        .unwrap();
                }
            }
        }
    };
    // if ident == "" {
        // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //     "GeneratePostgresqlCrud",
        //     &ident_tests_token_stream,
        // );
    // }
    let common_token_stream = quote::quote! {
        #impl_ident_token_stream
        #ident_create_token_stream
        #ident_where_many_token_stream
        #std_option_option_ident_where_many_token_stream
        #select_token_stream
        #ident_read_token_stream
        #ident_column_read_permission_token_stream
        #ident_update_token_stream
    };
    // if ident == "" {
        // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //     "GeneratePostgresqlCrud",
        //     &common_token_stream,
        // );
    // }
    let generated = quote::quote! {
        #common_token_stream
        #create_many_token_stream
        #create_one_token_stream
        #read_many_token_stream
        #read_one_token_stream
        #update_many_token_stream
        #update_one_token_stream
        #delete_many_token_stream
        #delete_one_token_stream
        #routes_token_stream
        #ident_tests_token_stream
    };
    // if ident == "" {
        // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //     "GeneratePostgresqlCrud",
        //     &generated,
        // );
    // }
    generated.into()
}
///////////
