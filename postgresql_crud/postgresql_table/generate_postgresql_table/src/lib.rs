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

#[proc_macro_derive(GeneratePostgresqlTable, attributes(generate_postgresql_table_primary_key))]
pub fn generate_postgresql_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Debug, Clone)]
    struct SynFieldWrapper {
        syn_field: syn::Field,
        field_ident: syn::Ident,
    }
    #[derive(Debug)]
    struct SynVariantWrapper {
        variant: syn::Variant,
        status_code: Option<macros_helpers::status_code::StatusCode>,
    }
    impl SynVariantWrapper {
        const fn get_syn_variant(&self) -> &syn::Variant {
            &self.variant
        }
        const fn get_option_status_code(&self) -> Option<&macros_helpers::status_code::StatusCode> {
            self.status_code.as_ref()
        }
    }
    enum ShouldAddBorrow {
        True,
        False,
    }
    impl quote::ToTokens for ShouldAddBorrow {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match &self {
                Self::True => quote::quote! {&}.to_tokens(tokens),
                Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
            }
        }
    }
    enum ShouldAddReturn {
        True,
        False,
    }
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
        const fn generate_postgresql_table_attribute_additional_error_variants(self) -> GeneratePostgresqlTableAttribute {
            match self {
                Self::CreateMany => GeneratePostgresqlTableAttribute::CreateManyAdditionalErrorVariants,
                Self::CreateOne => GeneratePostgresqlTableAttribute::CreateOneAdditionalErrorVariants,
                Self::ReadMany => GeneratePostgresqlTableAttribute::ReadManyAdditionalErrorVariants,
                Self::ReadOne => GeneratePostgresqlTableAttribute::ReadOneAdditionalErrorVariants,
                Self::UpdateMany => GeneratePostgresqlTableAttribute::UpdateManyAdditionalErrorVariants,
                Self::UpdateOne => GeneratePostgresqlTableAttribute::UpdateOneAdditionalErrorVariants,
                Self::DeleteMany => GeneratePostgresqlTableAttribute::DeleteManyAdditionalErrorVariants,
                Self::DeleteOne => GeneratePostgresqlTableAttribute::DeleteOneAdditionalErrorVariants,
            }
        }
        const fn generate_postgresql_table_attribute_additional_logic(self) -> GeneratePostgresqlTableAttribute {
            match self {
                Self::CreateMany => GeneratePostgresqlTableAttribute::CreateManyAdditionalLogic,
                Self::CreateOne => GeneratePostgresqlTableAttribute::CreateOneAdditionalLogic,
                Self::ReadMany => GeneratePostgresqlTableAttribute::ReadManyAdditionalLogic,
                Self::ReadOne => GeneratePostgresqlTableAttribute::ReadOneAdditionalLogic,
                Self::UpdateMany => GeneratePostgresqlTableAttribute::UpdateManyAdditionalLogic,
                Self::UpdateOne => GeneratePostgresqlTableAttribute::UpdateOneAdditionalLogic,
                Self::DeleteMany => GeneratePostgresqlTableAttribute::DeleteManyAdditionalLogic,
                Self::DeleteOne => GeneratePostgresqlTableAttribute::DeleteOneAdditionalLogic,
            }
        }
        fn operation_error_named_with_serialize_deserialize_snake_case(self) -> naming::parameter::SelfErrorNamedWithSerializeDeserializeSnakeCase {
            naming::parameter::SelfErrorNamedWithSerializeDeserializeSnakeCase::from_display(&self)
        }
        fn self_snake_case_stringified(self) -> String {
            naming::AsRefStrToSnakeCaseStringified::case(&self.to_string())
        }
        fn self_snake_case_token_stream(self) -> proc_macro2::TokenStream {
            naming::AsRefStrToSnakeCaseTokenStream::case_or_panic(&self.to_string())
        }
        fn self_handle_snake_case_token_stream(self) -> proc_macro2::TokenStream {
            let value = naming::parameter::SelfHandleSnakeCase::from_tokens(&self.self_snake_case_token_stream());
            quote::quote! {#value}
        }
        fn try_self_snake_case_token_stream(self) -> proc_macro2::TokenStream {
            let value = naming::parameter::TrySelfSnakeCase::from_tokens(&self.self_snake_case_token_stream());
            quote::quote! {#value}
        }
        fn try_self_handle_snake_case_token_stream(self) -> proc_macro2::TokenStream {
            let value = naming::parameter::TrySelfHandleSnakeCase::from_tokens(&self.self_snake_case_token_stream());
            quote::quote! {#value}
        }
        fn operation_payload_example_snake_case(self) -> impl naming::StdFmtDisplayPlusQuoteToTokens {
            naming::parameter::SelfPayloadExampleSnakeCase::from_display(&self)
        }
    }
    impl std::fmt::Display for Operation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                Self::CreateMany => write!(f, "CreateMany"),
                Self::CreateOne => write!(f, "CreateOne"),
                Self::ReadMany => write!(f, "ReadMany"),
                Self::ReadOne => write!(f, "ReadOne"),
                Self::UpdateMany => write!(f, "UpdateMany"),
                Self::UpdateOne => write!(f, "UpdateOne"),
                Self::DeleteMany => write!(f, "DeleteMany"),
                Self::DeleteOne => write!(f, "DeleteOne"),
            }
        }
    }
    impl From<&CreateOrUpdateOrDeleteMany> for Operation {
        fn from(value: &CreateOrUpdateOrDeleteMany) -> Self {
            match &value {
                CreateOrUpdateOrDeleteMany::Create => Self::CreateMany,
                CreateOrUpdateOrDeleteMany::Update => Self::UpdateMany,
                CreateOrUpdateOrDeleteMany::Delete => Self::DeleteMany,
            }
        }
    }
    impl From<&ReadManyOrDeleteMany> for Operation {
        fn from(value: &ReadManyOrDeleteMany) -> Self {
            match &value {
                ReadManyOrDeleteMany::ReadMany => Self::ReadMany,
                ReadManyOrDeleteMany::DeleteMany => Self::DeleteMany,
            }
        }
    }
    impl From<&ReadManyOrReadOne> for Operation {
        fn from(value: &ReadManyOrReadOne) -> Self {
            match &value {
                ReadManyOrReadOne::ReadMany => Self::ReadMany,
                ReadManyOrReadOne::ReadOne => Self::ReadOne,
            }
        }
    }
    impl From<&CreateOrUpdateOrDeleteOne> for Operation {
        fn from(value: &CreateOrUpdateOrDeleteOne) -> Self {
            match &value {
                CreateOrUpdateOrDeleteOne::Create => Self::CreateOne,
                CreateOrUpdateOrDeleteOne::Update => Self::UpdateOne,
                CreateOrUpdateOrDeleteOne::Delete => Self::DeleteOne,
            }
        }
    }
    #[derive(naming::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified)]
    enum OperationHttpMethod {
        Post,
        Patch,
        Delete,
    }
    enum ReadManyOrDeleteMany {
        ReadMany,
        DeleteMany,
    }
    enum ReadManyOrReadOne {
        ReadMany,
        ReadOne,
    }
    #[derive(Debug, strum_macros::Display)]
    enum GeneratePostgresqlTableAttribute {
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
    impl GeneratePostgresqlTableAttribute {
        fn generate_path_to_attribute(self) -> String {
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
    enum ShouldWrapIntoValue {
        True,
        False,
    }
    enum CreateOrUpdateOrDeleteMany {
        Create,
        Update,
        Delete,
    }
    enum CreateOrUpdateOrDeleteOne {
        Create,
        Update,
        Delete,
    }
    panic_location::panic_location();
    let generate_select_query_part_snake_case = naming::GenerateSelectQueryPartSnakeCase;
    let create_extension_if_not_exists_pg_jsonschema_upper_camel_case = naming::CreateExtensionIfNotExistsPgJsonschemaUpperCamelCase;
    let create_extension_if_not_exists_uuid_ossp_upper_camel_case = naming::CreateExtensionIfNotExistsUuidOsspUpperCamelCase;
    let prepare_postgresql_upper_camel_case = naming::PreparePostgresqlUpperCamelCase;
    let pool_snake_case = naming::PoolSnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let element_snake_case = naming::ElementSnakeCase;
    let update_snake_case = naming::UpdateSnakeCase;
    let no_fields_provided_upper_camel_case = naming::NoFieldsProvidedUpperCamelCase;
    let where_many_snake_case = naming::WhereManySnakeCase;
    let additional_parameters_snake_case = naming::AdditionalParametersSnakeCase;
    let postgresql_crud_snake_case = &naming::PostgresqlCrudSnakeCase;
    let value_upper_camel_case = naming::ValueUpperCamelCase;
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
    let table_snake_case = naming::TableSnakeCase;
    let table_name_snake_case = naming::TableNameSnakeCase;
    let update_for_query_upper_camel_case = naming::UpdateForQueryUpperCamelCase;
    let query_part_error_named_upper_camel_case = naming::QueryPartErrorNamedUpperCamelCase;
    let into_serialize_deserialize_version_snake_case = naming::IntoSerializeDeserializeVersionSnakeCase;
    let primary_key_snake_case = naming::PrimaryKeySnakeCase;
    let pagination_snake_case = naming::PaginationSnakeCase;
    let config_snake_case = naming::ConfigSnakeCase;
    let postgres_pool_snake_case = naming::PostgresPoolSnakeCase;
    let ident_create_default_snake_case = naming::IdentCreateDefaultSnakeCase;
    let postgres_pool_for_tokio_spawn_sync_move_snake_case = naming::PostgresPoolForTokioSpawnSyncMoveSnakeCase;
    let select_primary_key_snake_case = naming::SelectPrimaryKeySnakeCase;
    let update_for_query_vec_snake_case = naming::UpdateForQueryVecSnakeCase;
    let some_value_read_only_ids_returned_from_create_one_snake_case = naming::SomeValueReadOnlyIdsReturnedFromCreateOneSnakeCase;
    let common_read_only_ids_returned_from_create_one_snake_case = naming::CommonReadOnlyIdsReturnedFromCreateOneSnakeCase;
    let select_only_updated_ids_query_part_snake_case = naming::SelectOnlyUpdatedIdsQueryPartSnakeCase;
    let update_for_query_snake_case = naming::UpdateForQuerySnakeCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let create_snake_case = naming::CreateSnakeCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
    let create_table_column_query_part_snake_case = naming::CreateTableColumnQueryPartSnakeCase;
    let read_only_ids_merged_with_create_into_where_equal_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoWhereEqualSnakeCase;
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSnakeCase;
    let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_snake_case = naming::ReadOnlyIdsMergedWithCreateIntoOptionVecWhereEqualToJsonFieldSnakeCase;
    let create_into_postgresql_json_type_option_vec_where_length_equal_snake_case = naming::CreateIntoPostgresqlJsonTypeOptionVecWhereLengthEqualSnakeCase;
    // let create_into_postgresql_json_type_option_vec_where_length_greater_than_snake_case = naming::CreateIntoPostgresqlJsonTypeOptionVecWhereLengthGreaterThanSnakeCase;
    let create_into_postgresql_type_option_vec_where_dimension_one_equal_snake_case = naming::CreateIntoPostgresqlTypeOptionVecWhereDimensionOneEqualSnakeCase;
    let read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_snake_case = naming::ReadOnlyIdsMergedWithTableTypeDeclarationIntoPostgresqlTypeOptionWhereGreaterThanSnakeCase;
    let default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementUpperCamelCase;
    let default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;
    let read_only_ids_into_table_type_declaration_snake_case = naming::ReadOnlyIdsIntoTableTypeDeclarationSnakeCase;
    let read_only_ids_into_read_snake_case = naming::ReadOnlyIdsIntoReadSnakeCase;
    let read_only_ids_into_update_snake_case = naming::ReadOnlyIdsIntoUpdateSnakeCase;
    let read_into_table_type_declaration_snake_case = naming::ReadIntoTableTypeDeclarationSnakeCase;
    let prepare_postgresql_snake_case = naming::PreparePostgresqlSnakeCase;
    let prepare_extensions_snake_case = naming::PrepareExtensionsSnakeCase;
    let prepare_postgresql_table_snake_case = naming::PreparePostgresqlTableSnakeCase;
    let option_vec_create_snake_case = naming::OptionVecCreateSnakeCase;
    let postgresql_type_option_vec_where_greater_than_test_snake_case = naming::PostgresqlTypeOptionVecWhereGreaterThanTestSnakeCase;
    let routes_handle_snake_case = naming::RoutesHandleSnakeCase;
    let routes_snake_case = naming::RoutesSnakeCase;
    let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
    let from_handle_snake_case = naming::FromHandleSnakeCase;
    let select_only_ids_query_part_snake_case = naming::SelectOnlyIdsQueryPartSnakeCase;
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
    // let derive_debug_clone_copy = token_patterns::DeriveDebugCloneCopy;
    let ref_std_primitive_str = token_patterns::RefStdPrimitiveStr;
    let field_attribute_serde_skip_serializing_if_option_is_none_token_stream = token_patterns::FieldAttributeSerdeSkipSerializingIfOptionIsNone;
    let sqlx_row = token_patterns::SqlxRow;
    let postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
    let string_token_stream = token_patterns::StdStringString;

    let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrud;
    let return_err_query_part_error_named_write_into_buffer_token_stream = postgresql_crud_macros_common::generate_return_err_query_part_error_named_write_into_buffer_token_stream(import_path);

    // let postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_snake_case_stringified = naming::ToTokensToSnakeCaseStringified::case(&ident);
    let ident_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_snake_case_stringified);
    let self_table_name_call_token_stream = quote::quote! {Self::#table_name_snake_case()};
    let (primary_key_field, fields, fields_without_primary_key) = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            let mut option_primary_key_field: Option<SynFieldWrapper> = None;
            let mut fields = vec![];
            let mut fields_without_primary_key = vec![];
            for element in &fields_named.named {
                let field_ident = element.ident.clone().expect("error 915ef2ce-d4d5-4943-997a-a2a004807452");
                let field_ident_len = field_ident.to_string().len();
                let max_postgresql_column_length = 63;
                //todo write runtime check
                assert!(field_ident_len <= max_postgresql_column_length, "Postgresql truncates column names to {max_postgresql_column_length} characters, this is more: {field_ident} ({field_ident_len} characters)");
                fields.push(SynFieldWrapper { syn_field: element.clone(), field_ident: field_ident.clone() });
                let mut is_primary_key = false;
                {
                    for attr in &element.attrs {
                        if attr.path().segments.len() == 1 {
                            let first_segment_ident = &attr.path().segments.first().expect("no first value in punctuated").ident;
                            let generate_postgresql_table_primary_key_snake_case_stringified = naming::GeneratePostgresqlTablePrimaryKeySnakeCase.to_string();
                            if first_segment_ident == &generate_postgresql_table_primary_key_snake_case_stringified {
                                if option_primary_key_field.is_some() {
                                    panic!("two or more supported {generate_postgresql_table_primary_key_snake_case_stringified} attributes!");
                                } else {
                                    option_primary_key_field = Some(SynFieldWrapper { syn_field: element.clone(), field_ident: field_ident.clone() });
                                    is_primary_key = true;
                                }
                            }
                        }
                    }
                }
                if !is_primary_key {
                    fields_without_primary_key.push(SynFieldWrapper { syn_field: element.clone(), field_ident: field_ident.clone() });
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
    let primary_key_field_type_where_token_stream = naming::parameter::SelfWhereUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty);
    //todo must remove this and use trait type instead
    let primary_key_field_type_table_type_declaration_token_stream = naming::parameter::SelfTableTypeDeclarationUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty);
    let generate_as_postgresql_type_token_stream = |field_type: &dyn quote::ToTokens| {
        quote::quote! {<#field_type as postgresql_crud::PostgresqlType>::}
    };
    let primary_key_field_type_as_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&primary_key_field_type);
    let generate_as_postgresql_type_tokens_token_stream = |field_type: &dyn quote::ToTokens, tokens: &dyn quote::ToTokens| {
        let as_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&field_type);
        quote::quote! {#as_postgresql_type_token_stream #tokens}
    };
    // let generate_as_postgresql_type_table_type_declaration_token_stream = |field_type: &dyn quote::ToTokens| generate_as_postgresql_type_tokens_token_stream(&field_type, &naming::TableTypeDeclarationUpperCamelCase);
    // let primary_key_field_type_as_postgresql_type_table_type_declaration_token_stream = generate_as_postgresql_type_table_type_declaration_token_stream(&primary_key_field_type);
    let generate_as_postgresql_type_create_token_stream = |field_type: &dyn quote::ToTokens| generate_as_postgresql_type_tokens_token_stream(&field_type, &naming::CreateUpperCamelCase);
    let generate_as_postgresql_type_select_token_stream = |field_type: &dyn quote::ToTokens| generate_as_postgresql_type_tokens_token_stream(&field_type, &naming::SelectUpperCamelCase);
    let primary_key_field_type_as_postgresql_type_select_token_stream = generate_as_postgresql_type_select_token_stream(&primary_key_field_type);
    let generate_as_postgresql_type_where_token_stream = |field_type: &dyn quote::ToTokens| generate_as_postgresql_type_tokens_token_stream(&field_type, &naming::WhereUpperCamelCase);
    let primary_key_field_type_as_postgresql_type_where_token_stream = generate_as_postgresql_type_where_token_stream(&primary_key_field_type);
    let generate_as_postgresql_type_read_token_stream = |field_type: &dyn quote::ToTokens| generate_as_postgresql_type_tokens_token_stream(&field_type, &naming::ReadUpperCamelCase);
    let generate_as_postgresql_type_read_only_ids_token_stream = |field_type: &dyn quote::ToTokens| generate_as_postgresql_type_tokens_token_stream(&field_type, &naming::ReadOnlyIdsUpperCamelCase);
    let primary_key_field_type_as_postgresql_type_read_token_stream = generate_as_postgresql_type_read_token_stream(&primary_key_field_type);
    let generate_as_postgresql_type_update_token_stream = |field_type: &dyn quote::ToTokens| generate_as_postgresql_type_tokens_token_stream(&field_type, &naming::UpdateUpperCamelCase);
    let generate_as_postgresql_type_update_for_query_token_stream = |field_type: &dyn quote::ToTokens| generate_as_postgresql_type_tokens_token_stream(&field_type, &naming::UpdateForQueryUpperCamelCase);
    let primary_key_field_type_as_postgresql_type_read_upper_camel_case = quote::quote! {<#primary_key_field_type as postgresql_crud::#postgresql_type_upper_camel_case>::#read_upper_camel_case};
    let ident_read_only_ids_upper_camel_case = naming::parameter::SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident);
    let ident_delete_many_parameters_upper_camel_case = naming::parameter::SelfDeleteManyParametersUpperCamelCase::from_tokens(&ident);
    let ident_delete_many_payload_upper_camel_case = naming::parameter::SelfDeleteManyPayloadUpperCamelCase::from_tokens(&ident);
    let ident_delete_one_parameters_upper_camel_case = naming::parameter::SelfDeleteOneParametersUpperCamelCase::from_tokens(&ident);
    let ident_delete_one_payload_upper_camel_case = naming::parameter::SelfDeleteOnePayloadUpperCamelCase::from_tokens(&ident);
    let ident_try_read_one_error_named_upper_camel_case = naming::parameter::SelfTryReadOneErrorNamedUpperCamelCase::from_tokens(&ident);
    let ident_read_one_error_named_with_serialize_deserialize_upper_camel_case = naming::parameter::SelfReadOneErrorNamedWithSerializeDeserializeUpperCamelCase::from_tokens(&ident);
    let ident_try_delete_one_error_named_upper_camel_case = naming::parameter::SelfTryDeleteOneErrorNamedUpperCamelCase::from_tokens(&ident);
    let ident_delete_one_error_named_with_serialize_deserialize_upper_camel_case = naming::parameter::SelfDeleteOneErrorNamedWithSerializeDeserializeUpperCamelCase::from_tokens(&ident);
    let std_vec_vec_primary_key_field_type_read_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&primary_key_field_type_as_postgresql_type_read_upper_camel_case);
    let std_vec_vec_ident_read_only_ids_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&ident_read_only_ids_upper_camel_case);
    let primary_key_field_ident = &primary_key_field.field_ident;
    let primary_key_field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&primary_key_field_ident);
    let primary_key_field_type_update_token_stream = &naming::parameter::SelfUpdateUpperCamelCase::from_type_last_segment(primary_key_field_type);
    let primary_key_field_type_update_for_query_token_stream = &naming::parameter::SelfUpdateForQueryUpperCamelCase::from_type_last_segment(primary_key_field_type);
    let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
    let generate_from_handle_token_stream = |ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            fn #from_handle_snake_case(#value_snake_case: #ident_token_stream) -> Self {
                #content_token_stream
            }
        }
    };
    let generate_select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream = |should_add_borrow: &ShouldAddBorrow| {
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
    let none_token_stream = quote::quote! {None};
    let fields_named_with_comma_none_token_stream = generate_fields_named_with_comma_token_stream(&|_: &SynFieldWrapper| -> proc_macro2::TokenStream { none_token_stream.clone() });
    let fields_named_without_primary_key_with_comma_none_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|_: &SynFieldWrapper| -> proc_macro2::TokenStream { none_token_stream.clone() });
    let mut impl_ident_vec_token_stream = vec![];
    let impl_ident_token_stream = {
        let ident_prepare_postgresql_error_named_upper_camel_case = naming::parameter::SelfPreparePostgresqlErrorNamedUpperCamelCase::from_tokens(&ident);
        let content_token_stream = quote::quote! {
            #[eo_to_std_string_string]
            error: sqlx::Error,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        };
        let ident_prepare_postgresql_error_named_token_stream = quote::quote! {
            #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum #ident_prepare_postgresql_error_named_upper_camel_case {
                #create_extension_if_not_exists_pg_jsonschema_upper_camel_case {
                    #content_token_stream
                },
                #create_extension_if_not_exists_uuid_ossp_upper_camel_case {
                    #content_token_stream
                },
                #prepare_postgresql_upper_camel_case {
                    #content_token_stream
                },
            }
        };
        let pub_fn_table_token_stream = quote::quote! {
            pub const fn #table_name_snake_case() -> &'static str {
                #ident_snake_case_double_quotes_token_stream
            }
        };
        let fn_primary_key_token_stream = {
            let primary_key_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&primary_key_field_ident);
            quote::quote! {
                const fn #primary_key_snake_case() -> &'static str {
                    #primary_key_field_ident_double_quotes_token_stream
                }
            }
        };
        let pub_async_fn_prepare_extensions_token_stream = quote::quote! {
            pub async fn #prepare_extensions_snake_case(#pool_snake_case: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #ident_prepare_postgresql_error_named_upper_camel_case> {
                if let Err(#error_snake_case) = sqlx::query("create extension if not exists pg_jsonschema").execute(#pool_snake_case).await {
                    return Err(#ident_prepare_postgresql_error_named_upper_camel_case::#create_extension_if_not_exists_pg_jsonschema_upper_camel_case {
                        #error_snake_case,
                        code_occurence: error_occurence_lib::code_occurence!()
                    });
                }
                if let Err(#error_snake_case) = sqlx::query("create extension if not exists \"uuid-ossp\"").execute(#pool_snake_case).await {
                    return Err(#ident_prepare_postgresql_error_named_upper_camel_case::#create_extension_if_not_exists_uuid_ossp_upper_camel_case {
                        #error_snake_case,
                        code_occurence: error_occurence_lib::code_occurence!()
                    });
                }
                Ok(())
            }
        };
        let pub_async_fn_prepare_postgresql_table_token_stream = {
            let prepare_postgresql_double_quotes_token_stream = {
                let acc = {
                    let mut acc = String::new();
                    for _ in &fields {
                        acc.push_str("{},");
                    }
                    let _: Option<char> = acc.pop();
                    acc
                };
                generate_quotes::double_quotes_token_stream(&format!("create table if not exists {{table}} ({acc})"))
            };
            let serde_json_to_string_schemars_schema_for_generic_unwrap_token_stream = {
                let generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream = |field_type: &syn::Type, field_ident: &syn::Ident, is_primary_key: bool| {
                    let is_primary_key_token_stream: &dyn quote::ToTokens = if is_primary_key { &naming::TrueSnakeCase } else { &naming::FalseSnakeCase };
                    let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                    let field_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&field_type);
                    quote::quote! {
                        #field_type_postgresql_type_token_stream #create_table_column_query_part_snake_case(&#field_ident_double_quotes_token_stream, #is_primary_key_token_stream)
                    }
                };
                let mut acc = vec![generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream(primary_key_field_type, &primary_key_field.field_ident, true)];
                for element in &fields_without_primary_key {
                    acc.push(generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream(&element.syn_field.ty, &element.field_ident, false));
                }
                acc
            };
            quote::quote! {
                pub async fn #prepare_postgresql_table_snake_case(#pool_snake_case: &sqlx::Pool<sqlx::Postgres>, table: &str) -> Result<(), #ident_prepare_postgresql_error_named_upper_camel_case> {
                    if let Err(error) = sqlx::query(&format!(
                        #prepare_postgresql_double_quotes_token_stream,
                        #(#serde_json_to_string_schemars_schema_for_generic_unwrap_token_stream),*
                    )).execute(#pool_snake_case).await {
                        return Err(#ident_prepare_postgresql_error_named_upper_camel_case::#prepare_postgresql_upper_camel_case {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!()
                        });
                    }
                    Ok(())
                }
            }
        };
        let pub_async_fn_prepare_postgresql_token_stream = quote::quote! {
            pub async fn #prepare_postgresql_snake_case(#pool_snake_case: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #ident_prepare_postgresql_error_named_upper_camel_case> {
                Self::#prepare_extensions_snake_case(#pool_snake_case).await?;
                Self::#prepare_postgresql_table_snake_case(#pool_snake_case, #ident_snake_case_double_quotes_token_stream).await?;
                Ok(())
            }
        };
        let pub_fn_allow_methods_token_stream = {
            let http_method_token_stream = quote::quote! {http::Method};
            quote::quote! {
                pub const fn allow_methods() -> [#http_method_token_stream;4] {[
                    #http_method_token_stream::GET,
                    #http_method_token_stream::POST,
                    #http_method_token_stream::PATCH,
                    #http_method_token_stream::DELETE
                ]}
            }
        };
        let fn_generate_select_query_part_token_stream = {
            let variants_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                let initialization_token_stream = {
                    let field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                    let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
                    quote::quote! {
                        => match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #select_query_part_snake_case(
                            #column_snake_case,
                            #field_ident_string_double_quotes_token_stream
                        ) {
                            Ok(#value_snake_case) => #value_snake_case,
                            Err(#error_snake_case) => {
                                return Err(#error_snake_case);
                            }
                        }
                    }
                };
                quote::quote! {#ident_select_upper_camel_case::#field_ident_upper_camel_case_token_stream(#column_snake_case) #initialization_token_stream}
            });
            let std_option_option_std_primitive_char_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&token_patterns::StdPrimitiveChar);
            quote::quote! {
                fn #generate_select_query_part_snake_case(#select_borrow_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream) -> Result<#string_token_stream, #import_path ::#query_part_error_named_upper_camel_case> {
                    let mut #acc_snake_case = #string_token_stream::default();
                    for #element_snake_case in #select_snake_case.to_vec() {
                        #acc_snake_case.push_str(&match #element_snake_case {
                            #variants_token_stream
                        });
                        #acc_snake_case.push(',');
                    }
                    let _: #std_option_option_std_primitive_char_token_stream = #acc_snake_case.pop();
                    Ok(#acc_snake_case)
                }
            }
        };
        impl_ident_vec_token_stream.push(quote::quote! {
            #pub_fn_table_token_stream
            #fn_primary_key_token_stream
            #pub_async_fn_prepare_extensions_token_stream
            #pub_async_fn_prepare_postgresql_table_token_stream
            #pub_async_fn_prepare_postgresql_token_stream
            #pub_fn_allow_methods_token_stream
            #fn_generate_select_query_part_token_stream
        });
        quote::quote! {
            #ident_prepare_postgresql_error_named_token_stream
        }
    };
    let wrap_into_axum_response_token_stream = |axum_json_content_token_stream: &dyn quote::ToTokens, status_code_token_stream: &dyn quote::ToTokens, should_add_return: &ShouldAddReturn| {
        let return_content_token_stream = match should_add_return {
            ShouldAddReturn::True => quote::quote! {return response;},
            ShouldAddReturn::False => quote::quote! {response},
        };
        quote::quote! {
            let mut response = axum::response::IntoResponse::into_response(
                axum::Json(#axum_json_content_token_stream)
            );
            *response.status_mut() = #status_code_token_stream;
            #return_content_token_stream
        }
    };
    let eprintln_error_token_stream = quote::quote! {eprintln!("{error}");};
    let generate_ident_operation_error_named_upper_camel_case = |operation: &Operation| format!("{ident}{operation}ErrorNamed").parse::<proc_macro2::TokenStream>().expect("error 79ab147e-f603-4cb7-81af-2e35344780fe");
    let generate_ident_operation_response_variants_upper_camel_case = |operation: &Operation| format!("{ident}{operation}ResponseVariants").parse::<proc_macro2::TokenStream>().expect("error f386c0d4-6704-475a-8045-91431f1da815");
    let generate_initialization_token_stream = |syn_variant_wrapper: &SynVariantWrapper, file: &'static str, line: u32, column: u32| -> proc_macro2::TokenStream {
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
    let generate_operation_error_initialization_eprintln_response_creation_token_stream = |operation: &Operation, syn_variant_wrapper: &SynVariantWrapper, file: &'static str, line: u32, column: u32| {
        let ident_operation_error_named_upper_camel_case = generate_ident_operation_error_named_upper_camel_case(operation);
        let ident_operation_response_variants_upper_camel_case = generate_ident_operation_response_variants_upper_camel_case(operation);
        let syn_variant_initialization_token_stream = generate_initialization_token_stream(syn_variant_wrapper, file, line, column);
        let status_code_token_stream = syn_variant_wrapper.get_option_status_code().unwrap_or_else(|| panic!("option_status_code is None")).to_axum_http_status_code_token_stream();
        let wraped_into_axum_response_token_stream = wrap_into_axum_response_token_stream(&quote::quote! {#ident_operation_response_variants_upper_camel_case::#from_handle_snake_case(#error_snake_case)}, &status_code_token_stream, &ShouldAddReturn::True);
        quote::quote! {
            let #error_snake_case = #ident_operation_error_named_upper_camel_case::#syn_variant_initialization_token_stream;
            #eprintln_error_token_stream
            #wraped_into_axum_response_token_stream
        }
    };
    let new_syn_variant_wrapper = |
        variant_name: &dyn std::fmt::Display,
        status_code: Option<macros_helpers::status_code::StatusCode>,
        current_fields: Vec<(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute, &dyn std::fmt::Display, syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>)>
    | -> SynVariantWrapper {
        SynVariantWrapper {
            variant: syn::Variant {
                attrs: {
                    let mut attributes = Vec::new();
                    if let Some(value) = status_code.as_ref() {
                        let mut segments = syn::punctuated::Punctuated::new();
                        segments.push(syn::PathSegment {
                            ident: proc_macro2::Ident::new(
                                &naming::AsRefStrToSnakeCaseStringified::case(value),
                                proc_macro2::Span::call_site(),
                            ),
                            arguments: syn::PathArguments::None,
                        });
                        attributes.push(syn::Attribute {
                            pound_token: syn::token::Pound {
                                spans: [proc_macro2::Span::call_site()],
                            },
                            style: syn::AttrStyle::Outer,
                            bracket_token: syn::token::Bracket::default(),
                            meta: syn::Meta::Path(syn::Path {
                                leading_colon: None,
                                segments,
                            }),
                        });
                    }
                    attributes
                },
                ident: syn::Ident::new(&variant_name.to_string(), proc_macro2::Span::call_site()),
                fields: syn::Fields::Named(syn::FieldsNamed {
                    brace_token: syn::token::Brace::default(),
                    named: {
                        let mut handle = current_fields.into_iter().fold(syn::punctuated::Punctuated::new(), |mut acc, element| {
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
    let generate_select_query_part_parameters_payload_select_token_stream = |operation: &Operation| {
        let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
        quote::quote! {
            match Self::#generate_select_query_part_snake_case(&#parameters_snake_case.#payload_snake_case.#select_snake_case) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_0_token_stream) => {
                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
            }
        }
    };
    let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
    let generate_value_declaration_token_stream = |content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {#postgresql_crud_snake_case::#value_upper_camel_case<#content_token_stream>}
    };
    let generate_import_path_value_initialization_token_stream = |content_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::generate_value_initialization_token_stream(&import_path, &content_token_stream);
    let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream = |
        current_ident: &dyn quote::ToTokens,
        content_token_stream: &dyn quote::ToTokens
    | postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
        &current_ident,
        &proc_macro2::TokenStream::new(),
        &content_token_stream
    );
    let ident_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&ident);
    let ident_create_token_stream = {
        let ident_create_token_stream = {
            let content_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let element_syn_field_ty_as_postgresql_type_create_token_stream = generate_as_postgresql_type_create_token_stream(&element.syn_field.ty);
                quote::quote! {
                    pub #field_ident: #element_syn_field_ty_as_postgresql_type_create_token_stream
                }
            });
            quote::quote! {
                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
                pub struct #ident_create_upper_camel_case {
                    #content_token_stream
                }
            }
        };
        let impl_ident_create_token_stream = {
            let primary_key_field_type_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = {
                let primary_key_field_type_as_postgresql_type_create_token_stream = generate_as_postgresql_type_create_token_stream(&primary_key_field_type);
                quote::quote! {
                    <
                        #primary_key_field_type_as_postgresql_type_create_token_stream as #postgresql_crud_snake_case::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case
                    >::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case()
                }
            };
            let fn_create_query_part_token_stream = {
                let generate_match_as_postgresql_crud_postgresql_type_postgresql_type_create_query_part_token_stream = |field_type: &syn::Type, content_token_stream: &dyn quote::ToTokens| {
                    let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&field_type);
                    let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(&quote::quote! {#acc_snake_case, "{value},"}, &return_err_query_part_error_named_write_into_buffer_token_stream);
                    quote::quote! {
                        match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #create_query_part_snake_case(
                            &#content_token_stream,
                            #increment_snake_case
                        ) {
                            Ok(#value_snake_case) => {
                                #if_write_is_err_token_stream
                            }
                            Err(#error_0_token_stream) => {
                                return Err(#error_0_token_stream);
                            }
                        }
                    }
                };
                let primary_key_content_token_stream = generate_match_as_postgresql_crud_postgresql_type_postgresql_type_create_query_part_token_stream(primary_key_field_type, &primary_key_field_type_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream);
                let column_increments_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    generate_match_as_postgresql_crud_postgresql_type_postgresql_type_create_query_part_token_stream(&element.syn_field.ty, &{
                        let element_field_ident = &element.field_ident;
                        quote::quote! {self.#element_field_ident}
                    })
                });
                quote::quote! {
                    fn #create_query_part_snake_case(&self, #increment_snake_case: &mut u64) -> Result<#string_token_stream, postgresql_crud::#query_part_error_named_upper_camel_case> {
                        let mut #acc_snake_case = String::default();
                        #primary_key_content_token_stream
                        #column_increments_token_stream
                        let _: Option<char> = #acc_snake_case.pop();
                        Ok(#acc_snake_case)
                    }
                }
            };
            let fn_create_query_bind_token_stream = {
                let generate_query_as_postgresql_crud_postgresql_type_postgresql_type_create_query_bind_token_stream = |field_type: &syn::Type, content_token_stream: &dyn quote::ToTokens| {
                    let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&field_type);
                    quote::quote! {
                        match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #create_query_bind_snake_case(
                            #content_token_stream,
                            #query_snake_case
                        ) {
                            Ok(#value_snake_case) => {
                                #query_snake_case = #value_snake_case;
                            },
                            Err(#error_0_token_stream) => {
                                return Err(#error_0_token_stream);
                            }
                        }
                    }
                };
                let primary_key_content_token_stream = generate_query_as_postgresql_crud_postgresql_type_postgresql_type_create_query_bind_token_stream(primary_key_field_type, &primary_key_field_type_as_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream);
                let binded_query_modifications_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    generate_query_as_postgresql_crud_postgresql_type_postgresql_type_create_query_bind_token_stream(&element.syn_field.ty, &{
                        let field_ident = &element.field_ident;
                        quote::quote! {self.#field_ident}
                    })
                });
                quote::quote! {
                    fn #create_query_bind_snake_case(self, mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
                        sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                        String
                    > {
                        #primary_key_content_token_stream
                        #binded_query_modifications_token_stream
                        Ok(#query_snake_case)
                    }
                }
            };
            quote::quote! {
                impl #ident_create_upper_camel_case {
                    #fn_create_query_part_token_stream
                    #fn_create_query_bind_token_stream
                }
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_create_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(&ident_create_upper_camel_case, &{
            let fields_initialiation_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                quote::quote! {#field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}
            });
            quote::quote! {
                Self{#fields_initialiation_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream}
            }
        });
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
            let element_syn_field_ty_as_postgresql_type_where_token_stream = generate_as_postgresql_type_where_token_stream(&element.syn_field.ty);
            let std_option_option_postgresql_type_where_syn_field_ty_as_postgresql_type_where_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&quote::quote! {postgresql_crud::PostgresqlTypeWhere<#element_syn_field_ty_as_postgresql_type_where_token_stream>});
            quote::quote! {
                #field_ident: #std_option_option_postgresql_type_where_syn_field_ty_as_postgresql_type_where_token_stream
            }
        });
        let ident_where_many_token_stream = quote::quote! {
            #[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
            pub struct #ident_where_many_upper_camel_case {
                #fields_declaration_token_stream
            }
        };
        let ident_where_many_try_new_error_named_token_stream = {
            quote::quote! {
                #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #ident_where_many_try_new_error_named_upper_camel_case {
                    #no_fields_provided_upper_camel_case {
                        #[eo_to_std_string_string]
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
            }
        };
        let impl_pub_try_new_for_ident_where_many_token_stream = macros_helpers::generate_impl_pub_try_new_for_ident_token_stream(&ident_where_many_upper_camel_case, &fields_declaration_token_stream, &ident_where_many_try_new_error_named_upper_camel_case, &{
            let generate_fields_token_stream = |should_add_borrow: ShouldAddBorrow| {
                generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                    let field_ident = &element.field_ident;
                    quote::quote! {#should_add_borrow #field_ident}
                })
            };
            let fields_token_stream = generate_fields_token_stream(ShouldAddBorrow::True);
            let fields_inialization_token_stream = generate_fields_token_stream(ShouldAddBorrow::False);
            quote::quote! {
                if let (#fields_named_with_comma_none_token_stream) = (#fields_token_stream) {
                    return Err(#ident_where_many_try_new_error_named_upper_camel_case::#no_fields_provided_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
                Ok(Self {#fields_inialization_token_stream})
            }
        });
        let impl_serde_deserialize_for_ident_where_many_token_stream = postgresql_crud_macros_common::generate_impl_serde_deserialize_for_struct_token_stream(&ident_where_many_upper_camel_case, &fields.iter().map(|element| (&element.field_ident, &element.syn_field.ty)).collect::<Vec<(&syn::Ident, &syn::Type)>>(), fields_len, &|_: &syn::Ident, syn_type: &syn::Type| {
            let syn_type_as_postgresql_type_where_token_stream = generate_as_postgresql_type_where_token_stream(&syn_type);
            postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&quote::quote! {postgresql_crud::PostgresqlTypeWhere<#syn_type_as_postgresql_type_where_token_stream>})
        });
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_many_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(&ident_where_many_upper_camel_case, &{
            let fields_token_stream = generate_fields_named_without_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                quote::quote! {
                    #field_ident: Some(
                        #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                    ),
                }
            });
            quote::quote! {Self{#fields_token_stream}}
        });
        quote::quote! {
            #ident_where_many_token_stream
            #ident_where_many_try_new_error_named_token_stream
            #impl_pub_try_new_for_ident_where_many_token_stream
            #impl_serde_deserialize_for_ident_where_many_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_where_many_token_stream
        }
    };

    let std_option_option_ident_where_many_upper_camel_case = naming::parameter::StdOptionOptionSelfWhereManyUpperCamelCase::from_tokens(&ident);
    let std_option_option_ident_where_many_token_stream = {
        let std_option_option_ident_where_many_token_stream = {
            let std_option_option_ident_read_only_ids_standart_not_null_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&ident_where_many_upper_camel_case);
            quote::quote! {
                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
                pub struct #std_option_option_ident_where_many_upper_camel_case(pub #std_option_option_ident_read_only_ids_standart_not_null_token_stream);
            }
        };
        let impl_postgresql_type_where_filter_for_std_option_option_ident_where_many_token_stream = postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_token_stream(
            &quote::quote! {<'lifetime>},
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
                    } else {
                        quote::quote! {is_first_push_to_additional_parameters_already_happend = true;}
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
                quote::quote! {
                    Ok(match &self.0 {
                        Some(value) => {
                            let mut #additional_parameters_snake_case = #string_token_stream::from("where");
                            let mut is_first_push_to_additional_parameters_already_happend = false;
                            #(#additional_parameters_modification_token_stream)*
                            #additional_parameters_snake_case
                        },
                        None => #string_token_stream::default()
                    })
                }
            },
            &postgresql_crud_macros_common::IsQueryBindMutable::True,
            &{
                let binded_query_modifications_token_stream = generate_fields_named_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    quote::quote! {
                        if let Some(#value_snake_case) = #value_snake_case.#field_ident {
                            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(#value_snake_case, #query_snake_case) {
                                Ok(#value_snake_case) => {
                                    #query_snake_case = #value_snake_case;
                                },
                                Err(#error_0_token_stream) => {
                                    return Err(#error_0_token_stream);
                                }
                            }
                        }
                    }
                });
                quote::quote! {
                    if let Some(#value_snake_case) = self.0 {
                        #binded_query_modifications_token_stream
                    }
                    Ok(#query_snake_case)
                }
            },
            &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
        );
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_std_option_option_ident_where_many_token_stream =
            generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(&std_option_option_ident_where_many_upper_camel_case, &quote::quote! {Self(Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream))});
        quote::quote! {
            #std_option_option_ident_where_many_token_stream
            #impl_postgresql_type_where_filter_for_std_option_option_ident_where_many_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_std_option_option_ident_where_many_token_stream
        }
    };
    let pub_where_many_std_option_option_ident_where_many_token_stream = quote::quote! {pub #where_many_snake_case: #std_option_option_ident_where_many_upper_camel_case};
    let where_many_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = quote::quote! {
        #where_many_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
    };
    let generate_read_or_delete_many_additional_paramaters_initialization_token_stream = |read_many_or_delete_many: &ReadManyOrDeleteMany| {
        let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&Operation::from(read_many_or_delete_many), &query_part_syn_variant_wrapper, file!(), line!(), column!());
        quote::quote! {
            match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                &#parameters_snake_case.#payload_snake_case.#where_many_snake_case,
                &mut #increment_snake_case,
                &"",//useless
                false//useless
            ) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_0_token_stream) => {
                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
            }
        }
    };
    let macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize = macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize;
    let string_syn_punctuated_punctuated = macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["String"]);
    let try_bind_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::TryBindUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![(macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize, &naming::TryBindSnakeCase, string_syn_punctuated_punctuated.clone())],
    );
    let generate_query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_token_stream = |operation: &Operation| {
        let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &try_bind_syn_variant_wrapper, file!(), line!(), column!());
        quote::quote! {
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(#parameters_snake_case.#payload_snake_case.#where_many_snake_case, #query_snake_case) {
                Ok(#value_snake_case) => {
                    #query_snake_case = #value_snake_case;
                },
                Err(#error_0_token_stream) => {
                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                },
            }
        }
    };
    let try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_snake_case = naming::parameter::TryFromSqlxPostgresPgRowWithNotEmptyUniqueEnumVecSelfSelectSnakeCase::from_display(&ident);
    let sqlx_error_syn_punctuated_punctuated = macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["sqlx", "Error"]);
    let macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string = macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString;
    let postgresql_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::PostgresqlUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![(macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &naming::PostgresqlSnakeCase, sqlx_error_syn_punctuated_punctuated.clone())],
    );
    let generate_match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream = |read_many_or_read_one: &ReadManyOrReadOne| {
        let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&Operation::from(read_many_or_read_one), &postgresql_syn_variant_wrapper, file!(), line!(), column!());
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
            let variants = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| {
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
                    // Copy,
                )]
                pub enum #ident_select_upper_camel_case {
                    #variants
                }
            }
        };
        let impl_std_fmt_display_for_ident_select_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(&proc_macro2::TokenStream::new(), &ident_select_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {write!(f, "{}", serde_json::to_string(&self).unwrap_or_else(|element|format!("cannot serialize into json: {element:?}")))});
        let impl_error_occurence_lib_to_std_string_string_for_ident_select_token_stream = macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_select_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self}")});
        let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_upper_camel_case, &{
            let elements_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                quote::quote! {
                    Self::#field_ident_upper_camel_case_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                }
            });
            quote::quote! {vec![#elements_token_stream]}
        });
        quote::quote! {
            #ident_select_token_stream
            #impl_std_fmt_display_for_ident_select_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_select_token_stream
            #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream
        }
    };
    let select_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = quote::quote! {
        #select_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
    };
    let ident_read_token_stream = {
        let ident_read_token_stream = {
            let field_option_primary_key_token_stream = {
                let std_option_option_value_primary_key_field_type_as_postgresql_type_read_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_value_declaration_token_stream(&generate_as_postgresql_type_read_token_stream(&primary_key_field_type)));
                quote::quote! {
                    #field_attribute_serde_skip_serializing_if_option_is_none_token_stream
                    pub #primary_key_field_ident: #std_option_option_value_primary_key_field_type_as_postgresql_type_read_token_stream
                }
            };
            let fields_options_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                let field_vis = &element.syn_field.vis;
                let field_ident = &element.field_ident;
                let std_option_option_value_field_type_as_postgresql_type_read_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_value_declaration_token_stream(&generate_as_postgresql_type_read_token_stream(&element.syn_field.ty)));
                quote::quote! {
                    #field_attribute_serde_skip_serializing_if_option_is_none_token_stream
                    #field_vis #field_ident: #std_option_option_value_field_type_as_postgresql_type_read_token_stream
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
                    let std_option_option_value_primary_key_field_type_as_primary_key_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_value_declaration_token_stream(&primary_key_field_type_as_postgresql_type_read_upper_camel_case));
                    quote::quote! {
                        let mut #primary_key_field_ident: #std_option_option_value_primary_key_field_type_as_primary_key_token_stream = None;
                    }
                };
                let declaration_without_primary_key_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let std_option_option_value_field_type_as_postgresql_type_read_token_stream = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_value_declaration_token_stream(&generate_as_postgresql_type_read_token_stream(&element.syn_field.ty)));
                    quote::quote! {
                        let mut #field_ident: #std_option_option_value_field_type_as_postgresql_type_read_token_stream = None;
                    }
                });
                let value_initialization_token_stream = quote::quote! {#import_path::#value_upper_camel_case {#value_snake_case}};
                let assignment_variant_primary_key_token_stream = {
                    let primary_key_field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&primary_key_field_ident);
                    quote::quote! {
                        #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(_) => match sqlx::Row::try_get::<
                            #primary_key_field_type_as_postgresql_type_read_upper_camel_case,
                            #ref_std_primitive_str
                        >(
                            &#value_snake_case,
                            #primary_key_field_ident_string_double_quotes_token_stream
                        ) {
                            Ok(#value_snake_case) => {
                                #primary_key_field_ident = Some(#value_initialization_token_stream);
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
                                    #field_ident = Some(#value_initialization_token_stream);
                                },
                                Err(#error_0_token_stream) => {
                                    return Err(#error_0_token_stream);
                                }
                            }
                        }
                    })
                    .collect::<Vec<proc_macro2::TokenStream>>();
                let fields_initiation_token_stream = &fields.iter().map(|element| element.syn_field.ident.as_ref().unwrap_or_else(|| panic!("{}", naming::FIELD_IDENT_IS_NONE))).collect::<Vec<&syn::Ident>>();
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
    let ident_read_only_ids_token_stream = {
        enum WrapIntoOption {
            True,
            False,
        }
        let ident_read_only_ids_token_stream = {
            let generate_field_token_stream = |field_ident: &dyn quote::ToTokens, field_type: &dyn quote::ToTokens, wrap_into_option: &WrapIntoOption| {
                let field_type_token_stream = match &wrap_into_option {
                    WrapIntoOption::True => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&generate_as_postgresql_type_read_only_ids_token_stream(&field_type)),
                    WrapIntoOption::False => generate_as_postgresql_type_read_only_ids_token_stream(&field_type),
                };
                quote::quote! {
                    pub #field_ident: #field_type_token_stream
                }
            };
            let primary_key_token_stream = generate_field_token_stream(&primary_key_field_ident, &primary_key_field_type, &WrapIntoOption::False);
            let content_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| generate_field_token_stream(&element.field_ident, &element.syn_field.ty, &WrapIntoOption::True));
            quote::quote! {
                #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
                pub struct #ident_read_only_ids_upper_camel_case {
                    #primary_key_token_stream,
                    #content_token_stream
                }
            }
        };
        let impl_sqlx_row_for_ident_read_only_ids_token_stream = {
            let undescore_underscore_row = quote::quote! {__row};
            let where_field_types_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_type = &element.syn_field.ty;
                let element_syn_field_ty_as_postgresql_type_read_only_ids_token_stream = generate_as_postgresql_type_read_only_ids_token_stream(&field_type);
                quote::quote! {
                    #element_syn_field_ty_as_postgresql_type_read_only_ids_token_stream: ::sqlx::decode::Decode<'lifetime, R::Database>,
                    #element_syn_field_ty_as_postgresql_type_read_only_ids_token_stream: ::sqlx::types::Type<R::Database>
                }
            });
            let primary_key_token_stream = {
                let element_syn_field_ty_as_postgresql_type_read_only_ids_token_stream = generate_as_postgresql_type_read_only_ids_token_stream(&primary_key_field_type);
                let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&primary_key_field_ident);
                quote::quote! {
                    let #primary_key_field_ident = match sqlx::Row::try_get::<#element_syn_field_ty_as_postgresql_type_read_only_ids_token_stream, &str>(
                        &#undescore_underscore_row,
                        #field_ident_double_quotes_token_stream
                    ) {
                        Ok(#value_snake_case) => #value_snake_case,
                        Err(#error_0_token_stream) => {
                            return Err(#error_0_token_stream);
                        }
                    };
                }
            };
            let fields_initialization_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_type = &element.syn_field.ty;
                let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&quote::quote! {#field_ident});
                let element_syn_field_ty_as_postgresql_type_read_only_ids_token_stream = generate_as_postgresql_type_read_only_ids_token_stream(&field_type);
                quote::quote! {
                    let #field_ident = sqlx::Row::try_get::<
                        #element_syn_field_ty_as_postgresql_type_read_only_ids_token_stream,
                        &str
                    >(&#undescore_underscore_row, #field_ident_double_quotes_token_stream).ok();
                }
            });
            let self_fields_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                quote::quote! {#field_ident}
            });
            quote::quote! {
                impl<'lifetime, R: ::sqlx::Row<Database = sqlx::Postgres>> ::sqlx::FromRow<'lifetime, R> for #ident_read_only_ids_upper_camel_case
                where
                    &'lifetime ::std::primitive::str: ::sqlx::ColumnIndex<R>,
                    #where_field_types_token_stream
                {
                    fn from_row(#undescore_underscore_row: &'lifetime R) -> ::sqlx::Result<Self> {
                        #primary_key_token_stream
                        #fields_initialization_token_stream
                        Ok(Self { #self_fields_token_stream })
                    }
                }
            }
        };
        quote::quote! {
            #ident_read_only_ids_token_stream
            #impl_sqlx_row_for_ident_read_only_ids_token_stream
        }
    };
    // println!("{ident_read_only_ids_token_stream}");
    let generate_ident_try_operation_error_named_upper_camel_case = |operation: &Operation| format!("{ident}Try{operation}ErrorNamed").parse::<proc_macro2::TokenStream>().expect("error 6a5468b2-c8d6-4c5e-88a6-adce2bfe7467");
    let ident_try_read_many_error_named_upper_camel_case = generate_ident_try_operation_error_named_upper_camel_case(&Operation::ReadMany);
    let generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case = |operation: &Operation| format!("{ident}{operation}ErrorNamedWithSerializeDeserialize").parse::<proc_macro2::TokenStream>().expect("error f9e053d1-c5ce-4a8e-ac79-6cc30ba19bb9");
    let postgresql_crud_order_by_token_stream = quote::quote! {#postgresql_crud_snake_case::#order_by_upper_camel_case};
    let postgresql_crud_order_token_stream = quote::quote! {#postgresql_crud_snake_case::Order};
    //todo
    // let ident_column_read_permission_token_stream = {
    //     let ident_column_read_permission_upper_camel_case = naming::parameter::SelfColumnReadPermissionUpperCamelCase::from_display(&ident);
    //     let fields_permission_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| {
    //         let field_ident = &element.field_ident;
    //         //todo permissions for json
    //         quote::quote! {
    //             #field_ident: bool
    //         }
    //     });
    //     quote::quote! {
    //         #derive_debug_clone_copy
    //         pub struct #ident_column_read_permission_upper_camel_case {
    //             #fields_permission_token_stream
    //         }
    //     }
    // };
    let ident_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&ident);
    let ident_update_many_parameters_upper_camel_case = naming::parameter::SelfUpdateManyParametersUpperCamelCase::from_tokens(&ident);
    let ident_update_many_payload_upper_camel_case = naming::parameter::SelfUpdateManyPayloadUpperCamelCase::from_tokens(&ident);
    let ident_update_try_new_error_named_upper_camel_case = naming::parameter::SelfUpdateTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
    let ident_update_for_query_upper_camel_case = naming::parameter::SelfUpdateForQueryUpperCamelCase::from_tokens(&ident);
    let update_query_part_primary_key_snake_case = naming::UpdateQueryPartPrimaryKeySnakeCase;
    let ident_update_token_stream = {
        let generate_option_value_field_type_as_postgresql_type_update_token_stream = |syn_type: &syn::Type| {
            let path_value_token_stream = {
                let value = format!("{}::{}", naming::PostgresqlCrudSnakeCase, naming::ValueUpperCamelCase);
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let syn_type_as_postgresql_type_update_token_stream = generate_as_postgresql_type_update_token_stream(&syn_type);
            postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&quote::quote! {#path_value_token_stream<#syn_type_as_postgresql_type_update_token_stream>})
        };
        let fields_declaration_token_stream = {
            let fields_named_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                let field_ident = &element.field_ident;
                let option_value_field_type_as_postgresql_type_update_token_stream = generate_option_value_field_type_as_postgresql_type_update_token_stream(&element.syn_field.ty);
                quote::quote! {
                    #field_ident: #option_value_field_type_as_postgresql_type_update_token_stream
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
            quote::quote! {
                #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #ident_update_try_new_error_named_upper_camel_case {
                    #no_fields_provided_upper_camel_case {
                        #[eo_to_std_string_string]
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
            }
        };
        let impl_pub_try_new_for_ident_update_token_stream = macros_helpers::generate_impl_pub_try_new_for_ident_token_stream(&ident_update_upper_camel_case, &fields_declaration_token_stream, &ident_update_try_new_error_named_upper_camel_case, &{
            let (left_token_stream, right_token_stream) = {
                let maybe_wrap_into_braces_handle_token_stream = |content_token_stream: &dyn quote::ToTokens| postgresql_crud_macros_common::maybe_wrap_into_braces_token_stream(content_token_stream, fields_len_without_primary_key > 1);
                (
                    maybe_wrap_into_braces_handle_token_stream(&fields_named_without_primary_key_with_comma_none_token_stream),
                    maybe_wrap_into_braces_handle_token_stream(&generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                        let field_ident = &element.field_ident;
                        quote::quote! {&#field_ident}
                    })),
                )
            };
            let fields_inialization_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                let field_ident = &element.field_ident;
                quote::quote! {#field_ident}
            });
            quote::quote! {
                if let #left_token_stream = #right_token_stream {
                    return Err(#ident_update_try_new_error_named_upper_camel_case::#no_fields_provided_upper_camel_case {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
                Ok(Self {#fields_inialization_token_stream})
            }
        });
        let impl_serde_deserialize_for_ident_update_token_stream = postgresql_crud_macros_common::generate_impl_serde_deserialize_for_struct_token_stream(&ident_update_upper_camel_case, &fields.iter().map(|element| (&element.field_ident, &element.syn_field.ty)).collect::<Vec<(&syn::Ident, &syn::Type)>>(), fields_len, &|syn_ident: &syn::Ident, syn_type: &syn::Type| {
            if syn_ident == primary_key_field_ident {
                quote::quote! {#primary_key_field_type_update_token_stream}
            } else {
                generate_option_value_field_type_as_postgresql_type_update_token_stream(syn_type)
            }
        });
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(&ident_update_upper_camel_case, &{
            let primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                quote::quote! {
                    #primary_key_field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                }
            };
            let fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
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
        });
        quote::quote! {
            #ident_update_token_stream
            #ident_update_try_new_error_named_token_stream
            #impl_pub_try_new_for_ident_update_token_stream
            #impl_serde_deserialize_for_ident_update_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_update_token_stream
        }
    };
    let ident_update_for_query_token_stream = {
        let generate_option_value_field_type_as_postgresql_type_update_for_query_token_stream = |syn_type: &syn::Type| {
            let path_value_token_stream = {
                let value = format!("{}::{}", naming::PostgresqlCrudSnakeCase, naming::ValueUpperCamelCase);
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let syn_type_as_postgresql_type_update_for_query_token_stream = generate_as_postgresql_type_update_for_query_token_stream(&syn_type);
            postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_token_stream(&quote::quote! {#path_value_token_stream<#syn_type_as_postgresql_type_update_for_query_token_stream>})
        };
        let fields_declaration_token_stream = {
            let fields_named_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                let field_ident = &element.field_ident;
                let option_value_field_type_as_postgresql_type_update_for_query_token_stream = generate_option_value_field_type_as_postgresql_type_update_for_query_token_stream(&element.syn_field.ty);
                quote::quote! {
                    #field_ident: #option_value_field_type_as_postgresql_type_update_for_query_token_stream
                }
            });
            quote::quote! {
                #primary_key_field_ident: #primary_key_field_type_update_for_query_token_stream,
                #fields_named_without_primary_key_token_stream
            }
        };
        let ident_update_for_query_token_stream = quote::quote! {
            #[derive(Debug, serde::Serialize, utoipa::ToSchema)]
            pub struct #ident_update_for_query_upper_camel_case {
                #fields_declaration_token_stream
            }
        };
        let impl_ident_update_for_query_token_stream = {
            let update_query_part_primary_key_token_stream = {
                quote::quote! {
                    fn #update_query_part_primary_key_snake_case(&self, #increment_snake_case: &mut u64) -> Result<#string_token_stream, #postgresql_crud_snake_case::#query_part_error_named_upper_camel_case> {
                        match #primary_key_field_type_as_postgresql_type_token_stream #update_query_part_snake_case(
                            &self.#primary_key_field_ident,
                            "",
                            #ident::#primary_key_snake_case(),
                            "",
                            #increment_snake_case,
                        ) {
                            Ok(value_snake_case) => Ok(value_snake_case),
                            Err(#error_0_token_stream) => Err(#error_0_token_stream)
                        }
                    }
                }
            };
            let update_query_part_fields_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                let update_query_part_field_ident_snake_case = naming::parameter::UpdateQueryPartSelfSnakeCase::from_tokens(&field_ident);
                let field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
                quote::quote! {
                    fn #update_query_part_field_ident_snake_case(
                        #value_snake_case: &postgresql_crud::Value<#field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_for_query_upper_camel_case>,
                        #increment_snake_case: &mut u64
                    ) -> Result<#string_token_stream, #postgresql_crud_snake_case::#query_part_error_named_upper_camel_case> {
                        match #field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_part_snake_case(
                            &#value_snake_case.#value_snake_case,
                            #field_ident_double_quotes_token_stream,
                            #field_ident_double_quotes_token_stream,
                            "",
                            #increment_snake_case
                        ) {
                            Ok(#value_snake_case) => Ok(#value_snake_case),
                            Err(#error_0_token_stream) => Err(#error_0_token_stream),
                        }
                    }
                }
            });
            let select_only_updated_ids_query_part_token_stream = {
                let primary_key_content_token_stream = {
                    let primary_key_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&primary_key_field_ident);
                    quote::quote! {
                        acc.push_str(&match <#primary_key_field_type as postgresql_crud::PostgresqlType>::#select_only_updated_ids_query_part_snake_case(
                            &self.#primary_key_field_ident,
                            #primary_key_field_ident_double_quotes_token_stream,
                            increment,
                        ){
                            Ok(value) => value,
                            Err(error) => {
                                return Err(error);
                            }
                        });
                    }
                };
                let content_token_stream = fields_without_primary_key.iter().map(|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                    let field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
                    quote::quote! {
                        if let Some(#value_snake_case) = &self.#field_ident {
                            #acc_snake_case.push_str(&match #field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #select_only_updated_ids_query_part_snake_case(
                                &#value_snake_case.#value_snake_case,
                                #field_ident_double_quotes_token_stream,
                                increment,
                            ){
                                Ok(value1) => value1,
                                Err(#error_snake_case) => {
                                    return Err(#error_snake_case);
                                }
                            });
                        }
                    }
                });
                quote::quote! {
                    fn #select_only_updated_ids_query_part_snake_case(&self, #increment_snake_case: &mut u64) -> Result<#string_token_stream, postgresql_crud::QueryPartErrorNamed> {
                        let mut #acc_snake_case = String::new();
                        #primary_key_content_token_stream
                        #(#content_token_stream)*
                        let _: Option<char> = #acc_snake_case.pop();
                        Ok(#acc_snake_case)
                    }
                }
            };
            let update_handle_token_stream = generate_from_handle_token_stream(&ident_update_upper_camel_case, &{
                let primary_key_field_type_as_postgresql_type_update_for_query_token_stream = generate_as_postgresql_type_update_for_query_token_stream(&primary_key_field_type);
                let fields_named_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
                    let field_ident = &element.field_ident;
                    let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&{
                        let field_type_as_postgresql_type_update_for_query_token_stream = generate_as_postgresql_type_update_for_query_token_stream(&element.syn_field.ty);
                        quote::quote! {
                             #field_type_as_postgresql_type_update_for_query_token_stream::from(value1.#value_snake_case)
                        }
                    });
                    quote::quote! {
                        #field_ident: match #value_snake_case.#field_ident {
                            Some(value1) => Some(#value_initialization_token_stream),
                            None => None
                        }
                    }
                });
                quote::quote! {
                    Self {
                        #primary_key_field_ident: #primary_key_field_type_as_postgresql_type_update_for_query_token_stream::from(#value_snake_case.#primary_key_field_ident),
                        #fields_named_without_primary_key_token_stream
                    }
                }
            });
            quote::quote! {
                impl #ident_update_for_query_upper_camel_case {
                    #update_query_part_primary_key_token_stream
                    #update_query_part_fields_token_stream
                    #select_only_updated_ids_query_part_token_stream
                    #update_handle_token_stream
                }
            }
        };
        quote::quote! {
            #ident_update_for_query_token_stream
            #impl_ident_update_for_query_token_stream
        }
    };
    let generate_match_update_query_part_primary_key_token_stream = |operation: &Operation, content_token_stream: &dyn quote::ToTokens| {
        let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
        quote::quote! {
            match #content_token_stream.#update_query_part_primary_key_snake_case(&mut #increment_snake_case) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_0_token_stream) => {
                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
            }
        }
    };
    let row_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::RowAndRollbackUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &naming::RowSnakeCase, sqlx_error_syn_punctuated_punctuated.clone()),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &rollback_snake_case, sqlx_error_syn_punctuated_punctuated),
        ],
    );
    let sqlx_query_sqlx_postgres_token_stream = quote::quote! {sqlx::query::<sqlx::Postgres>};
    let (postgresql_crud_postgresql_type_where_filter_query_part_token_stream, postgresql_crud_postgresql_type_where_filter_query_bind_token_stream) = {
        let postgresql_crud_postgresql_type_where_filter_token_stream = quote::quote! {#postgresql_crud_snake_case::PostgresqlTypeWhereFilter::};
        (quote::quote! {#postgresql_crud_postgresql_type_where_filter_token_stream #query_part_snake_case}, quote::quote! {#postgresql_crud_postgresql_type_where_filter_token_stream #query_bind_snake_case})
    };

    let std_vec_vec_struct_options_ident_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&ident_read_upper_camel_case);
    let not_unique_field_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NotUniqueFieldUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize,
            &naming::NotUniqueFieldSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&ident_select_upper_camel_case.to_string()]),
        )],
    );
    let serde_json_to_string_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::SerdeJsonToStringUpperCamelCase,
        None,
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
            &naming::SerdeJsonToStringSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
        )],
    );
    let failed_to_get_response_text_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::FailedToGetResponseTextUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &status_code_snake_case, macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["http", "StatusCode"])),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
                &naming::HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "header", "HeaderMap"]),
            ),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &naming::ReqwestSnakeCase, macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "Error"])),
        ],
    );
    let deserialize_response_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::DeserializeResponseUpperCamelCase,
        None,
        vec![
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &status_code_snake_case, macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["http", "StatusCode"])),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
                &naming::HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "header", "HeaderMap"]),
            ),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize, &naming::ResponseTextSnakeCase, string_syn_punctuated_punctuated),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &naming::SerdeSnakeCase, macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"])),
        ],
    );
    let reqwest_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::ReqwestUpperCamelCase,
        None,
        vec![(macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &naming::ReqwestSnakeCase, macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "Error"]))],
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
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
            &naming::SerdeJsonSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
        )],
    );
    let header_content_type_application_json_not_found_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::HeaderContentTypeApplicationJsonNotFoundUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        Vec::<(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute, &'static dyn std::fmt::Display, syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>)>::default(),
    );
    let common_http_request_syn_variants = {
        vec![
            serde_json_to_string_syn_variant_wrapper.get_syn_variant().clone(),
            failed_to_get_response_text_syn_variant_wrapper.get_syn_variant().clone(),
            deserialize_response_syn_variant_wrapper.get_syn_variant().clone(),
            reqwest_syn_variant_wrapper.get_syn_variant().clone(),
        ]
    };
    let generate_additional_error_variants = |current_syn_derive_input: &syn::DeriveInput, generate_postgresql_table_attribute: GeneratePostgresqlTableAttribute| -> Vec<syn::Variant> {
        let generate_postgresql_table_attribute_stringified = generate_postgresql_table_attribute.to_string();
        let common_additional_error_variants_attribute_token_stream = macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(&current_syn_derive_input.attrs, &generate_postgresql_table_attribute.generate_path_to_attribute());
        let value: syn::DeriveInput = syn::parse((*common_additional_error_variants_attribute_token_stream).clone().into()).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
        let value_ident_stringified = value.ident.to_string();
        assert!(value_ident_stringified == generate_postgresql_table_attribute_stringified, "{value_ident_stringified} is not equal to {generate_postgresql_table_attribute_stringified}");
        let variants = if let syn::Data::Enum(data_enum) = value.data {
            data_enum.variants
        } else {
            panic!("value.data is not syn::Data::Enum");
        };
        variants.into_iter().collect()
    };
    let common_additional_error_variants = generate_additional_error_variants(&syn_derive_input, GeneratePostgresqlTableAttribute::CommonAdditionalErrorVariants);
    let common_route_syn_variants = {
        let common_additional_error_variants_vec = common_additional_error_variants.iter().collect::<Vec<&syn::Variant>>();
        let mut value = vec![check_body_size_syn_variant_wrapper.get_syn_variant(), postgresql_syn_variant_wrapper.get_syn_variant(), serde_json_syn_variant_wrapper.get_syn_variant(), header_content_type_application_json_not_found_syn_variant_wrapper.get_syn_variant()];
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
    let common_additional_logic_token_stream = macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(&syn_derive_input.attrs, &GeneratePostgresqlTableAttribute::CommonAdditionalLogic.generate_path_to_attribute());
    let generate_pub_handle_token_stream = |is_pub: bool| {
        if is_pub {
            quote::quote! {pub}
        } else {
            proc_macro2::TokenStream::new()
        }
    };
    let generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = |primary_key_type_token_stream: &dyn quote::ToTokens| {
        let is_pub = true;
        let pub_handle_token_stream = generate_pub_handle_token_stream(is_pub);
        quote::quote! {#pub_handle_token_stream #primary_key_field_ident: #primary_key_type_token_stream}
    };
    let generate_match_postgres_transaction_rollback_await_token_stream = |operation: &Operation, postgresql_file: &'static str, postgresql_line: u32, postgresql_column: u32, row_and_rollback_file: &'static str, row_and_rollback_line: u32, row_and_rollback_column: u32| {
        let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, postgresql_file, postgresql_line, postgresql_column);
        let row_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &row_and_rollback_syn_variant_wrapper, row_and_rollback_file, row_and_rollback_line, row_and_rollback_column);
        quote::quote! {{
            if let Err(#error_1_token_stream) = #executor_snake_case.#rollback_snake_case().await {
                #row_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream
            }
            #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
        }}
    };
    let generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream = |operation: &Operation, postgresql_file: &'static str, postgresql_line: u32, postgresql_column: u32, row_and_rollback_file: &'static str, row_and_rollback_line: u32, row_and_rollback_column: u32| {
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
    let generate_fetch_token_stream = |value_handle_token_stream: &dyn quote::ToTokens, try_next_error_initialization_token_stream: &dyn quote::ToTokens, should_wrap_into_value: &ShouldWrapIntoValue| {
        let content_token_stream = quote::quote! {
            let mut #rows_snake_case = #binded_query_snake_case.fetch(#executor_snake_case.as_mut());
            let mut #acc_snake_case = Vec::new();
            while let Some(#value_snake_case) = match #postgresql_crud_snake_case::TryStreamExt::try_next(&mut #rows_snake_case).await {
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
        };
        match should_wrap_into_value {
            ShouldWrapIntoValue::True => wrap_into_value_token_stream(&content_token_stream),
            ShouldWrapIntoValue::False => content_token_stream,
        }
    };
    let generate_fetch_one_token_stream = |value_handle_token_stream: &dyn quote::ToTokens, fetch_one_error_initialization_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            match #binded_query_snake_case.fetch_one(#executor_snake_case.as_mut()).await {
                Ok(#value_snake_case) => {
                    #value_handle_token_stream
                },
                Err(#error_0_token_stream) => {
                    #fetch_one_error_initialization_token_stream
                }
            }
        }
    };
    let generate_sqlx_row_try_get_primary_key_token_stream = |sqlx_row_try_get_type_token_stream: &dyn quote::ToTokens, ok_token_stream: &dyn quote::ToTokens, err_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            match #sqlx_row::try_get::<
                #sqlx_row_try_get_type_token_stream,
                #ref_std_primitive_str
            >(&#value_snake_case, Self::#primary_key_snake_case()) {
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
                        if let Ok(value) = { <macros_helpers::error_occurence::ErrorOccurenceFieldAttribute as std::str::FromStr>::from_str(&segment.ident.to_string()) } {
                            match error_occurence_attribute {
                                Some(current_value) => panic!(
                                    "duplicated attributes ({}) are not supported",
                                    macros_helpers::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&current_value)
                                ),
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
    let generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream = |operation: &Operation, desirable_type_token_stream: &dyn quote::ToTokens, type_variants_from_request_response_syn_variants: &Vec<syn::Variant>| -> proc_macro2::TokenStream {
        let ident_operation_response_variants_upper_camel_case = generate_ident_operation_response_variants_upper_camel_case(operation);
        let ident_try_operation_logic_response_variants_token_stream = {
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(macros_helpers::error_occurence::generate_serialize_deserialize_version_of_named_syn_variant);
            quote::quote! {
                #derive_debug_serde_serialize_serde_deserialize
                pub enum #ident_operation_response_variants_upper_camel_case {
                    #desirable_upper_camel_case(#desirable_type_token_stream),
                    #(#variants_token_stream),*
                }
            }
        };
        let ident_operation_error_named_upper_camel_case = generate_ident_operation_error_named_upper_camel_case(operation);
        let impl_ident_operation_response_variants_token_stream = {
            let from_handle_token_stream = generate_from_handle_token_stream(&ident_operation_error_named_upper_camel_case, &{
                let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(|element| {
                    let variant_ident = &element.ident;
                    let syn::Fields::Named(fields_named) = &element.fields else {
                        panic!("expected fields would be named");
                    };
                    let fields_mapped_into_token_stream = {
                        let fields_token_stream = fields_named.named.iter().map(|field| &field.ident);
                        quote::quote! {#(#fields_token_stream),*}
                    };
                    let ident_operation_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case(operation);
                    quote::quote! {
                        #ident_operation_error_named_with_serialize_deserialize_upper_camel_case::#variant_ident {
                            #fields_mapped_into_token_stream
                        } => Self::#variant_ident {
                            #fields_mapped_into_token_stream
                        }
                    }
                });
                quote::quote! {
                    match #value_snake_case.#into_serialize_deserialize_version_snake_case() {
                        #(#variants_token_stream),*
                    }
                }
            });
            quote::quote! {
                impl #ident_operation_response_variants_upper_camel_case {
                    #from_handle_token_stream
                }
            }
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
            #impl_ident_operation_response_variants_token_stream
            #ident_operation_error_named_token_stream
        }
    };
    let generate_ident_operation_payload_upper_camel_case = |operation: &Operation| match &operation {
        Operation::CreateOne => quote::quote! {#ident_create_upper_camel_case},
        Operation::UpdateOne => quote::quote! {#ident_update_upper_camel_case},
        Operation::CreateMany | Operation::ReadMany | Operation::ReadOne | Operation::UpdateMany | Operation::DeleteMany | Operation::DeleteOne => format!("{ident}{operation}{}", naming::PayloadUpperCamelCase).parse::<proc_macro2::TokenStream>().expect("error c042f504-5275-4388-80cc-a141c066daf8"),
    };
    let generate_ident_operation_parameters_upper_camel_case = |operation: &Operation| format!("{ident}{operation}Parameters").parse::<proc_macro2::TokenStream>().expect("error c1203fc6-3dbd-4a37-a407-f9913aa7964d");
    let generate_parameters_pattern_token_stream = |operation: &Operation, payload_token_stream: proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let parameters_token_stream = {
            let ident_operation_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(operation);
            let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(operation);
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
    let generate_parameters_payload_and_default_token_stream = |operation: &Operation, declaration_token_stream: &dyn quote::ToTokens, default_init_content_token_stream: &dyn quote::ToTokens| {
        let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(operation);
        let ident_operation_payload_token_stream = {
            quote::quote! {
                #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
                pub struct #ident_operation_payload_upper_camel_case #declaration_token_stream
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(&ident_operation_payload_upper_camel_case, &quote::quote! {Self #default_init_content_token_stream});
        quote::quote! {
            #ident_operation_payload_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
        }
    };
    let generate_type_variants_from_request_response_syn_variants = |syn_variants: &Vec<&syn::Variant>, operation: &Operation| -> Vec<syn::Variant> {
        let mut type_variants_from_request_response_syn_variants = Vec::new();
        for element in syn_variants {
            type_variants_from_request_response_syn_variants.push((*element).clone());
        }
        let operation_additional_error_variants = generate_additional_error_variants(&syn_derive_input, operation.generate_postgresql_table_attribute_additional_error_variants());
        for element in operation_additional_error_variants {
            type_variants_from_request_response_syn_variants.push(element.clone());
        }
        type_variants_from_request_response_syn_variants
    };
    let generate_ident_try_operation_error_named_token_stream = |operation: &Operation, syn_variants: &Vec<syn::Variant>| -> proc_macro2::TokenStream {
        let ident_try_operation_error_named_upper_camel_case = generate_ident_try_operation_error_named_upper_camel_case(operation);
        let variants = {
            let mut value = vec![];
            for element in syn_variants {
                value.push(element.clone());
            }
            value.push({
                let ident_operation_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case(operation);
                new_syn_variant_wrapper(
                    &ident_operation_error_named_with_serialize_deserialize_upper_camel_case,
                    None,
                    vec![(
                        macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
                        &operation.operation_error_named_with_serialize_deserialize_snake_case(),
                        macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&ident_operation_error_named_with_serialize_deserialize_upper_camel_case.to_string()]),
                    )],
                )
                .get_syn_variant()
                .clone()
            });
            value
        };
        let variants_token_stream = variants.iter().map(generate_error_occurence_variant_token_stream);
        quote::quote! {
            #derive_debug_thiserror_error_occurence
            pub enum #ident_try_operation_error_named_upper_camel_case {
                #(#variants_token_stream),*
            }
        }
    };
    let std_sync_arc_combination_of_app_state_logic_traits_token_stream = quote::quote! {std::sync::Arc<dyn #postgresql_crud_snake_case::CombinationOfAppStateLogicTraits>};
    let generate_operation_token_stream = |
        operation: &Operation,
        current_additional_logic_token_stream: &dyn quote::ToTokens,
        parameters_logic_token_stream: &dyn quote::ToTokens,
        expected_updated_primary_keys_token_stream: &dyn quote::ToTokens,
        query_string_token_stream: &dyn quote::ToTokens,
        binded_query_token_stream: &dyn quote::ToTokens,
        postgresql_logic_token_stream: &dyn quote::ToTokens
    | -> proc_macro2::TokenStream {
            let operation_handle_snake_case_token_stream = operation.self_handle_snake_case_token_stream();
            let operation_snake_case_token_stream = operation.self_snake_case_token_stream();
            let request_parts_preparation_token_stream = {
                let header_content_type_application_json_not_found_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = &generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &header_content_type_application_json_not_found_syn_variant_wrapper, file!(), line!(), column!());
                let check_body_size_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = &generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &check_body_size_syn_variant_wrapper, file!(), line!(), column!());
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
                let operation_additional_logic_token_stream = macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(&syn_derive_input.attrs, &operation.generate_postgresql_table_attribute_additional_logic().generate_path_to_attribute());
                quote::quote! {
                    #current_additional_logic_token_stream
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
                    let ident_operation_response_variants_upper_camel_case = generate_ident_operation_response_variants_upper_camel_case(operation);
                    quote::quote! {#ident_operation_response_variants_upper_camel_case::#desirable_upper_camel_case(#value_snake_case)}
                },
                &operation.desirable_status_code().to_axum_http_status_code_token_stream(),
                &ShouldAddReturn::False,
            );
            quote::quote! {
                async fn #operation_handle_snake_case_token_stream(
                    #app_state_snake_case: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_token_stream>,
                    #request_snake_case: axum::extract::Request,
                    #table_snake_case: &str,
                ) -> axum::response::Response {
                    #request_parts_preparation_token_stream
                    #additional_validators_token_stream
                    #parameters_logic_token_stream
                    #expected_updated_primary_keys_token_stream
                    let #query_string_snake_case = #query_string_token_stream;
                    // println!("{}", #query_string_snake_case);
                    let #binded_query_snake_case = {
                        #binded_query_token_stream
                    };
                    #acquire_pool_and_connection_token_stream
                    let #value_snake_case = {
                        #postgresql_logic_token_stream
                    };
                    #wraped_into_axum_response_token_stream
                }
                pub async fn #operation_snake_case_token_stream(
                    #app_state_snake_case: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_token_stream>,
                    #request_snake_case: axum::extract::Request,
                ) -> axum::response::Response {
                    Self::#operation_handle_snake_case_token_stream(#app_state_snake_case, #request_snake_case, #self_table_name_call_token_stream).await
                }
            }
        };
    let generate_parameters_logic_token_stream = |operation: &Operation, operation_payload_with_serialize_deserialize_check_token_stream: &dyn quote::ToTokens| -> proc_macro2::TokenStream {
        let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(operation);
        let serde_json_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &serde_json_syn_variant_wrapper, file!(), line!(), column!());
        let ident_operation_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(operation);
        //todo in case of large type there is a stackoverflow. for example it was a 3.5md json file generated by create_many_payload_example. 3400 fields = success. 16000 = stackoverflow
        quote::quote! {
            let #parameters_snake_case = #ident_operation_parameters_upper_camel_case {
                #payload_snake_case: match serde_json::from_slice::<#ident_operation_payload_upper_camel_case>(
                    &#body_bytes_snake_case,
                ) {
                    Ok(#value_snake_case) => {
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
    let generate_try_operation_token_stream = |operation: &Operation, type_variants_from_request_response_syn_variants: &[syn::Variant], result_ok_type_token_stream: &dyn quote::ToTokens, desirable_from_or_try_from_desirable_with_serialize_deserialize_token_stream: &dyn quote::ToTokens| {
        let try_operation_snake_case_token_stream = operation.try_self_snake_case_token_stream();
        let try_operation_handle_snake_case_token_stream = operation.try_self_handle_snake_case_token_stream();
        let ident_try_operation_error_named_upper_camel_case = generate_ident_try_operation_error_named_upper_camel_case(operation);
        let ident_operation_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(operation);
        let payload_token_stream = {
            let serde_json_to_string_syn_variant_initialization_token_stream = generate_initialization_token_stream(&serde_json_to_string_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #payload_snake_case = {
                    match serde_json::to_string(&#parameters_snake_case.#payload_snake_case) {
                        Ok(#value_snake_case) => #value_snake_case,
                        Err(#error_0_token_stream) => {
                            return Err(#ident_try_operation_error_named_upper_camel_case::#serde_json_to_string_syn_variant_initialization_token_stream);
                        }
                    }
                };
            }
        };
        let url_token_stream = {
            let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{endpoint_location}}/{{table}}/{}", operation.self_snake_case_stringified()));
            quote::quote! {let #url_snake_case = format!(#format_handle_token_stream);}
        };
        let future_token_stream = {
            let operation_http_method_snake_case_token_stream = naming::AsRefStrToSnakeCaseTokenStream::case_or_panic(&operation.http_method());
            let commit_header_addition_token_stream = quote::quote! {
                .header(
                    &"commit".to_string(),//todo remove it
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
        let ident_operation_response_variants_upper_camel_case = generate_ident_operation_response_variants_upper_camel_case(operation);
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
        let try_operation_logic_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case(operation);
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
                        return Ok(#desirable_from_or_try_from_desirable_with_serialize_deserialize_token_stream);
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
            async fn #try_operation_handle_snake_case_token_stream(
                #endpoint_location_snake_case: #ref_std_primitive_str,
                #parameters_snake_case: #ident_operation_parameters_upper_camel_case,
                #table_snake_case: &str,
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
            pub async fn #try_operation_snake_case_token_stream(
                #endpoint_location_snake_case: #ref_std_primitive_str,
                #parameters_snake_case: #ident_operation_parameters_upper_camel_case
            ) -> Result<#result_ok_type_token_stream, #ident_try_operation_error_named_upper_camel_case> {
                Self::#try_operation_handle_snake_case_token_stream(
                    #endpoint_location_snake_case,
                    #parameters_snake_case,
                    #self_table_name_call_token_stream
                ).await
            }
        }
    };
    let generate_create_update_delete_many_fetch_token_stream = |create_or_update_or_delete_many: &CreateOrUpdateOrDeleteMany| {
        let current_operation = Operation::from(create_or_update_or_delete_many);
        generate_fetch_token_stream(
            &generate_sqlx_row_try_get_primary_key_token_stream(
                &primary_key_field_type_as_postgresql_type_read_upper_camel_case,
                &quote::quote! {Some(#value_snake_case)},
                &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
            ),
            &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
            &ShouldWrapIntoValue::True,
        )
    };
    let generate_create_update_delete_one_fetch_token_stream = |create_or_update_or_delete_one: &CreateOrUpdateOrDeleteOne| {
        let current_operation = Operation::from(create_or_update_or_delete_one);
        wrap_into_value_token_stream(&generate_fetch_one_token_stream(
            &generate_sqlx_row_try_get_primary_key_token_stream(
                &quote::quote! {#primary_key_field_type_as_postgresql_type_read_upper_camel_case},
                &value_snake_case,
                &generate_match_postgres_transaction_rollback_await_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
            ),
            &generate_match_postgres_transaction_rollback_await_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
        ))
    };
    let generate_operation_payload_example_token_stream = |operation: &Operation| {
        let operation_payload_example_snake_case = operation.operation_payload_example_snake_case();
        let wraped_into_axum_response_token_stream = wrap_into_axum_response_token_stream(
            &{
                let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(operation);
                quote::quote! {<#ident_operation_payload_upper_camel_case as postgresql_crud::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case>::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case()}
            },
            &quote::quote! {axum::http::StatusCode::OK},
            &ShouldAddReturn::False,
        );
        quote::quote! {
            pub async fn #operation_payload_example_snake_case() -> axum::response::Response {
                #wraped_into_axum_response_token_stream
            }
        }
    };
    let increment_initialization_token_stream = quote::quote! {let mut #increment_snake_case: u64 = 0;};
    let column_names = {
        let mut value = fields.iter().fold(String::default(), |mut acc, element| {
            use std::fmt::Write as _;
            assert!(write!(acc, "{}", &element.field_ident).is_ok(), "error b9fe50dc-69a2-4af1-801d-69b7839a1471");
            acc.push(',');
            acc
        });
        let _: Option<char> = value.pop();
        value
    };
    let column_names_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&column_names);
    let generate_select_only_ids_query_part_token_stream = |operation: &Operation| {
        let select_only_ids_query_part_initialization_token_stream = fields.iter().map(|element: &SynFieldWrapper| {
            let field_ident = &element.field_ident;
            let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
            let field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
            let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                match #field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #select_only_ids_query_part_snake_case(#field_ident_double_quotes_token_stream) {
                    Ok(#value_snake_case) => {
                        #acc_snake_case.push_str(&#value_snake_case);
                    },
                    Err(#error_0_token_stream) => {
                        #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                }
            }
        });
        quote::quote! {
            {
                let mut #acc_snake_case = #string_token_stream::new();
                #(#select_only_ids_query_part_initialization_token_stream)*
                let _: Option<char> = #acc_snake_case.pop();
                #acc_snake_case
            }
        }
    };
    let generate_write_into_buffer_query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = |operation: &Operation| {
        let query_part_error_named_write_into_buffer_token_stream = postgresql_crud_macros_common::generate_query_part_error_named_write_into_buffer_token_stream(import_path);
        let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
        quote::quote! {
            let #error_0_token_stream = #query_part_error_named_write_into_buffer_token_stream;
            #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
        }
    };
    let generate_match_ident_read_only_ids_as_from_row_from_row_token_stream = |content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            match <#ident_read_only_ids_upper_camel_case as sqlx::FromRow<'_, sqlx::postgres::PgRow>>::from_row(&#value_snake_case) {
                Ok(#value_snake_case) => #value_snake_case,
                Err(#error_0_token_stream) => #content_token_stream
            }
        }
    };
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
                value.push(try_bind_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_parameters_payload_and_default_token_stream(
                &operation,
                &{
                    let std_vec_vec_ident_create_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&ident_create_upper_camel_case);
                    quote::quote! {(pub #std_vec_vec_ident_create_token_stream);}
                },
                &quote::quote! {(vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])},
            ),
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(&operation, &std_vec_vec_ident_read_only_ids_token_stream, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(&quote::quote! {#acc_snake_case, "({value}),"}, &generate_write_into_buffer_query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream(&operation));
                    let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    let select_only_ids_query_part_token_stream = generate_select_only_ids_query_part_token_stream(&operation);
                    quote::quote! {#postgresql_crud_snake_case::generate_create_many_query_string(
                        #table_snake_case,
                        #column_names_double_quotes_token_stream,
                        &{
                            #increment_initialization_token_stream
                            let mut #acc_snake_case = #string_token_stream::default();
                            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                match #element_snake_case.#create_query_part_snake_case(&mut #increment_snake_case) {
                                    Ok(#value_snake_case) => {
                                        #if_write_is_err_token_stream
                                    },
                                    Err(#error_0_token_stream) => {
                                        #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                            let _: Option<char> = #acc_snake_case.pop();
                            #acc_snake_case
                        },
                        &#select_only_ids_query_part_token_stream
                    )}
                };
                let binded_query_token_stream = {
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &try_bind_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {
                        let mut #query_snake_case = sqlx::query::<sqlx::Postgres>(&#query_string_snake_case);
                        for #element_snake_case in #parameters_snake_case.#payload_snake_case.0 {
                            match #element_snake_case.#create_query_bind_snake_case(#query_snake_case) {
                                Ok(#value_snake_case) => {
                                    #query_snake_case = #value_snake_case;
                                },
                                Err(#error_0_token_stream) => {
                                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    // &generate_create_update_delete_many_fetch_token_stream(
                    //     &CreateOrUpdateOrDeleteMany::Create
                    // )
                    //todo reuse
                    &{
                        let current_operation = Operation::from(&CreateOrUpdateOrDeleteMany::Create);
                        generate_fetch_token_stream(
                            &{
                                let content_token_stream = generate_match_ident_read_only_ids_as_from_row_from_row_token_stream(&{
                                    let content_token_stream = generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!());
                                    quote::quote! {{#content_token_stream}}
                                });
                                quote::quote! {Some(#content_token_stream)}
                            },
                            &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
                            &ShouldWrapIntoValue::True,
                        )
                    },
                );
                impl_ident_vec_token_stream.push(generate_operation_token_stream(&operation, &common_additional_logic_token_stream, &parameters_logic_token_stream, &proc_macro2::TokenStream::new(), &query_string_token_stream, &binded_query_token_stream, &postgresql_logic_token_stream));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            impl_ident_vec_token_stream.push(generate_try_operation_token_stream(&operation, &type_variants_from_request_response_syn_variants, &std_vec_vec_ident_read_only_ids_token_stream, &value_snake_case));
            quote::quote! {
                #try_operation_error_named_token_stream
            }
        };
        impl_ident_vec_token_stream.push(generate_operation_payload_example_token_stream(&operation));
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"create_many",
    //     &create_many_token_stream,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
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
                value.push(try_bind_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(&operation, proc_macro2::TokenStream::new());
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(&operation, &ident_read_only_ids_upper_camel_case, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    let select_only_ids_query_part_token_stream = generate_select_only_ids_query_part_token_stream(&operation);
                    quote::quote! {
                        #postgresql_crud_snake_case::generate_create_one_query_string(
                            #table_snake_case,
                            #column_names_double_quotes_token_stream,
                            &match #parameters_snake_case.#payload_snake_case.#create_query_part_snake_case(&mut 0) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_0_token_stream) => {
                                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            },
                            &#select_only_ids_query_part_token_stream
                        )
                    }
                };
                let binded_query_token_stream = {
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &try_bind_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        match #parameters_snake_case.#payload_snake_case.#create_query_bind_snake_case(#query_snake_case) {
                            Ok(#value_snake_case) => {
                                #query_snake_case = #value_snake_case;
                            },
                            Err(#error_0_token_stream) => {
                                #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            }
                        }
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    // &generate_create_update_delete_one_fetch_token_stream(&CreateOrUpdateOrDeleteOne::Create)
                    &{
                        let current_operation = Operation::from(&CreateOrUpdateOrDeleteOne::Create);
                        wrap_into_value_token_stream(&generate_fetch_one_token_stream(
                            &generate_match_ident_read_only_ids_as_from_row_from_row_token_stream(&{
                                let content_token_stream = generate_match_postgres_transaction_rollback_await_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!());
                                quote::quote! {{#content_token_stream}}
                            }),
                            &generate_match_postgres_transaction_rollback_await_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
                        ))
                    },
                );
                impl_ident_vec_token_stream.push(generate_operation_token_stream(&operation, &common_additional_logic_token_stream, &parameters_logic_token_stream, &proc_macro2::TokenStream::new(), &query_string_token_stream, &binded_query_token_stream, &postgresql_logic_token_stream));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            impl_ident_vec_token_stream.push(generate_try_operation_token_stream(&operation, &type_variants_from_request_response_syn_variants, &ident_read_only_ids_upper_camel_case, &value_snake_case));
            quote::quote! {
                #try_operation_error_named_token_stream
            }
        };
        impl_ident_vec_token_stream.push(generate_operation_payload_example_token_stream(&operation));
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"create_one",
    //     &create_one_token_stream,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
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
                value.push(try_bind_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_parameters_payload_and_default_token_stream(
                &operation,
                &quote::quote! {{
                    #pub_where_many_std_option_option_ident_where_many_token_stream,
                    #pub_select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream,
                    pub #order_by_snake_case: #postgresql_crud_order_by_token_stream<#ident_select_upper_camel_case>,
                    pub #pagination_snake_case: postgresql_crud::PaginationStartsWithZero,
                }},
                &quote::quote! {{
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
            ),
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(&operation, &std_vec_vec_struct_options_ident_token_stream, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let select_query_part_parameters_payload_select_token_stream = generate_select_query_part_parameters_payload_select_token_stream(&operation);
                    let additional_paramaters_initialization_token_stream = generate_read_or_delete_many_additional_paramaters_initialization_token_stream(&ReadManyOrDeleteMany::ReadMany);
                    let additional_parameters_order_by_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}{order_snake_case} {by_snake_case} {{}} {{}}"));
                    let prefix_to_additional_parameters_token_stream = quote::quote! {let #prefix_snake_case = if additional_parameters.is_empty() {""} else {" "};};
                    let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    let order_by_column_match_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| {
                        let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                        quote::quote! {
                            #ident_select_upper_camel_case::#field_ident_upper_camel_case(_) => #field_ident_double_quotes_token_stream
                        }
                    });
                    let if_write_is_err_curly_braces_0_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(
                        &quote::quote! {
                            #additional_parameters_snake_case,
                            #additional_parameters_order_by_handle_token_stream,
                            #prefix_snake_case,
                            &match &#value_snake_case.#column_snake_case {
                                #order_by_column_match_token_stream
                            },
                            #order_snake_case,
                        },
                        &generate_write_into_buffer_query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream(&operation),
                    );
                    let if_write_is_err_curly_braces_1_token_stream = macros_helpers::generate_if_write_is_err_curly_braces_token_stream(&quote::quote! {#additional_parameters_snake_case, "{prefix}{value}"}, &generate_write_into_buffer_query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream(&operation));
                    quote::quote! {#postgresql_crud_snake_case::generate_read_many_query_string(
                        #table_snake_case,
                        &#select_query_part_parameters_payload_select_token_stream,
                        &{
                            #increment_initialization_token_stream
                            let mut #additional_parameters_snake_case = #additional_paramaters_initialization_token_stream;
                            {
                                #prefix_to_additional_parameters_token_stream
                                let #value_snake_case = &#parameters_snake_case.#payload_snake_case.#order_by_snake_case;
                                let #order_snake_case = match &#value_snake_case.#order_snake_case {
                                    Some(#value_snake_case) => #value_snake_case.to_snake_case_stringified(),
                                    None => #postgresql_crud_order_token_stream::default().to_snake_case_stringified(),
                                };
                                #if_write_is_err_curly_braces_0_token_stream
                            };
                            {
                                #prefix_to_additional_parameters_token_stream
                                let #value_snake_case = match #postgresql_crud_postgresql_type_where_filter_query_part_token_stream(
                                    &#parameters_snake_case.#payload_snake_case.pagination,
                                    &mut #increment_snake_case,
                                    &"",
                                    bool::default()
                                ) {
                                    Ok(#value_snake_case) => #value_snake_case,
                                    Err(#error_0_token_stream) => {
                                        #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    },
                                };
                                #if_write_is_err_curly_braces_1_token_stream
                            };
                            #additional_parameters_snake_case
                        }
                    )}
                };
                let binded_query_token_stream = {
                    let query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_token_stream = generate_query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_token_stream(&operation);
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &try_bind_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_token_stream
                        match #postgresql_crud_postgresql_type_where_filter_query_bind_token_stream(
                            #parameters_snake_case.#payload_snake_case.pagination,
                            #query_snake_case,
                        ) {
                            Ok(#value_snake_case) => {
                                #query_snake_case = #value_snake_case;
                            },
                            Err(#error_0_token_stream) => {
                                #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            }
                        }
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = {
                    let fetch_token_stream = generate_fetch_token_stream(
                        &{
                            let match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream = generate_match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream(&ReadManyOrReadOne::ReadMany);
                            quote::quote! {Some(#match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream)}
                        },
                        &generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!()),
                        &ShouldWrapIntoValue::False,
                    );
                    quote::quote! {{
                        #fetch_token_stream
                    }}
                };
                impl_ident_vec_token_stream.push(generate_operation_token_stream(&operation, &common_additional_logic_token_stream, &parameters_logic_token_stream, &proc_macro2::TokenStream::new(), &query_string_token_stream, &binded_query_token_stream, &postgresql_logic_token_stream));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(not_unique_field_syn_variant_wrapper.get_syn_variant().clone());
                value
            });
            impl_ident_vec_token_stream.push(generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_struct_options_ident_token_stream,
                &quote::quote! {
                    #value_snake_case
                    .into_iter()
                    .fold(Vec::new(), |mut #acc_snake_case, #element_snake_case| {
                        #acc_snake_case.push(#element_snake_case);
                        #acc_snake_case
                    })
                },
            ));
            quote::quote! {
                #try_operation_error_named_token_stream
            }
        };
        impl_ident_vec_token_stream.push(generate_operation_payload_example_token_stream(&operation));
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"read_many",
    //     &read_many_token_stream,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
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
                value.push(try_bind_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_parameters_payload_and_default_token_stream(
                &operation,
                &{
                    let pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream(&naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(primary_key_field_type));
                    quote::quote! {{
                        #pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream,
                        #pub_select_postgresql_crud_not_empty_unique_enum_vec_ident_select_token_stream,
                    }}
                },
                &quote::quote! {{
                    #primary_key_field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                    #select_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                }},
            ),
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream = generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(&operation, &ident_read_upper_camel_case, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let select_query_part_parameters_payload_select_token_stream = generate_select_query_part_parameters_payload_select_token_stream(&operation);
                    let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {#postgresql_crud_snake_case::generate_read_one_query_string(
                        #table_snake_case,
                        &#select_query_part_parameters_payload_select_token_stream,
                        &match #postgresql_crud_postgresql_type_where_filter_query_part_token_stream(
                            &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            &mut 0,
                            &Self::#primary_key_snake_case(),
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
                    let binded_query_modifications_token_stream = {
                        let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &try_bind_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            match #postgresql_crud_postgresql_type_where_filter_query_bind_token_stream(#parameters_snake_case.#payload_snake_case.#primary_key_field_ident, #query_snake_case) {
                                Ok(#value_snake_case) => {
                                    #query_snake_case = #value_snake_case;
                                },
                                Err(#error_0_token_stream) => {
                                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_modifications_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = generate_fetch_one_token_stream(
                    &generate_match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_ident_select_token_stream(&ReadManyOrReadOne::ReadOne),
                    &generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!()),
                );
                impl_ident_vec_token_stream.push(generate_operation_token_stream(&operation, &common_additional_logic_token_stream, &parameters_logic_token_stream, &proc_macro2::TokenStream::new(), &query_string_token_stream, &binded_query_token_stream, &postgresql_logic_token_stream));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(not_unique_field_syn_variant_wrapper.get_syn_variant().clone());
                value
            });
            impl_ident_vec_token_stream.push(generate_try_operation_token_stream(&operation, &type_variants_from_request_response_syn_variants, &ident_read_upper_camel_case, &value_snake_case));
            quote::quote! {
                #try_operation_error_named_token_stream
            }
        };
        impl_ident_vec_token_stream.push(generate_operation_payload_example_token_stream(&operation));
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"read_one",
    //     &read_one_token_stream,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
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
                value.push(try_bind_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(&operation, {
            let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
            let std_vec_vec_ident_update_token_stream = postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_token_stream(&ident_update_upper_camel_case);
            let ident_operation_payload_vec_token_stream = quote::quote! {
                #[derive(Debug, serde::Serialize, utoipa::ToSchema)]
                pub struct #ident_operation_payload_upper_camel_case(#std_vec_vec_ident_update_token_stream);
            };
            let ident_operation_payload_try_new_error_named_upper_camel_case = format!("{ident}{operation}PayloadTryNewErrorNamed").parse::<proc_macro2::TokenStream>().expect("error 3da248bb-84ba-48c9-9b7c-e0853198e0aa");
            let not_unique_primary_key_upper_camel_case = naming::NotUniquePrimaryKeyUpperCamelCase;
            let not_unique_primary_key_snake_case = naming::NotUniquePrimaryKeySnakeCase;
            let ident_operation_payload_try_new_error_named_token_stream = {
                quote::quote! {
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
            let impl_pub_try_new_for_ident_operation_payload_token_stream = macros_helpers::generate_impl_pub_try_new_for_ident_token_stream(
                &generate_ident_operation_payload_upper_camel_case(&operation),
                &quote::quote! {#value_snake_case: #std_vec_vec_ident_update_token_stream},
                &ident_operation_payload_try_new_error_named_upper_camel_case,
                &quote::quote! {
                    let mut #acc_snake_case = Vec::new();
                    for #element_snake_case in &#value_snake_case {
                        if #acc_snake_case.contains(&&#element_snake_case.#primary_key_field_ident) {
                            return Err(#ident_operation_payload_try_new_error_named_upper_camel_case::#not_unique_primary_key_upper_camel_case {
                                #not_unique_primary_key_snake_case: #element_snake_case.#primary_key_field_ident.clone(),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        #acc_snake_case.push(&#element_snake_case.#primary_key_field_ident);
                    }
                    Ok(Self(#value_snake_case))
                },
            );
            let impl_serde_deserialize_for_ident_update_many_payload_token_stream = {
                let tuple_struct_ident_operation_payload_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("tuple struct {ident_operation_payload_upper_camel_case}"));
                let tuple_struct_ident_operation_payload_with_1_element_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("tuple struct {ident_operation_payload_upper_camel_case} with 1 element"));
                let match_ident_update_many_payload_try_new_field0_token_stream = postgresql_crud_macros_common::generate_match_try_new_in_deserialize_token_stream(&ident_operation_payload_upper_camel_case, &quote::quote! {__field0});
                let ident_operation_payload_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_operation_payload_upper_camel_case);
                quote::quote! {
                    const _: () = {
                        #[allow(unused_extern_crates, clippy::useless_attribute)]
                        extern crate serde as _serde;
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for #ident_operation_payload_upper_camel_case {
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> Result<Self, __D::Error>
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
                                    ) -> Result<Self::Value, __E::Error>
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
                                    ) -> Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<#std_vec_vec_ident_update_token_stream>(&mut __seq)? {
                                            Some(__value) => __value,
                                            None => {
                                                return Err(
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
                                        marker: _serde::__private::PhantomData::<Self>,
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
                #impl_pub_try_new_for_ident_operation_payload_token_stream
                #impl_serde_deserialize_for_ident_update_many_payload_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
            }
        });
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(&operation, &std_vec_vec_ident_read_only_ids_token_stream, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_token_stream = {
                    let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                    quote::quote! {
                        #parameters_logic_token_stream
                        let #update_for_query_vec_snake_case = #parameters_snake_case.#payload_snake_case.0.into_iter()
                        .map(|#element_snake_case|#ident_update_for_query_upper_camel_case::#from_handle_snake_case(#element_snake_case))
                        .collect::<Vec<#ident_update_for_query_upper_camel_case>>();
                    }
                };
                let query_string_token_stream = {
                    let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    let match_update_query_part_primary_key_token_stream = generate_match_update_query_part_primary_key_token_stream(&operation, &element_snake_case);
                    let fields_named_without_primary_key_update_assignment_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                        let is_field_ident_update_exists_snake_case = naming::parameter::IsSelfUpdateExistSnakeCase::from_tokens(&field_ident);
                        let update_query_part_field_ident_snake_case = naming::parameter::UpdateQueryPartSelfSnakeCase::from_tokens(&field_ident);
                        let generate_when_column_id_then_value_update_many_query_part_snake_case = naming::GenerateWhenColumnIdThenValueUpdateManyQueryPartSnakeCase;
                        quote::quote! {
                            {
                                let mut #is_field_ident_update_exists_snake_case = false;
                                for #element_snake_case in &#update_for_query_vec_snake_case {
                                    if #element_snake_case.#field_ident.is_some() {
                                        #is_field_ident_update_exists_snake_case = true;
                                        break;
                                    }
                                }
                                if #is_field_ident_update_exists_snake_case {
                                    #acc_snake_case.push_str(&
                                        postgresql_crud::generate_column_equals_case_acc_else_column_end_comma_update_many_query_part(
                                            #field_ident_double_quotes_token_stream,
                                            &{
                                                let mut #acc_snake_case = #string_token_stream::default();
                                                for #element_snake_case in &#update_for_query_vec_snake_case {
                                                    if let Some(#value_snake_case) = &#element_snake_case.#field_ident {
                                                        #acc_snake_case.push_str(&#postgresql_crud_snake_case::#generate_when_column_id_then_value_update_many_query_part_snake_case(
                                                            Self::#primary_key_snake_case(),
                                                            &match #element_snake_case.#update_query_part_primary_key_snake_case(&mut #increment_snake_case) {
                                                                Ok(#value_snake_case) => #value_snake_case,
                                                                Err(#error_0_token_stream) => {
                                                                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                                }
                                                            },
                                                            &match #ident_update_for_query_upper_camel_case::#update_query_part_field_ident_snake_case(#value_snake_case, &mut #increment_snake_case) {
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
                    let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream(&quote::quote! {#acc_snake_case, "{},", #match_update_query_part_primary_key_token_stream}, &generate_write_into_buffer_query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream(&operation));
                    quote::quote! {
                        {
                            #increment_initialization_token_stream
                            let elements = {
                                let mut #acc_snake_case = #string_token_stream::default();
                                #fields_named_without_primary_key_update_assignment_token_stream
                                let _: Option<char> = #acc_snake_case.pop();
                                #acc_snake_case
                            };
                            let primary_keys = {
                                let mut #acc_snake_case = #string_token_stream::default();
                                for #element_snake_case in &#update_for_query_vec_snake_case {
                                    #if_write_is_err_token_stream
                                }
                                let _: Option<char> = #acc_snake_case.pop();
                                #acc_snake_case
                            };
                            //todo refactor\reuse
                            let return_columns = {
                                let mut #acc_snake_case = String::new();
                                for #element_snake_case in &#update_for_query_vec_snake_case {
                                    match #element_snake_case.select_only_updated_ids_query_part(&mut #increment_snake_case) {
                                        Ok(#value_snake_case) => {
                                            #acc_snake_case.push_str(&#value_snake_case);
                                        },
                                        Err(#error_0_token_stream) => {
                                            #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        }
                                    }
                                }
                                #acc_snake_case
                            };
                            postgresql_crud::generate_update_many_query_string(
                                #table_snake_case,
                                &elements,
                                Self::#primary_key_snake_case(),
                                &primary_keys,
                                &return_columns
                            )
                        }
                    }
                };
                let binded_query_token_stream = {
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &try_bind_syn_variant_wrapper, file!(), line!(), column!());
                    let fields_named_without_primary_key_update_assignment_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
                        quote::quote! {
                            for #element_snake_case in &#update_for_query_vec_snake_case {
                                if let Some(#value_snake_case) = &#element_snake_case.#field_ident {
                                    if let Err(#error_0_token_stream) = #query_snake_case.try_bind(#element_snake_case.#primary_key_field_ident.clone()) {
                                        let #error_0_token_stream = #error_0_token_stream.to_string();
                                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                    match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_bind_snake_case(
                                        #value_snake_case.#value_snake_case.clone(),
                                        #query_snake_case,
                                    ) {
                                        Ok(#value_snake_case) => {
                                            #query_snake_case = #value_snake_case;
                                        },
                                        Err(#error_0_token_stream) => {
                                            #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        }
                                    }
                                }
                            }
                        }
                    });
                    let primary_key_update_assignment_token_stream = quote::quote! {
                        for #element_snake_case in &#update_for_query_vec_snake_case {
                            match #primary_key_field_type_as_postgresql_type_token_stream #update_query_bind_snake_case(
                                #element_snake_case.#primary_key_field_ident.clone(),
                                #query_snake_case,
                            ) {
                                Ok(#value_snake_case) => {
                                    #query_snake_case = #value_snake_case;
                                },
                                Err(#error_0_token_stream) => {
                                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                    };
                    let binded_query_select_only_updated_ids_query_bind_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
                        quote::quote! {
                            for #element_snake_case in &#update_for_query_vec_snake_case {
                                if let Some(#value_snake_case) = &#element_snake_case.#field_ident {
                                    match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream select_only_updated_ids_query_bind(
                                        &#value_snake_case.#value_snake_case,
                                        #query_snake_case
                                    ) {
                                        Ok(#value_snake_case) => {
                                            #query_snake_case = #value_snake_case;
                                        },
                                        Err(#error_0_token_stream) => {
                                            #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        }
                                    }
                                }
                            }
                        }
                    });
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #fields_named_without_primary_key_update_assignment_token_stream
                        #primary_key_update_assignment_token_stream
                        #binded_query_select_only_updated_ids_query_bind_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    // &generate_create_update_delete_many_fetch_token_stream(&CreateOrUpdateOrDeleteMany::UpdateMany)
                    // &{
                    //     let current_operation = Operation::from(&CreateOrUpdateOrDeleteMany::Update);
                    //     generate_fetch_token_stream(
                    //         &generate_sqlx_row_try_get_primary_key_token_stream(
                    //             &ident_read_only_ids_upper_camel_case,
                    //             &quote::quote! {Some(#value_snake_case)},
                    //             &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
                    //         ),
                    //         &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
                    //         &ShouldWrapIntoValue::True,
                    //     )
                    // }
                    &{
                        let current_operation = Operation::from(&CreateOrUpdateOrDeleteMany::Update);
                        generate_fetch_token_stream(
                            &{
                                let content_token_stream = generate_match_ident_read_only_ids_as_from_row_from_row_token_stream(&{
                                    let content_token_stream = generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!());
                                    quote::quote! {{#content_token_stream}}
                                });
                                quote::quote! {Some(#content_token_stream)}
                            },
                            &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
                            &ShouldWrapIntoValue::True,
                        )
                    },
                );
                impl_ident_vec_token_stream.push(generate_operation_token_stream(&operation, &common_additional_logic_token_stream, &parameters_logic_token_stream, &proc_macro2::TokenStream::new(), &query_string_token_stream, &binded_query_token_stream, &postgresql_logic_token_stream));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            impl_ident_vec_token_stream.push(generate_try_operation_token_stream(&operation, &type_variants_from_request_response_syn_variants, &std_vec_vec_ident_read_only_ids_token_stream, &value_snake_case));
            quote::quote! {
                #try_operation_error_named_token_stream
            }
        };
        impl_ident_vec_token_stream.push(generate_operation_payload_example_token_stream(&operation));
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"update_many",
    //     &update_many_token_stream,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
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
                value.push(try_bind_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(&operation, proc_macro2::TokenStream::new());
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(&operation, &ident_read_only_ids_upper_camel_case, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_token_stream = {
                    let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                    quote::quote! {
                        #parameters_logic_token_stream
                        let #update_for_query_snake_case = #ident_update_for_query_upper_camel_case::#from_handle_snake_case(#parameters_snake_case.#payload_snake_case);
                    }
                };
                let query_string_token_stream = {
                    let additional_parameters_modification_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                        let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                        let generate_column_queals_value_comma_update_one_query_part_snake_case = naming::GenerateColumnQuealsValueCommaUpdateOneQueryPartSnakeCase;
                        let update_query_part_field_ident_snake_case = naming::parameter::UpdateQueryPartSelfSnakeCase::from_tokens(&field_ident);
                        quote::quote! {
                            if let Some(#value_snake_case) = &#update_for_query_snake_case.#field_ident {
                                #acc_snake_case.push_str(&#postgresql_crud_snake_case::#generate_column_queals_value_comma_update_one_query_part_snake_case(
                                    #field_ident_double_quotes_token_stream,
                                    &match #ident_update_for_query_upper_camel_case::#update_query_part_field_ident_snake_case(#value_snake_case, &mut #increment_snake_case) {
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
                    let additional_parameters_primary_key_modification_token_stream = generate_match_update_query_part_primary_key_token_stream(&operation, &quote::quote! {#update_for_query_snake_case});
                    //todo refactor
                    let query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {
                        {
                            #increment_initialization_token_stream
                            let #columns_snake_case = {
                                let mut #acc_snake_case = #string_token_stream::default();
                                #additional_parameters_modification_token_stream
                                let _: Option<char> = #acc_snake_case.pop();
                                #acc_snake_case
                            };
                            let #primary_key_query_part_snake_case = #additional_parameters_primary_key_modification_token_stream;
                            //todo refactor\reuse
                            let return_columns = match #update_for_query_snake_case.select_only_updated_ids_query_part(&mut #increment_snake_case) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_0_token_stream) => {
                                    #query_part_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            };
                            #postgresql_crud_snake_case::generate_update_one_query_string(
                                #table_snake_case,
                                &#columns_snake_case,
                                Self::#primary_key_snake_case(),
                                &#primary_key_query_part_snake_case,
                                &return_columns
                            )
                        }
                    }
                };
                let binded_query_token_stream = {
                    //todo rename and all copies too (must be named try_bind instead of postgresql)
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &try_bind_syn_variant_wrapper, file!(), line!(), column!());
                    let binded_query_modifications_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
                        quote::quote! {
                            if let Some(#value_snake_case) = &#update_for_query_snake_case.#field_ident {
                                match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_bind_snake_case(
                                    #value_snake_case.#value_snake_case.clone(),//todo is there a way to remove .clone here?
                                    #query_snake_case
                                ) {
                                    Ok(#value_snake_case) => {
                                        #query_snake_case = #value_snake_case;
                                    }
                                    Err(#error_0_token_stream) => {
                                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                        }
                    });
                    let binded_query_primary_key_modification_token_stream = quote::quote! {
                        match #primary_key_field_type_as_postgresql_type_token_stream #update_query_bind_snake_case(
                            #update_for_query_snake_case.#primary_key_field_ident,
                            #query_snake_case,
                        ) {
                            Ok(#value_snake_case) => {
                                #query_snake_case = #value_snake_case;
                            },
                            Err(#error_0_token_stream) => {
                                #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            }
                        }
                    };
                    let binded_query_select_only_updated_ids_query_bind_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_type_token_stream(&element.syn_field.ty);
                        quote::quote! {
                            if let Some(#value_snake_case) = &#update_for_query_snake_case.#field_ident {
                                match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream select_only_updated_ids_query_bind(
                                    &#value_snake_case.#value_snake_case,
                                    #query_snake_case
                                ) {
                                    Ok(#value_snake_case) => {
                                        #query_snake_case = #value_snake_case;
                                    },
                                    Err(#error_0_token_stream) => {
                                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                        }
                    });
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_modifications_token_stream
                        #binded_query_primary_key_modification_token_stream
                        #binded_query_select_only_updated_ids_query_bind_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    // &generate_create_update_delete_one_fetch_token_stream(&CreateOrUpdateOrDeleteOne::Update)
                    &{
                        let current_operation = Operation::from(&CreateOrUpdateOrDeleteOne::Update);
                        wrap_into_value_token_stream(&generate_fetch_one_token_stream(
                            &generate_match_ident_read_only_ids_as_from_row_from_row_token_stream(
                                &generate_match_postgres_transaction_rollback_await_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!())
                            ),
                            &generate_match_postgres_transaction_rollback_await_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
                        ))
                    },
                );
                impl_ident_vec_token_stream.push(generate_operation_token_stream(&operation, &common_additional_logic_token_stream, &parameters_logic_token_stream, &proc_macro2::TokenStream::new(), &query_string_token_stream, &binded_query_token_stream, &postgresql_logic_token_stream));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            impl_ident_vec_token_stream.push(generate_try_operation_token_stream(&operation, &type_variants_from_request_response_syn_variants, &ident_read_only_ids_upper_camel_case, &value_snake_case));
            quote::quote! {
                #try_operation_error_named_token_stream
            }
        };
        impl_ident_vec_token_stream.push(generate_operation_payload_example_token_stream(&operation));
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"update_one",
    //     &update_one_token_stream,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
    // );
    //todo return deleted rows ids vec
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
                value.push(try_bind_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_parameters_payload_and_default_token_stream(&operation, &quote::quote! {{#pub_where_many_std_option_option_ident_where_many_token_stream}}, &quote::quote! {{#where_many_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream}}),
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(&operation, &std_vec_vec_primary_key_field_type_read_token_stream, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let additional_paramaters_initialization_token_stream = generate_read_or_delete_many_additional_paramaters_initialization_token_stream(&ReadManyOrDeleteMany::DeleteMany);
                    quote::quote! {#postgresql_crud_snake_case::generate_delete_many_query_string(
                        #table_snake_case,
                        &{
                            #increment_initialization_token_stream
                            #additional_paramaters_initialization_token_stream
                        },
                        Self::#primary_key_snake_case(),
                    )}
                };
                let binded_query_token_stream = {
                    let query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_token_stream = generate_query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_token_stream(&operation);
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_many_fetch_token_stream(&CreateOrUpdateOrDeleteMany::Delete)
                );
                impl_ident_vec_token_stream.push(generate_operation_token_stream(&operation, &common_additional_logic_token_stream, &parameters_logic_token_stream, &proc_macro2::TokenStream::new(), &query_string_token_stream, &binded_query_token_stream, &postgresql_logic_token_stream));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            impl_ident_vec_token_stream.push(generate_try_operation_token_stream(&operation, &type_variants_from_request_response_syn_variants, &std_vec_vec_primary_key_field_type_read_token_stream, &value_snake_case));
            quote::quote! {
                #try_operation_error_named_token_stream
            }
        };
        impl_ident_vec_token_stream.push(generate_operation_payload_example_token_stream(&operation));
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"delete_many",
    //     &delete_many_token_stream,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
    // );
    let delete_one_token_stream = {
        let operation = Operation::DeleteOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(try_bind_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_parameters_payload_and_default_token_stream(
                &operation,
                &{
                    let content_token_stream = generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream(&naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(primary_key_field_type));
                    quote::quote! {{#content_token_stream}}
                },
                &{
                    let primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                        quote::quote! {
                            #primary_key_field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                        }
                    };
                    quote::quote! {{#primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream}}
                },
            ),
        );
        let operation_token_stream = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_token_stream(&operation, &primary_key_field_type_as_postgresql_type_read_upper_camel_case, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = quote::quote! {#postgresql_crud_snake_case::generate_delete_one_query_string(
                    #table_snake_case,
                    Self::#primary_key_snake_case(),
                )};
                let binded_query_token_stream = {
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &try_bind_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(
                            #parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            #query_snake_case
                        ) {
                            Ok(#value_snake_case) => {
                                #query_snake_case = #value_snake_case;
                            },
                            Err(#error_0_token_stream) => {
                                #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            }
                        }
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_one_fetch_token_stream(&CreateOrUpdateOrDeleteOne::Delete)
                );
                impl_ident_vec_token_stream.push(generate_operation_token_stream(&operation, &common_additional_logic_token_stream, &parameters_logic_token_stream, &proc_macro2::TokenStream::new(), &query_string_token_stream, &binded_query_token_stream, &postgresql_logic_token_stream));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            impl_ident_vec_token_stream.push(generate_try_operation_token_stream(&operation, &type_variants_from_request_response_syn_variants, &primary_key_field_type_as_postgresql_type_read_upper_camel_case, &value_snake_case));
            quote::quote! {
                #try_operation_error_named_token_stream
            }
        };
        impl_ident_vec_token_stream.push(generate_operation_payload_example_token_stream(&operation));
        quote::quote! {
            #parameters_token_stream
            #operation_token_stream
            #try_operation_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"delete_one",
    //     &delete_one_token_stream,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
    // );
    {
        let generate_slash_route_double_quotes_token_stream = |value: &dyn std::fmt::Display| generate_quotes::double_quotes_token_stream(&format!("/{value}"));
        let create_many = Operation::CreateMany;
        let create_one = Operation::CreateOne;
        let read_many = Operation::ReadMany;
        let read_one = Operation::ReadOne;
        let update_many = Operation::UpdateMany;
        let update_one = Operation::UpdateOne;
        let delete_many = Operation::DeleteMany;
        let delete_one = Operation::DeleteOne;
        let slash_create_many_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&create_many.self_snake_case_stringified());
        let slash_create_one_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&create_one.self_snake_case_stringified());
        let slash_read_many_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&read_many.self_snake_case_stringified());
        let slash_read_one_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&read_one.self_snake_case_stringified());
        let slash_update_many_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&update_many.self_snake_case_stringified());
        let slash_update_one_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&update_one.self_snake_case_stringified());
        let slash_delete_many_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&delete_many.self_snake_case_stringified());
        let slash_delete_one_double_quotes_token_stream = generate_slash_route_double_quotes_token_stream(&delete_one.self_snake_case_stringified());
        let create_many_snake_case_token_stream = create_many.self_handle_snake_case_token_stream();
        let create_one_snake_case_token_stream = create_one.self_handle_snake_case_token_stream();
        let read_many_snake_case_token_stream = read_many.self_handle_snake_case_token_stream();
        let read_one_snake_case_token_stream = read_one.self_handle_snake_case_token_stream();
        let update_many_snake_case_token_stream = update_many.self_handle_snake_case_token_stream();
        let update_one_snake_case_token_stream = update_one.self_handle_snake_case_token_stream();
        let delete_many_snake_case_token_stream = delete_many.self_handle_snake_case_token_stream();
        let delete_one_snake_case_token_stream = delete_one.self_handle_snake_case_token_stream();
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
        impl_ident_vec_token_stream.push(quote::quote! {
            fn #routes_handle_snake_case(#app_state_snake_case: #std_sync_arc_combination_of_app_state_logic_traits_token_stream, #table_snake_case: &str) -> axum::Router {
                axum::Router::new().nest(
                    &format!("/{table}"),
                    axum::Router::new()
                    .route(#slash_create_many_double_quotes_token_stream, axum::routing::post(
                        {
                            let table = table.to_string();
                            move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                                let table = table.clone();
                                async move {
                                    Self::#create_many_snake_case_token_stream(app_state, request, &table).await
                                }
                            }
                        }
                    ))
                    .route(#slash_create_many_example_double_quotes_token_stream, axum::routing::get(Self::#create_many_payload_example_snake_case))
                    .route(#slash_create_one_double_quotes_token_stream, axum::routing::post(
                        {
                            let table = table.to_string();
                            move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                                let table = table.clone();
                                async move {
                                    Self::#create_one_snake_case_token_stream(app_state, request, &table).await
                                }
                            }
                        }
                    ))
                    .route(#slash_create_one_example_double_quotes_token_stream, axum::routing::get(Self::#create_one_payload_example_snake_case))
                    .route(#slash_read_many_double_quotes_token_stream, axum::routing::post(
                        {
                            let table = table.to_string();
                            move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                                let table = table.clone();
                                async move {
                                    Self::#read_many_snake_case_token_stream(app_state, request, &table).await
                                }
                            }
                        }
                    ))
                    .route(#slash_read_many_example_double_quotes_token_stream, axum::routing::get(Self::#read_many_payload_example_snake_case))
                    .route(#slash_read_one_double_quotes_token_stream, axum::routing::post(
                        {
                            let table = table.to_string();
                            move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                                let table = table.clone();
                                async move {
                                    Self::#read_one_snake_case_token_stream(app_state, request, &table).await
                                }
                            }
                        }
                    ))
                    .route(#slash_read_one_example_double_quotes_token_stream, axum::routing::get(Self::#read_one_payload_example_snake_case))
                    .route(#slash_update_many_double_quotes_token_stream, axum::routing::patch(
                        {
                            let table = table.to_string();
                            move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                                let table = table.clone();
                                async move {
                                    Self::#update_many_snake_case_token_stream(app_state, request, &table).await
                                }
                            }
                        }
                    ))
                    .route(#slash_update_many_example_double_quotes_token_stream, axum::routing::get(Self::#update_many_payload_example_snake_case))
                    .route(#slash_update_one_double_quotes_token_stream, axum::routing::patch(
                        {
                            let table = table.to_string();
                            move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                                let table = table.clone();
                                async move {
                                    Self::#update_one_snake_case_token_stream(app_state, request, &table).await
                                }
                            }
                        }
                    ))
                    .route(#slash_update_one_example_double_quotes_token_stream, axum::routing::get(Self::#update_one_payload_example_snake_case))
                    .route(#slash_delete_many_double_quotes_token_stream, axum::routing::delete(
                        {
                            let table = table.to_string();
                            move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                                let table = table.clone();
                                async move {
                                    Self::#delete_many_snake_case_token_stream(app_state, request, &table).await
                                }
                            }
                        }
                    ))
                    .route(#slash_delete_many_example_double_quotes_token_stream, axum::routing::get(Self::#delete_many_payload_example_snake_case))
                    .route(#slash_delete_one_double_quotes_token_stream, axum::routing::delete(
                        {
                            let table = table.to_string();
                            move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                                let table = table.clone();
                                async move {
                                    Self::#delete_one_snake_case_token_stream(app_state, request, &table).await
                                }
                            }
                        }
                    ))
                    .route(#slash_delete_one_example_double_quotes_token_stream, axum::routing::get(Self::#delete_one_payload_example_snake_case))
                    // .layer(tower_http::cors::CorsLayer::new().allow_methods(#ident::allow_methods()))
                    .with_state(#app_state_snake_case)
                )
            }
            pub fn #routes_snake_case(#app_state_snake_case: #std_sync_arc_combination_of_app_state_logic_traits_token_stream) -> axum::Router {
                Self::#routes_handle_snake_case(#app_state_snake_case, #self_table_name_call_token_stream)
            }
        });
    };
    let ident_tests_token_stream = {
        let ident_tests_snake_case = naming::parameter::SelfTestsSnakeCase::from_display(&ident);
        let ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&naming::DisplayToSnakeCaseStringified::case(&ident));
        let ident_create_many_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::CreateMany);
        let ident_read_many_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::ReadMany);
        let ident_create_many_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&Operation::CreateMany);
        let ident_read_many_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&Operation::ReadMany);
        let ident_create_one_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::CreateOne);
        let ident_read_one_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::ReadOne);
        let ident_read_one_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&Operation::ReadOne);
        let ident_update_one_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&Operation::UpdateOne);
        let config_path_token_stream = quote::quote! {server_config::Config};
        let config_upper_case_token_stream = quote::quote! {CONFIG};
        let underscore_unused_token_stream = quote::quote! {_unused};
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
        //todo instead of first dropping table - check if its not exists. if exists test must fail
        let select_default_all_with_max_page_size_not_empty_unique_enum_vec_token_stream = {
            let content_token_stream = generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_type = &element.syn_field.ty;
                let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                let upper_camel_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSizeUpperCamelCase;
                let snake_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSizeSnakeCase;
                quote::quote! {
                    #ident_select_upper_camel_case::#field_ident_upper_camel_case(
                        <
                            <
                                #field_type
                                as
                                postgresql_crud::PostgresqlType
                            >::Select
                            as
                            postgresql_crud::#upper_camel_case
                        >::#snake_case()
                    )
                }
            });
            let warning_message_double_quote_token_stream = generate_quotes::double_quotes_token_stream(&"error 8f42ee4f-00d9-4b67-8ead-adddf5bcdf94".to_owned());
            quote::quote! {
                let select_default_all_with_max_page_size = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![
                    #content_token_stream
                ]).expect(#warning_message_double_quote_token_stream);
            }
        };
        let generate_primary_key_field_type_as_postgresql_type_primary_key_method_call_token_stream = |method_token_stream: &dyn quote::ToTokens, parameters_token_stream: &dyn quote::ToTokens| {
            quote::quote! {
                <
                    #primary_key_field_type
                    as
                    postgresql_crud::PostgresqlTypePrimaryKey
                >::#method_token_stream(
                    #parameters_token_stream
                )
            }
        };
        let primary_key_field_type_read_into_table_type_declaration_element_primary_key_field_ident_clone_token_stream = generate_primary_key_field_type_as_postgresql_type_primary_key_method_call_token_stream(&read_into_table_type_declaration_snake_case, &element_snake_case);
        let primary_key_field_type_read_only_ids_into_table_type_declaration_element_primary_key_field_ident_clone_token_stream = generate_primary_key_field_type_as_postgresql_type_primary_key_method_call_token_stream(&read_only_ids_into_table_type_declaration_snake_case, &quote::quote! {#element_snake_case.#primary_key_field_ident.clone()});
        let (
            primary_key_field_type_read_only_ids_into_read_element_primary_key_field_ident_token_stream,
            primary_key_field_type_read_only_ids_into_read_element_primary_key_field_ident_clone_token_stream,
            primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_clone_token_stream,
            primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_default_primary_key_field_ident_clone_token_stream,
            primary_key_field_type_read_only_is_into_read_read_only_ids_current_element_primary_key_field_ident_clone_token_stream,
            primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream,
            primary_key_field_type_read_only_ids_into_read_common_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream,
        ) = {
            let generate_read_only_ids_into_read_token_stream = |content_token_stream: &dyn quote::ToTokens| generate_primary_key_field_type_as_postgresql_type_primary_key_method_call_token_stream(&read_only_ids_into_read_snake_case, &content_token_stream);
            (
                generate_read_only_ids_into_read_token_stream(&quote::quote! {#element_snake_case.#primary_key_field_ident}),
                generate_read_only_ids_into_read_token_stream(&quote::quote! {#element_snake_case.#primary_key_field_ident.clone()}),
                generate_read_only_ids_into_read_token_stream(&quote::quote! {read_only_ids_from_try_create_one.#primary_key_field_ident.clone()}),
                generate_read_only_ids_into_read_token_stream(&quote::quote! {read_only_ids_from_try_create_one_default.#primary_key_field_ident.clone()}),
                generate_read_only_ids_into_read_token_stream(&quote::quote! {read_only_ids_current_element.#primary_key_field_ident.clone()}),
                generate_read_only_ids_into_read_token_stream(&quote::quote! {read_only_ids_returned_from_create_one.#primary_key_field_ident.clone()}),
                generate_read_only_ids_into_read_token_stream(&quote::quote! {#common_read_only_ids_returned_from_create_one_snake_case.#primary_key_field_ident.clone()}),
            )
        };
        let primary_key_field_type_as_postgresql_type_update_as_postgresql_type_primary_key_read_only_ids_into_update_token_stream = {
            let method_call_token_stream = generate_primary_key_field_type_as_postgresql_type_primary_key_method_call_token_stream(&read_only_ids_into_update_snake_case, &quote::quote! {read_only_ids_current_element.#primary_key_field_ident.clone()});
            quote::quote! {
                <
                    #primary_key_field_type
                    as
                    postgresql_crud::PostgresqlType
                >::Update::from(#method_call_token_stream)
            }
        };
        let (
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_and_create_token_stream,
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_from_try_create_one_ident_create_token_stream,
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_create_token_stream,
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_clone_ident_create_clone_token_stream,
        ) = {
            enum ShouldAddDotClone {
                True,
                False,
            }
            let generate_field_ident_read_only_ids_merged_with_create_into_option_value_read_token_stream = |read_only_ids_content_token_stream: &dyn quote::ToTokens, create_content_token_stream: &dyn quote::ToTokens, should_add_dot_clone: &ShouldAddDotClone| {
                generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
                    let current_field_ident = &element.field_ident;
                    let current_field_type = &element.syn_field.ty;
                    let maybe_dot_clone_token_stream = match &should_add_dot_clone {
                        ShouldAddDotClone::True => quote::quote! {.clone()},
                        ShouldAddDotClone::False => proc_macro2::TokenStream::new(),
                    };
                    quote::quote! {
                        #current_field_ident: <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(
                            #read_only_ids_content_token_stream.#current_field_ident #maybe_dot_clone_token_stream.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"),
                            #create_content_token_stream.#current_field_ident #maybe_dot_clone_token_stream
                        )
                    }
                })
            };
            (
                generate_field_ident_read_only_ids_merged_with_create_into_option_value_read_token_stream(&read_only_ids_snake_case, &create_snake_case, &ShouldAddDotClone::False),
                generate_field_ident_read_only_ids_merged_with_create_into_option_value_read_token_stream(&quote::quote! {read_only_ids_from_try_create_one}, &quote::quote! {ident_create}, &ShouldAddDotClone::False),
                generate_field_ident_read_only_ids_merged_with_create_into_option_value_read_token_stream(&quote::quote! {read_only_ids_returned_from_create_one}, &quote::quote! {ident_create_default}, &ShouldAddDotClone::False),
                generate_field_ident_read_only_ids_merged_with_create_into_option_value_read_token_stream(&quote::quote! {read_only_ids_returned_from_create_one}, &quote::quote! {ident_create}, &ShouldAddDotClone::True),
            )
        };
        let std_option_option_ident_where_many_content_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
            let current_field_ident = &element.field_ident;
            quote::quote! {
                #current_field_ident: None
            }
        });
        let select_default_all_with_max_page_size_clone_token_stream = quote::quote!{select_default_all_with_max_page_size.clone()};
        let common_read_only_ids_returned_from_create_one_token_stream = {
            let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&primary_key_field_type_read_only_ids_into_read_common_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream);
            quote::quote! {
                let #common_read_only_ids_returned_from_create_one_snake_case = #ident::try_create_one(
                    &#url_snake_case,
                    #ident_create_one_parameters_upper_camel_case {
                        #payload_snake_case: #ident_create_default_snake_case.clone()
                    }
                ).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
                let #some_value_read_only_ids_returned_from_create_one_snake_case = Some(#value_initialization_token_stream);
                assert_eq!(
                    #ident_read_upper_camel_case {
                        #primary_key_field_ident: #some_value_read_only_ids_returned_from_create_one_snake_case.clone(),
                        #fields_none_initialization_token_stream
                    },
                    #ident::try_read_one(
                        &#url_snake_case,
                        #ident_read_one_parameters_upper_camel_case {
                            #payload_snake_case: #ident_read_one_payload_upper_camel_case {
                                #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_common_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream,
                                #select_snake_case: #select_primary_key_snake_case.clone(),
                            },
                        },
                    )
                    .await
                    .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                    "error 3d9f2ec0-e374-48d2-a36b-486f5598b0b4"
                );
                assert_eq!(
                    #ident::try_delete_one(
                        &url,
                        #ident_delete_one_parameters_upper_camel_case {
                            payload: #ident_delete_one_payload_upper_camel_case {
                                #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_common_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream
                            }
                        }
                    ).await.expect("error 006b18e9-c965-45ee-afc0-a4f6b850ed06"),
                    #primary_key_field_type_read_only_ids_into_read_common_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream,
                    "error 26e2058b-4bc1-42da-8f35-0ab993904de5"
                );
                if let Err(#error_snake_case) = #ident::try_read_one(
                    &url,
                    #ident_read_one_parameters_upper_camel_case {
                        payload: #ident_read_one_payload_upper_camel_case {
                            #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_common_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream,
                            select: #select_default_all_with_max_page_size_clone_token_stream
                        }
                    }
                ).await {
                    if let #ident_try_read_one_error_named_upper_camel_case::#ident_read_one_error_named_with_serialize_deserialize_upper_camel_case {
                        read_one_error_named_with_serialize_deserialize,
                        ..
                    } = error {
                        if let #ident_read_one_error_named_with_serialize_deserialize_upper_camel_case::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                            assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 58b9a6a4-cf9b-49f3-a20f-7007deea40fd");
                        } else {
                            panic!("error 0ad0117b-a2e0-4629-99d0-71935cd93d15");
                        }
                    } else {
                        panic!("error c6695392-4b5f-4482-86aa-b2f19c33a746")
                    }
                } else {
                    panic!("error 67e43b7a-d3ec-4a3b-a3f1-8c11499fd090")
                }
            }
        };
        let generate_ident_create_content_token_stream = |field_ident: &syn::Ident, content_token_stream: &dyn quote::ToTokens| {
            generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
                let current_field_ident = &element.field_ident;
                let current_field_type = &element.syn_field.ty;
                if field_ident == current_field_ident {
                    quote::quote! {
                        #current_field_ident: #content_token_stream
                    }
                } else {
                    quote::quote! {
                        #current_field_ident: <
                            <#current_field_type as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
                        >::default_but_option_is_always_some_and_vec_always_contains_one_element()
                    }
                }
            })
        };
        let generate_ident_create_content_element_token_stream = |field_ident: &syn::Ident| generate_ident_create_content_token_stream(field_ident, &element_snake_case);
        let generate_ident_create_content_element_create_token_stream = |field_ident: &syn::Ident| generate_ident_create_content_token_stream(field_ident, &quote::quote! {#element_snake_case.#create_snake_case});

        let generate_table_test_name_field_ident_cloned2_token_stream = |test_name: &str, field_ident: &syn::Ident| format!("table_{test_name}_{field_ident}_cloned2").parse::<proc_macro2::TokenStream>().expect("error 2003ad9f-013a-48ba-b0ef-d2d48774d60c");
        let mut table_field_idents_initialization_vec_token_stream = vec![];
        let mut table_field_idents_clones_vec_token_stream = vec![];
        let mut table_field_idents_clones2_vec_token_stream = vec![];
        let mut table_field_idents_to_drop_table_if_exists_vec_token_stream = vec![];
        let mut table_field_idents_for_prepare_postgresql_table_vec_token_stream = vec![];
        let mut table_field_idents_for_routes_handle_vec_token_stream = vec![];
        let mut fill_table_field_idents_vec_token_stream = |test_names: Vec<&str>| {
            for test_name in test_names {
                let generate_initialization_variable_name_token_stream = |field_ident: &syn::Ident| format!("table_{test_name}_{field_ident}").parse::<proc_macro2::TokenStream>().expect("error 2003ad9f-013a-48ba-b0ef-d2d48774d60c");
                let generate_variable_name_cloned_token_stream = |field_ident: &syn::Ident| format!("table_{test_name}_{field_ident}_cloned").parse::<proc_macro2::TokenStream>().expect("error 2003ad9f-013a-48ba-b0ef-d2d48774d60c");
                table_field_idents_initialization_vec_token_stream.push(generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let initialization_variable_name_token_stream = generate_initialization_variable_name_token_stream(field_ident);
                    let format_content_token_stream = generate_quotes::double_quotes_token_stream(&format!("{test_name}_{field_ident}"));
                    quote::quote! {
                        let #initialization_variable_name_token_stream = add_table_postfix(&#format_content_token_stream);
                    }
                }));
                table_field_idents_clones_vec_token_stream.push(generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let variable_name_cloned_token_stream = generate_variable_name_cloned_token_stream(field_ident);
                    let initialization_variable_name_token_stream = generate_initialization_variable_name_token_stream(field_ident);
                    quote::quote! {
                        let #variable_name_cloned_token_stream = #initialization_variable_name_token_stream.clone();
                    }
                }));
                table_field_idents_clones2_vec_token_stream.push(generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let table_test_name_field_ident_cloned2_token_stream = generate_table_test_name_field_ident_cloned2_token_stream(test_name, field_ident);
                    let initialization_variable_name_token_stream = generate_initialization_variable_name_token_stream(field_ident);
                    quote::quote! {
                        let #table_test_name_field_ident_cloned2_token_stream = #initialization_variable_name_token_stream.clone();
                    }
                }));
                table_field_idents_to_drop_table_if_exists_vec_token_stream.push(generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let initialization_variable_name_token_stream = generate_initialization_variable_name_token_stream(field_ident);
                    quote::quote! {&#initialization_variable_name_token_stream,}
                }));
                table_field_idents_for_prepare_postgresql_table_vec_token_stream.push(generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let variable_name_cloned_token_stream = generate_variable_name_cloned_token_stream(field_ident);
                    quote::quote! {&#variable_name_cloned_token_stream,}
                }));
                table_field_idents_for_routes_handle_vec_token_stream.push(generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let variable_name_cloned_token_stream = generate_variable_name_cloned_token_stream(field_ident);
                    quote::quote! {&#variable_name_cloned_token_stream,}
                }));
            }
        };
        let table_read_only_ids_merged_with_create_into_where_equal_name = "8e427ad7_5231_4f1e_8579_2e1aaa5da988";
        let table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name = "eb24448c_fa63_4259_bb05_3215802a78f6";
        let table_read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_name = "9ac6d79a_2673_4c07_be4a_01c5c20ff1ab";
        let table_create_into_postgresql_type_option_vec_where_dimension_one_equal_name = "72940b0e_cd26_493f_9ec1_2d999d9a4401";
        let table_read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_name = "5a52af33_a590_403b_808e_961df6d7e7aa";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_name = "1f388ef8_dc28_489d_bed9_ca4e7f640dd5";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_name = "581c947f_9b0f_452f_8e52_524088bbb2e7";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_name = "de556c26_9297_4adb_9483_22d474cf1e7d";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_name = "35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d";
        let table_create_into_postgresql_json_type_option_vec_where_length_equal_name = "1ce53b67_1e94_413e_83cf_c6d7094289a8";
        // let table_create_into_postgresql_json_type_option_vec_where_length_greater_than_name = "6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c";

        fill_table_field_idents_vec_token_stream(vec![
            &table_read_only_ids_merged_with_create_into_where_equal_name,
            &table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name,
            &table_read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_name,
            &table_create_into_postgresql_type_option_vec_where_dimension_one_equal_name,
            &table_read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_name,
            &table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_name,
            &table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_name,
            &table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_name,
            &table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_name,
            &table_create_into_postgresql_json_type_option_vec_where_length_equal_name,
            // &table_create_into_postgresql_json_type_option_vec_where_length_greater_than_name,
        ]);
        let select_default_all_with_max_page_size_cloned_clone_token_stream = quote::quote!{select_default_all_with_max_page_size_cloned.clone()};
        //todo maybe remove it?\
        let generate_some_postgresql_type_where_try_new_and_token_stream = |content_token_stream: &dyn quote::ToTokens|{
            quote::quote!{
                Some(
                    #import_path::PostgresqlTypeWhere::try_new(
                        #import_path::LogicalOperator::And,
                        #content_token_stream
                    ).expect("error 6b0491b2-1555-4f1c-81f7-5b22d7d353fb"),
                )
            }
        };
        let generate_ident_where_many_pripery_key_others_none_content_token_stream = quote::quote!{
            fn generate_ident_where_many_pripery_key_others_none(
                option_postgresql_type_where: Option<#import_path::PostgresqlTypeWhere<#primary_key_field_type_as_postgresql_type_where_token_stream>>,
            ) -> #ident_where_many_upper_camel_case {
                #ident_where_many_upper_camel_case::try_new(
                    option_postgresql_type_where,
                    #fields_named_without_primary_key_with_comma_none_token_stream
                )
                .expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5")
            }
        };
        let generate_some_postgresql_type_where_try_new_primary_key_content_token_stream = quote::quote!{
            fn generate_some_postgresql_type_where_try_new_primary_key(
                logical_operator: #import_path::LogicalOperator,
                vec: Vec<#primary_key_field_type_where_token_stream>
            ) -> Option<#import_path::PostgresqlTypeWhere<#primary_key_field_type_as_postgresql_type_where_token_stream>> {
                Some(
                    #import_path::PostgresqlTypeWhere::try_new(
                        logical_operator,
                        vec
                    ).expect("error dbfe049c-4142-469f-907c-4ecc5dd132dc")
                )
            }
        };
        let generate_some_postgresql_type_where_try_new_or_primary_keys_content_token_stream = quote::quote!{
            fn generate_some_postgresql_type_where_try_new_or_primary_keys(
                vec_read_only_ids: &[#ident_read_only_ids_upper_camel_case]
            ) -> Option<#import_path::PostgresqlTypeWhere<#primary_key_field_type_as_postgresql_type_where_token_stream>> {
                generate_some_postgresql_type_where_try_new_primary_key(
                    #import_path::LogicalOperator::Or,
                    vec_read_only_ids.iter().map(|#element_snake_case| #primary_key_field_type_where_token_stream::Equal(#import_path::PostgresqlTypeWhereEqual {
                        logical_operator: #import_path::LogicalOperator::Or,
                        value: #primary_key_field_type_table_type_declaration_token_stream::new(
                            #primary_key_field_type_as_postgresql_type_token_stream into_inner(
                                <#primary_key_field_type as #import_path::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                    #element_snake_case.#primary_key_field_ident.clone()
                                ),
                            )
                        ),
                    })).collect()
                )
            }
        };
        let generate_try_read_many_order_by_primary_key_asc_with_big_pagination_content_token_stream = quote::quote!{
            async fn generate_try_read_many_order_by_primary_key_with_big_pagination(
                endpoint_location: &str,
                current_ident_where_many: #ident_where_many_upper_camel_case,
                select: #import_path::NotEmptyUniqueEnumVec<#ident_select_upper_camel_case>,
                table: &str
            ) -> Result<Vec<#ident_read_upper_camel_case>, #ident_try_read_many_error_named_upper_camel_case> {
                #ident::try_read_many_handle(
                    &endpoint_location,
                    #ident_read_many_parameters_upper_camel_case {
                        payload: #ident_read_many_payload_upper_camel_case {
                            where_many: #std_option_option_ident_where_many_upper_camel_case(Some(
                                current_ident_where_many
                            )),
                            select,
                            order_by: #import_path::OrderBy {
                                column: #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(
                                    #primary_key_field_type_as_postgresql_type_select_token_stream::default()
                                ),
                                order: Some(#import_path::Order::Asc)
                            },
                            pagination: #import_path::PaginationStartsWithZero::try_new(10000, 0).expect("error b0cdf0cb-1e31-4a7e-9e53-d2ff71efb983"),
                        }
                    },
                    &table
                )
                .await
            }
        };
        let create_many_tests_token_stream = {
            let create_many_tests_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_type = &element.syn_field.ty;
                let ident_create_content_token_stream = generate_ident_create_content_element_token_stream(field_ident);
                quote::quote! {{
                    for chunk in <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#option_vec_create_snake_case().unwrap_or(vec![])
                        .chunks(10)
                        .map(Vec::from)
                    {
                        let current_table = current_table.clone();
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                        #acc_snake_case.push(futures::FutureExt::boxed(async move {
                            let ident_vec_create = {
                                let mut #acc_snake_case = vec![];
                                for #element_snake_case in chunk {
                                    #acc_snake_case.push(#ident_create_upper_camel_case {
                                        #ident_create_content_token_stream
                                    });
                                }
                                #acc_snake_case
                            };
                            let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                                &url_cloned,
                                #ident_create_many_parameters_upper_camel_case {
                                    #payload_snake_case: #ident_create_many_payload_upper_camel_case(ident_vec_create.clone())
                                },
                                &current_table.clone()
                            ).await.expect("error 5eecedc4-bb02-454a-acd9-0af758f30b2e");
                            assert_eq!(
                                {
                                    let mut #acc_snake_case = vec![];
                                    assert_eq!(read_only_ids_from_try_create_many.len(), ident_vec_create.len(), "error 39572295-b6a4-49d7-a65a-16f8bcf44ede");
                                    for (read_only_ids, create) in read_only_ids_from_try_create_many.clone().into_iter().zip(ident_vec_create.into_iter()) {
                                        #acc_snake_case.push(#ident_read_upper_camel_case {
                                            #primary_key_field_ident: <#primary_key_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                                &read_only_ids.#primary_key_field_ident
                                            ),
                                            #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_and_create_token_stream
                                        });
                                    }
                                    #acc_snake_case.sort_by(|first, second| {
                                        if let (Some(first), Some(second)) = (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                            first.#value_snake_case.cmp(&second.#value_snake_case)
                                        } else {
                                            panic!("error 4428083a-53be-4184-a5b7-94ae2de21d40");
                                        }
                                    });
                                    #acc_snake_case
                                },
                                generate_try_read_many_order_by_primary_key_with_big_pagination(
                                    &url_cloned,
                                    generate_ident_where_many_pripery_key_others_none(
                                        generate_some_postgresql_type_where_try_new_primary_key(
                                            postgresql_crud::LogicalOperator::Or,
                                            {
                                                let mut #acc_snake_case = vec![];
                                                for #element_snake_case in &read_only_ids_from_try_create_many {
                                                    #acc_snake_case.push(#primary_key_field_type_as_postgresql_type_where_token_stream::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                        logical_operator: postgresql_crud::LogicalOperator::Or,
                                                        //todo must use trait type instead
                                                        #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(<#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                            #primary_key_field_type_read_only_ids_into_read_element_primary_key_field_ident_clone_token_stream
                                                        )),
                                                    }));
                                                }
                                                #acc_snake_case
                                            }
                                        )
                                    ),
                                    #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                    &current_table
                                ).await.expect("error 82cb984b-8312-4952-a649-389f7c5adcff"),
                                "error d19bbbf6-f64c-4151-8b5b-998a93e13af5"
                            );
                            let read_only_ids_from_try_delete_many = {
                                let mut #acc_snake_case = #ident::try_delete_many_handle(
                                    &url_cloned,
                                    #ident_delete_many_parameters_upper_camel_case {
                                        //todo rewrite it using new\try_new?
                                        payload: #ident_delete_many_payload_upper_camel_case {
                                            where_many: #std_option_option_ident_where_many_upper_camel_case(Some(#ident_where_many_upper_camel_case {
                                                #primary_key_field_ident: generate_some_postgresql_type_where_try_new_or_primary_keys(&read_only_ids_from_try_create_many),
                                                #std_option_option_ident_where_many_content_token_stream
                                            }))
                                        }
                                    },
                                    &current_table
                                ).await.expect("error 716e470e-d738-4642-adfc-df1f9b945d27");
                                #acc_snake_case.sort();
                                #acc_snake_case
                            };
                            assert_eq!(
                                read_only_ids_from_try_delete_many,
                                {
                                    let mut #acc_snake_case = read_only_ids_from_try_create_many.into_iter().map(|element|
                                        #primary_key_field_type_read_only_ids_into_read_element_primary_key_field_ident_token_stream
                                    ).collect::<Vec<#primary_key_field_type_as_postgresql_type_read_token_stream>>();
                                    #acc_snake_case.sort();
                                    #acc_snake_case
                                },
                                "error f58f5572-4286-4a74-8006-0507339910d4"
                            );
                            match generate_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                generate_ident_where_many_pripery_key_others_none(
                                    generate_some_postgresql_type_where_try_new_primary_key(
                                        postgresql_crud::LogicalOperator::Or,
                                        {
                                            let mut #acc_snake_case = vec![];
                                            for #element_snake_case in &read_only_ids_from_try_delete_many {
                                                #acc_snake_case.push(#primary_key_field_type_where_token_stream::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                    #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(
                                                        <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(#element_snake_case.clone())
                                                    ),
                                                }));
                                            }
                                            #acc_snake_case
                                        }
                                    )
                                ),
                                #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                &current_table
                            ).await {
                                Ok(#value_snake_case) => assert!(#value_snake_case == Vec::new(), "error 4e88679a-0d23-418f-8767-4e9b7531429c"),
                                Err(#error_snake_case) => panic!("error 24ab86d6-15c9-47f1-a43f-c5fac4b38188 {error:#?}")
                            }
                        }));
                    }
                };}
            });
            quote::quote! {{
                let current_table = table_create_many_cloned2.clone();
                #create_many_tests_token_stream
            };}
        };
        let create_one_tests_token_stream = {
            let create_one_tests_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_type = &element.syn_field.ty;
                let ident_create_content_token_stream = generate_ident_create_content_element_token_stream(field_ident);
                let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_clone_token_stream);
                quote::quote! {{
                    for #element_snake_case in <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#option_vec_create_snake_case().unwrap_or(vec![]) {
                        let current_table = current_table.clone();
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                        #acc_snake_case.push(futures::FutureExt::boxed(async move {
                            let ident_create = #ident_create_upper_camel_case {
                                #ident_create_content_token_stream
                            };
                            let read_only_ids_from_try_create_one = #ident::try_create_one_handle(
                                &url_cloned,
                                #ident_create_one_parameters_upper_camel_case {
                                    payload: ident_create.clone()
                                },
                                &current_table
                            ).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
                            assert_eq!(
                                #ident_read_upper_camel_case {
                                    #primary_key_field_ident: Some(#value_initialization_token_stream),
                                    #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_from_try_create_one_ident_create_token_stream
                                },
                                #ident::try_read_one_handle(
                                    &url_cloned,
                                    #ident_read_one_parameters_upper_camel_case {
                                        #payload_snake_case: #ident_read_one_payload_upper_camel_case {
                                            #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_clone_token_stream,
                                            #select_snake_case: #select_default_all_with_max_page_size_cloned_clone_token_stream
                                        }
                                    },
                                    &current_table
                                )
                                .await
                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                                "error 5f2adbed-f716-440e-a990-4f1c258808b1"
                            );
                            assert_eq!(
                                #ident::try_delete_one_handle(
                                    &url_cloned,
                                    #ident_delete_one_parameters_upper_camel_case {
                                        #payload_snake_case: #ident_delete_one_payload_upper_camel_case {
                                            #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_clone_token_stream
                                        }
                                    },
                                    &current_table
                                ).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30"),
                                #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_clone_token_stream,
                                "error 4f563faf-1d9b-4ef3-8636-f93fde8ef235"
                            );
                            if let Err(#error_snake_case) = #ident::try_read_one_handle(
                                &url_cloned,
                                #ident_read_one_parameters_upper_camel_case {
                                    #payload_snake_case: #ident_read_one_payload_upper_camel_case {
                                        #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_clone_token_stream,
                                        #select_snake_case: select_default_all_with_max_page_size_cloned
                                    }
                                },
                                &current_table
                            )
                            .await {
                                if let #ident_try_read_one_error_named_upper_camel_case::#ident_read_one_error_named_with_serialize_deserialize_upper_camel_case {
                                    read_one_error_named_with_serialize_deserialize,
                                    ..
                                } = #error_snake_case {
                                    if let #ident_read_one_error_named_with_serialize_deserialize_upper_camel_case::Postgresql {
                                        postgresql,
                                        ..
                                    } = read_one_error_named_with_serialize_deserialize {
                                        assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error d7152378-3a59-4050-8710-87b7000c8e3d");
                                    }
                                    else {
                                         panic!("error e1ac93a5-59e6-477e-a99d-c02e99497421");
                                    }
                                }
                                else {
                                    panic!("error bcd3f9bf-d6b7-4594-b078-8fe9c34bcf18")
                                }
                            }
                            else {
                                panic!("error 893263c9-7c62-4551-9225-74153c6e1c57")
                            }
                        }));
                    }
                };}
            });
            quote::quote! {{
                let current_table = table_create_one_cloned2.clone();
                #create_one_tests_token_stream
            };}
        };
        let add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_token_stream = |content_token_stream: &dyn quote::ToTokens| {
            quote::quote! {
                let read_only_ids_from_try_create_one_default = #ident::try_create_one_handle(
                    url,
                    #ident_create_one_parameters_upper_camel_case {
                        payload: ident_create_default.clone()
                    },
                    current_table
                ).await.expect("error 71632985-ec25-4928-aa9e-1e224a7478c1");
                #content_token_stream
                let _: #primary_key_field_type_as_postgresql_type_read_token_stream = #ident::try_delete_one_handle(
                    url,
                    #ident_delete_one_parameters_upper_camel_case {
                        #payload_snake_case: #ident_delete_one_payload_upper_camel_case {
                            #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_default_primary_key_field_ident_clone_token_stream
                        }
                    },
                    current_table
                ).await.expect("error cc3958f0-1a4a-4440-97c7-ca63611405c5");
                if let Err(#error_snake_case) = #ident::try_read_one_handle(
                    url,
                    #ident_read_one_parameters_upper_camel_case {
                        #payload_snake_case: #ident_read_one_payload_upper_camel_case {
                            #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_default_primary_key_field_ident_clone_token_stream,
                            #select_snake_case: #select_default_all_with_max_page_size_clone_token_stream
                        }
                    },
                    current_table
                )
                .await {
                    if let #ident_try_read_one_error_named_upper_camel_case::#ident_read_one_error_named_with_serialize_deserialize_upper_camel_case {
                        read_one_error_named_with_serialize_deserialize,
                        ..
                    } = #error_snake_case {
                        if let #ident_read_one_error_named_with_serialize_deserialize_upper_camel_case::Postgresql {
                            postgresql,
                            ..
                        } = read_one_error_named_with_serialize_deserialize {
                            assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 99bd4d82-4976-4e1e-8022-543b01221a91");
                        }
                        else {
                             panic!("error 5a86690f-80e2-4dbc-9853-1826f94748bd");
                        }
                    }
                    else {
                        panic!("error d90d6d02-33f7-4886-99df-dee76b83400f")
                    }
                }
                else {
                    panic!("error 62e65598-2c1f-4912-b3a2-dccd6e1714a1")
                }
            }
        };
        let read_many_tests_token_stream = {
            //todo additional read_many checks
            let test_read_many_by_non_existent_primary_keys_token_stream = {
                let content_token_stream = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_token_stream(&{
                    quote::quote! {
                        match generate_try_read_many_order_by_primary_key_with_big_pagination(
                            url,
                            generate_ident_where_many_pripery_key_others_none(
                                generate_some_postgresql_type_where_try_new_primary_key(
                                    postgresql_crud::LogicalOperator::Or,
                                    {
                                        let mut #acc_snake_case = vec![];
                                        for _ in 1..=length {
                                            #acc_snake_case.push(#primary_key_field_type_as_postgresql_type_where_token_stream::Equal(
                                                postgresql_crud::PostgresqlTypeWhereEqual {
                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                    #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(
                                                        uuid::Uuid::new_v4()
                                                    )
                                                }
                                            ));
                                        }
                                        #acc_snake_case
                                    }
                                )
                            ),
                            #select_default_all_with_max_page_size_clone_token_stream,
                            current_table
                        ).await {
                            Ok(#value_snake_case) => assert!(#value_snake_case.is_empty(), "error 06df4025-e2d1-4128-b819-c06613c6ae3f"),
                            Err(#error_snake_case) => {
                                panic!("error e661c49b-2288-4548-8783-35495e193976: {error:#?}");
                            },
                        }
                    }
                });
                quote::quote! {{
                    let current_table = table_test_read_many_by_non_existent_primary_keys_cloned2.clone();
                    async fn generate_test_read_many_by_non_existent_primary_keys(
                        length: usize,
                        url: &str,
                        select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueEnumVec<#ident_select_upper_camel_case>,
                        current_table: &str,
                        ident_create_default: #ident_create_upper_camel_case,
                        no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row: &str,
                    ){
                        #content_token_stream
                    }
                    let lengths = vec![1,2];
                    for element in lengths {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                        let current_table = current_table.clone();
                        let ident_create_default_cloned = ident_create_default.clone();
                        #acc_snake_case.push(futures::FutureExt::boxed(async move {
                            generate_test_read_many_by_non_existent_primary_keys(
                                element,
                                &url_cloned,
                                select_default_all_with_max_page_size_cloned,
                                &current_table,
                                ident_create_default_cloned,
                                no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row,
                            ).await;
                        }));
                    }
                };}
            };
            let test_read_many_by_equal_to_created_primary_keys_token_stream = {
                let content_token_stream = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_token_stream(&{
                    quote::quote! {
                        let ident_vec_create = {
                            let mut #acc_snake_case = vec![];
                            for _ in 1..=length {
                                #acc_snake_case.push(ident_create_default.clone());
                            }
                            #acc_snake_case
                        };
                        let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                            url,
                            #ident_create_many_parameters_upper_camel_case {
                                payload: #ident_create_many_payload_upper_camel_case(ident_vec_create.clone())
                            },
                            current_table
                        ).await.expect("error d775179f-f7b1-41d3-9c83-4ca8bd1abeec");
                        assert_eq!(
                            {
                                let mut #acc_snake_case = vec![];
                                assert_eq!(read_only_ids_from_try_create_many.len(), ident_vec_create.len(), "error 52c9d1ea-1593-4b32-97d1-0ed4a529a74a");
                                for (read_only_ids, create) in read_only_ids_from_try_create_many.clone()
                                    .into_iter()
                                    .zip(ident_vec_create.into_iter()) {
                                    #acc_snake_case.push(#ident_read_upper_camel_case {
                                        #primary_key_field_ident: <
                                            #primary_key_field_type
                                            as
                                            postgresql_crud::PostgresqlTypeTestCases
                                        >::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                            &read_only_ids.#primary_key_field_ident
                                        ),
                                        #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_and_create_token_stream
                                    });
                                }
                                #acc_snake_case.sort_by(|first, second| {
                                    if let (Some(first), Some(second)) = (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                        first.#value_snake_case.cmp(&second.#value_snake_case)
                                    } else {
                                        panic!("error 0faa6fb3-a7c0-44ca-9b51-13f6ca2fc543");
                                    }
                                });
                                #acc_snake_case
                            },
                            generate_try_read_many_order_by_primary_key_with_big_pagination(
                                url,
                                generate_ident_where_many_pripery_key_others_none(
                                    generate_some_postgresql_type_where_try_new_primary_key(
                                        postgresql_crud::LogicalOperator::Or,
                                        {
                                            let mut #acc_snake_case = vec![];
                                            for #element_snake_case in &read_only_ids_from_try_create_many {
                                                #acc_snake_case.push(#primary_key_field_type_where_token_stream::Equal(
                                                    postgresql_crud::PostgresqlTypeWhereEqual {
                                                        logical_operator: postgresql_crud::LogicalOperator::Or,
                                                        #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(
                                                            <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                                #primary_key_field_type_read_only_ids_into_read_element_primary_key_field_ident_clone_token_stream
                                                            )
                                                        )
                                                    }
                                                ));
                                            }
                                            #acc_snake_case
                                        }
                                    )
                                ),
                                #select_default_all_with_max_page_size_clone_token_stream,
                                &current_table
                            ).await.expect("error 82cb984b-8312-4952-a649-389f7c5adcff"),
                            "error 3b2cf1f5-2c4e-4908-ba66-f4af84fe0893"
                        );
                        let read_only_ids_from_try_delete_many = {
                            let mut #acc_snake_case = #ident::try_delete_many_handle(
                                url,
                                #ident_delete_many_parameters_upper_camel_case {
                                    payload: #ident_delete_many_payload_upper_camel_case {
                                        where_many: #std_option_option_ident_where_many_upper_camel_case(Some(#ident_where_many_upper_camel_case {
                                            #primary_key_field_ident: generate_some_postgresql_type_where_try_new_or_primary_keys(&read_only_ids_from_try_create_many),
                                            #std_option_option_ident_where_many_content_token_stream
                                        })),
                                    },
                                },
                                current_table
                            )
                            .await
                            .expect("error d5c23a9d-eb02-44e4-8654-e2a3d7752f51");
                            #acc_snake_case.sort();
                            #acc_snake_case
                        };
                        assert_eq!(
                            read_only_ids_from_try_delete_many,
                            {
                                let mut #acc_snake_case = read_only_ids_from_try_create_many.into_iter().map(|#element_snake_case|
                                    #primary_key_field_type_read_only_ids_into_read_element_primary_key_field_ident_token_stream
                                ).collect::<Vec<<#primary_key_field_type as postgresql_crud::PostgresqlType>::Read>>();
                                #acc_snake_case.sort();
                                #acc_snake_case
                            },
                            "error ebbbea6e-c402-4637-9bab-02678c11926c"
                        );
                        match generate_try_read_many_order_by_primary_key_with_big_pagination(
                            url,
                            generate_ident_where_many_pripery_key_others_none(
                                generate_some_postgresql_type_where_try_new_primary_key(
                                    postgresql_crud::LogicalOperator::Or,
                                    {
                                        let mut #acc_snake_case = vec![];
                                        for element in &read_only_ids_from_try_delete_many {
                                            #acc_snake_case.push(#primary_key_field_type_where_token_stream::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(
                                                    <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                        #element_snake_case.clone()
                                                    )
                                                )
                                            }));
                                        }
                                        #acc_snake_case
                                    }
                                )
                            ),
                            #select_default_all_with_max_page_size_clone_token_stream,
                            current_table
                        ).await {
                            Ok(#value_snake_case) => assert!(#value_snake_case == Vec::new(), "error d79c0af3-5e2e-4891-a7ff-d1007b573e77"),
                            Err(#error_snake_case) => {
                                panic!("error 1f079962-06af-4d21-a837-c88b0e7db265 {error:#?}");
                            }
                        }
                    }
                });
                quote::quote! {{
                    let current_table = table_test_read_many_by_equal_to_created_primary_keys_cloned2.clone();
                    async fn generate_test_read_many_by_equal_to_created_primary_keys(
                        length: usize,
                        url: &str,
                        select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueEnumVec<#ident_select_upper_camel_case>,
                        current_table: &str,
                        ident_create_default: #ident_create_upper_camel_case,
                        no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row: &str,
                    ) {
                        #content_token_stream
                    }
                    let lengths = vec![1,2];
                    for element in lengths {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                        let current_table = current_table.clone();
                        let ident_create_default_cloned = ident_create_default.clone();
                        #acc_snake_case.push(futures::FutureExt::boxed(async move {
                            generate_test_read_many_by_equal_to_created_primary_keys(
                                element,
                                &url_cloned,
                                select_default_all_with_max_page_size_cloned,
                                &current_table,
                                ident_create_default_cloned,
                                no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row,
                            ).await;
                        }));
                    }
                };}
            };
            let generate_read_only_ids_merged_with_create_into_where_assert_eq_token_stream = |ident_where_many_try_new_parameters_content_token_stream: &dyn quote::ToTokens| {
                quote::quote! {
                    assert_eq!(
                        vec![
                            #ident_read_upper_camel_case {
                                #primary_key_field_ident: <
                                    #primary_key_field_type
                                    as
                                    postgresql_crud::PostgresqlTypeTestCases
                                >::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                    &read_only_ids_returned_from_create_one.#primary_key_field_ident.clone()
                                ),
                                #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_clone_ident_create_clone_token_stream
                            }
                        ],
                        generate_try_read_many_order_by_primary_key_with_big_pagination(
                            &url_cloned,
                            #ident_where_many_upper_camel_case::try_new(#ident_where_many_try_new_parameters_content_token_stream).expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5"),
                            #select_default_all_with_max_page_size_cloned_clone_token_stream,
                            &current_table
                        ).await.expect("error 82cb984b-8312-4952-a649-389f7c5adcff"),
                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                    );
                }
            };
            let generate_option_vec_create_call_unwrap_or_vec_token_stream = |_: &syn::Ident, field_type: &syn::Type| {
                quote::quote! {
                    <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#option_vec_create_snake_case().unwrap_or(vec![])
                }
            };
            let generate_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_token_stream = |field_ident: &syn::Ident, field_type: &syn::Type| {
                quote::quote! {
                    {
                        let mut #acc_snake_case = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#option_vec_create_snake_case().unwrap_or(vec![]);
                        if #acc_snake_case.is_empty() {
                            #acc_snake_case.push(ident_create_default.#field_ident.clone());
                        }
                        #acc_snake_case
                    }
                }
            };
            let generate_postgresql_type_option_vec_where_greater_than_test_unwrap_or_else_vec_call_token_stream = |_: &syn::Ident, field_type: &syn::Type| {
                quote::quote! {
                    <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#postgresql_type_option_vec_where_greater_than_test_snake_case().unwrap_or(vec![])
                }
            };
            let generate_read_test_token_stream = |test_name: &str, generate_method_call_token_stream: &dyn Fn(&syn::Ident, &syn::Type) -> proc_macro2::TokenStream, generate_create_content_token_stream: &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream, generate_content_token_stream: &dyn Fn(&SynFieldWrapper) -> proc_macro2::TokenStream| {
                generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.syn_field.ty;
                    let method_call_token_stream = generate_method_call_token_stream(field_ident, field_type);
                    let table_test_name_field_ident_cloned2_token_stream = generate_table_test_name_field_ident_cloned2_token_stream(test_name, field_ident);
                    let ident_create_content_token_stream = generate_create_content_token_stream(field_ident);
                    let content_token_stream = generate_content_token_stream(element);
                    quote::quote! {{
                        let current_table = #table_test_name_field_ident_cloned2_token_stream.clone();
                        for #element_snake_case in #method_call_token_stream {
                            let current_table = current_table.clone();
                            let url_cloned = url.clone();
                            let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                            #acc_snake_case.push(futures::FutureExt::boxed(async move {
                                let ident_create = #ident_create_upper_camel_case {
                                    #ident_create_content_token_stream
                                };
                                let read_only_ids_returned_from_create_one = #ident::try_create_one_handle(
                                    &url_cloned,
                                    #ident_create_one_parameters_upper_camel_case {
                                        payload: ident_create.clone()
                                    },
                                    &current_table
                                ).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                #content_token_stream
                                let read_only_ids_from_try_delete_many = {
                                    let mut #acc_snake_case = #ident::try_delete_many_handle(
                                        &url_cloned,
                                        #ident_delete_many_parameters_upper_camel_case {
                                            payload: #ident_delete_many_payload_upper_camel_case {
                                                where_many: #std_option_option_ident_where_many_upper_camel_case(Some(#ident_where_many_upper_camel_case {
                                                    #primary_key_field_ident: generate_some_postgresql_type_where_try_new_primary_key(
                                                        postgresql_crud::LogicalOperator::Or,
                                                        vec![
                                                            #primary_key_field_type_where_token_stream::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(
                                                                    <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                                        #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream
                                                                    )
                                                                )
                                                            })
                                                        ]
                                                    ),
                                                    #std_option_option_ident_where_many_content_token_stream
                                                })),
                                            },
                                        },
                                        &current_table
                                    )
                                    .await
                                    .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                    #acc_snake_case.sort();
                                    #acc_snake_case
                                };
                                assert_eq!(
                                    read_only_ids_from_try_delete_many,
                                    vec![#primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream],
                                    "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9"
                                );
                                match generate_try_read_many_order_by_primary_key_with_big_pagination(
                                    &url_cloned,
                                    generate_ident_where_many_pripery_key_others_none(
                                        generate_some_postgresql_type_where_try_new_primary_key(
                                            postgresql_crud::LogicalOperator::Or,
                                            vec![
                                                #primary_key_field_type_where_token_stream::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                    #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(
                                                        <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                            #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream
                                                        )
                                                    )
                                                })
                                            ]
                                        )
                                    ),
                                    #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                    &current_table
                                ).await {
                                    Ok(#value_snake_case) => assert!(#value_snake_case == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                    Err(#error_snake_case) => {
                                        panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                    }
                                }
                            }));
                        }
                    };}
                })
            };
            let some_primary_key_where_initialization_token_stream = quote::quote!{
                generate_some_postgresql_type_where_try_new_primary_key(
                    postgresql_crud::LogicalOperator::And,
                    vec![
                        <#primary_key_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                            read_only_ids_returned_from_create_one.#primary_key_field_ident.clone(),
                            #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                        )
                    ]
                )
            };
            let (read_only_ids_merged_with_create_into_where_equal_token_stream, read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream) = {
                let generate_test_read_many_by_equal_one_column_value_token_stream = |test_name: &str, equal_or_equal_using_fields: &postgresql_crud_macros_common::EqualOrEqualUsingFields| {
                    generate_read_test_token_stream(test_name, &generate_option_vec_create_call_unwrap_or_vec_token_stream, &generate_ident_create_content_element_token_stream, &|element: &SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        generate_read_only_ids_merged_with_create_into_where_assert_eq_token_stream(&generate_fields_named_with_comma_token_stream(&|current_element: &SynFieldWrapper| {
                            let current_field_ident = &current_element.field_ident;
                            let current_field_type = &current_element.syn_field.ty;
                            if current_field_ident == primary_key_field_ident {
                                some_primary_key_where_initialization_token_stream.clone()
                            } else if current_field_ident == field_ident {
                                generate_some_postgresql_type_where_try_new_and_token_stream(&{
                                    let generate_token_stream = |method_token_stream: &dyn quote::ToTokens| {
                                        quote::quote! {
                                            <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::#method_token_stream(
                                                read_only_ids_returned_from_create_one.#current_field_ident.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"),
                                                ident_create.#current_field_ident.clone()
                                            )
                                        }
                                    };
                                    match &equal_or_equal_using_fields {
                                        postgresql_crud_macros_common::EqualOrEqualUsingFields::Equal => {
                                            let content_token_stream = generate_token_stream(&read_only_ids_merged_with_create_into_where_equal_snake_case);
                                            quote::quote! {vec![#content_token_stream]}
                                        }
                                        postgresql_crud_macros_common::EqualOrEqualUsingFields::EqualUsingFields => generate_token_stream(&read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case),
                                    }
                                })
                            } else {
                                none_token_stream.clone()
                            }
                        }))
                    })
                };
                (
                    generate_test_read_many_by_equal_one_column_value_token_stream(table_read_only_ids_merged_with_create_into_where_equal_name, &postgresql_crud_macros_common::EqualOrEqualUsingFields::Equal),
                    generate_test_read_many_by_equal_one_column_value_token_stream(table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name, &postgresql_crud_macros_common::EqualOrEqualUsingFields::EqualUsingFields),
                )
            };
            let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_token_stream = generate_read_test_token_stream(table_read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_name, &generate_option_vec_create_call_unwrap_or_vec_token_stream, &generate_ident_create_content_element_token_stream, &|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_type = &element.syn_field.ty;
                let assert_eq_token_stream = generate_read_only_ids_merged_with_create_into_where_assert_eq_token_stream(&generate_fields_named_with_comma_token_stream(&|current_element: &SynFieldWrapper| {
                    let current_field_ident = &current_element.field_ident;
                    if current_field_ident == primary_key_field_ident {
                        some_primary_key_where_initialization_token_stream.clone()
                    } else if current_field_ident == field_ident {
                        generate_some_postgresql_type_where_try_new_and_token_stream(&quote::quote! {vec![#element_snake_case]})
                    } else {
                        none_token_stream.clone()
                    }
                }));
                quote::quote! {
                    if let Some(#value_snake_case) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_snake_case(
                        read_only_ids_returned_from_create_one.#field_ident.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"),
                        ident_create.#field_ident.clone()
                    ) {
                        for #element_snake_case in #value_snake_case {
                            #assert_eq_token_stream
                        }
                    }
                }
            });
            let create_into_postgresql_type_option_vec_where_dimension_one_equal_token_stream = generate_read_test_token_stream(table_create_into_postgresql_type_option_vec_where_dimension_one_equal_name, &generate_option_vec_create_call_unwrap_or_vec_token_stream, &generate_ident_create_content_element_token_stream, &|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_type = &element.syn_field.ty;
                let assert_eq_token_stream = generate_read_only_ids_merged_with_create_into_where_assert_eq_token_stream(&generate_fields_named_with_comma_token_stream(&|current_element: &SynFieldWrapper| {
                    let current_field_ident = &current_element.field_ident;
                    if primary_key_field_ident == current_field_ident {
                        some_primary_key_where_initialization_token_stream.clone()
                    } else if current_field_ident == field_ident {
                        generate_some_postgresql_type_where_try_new_and_token_stream(&quote::quote! {vec![#element_snake_case]})
                    } else {
                        none_token_stream.clone()
                    }
                }));
                quote::quote! {
                    if let Some(#value_snake_case) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#create_into_postgresql_type_option_vec_where_dimension_one_equal_snake_case(
                        ident_create.#field_ident.clone()
                    ) {
                        for #element_snake_case in #value_snake_case {
                            #assert_eq_token_stream
                        }
                    }
                }
            });
            let read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_token_stream = generate_read_test_token_stream(
                table_read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_name,
                &generate_postgresql_type_option_vec_where_greater_than_test_unwrap_or_else_vec_call_token_stream,
                &generate_ident_create_content_element_create_token_stream,
                &|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.syn_field.ty;
                    let assert_eq_token_stream = generate_read_only_ids_merged_with_create_into_where_assert_eq_token_stream(&generate_fields_named_with_comma_token_stream(&|current_element: &SynFieldWrapper| {
                        let current_field_ident = &current_element.field_ident;
                        if current_field_ident == primary_key_field_ident {
                            some_primary_key_where_initialization_token_stream.clone()
                        } else if current_field_ident == field_ident {
                            generate_some_postgresql_type_where_try_new_and_token_stream(&quote::quote! {vec![#value_snake_case]})
                        } else {
                            none_token_stream.clone()
                        }
                    }));
                    quote::quote! {
                        if let Some(#value_snake_case) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_snake_case(
                            #element_snake_case.variant,
                            read_only_ids_returned_from_create_one.#field_ident.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"),
                            #element_snake_case.greater_than,
                        ) {
                            #assert_eq_token_stream
                        }
                    }
                },
            );
            let (
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream,
            ) = {
                //todo if vec_create is empty then do different logic (for uuid). now uuid tested using one default case
                let generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_token_stream = |test_name: &str, dimension: &postgresql_crud_macros_common::Dimension| {
                    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case = dimension.read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case();
                    generate_read_test_token_stream(test_name, &generate_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_token_stream, &generate_ident_create_content_element_token_stream, &|element: &SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let field_type = &element.syn_field.ty;
                        let assert_eq_token_stream = generate_read_only_ids_merged_with_create_into_where_assert_eq_token_stream(&generate_fields_named_with_comma_token_stream(&|current_element: &SynFieldWrapper| {
                            let current_field_ident = &current_element.field_ident;
                            if current_field_ident == primary_key_field_ident {
                                some_primary_key_where_initialization_token_stream.clone()
                            } else if current_field_ident == field_ident {
                                generate_some_postgresql_type_where_try_new_and_token_stream(&quote::quote! {vec![#element_snake_case]})
                            } else {
                                none_token_stream.clone()
                            }
                        }));
                        quote::quote! {
                            if let Some(#value_snake_case) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case(
                                read_only_ids_returned_from_create_one.#field_ident.clone().expect("error 2ed000a5-cf70-4df1-903a-c1f6d224e926"),
                                ident_create.#field_ident.clone()
                            ) {
                                for #element_snake_case in #value_snake_case {
                                    #assert_eq_token_stream
                                }
                            }
                        }
                    })
                };
                (
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_token_stream(table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_name, &postgresql_crud_macros_common::Dimension::One),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_token_stream(table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_name, &postgresql_crud_macros_common::Dimension::Two),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_token_stream(table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_name, &postgresql_crud_macros_common::Dimension::Three),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_token_stream(table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_name, &postgresql_crud_macros_common::Dimension::Four),
                )
            };
            let create_into_postgresql_json_type_option_vec_where_length_equal_token_stream = generate_read_test_token_stream(
                table_create_into_postgresql_json_type_option_vec_where_length_equal_name,
                &generate_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_token_stream,
                &generate_ident_create_content_element_token_stream,
                &|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.syn_field.ty;
                    let assert_eq_token_stream = generate_read_only_ids_merged_with_create_into_where_assert_eq_token_stream(&generate_fields_named_with_comma_token_stream(&|current_element: &SynFieldWrapper| {
                        let current_field_ident = &current_element.field_ident;
                        if current_field_ident == primary_key_field_ident {
                            some_primary_key_where_initialization_token_stream.clone()
                        } else if current_field_ident == field_ident {
                            generate_some_postgresql_type_where_try_new_and_token_stream(&quote::quote! {vec![#element_snake_case]})
                        } else {
                            none_token_stream.clone()
                        }
                    }));
                    quote::quote! {
                        if let Some(#value_snake_case) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#create_into_postgresql_json_type_option_vec_where_length_equal_snake_case(
                            ident_create.#field_ident.clone()
                        ) {
                            for #element_snake_case in #value_snake_case {
                                #assert_eq_token_stream
                            }
                        }
                    }
                },
            );
            // let _create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream = generate_read_test_token_stream(
            //     table_create_into_postgresql_json_type_option_vec_where_length_greater_than_name,
            //     &generate_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_token_stream,
            //     &generate_ident_create_content_element_token_stream,
            //     &|element: &SynFieldWrapper|{
            //         let field_ident = &element.field_ident;
            //         let field_type = &element.syn_field.ty;
            //         let assert_eq_token_stream = generate_read_only_ids_merged_with_create_into_where_assert_eq_token_stream(
            //             &generate_fields_named_with_comma_token_stream(&|element: &SynFieldWrapper|{
            //                 let current_field_ident = &element.field_ident;
            //                 if current_field_ident == primary_key_field_ident {
            //                     some_primary_key_where_initialization_token_stream.clone()
            //                 }
            //                 else if current_field_ident == field_ident {
            //                     generate_some_postgresql_type_where_try_new_and_token_stream(&quote::quote!{vec![#element_snake_case]})
            //                 } else {
            //                     none_token_stream.clone()
            //                 }
            //             })
            //         );
            //         quote::quote!{
            //             if let Some(#value_snake_case) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#create_into_postgresql_json_type_option_vec_where_length_greater_than_snake_case(
            //                 ident_create.#field_ident.clone()
            //             ) {
            //                 for #element_snake_case in #value_snake_case {
            //                     #assert_eq_token_stream
            //                 }
            //             }
            //         }
            //     }
            // );
            quote::quote! {{
                #test_read_many_by_non_existent_primary_keys_token_stream
                #test_read_many_by_equal_to_created_primary_keys_token_stream
                #read_only_ids_merged_with_create_into_where_equal_token_stream
                #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_token_stream
                #read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_token_stream
                #create_into_postgresql_type_option_vec_where_dimension_one_equal_token_stream
                #read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_token_stream
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_token_stream
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_token_stream
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_token_stream
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_token_stream
                #create_into_postgresql_json_type_option_vec_where_length_equal_token_stream
                // #create_into_postgresql_json_type_option_vec_where_length_greater_than_token_stream
            };}
        };
        let read_one_tests_token_stream = quote::quote! {{
            let current_table = table_read_one_cloned2.clone();
            let url_cloned = url.clone();
            let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
            #acc_snake_case.push(futures::FutureExt::boxed(async move {
                if let Err(#error_snake_case) = #ident::try_read_one_handle(
                    &url_cloned,
                    #ident_read_one_parameters_upper_camel_case {
                        payload: #ident_read_one_payload_upper_camel_case {
                            #primary_key_field_ident: #primary_key_field_type_as_postgresql_type_read_token_stream::new(uuid::Uuid::new_v4()),
                            select: #select_default_all_with_max_page_size_cloned_clone_token_stream
                        }
                    },
                    &current_table
                ).await {
                    if let #ident_try_read_one_error_named_upper_camel_case::#ident_read_one_error_named_with_serialize_deserialize_upper_camel_case {
                        read_one_error_named_with_serialize_deserialize,
                        ..
                    } = #error_snake_case {
                        if let #ident_read_one_error_named_with_serialize_deserialize_upper_camel_case::Postgresql {
                            postgresql,
                            ..
                        } = read_one_error_named_with_serialize_deserialize {
                            assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 10010cca-57ec-4620-8ddf-4a3227999b06");
                        } else {
                            panic!("error c77029fe-1f95-4df5-a5fb-ef663d7bc08d");
                        }
                    } else {
                        panic!("error 8031870d-aea7-44ef-a91b-1b1ea068e5dd")
                    }
                } else {
                    panic!("error 9153abfc-f12f-45dd-8d64-52147577f8dd")
                }
            }));
        };};
        let update_many_tests_token_stream = {
            //todo add test for trying to update empty vec
            let update_many_only_one_column_tests_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|current_element: &SynFieldWrapper| {
                let field_ident = &current_element.field_ident;
                let field_type = &current_element.syn_field.ty;
                let warning_message_double_quote_token_stream = generate_quotes::double_quotes_token_stream(&format!("PostgresqlTypeTestCases read_only_ids_to_two_dimensional_vec_read_inner is empty for {field_ident}"));
                let is_fields_without_primary_key_len_greater_than_one = fields_without_primary_key.len() > 1;
                let maybe_previous_read_token_stream = if is_fields_without_primary_key_len_greater_than_one {
                    quote::quote! {
                        let previous_read = {
                            let mut #acc_snake_case = generate_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                generate_ident_where_many_pripery_key_others_none(
                                    generate_some_postgresql_type_where_try_new_primary_key(
                                        postgresql_crud::LogicalOperator::Or,
                                        vec![
                                            #primary_key_field_type_as_postgresql_type_where_token_stream::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                value: #primary_key_field_type_table_type_declaration_token_stream::new(
                                                    #primary_key_field_type_as_postgresql_type_token_stream into_inner(
                                                        #primary_key_field_type_read_only_is_into_read_read_only_ids_current_element_primary_key_field_ident_clone_token_stream
                                                    )
                                                )
                                            })
                                        ]
                                    )
                                ),
                                #select_default_all_with_max_page_size_cloned_clone_token_stream,
                                &current_table
                            ).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                            #acc_snake_case.sort_by(|first, second| {
                                if let (Some(value_first), Some(value_second)) = (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                    value_first.#value_snake_case.cmp(&value_second.#value_snake_case)
                                } else {
                                    panic!("must not be what");
                                }
                            });
                            #acc_snake_case
                        };
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let ident_create_defaults_for_column_read_only_ids_to_two_dimensional_vec_read_inner_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|current_current_element: &SynFieldWrapper| {
                    let current_field_ident = &current_current_element.field_ident;
                    let current_field_type = &current_current_element.syn_field.ty;
                    if field_ident == current_field_ident {
                        quote::quote! {
                            if let Some(#value_snake_case) = &common_read_only_ids_returned_from_create_one.#current_field_ident {
                                for #element_snake_case in <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(#value_snake_case) {
                                    for _ in #element_snake_case {
                                        #acc_snake_case.push(ident_create_default.clone());
                                    }
                                }
                            }
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    }
                });
                let ident_read_fields_initialization_without_primary_key_after_create_many_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|syn_field_wrapper: &SynFieldWrapper| {
                    let current_field_ident = &syn_field_wrapper.field_ident;
                    let current_field_type = &syn_field_wrapper.syn_field.ty;
                    let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream);
                    quote::quote! {
                        #current_field_ident: match &#element_snake_case.#current_field_ident {
                            Some(#value_snake_case) => <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(#value_snake_case),
                            None => Some(#value_initialization_token_stream),
                        }
                    }
                });
                let ident_read_only_ids_upper_fields_initialization_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|syn_field_wrapper: &SynFieldWrapper| {
                    let current_field_ident = &syn_field_wrapper.field_ident;
                    let current_field_type = &syn_field_wrapper.syn_field.ty;
                    if field_ident == current_field_ident {
                        quote::quote! {#current_field_ident: Some(<#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update))}
                    } else {
                        quote::quote! {#current_field_ident: None}
                    }
                });
                let ident_update_parameters_initialization_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|syn_field_wrapper: &SynFieldWrapper| {
                    let current_field_ident = &syn_field_wrapper.field_ident;
                    if field_ident == current_field_ident {
                        let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&quote::quote! {
                            #update_snake_case.clone()
                        });
                        quote::quote! {Some(#value_initialization_token_stream)}
                    } else {
                        none_token_stream.clone()
                    }
                });
                let ident_read_fields_initialization_without_primary_key_after_update_one_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|syn_field_wrapper: &SynFieldWrapper| {
                    let current_field_ident = &syn_field_wrapper.field_ident;
                    if field_ident == current_field_ident {
                        let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&{
                            let current_field_type = &syn_field_wrapper.syn_field.ty;
                            quote::quote! {
                                <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::previous_read_merged_with_option_update_into_read(
                                    <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                        &read_only_ids_current_element.#current_field_ident.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129")
                                    ).expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5").#value_snake_case,
                                    Some(#update_snake_case.clone())
                                )
                            }
                        });
                        quote::quote! {
                            #current_field_ident: Some(#value_initialization_token_stream)
                        }
                    } else {
                        quote::quote! {
                            #current_field_ident: #element_snake_case.#current_field_ident
                        }
                    }
                });
                let expected_read_many_token_stream = if is_fields_without_primary_key_len_greater_than_one {
                    let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&primary_key_field_type_read_only_is_into_read_read_only_ids_current_element_primary_key_field_ident_clone_token_stream);
                    quote::quote! {
                        let mut #acc_snake_case = vec![];
                        for #element_snake_case in previous_read {
                            #acc_snake_case.push(#ident_read_upper_camel_case {
                                #primary_key_field_ident: Some(#value_initialization_token_stream),
                                #ident_read_fields_initialization_without_primary_key_after_update_one_token_stream
                            });
                        }
                        #acc_snake_case
                    }
                } else {
                    let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&primary_key_field_type_read_only_is_into_read_read_only_ids_current_element_primary_key_field_ident_clone_token_stream);
                    quote::quote! {
                        vec![
                            #ident_read_upper_camel_case {
                                #primary_key_field_ident: Some(#value_initialization_token_stream),
                                #ident_read_fields_initialization_without_primary_key_after_update_one_token_stream
                            }
                        ]
                    }
                };
                quote::quote! {{
                    let read_only_ids_to_two_dimensional_vec_read_inner_acc = {
                        let mut #acc_snake_case = vec![];
                        #ident_create_defaults_for_column_read_only_ids_to_two_dimensional_vec_read_inner_token_stream
                        #acc_snake_case
                    };
                    if read_only_ids_to_two_dimensional_vec_read_inner_acc.is_empty() {
                        println!(#warning_message_double_quote_token_stream);
                    } else {
                        let current_table = current_table.clone();
                        let read_only_ids_current_elements = {
                            futures::StreamExt::collect::<Vec<Vec<#ident_read_only_ids_upper_camel_case>>>(
                                futures::StreamExt::buffer_unordered(
                                    futures::stream::iter(
                                        read_only_ids_to_two_dimensional_vec_read_inner_acc
                                        .chunks(25)
                                        .map(Vec::from)
                                        .map(|#element_snake_case| {
                                            let current_table = current_table.clone();
                                            let url_cloned = url.clone();
                                            futures::FutureExt::boxed(async move { #ident::try_create_many_handle(
                                                &url_cloned,
                                                #ident_create_many_parameters_upper_camel_case {
                                                    payload: #ident_create_many_payload_upper_camel_case(#element_snake_case)
                                                },
                                                &current_table
                                            ).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
                                        })
                                    ),
                                    5
                                )
                            )
                            .await
                            .into_iter()
                            .flatten()
                            .collect::<Vec<#ident_read_only_ids_upper_camel_case>>()
                        };
                        assert_eq!(
                            {
                                let mut #acc_snake_case = vec![];
                                for #element_snake_case in &read_only_ids_current_elements {
                                    #acc_snake_case.push(#ident_read_upper_camel_case {
                                        #primary_key_field_ident: <
                                            #primary_key_field_type
                                            as
                                            postgresql_crud::PostgresqlTypeTestCases
                                        >::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                            &#element_snake_case.#primary_key_field_ident
                                        ),
                                        #ident_read_fields_initialization_without_primary_key_after_create_many_token_stream
                                    });
                                }
                                #acc_snake_case.sort_by(|first, second| {
                                    if let (Some(value_first), Some(value_second)) = (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                        value_first.#value_snake_case.cmp(&value_second.#value_snake_case)
                                    }
                                    else {
                                        panic!("must not be what");
                                    }
                                });
                                #acc_snake_case
                            },
                            {
                                let mut #acc_snake_case = generate_try_read_many_order_by_primary_key_with_big_pagination(
                                    &url,
                                    generate_ident_where_many_pripery_key_others_none(
                                        generate_some_postgresql_type_where_try_new_primary_key(
                                            postgresql_crud::LogicalOperator::Or,
                                            {
                                                let mut #acc_snake_case = vec![];
                                                for #element_snake_case in &read_only_ids_current_elements {
                                                    #acc_snake_case.push(#primary_key_field_type_where_token_stream::Equal(
                                                        postgresql_crud::PostgresqlTypeWhereEqual {
                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                            #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(
                                                                <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                                    #primary_key_field_type_read_only_ids_into_read_element_primary_key_field_ident_clone_token_stream
                                                                )
                                                            ),
                                                        }
                                                    ));
                                                }
                                                #acc_snake_case
                                            }
                                        )
                                    ),
                                    #select_default_all_with_max_page_size_clone_token_stream,
                                    &current_table
                                ).await.expect("error 82cb984b-8312-4952-a649-389f7c5adcff");
                                #acc_snake_case.sort_by(|first, second| {
                                    if let (Some(value_first), Some(value_second)) = (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                        value_first.#value_snake_case.cmp(&value_second.#value_snake_case)
                                    }
                                    else {
                                        panic!("must not be what")
                                    }
                                });
                                #acc_snake_case
                            },
                            "error 50198a7f-e65c-4e4e-8d7f-9881cfd42453"
                        );
                        for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
                            let current_table = table_update_many_cloned2.clone();
                            let url_cloned = url.clone();
                            let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                            #acc_snake_case.push(futures::FutureExt::boxed(async move {
                                #maybe_previous_read_token_stream
                                let update = <
                                    #field_type
                                    as
                                    postgresql_crud::PostgresqlTypeTestCases
                                >::read_inner_into_update_with_new_or_try_new_unwraped({
                                    let mut local_increment: usize = 0;
                                    let mut option_test_case = None;
                                    for element_0 in <#field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(
                                        &read_only_ids_current_element.#field_ident.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")
                                    ) {
                                        let mut should_break = false;
                                        for element_1 in element_0 {
                                            if local_increment == increment {
                                                option_test_case = Some(element_1);
                                                should_break = true;
                                                break;
                                            }
                                            local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                        }
                                        if should_break {
                                            break;
                                        }
                                    }
                                    option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                });
                                assert_eq!(
                                    vec![
                                        #ident_read_only_ids_upper_camel_case {
                                            #primary_key_field_ident: read_only_ids_current_element.#primary_key_field_ident.clone(),
                                            #ident_read_only_ids_upper_fields_initialization_without_primary_key_token_stream
                                        }
                                    ],
                                    #ident::try_update_many_handle(
                                        &url_cloned,
                                        #ident_update_many_parameters_upper_camel_case {
                                            payload: #ident_update_many_payload_upper_camel_case::try_new(vec![
                                                #ident_update_upper_camel_case::try_new(
                                                    #primary_key_field_type_as_postgresql_type_update_as_postgresql_type_primary_key_read_only_ids_into_update_token_stream,
                                                    #ident_update_parameters_initialization_without_primary_key_token_stream
                                                ).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2")
                                            ]).expect("error 69e1bd8a-fe78-4301-85ca-f4f3958d7493")
                                        },
                                        &current_table
                                    ).await.expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"),
                                    "error 34bfb3c7-7a53-479e-9d4f-0856003573e1"
                                );
                                assert_eq!(
                                    {
                                        #expected_read_many_token_stream
                                    },
                                    {
                                        let mut #acc_snake_case = generate_try_read_many_order_by_primary_key_with_big_pagination(
                                            &url_cloned,
                                            generate_ident_where_many_pripery_key_others_none(
                                                generate_some_postgresql_type_where_try_new_primary_key(
                                                    postgresql_crud::LogicalOperator::Or,
                                                    vec![
                                                        #primary_key_field_type_where_token_stream::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                            #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(<#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                                #primary_key_field_type_read_only_is_into_read_read_only_ids_current_element_primary_key_field_ident_clone_token_stream
                                                            )),
                                                        })
                                                    ]
                                                )
                                            ),
                                            select_default_all_with_max_page_size_cloned,
                                            &current_table
                                        ).await.expect("error 82cb984b-8312-4952-a649-389f7c5adcff");
                                        #acc_snake_case.sort_by(|first, second| {
                                            if let (Some(value_first), Some(value_second)) = (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                                value_first.#value_snake_case.cmp(&value_second.#value_snake_case)
                                            } else {
                                                panic!("must not be what");
                                            }
                                        });
                                        #acc_snake_case
                                    },
                                    "error ae2a2da5-3697-4fd7-9ad2-4a535618fbc3"
                                );
                            }));
                        }
                    }
                };}
            });
            quote::quote! {{
                let current_table = table_update_many_cloned2.clone();
                #update_many_only_one_column_tests_token_stream
            };}
        };
        let update_one_tests_token_stream = {
            let update_one_only_one_column_tests_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|current_element: &SynFieldWrapper| {
                let field_ident = &current_element.field_ident;
                let field_type = &current_element.syn_field.ty;
                let warning_message_double_quote_token_stream = generate_quotes::double_quotes_token_stream(&format!("PostgresqlTypeTestCases read_only_ids_to_two_dimensional_vec_read_inner is empty for {field_ident}"));
                // let try_create_one_error_message_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("error 870927ab-3ba2-445f-96b5-0f7b8618fc63 {field_ident}"));
                // let update_try_new_error_message_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2 {field_ident}"));
                // let try_update_one_expect_error_message_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("error d2de0bd6-1b01-4ef2-b074-a60878241b52 {field_ident}"));
                let maybe_previous_read_token_stream = if fields_without_primary_key.len() > 1 {
                    quote::quote! {
                        let previous_read = #ident::try_read_one_handle(
                            &url_cloned,
                            #ident_read_one_parameters_upper_camel_case {
                                payload: #ident_read_one_payload_upper_camel_case {
                                    #primary_key_field_ident: #primary_key_field_type_read_only_is_into_read_read_only_ids_current_element_primary_key_field_ident_clone_token_stream,
                                    select: #select_default_all_with_max_page_size_cloned_clone_token_stream
                                }
                            },
                            &current_table
                        ).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let ident_create_defaults_for_column_read_only_ids_to_two_dimensional_vec_read_inner_token_stream = generate_fields_named_without_primary_key_without_comma_token_stream(&|element: &SynFieldWrapper| {
                    let current_field_ident = &element.field_ident;
                    let current_field_type = &element.syn_field.ty;
                    if field_ident == current_field_ident {
                        quote::quote! {
                            if let Some(#value_snake_case) = &common_read_only_ids_returned_from_create_one.#current_field_ident {
                                for #element_snake_case in <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(#value_snake_case) {
                                    for _ in #element_snake_case {
                                        #acc_snake_case.push(ident_create_default.clone());
                                    }
                                }
                            }
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    }
                });
                let ident_read_fields_initialization_without_primary_key_after_create_many_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
                    let current_field_ident = &element.field_ident;
                    let current_field_type = &element.syn_field.ty;
                    let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream);
                    quote::quote! {
                        #current_field_ident: match &#element_snake_case.#current_field_ident {
                            Some(#value_snake_case) => <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(#value_snake_case),
                            None => Some(#value_initialization_token_stream),
                        }
                    }
                });
                let ident_read_only_ids_upper_fields_initialization_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
                    let current_field_ident = &element.field_ident;
                    let current_field_type = &element.syn_field.ty;
                    if field_ident == current_field_ident {
                        quote::quote! {#current_field_ident: Some(<#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update))}
                    } else {
                        quote::quote! {#current_field_ident: None}
                    }
                });
                let ident_update_parameters_initialization_without_primary_key_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
                    let current_field_ident = &element.field_ident;
                    if field_ident == current_field_ident {
                        let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&quote::quote! {#update_snake_case.clone()});
                        quote::quote! {Some(#value_initialization_token_stream)}
                    } else {
                        none_token_stream.clone()
                    }
                });
                let ident_read_fields_initialization_without_primary_key_after_update_one_token_stream = generate_fields_named_without_primary_key_with_comma_token_stream(&|element: &SynFieldWrapper| {
                    let current_field_ident = &element.field_ident;
                    let current_field_type = &element.syn_field.ty;
                    if field_ident == current_field_ident {
                        let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&quote::quote! {
                            <
                                #current_field_type
                                as
                                postgresql_crud::PostgresqlTypeTestCases
                            >::previous_read_merged_with_option_update_into_read(
                                <
                                    #current_field_type
                                    as
                                    postgresql_crud::PostgresqlTypeTestCases
                                >::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                    &read_only_ids_current_element.#current_field_ident.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129")
                                ).expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5").#value_snake_case,
                                Some(#update_snake_case.clone())
                            )
                        });
                        quote::quote! {
                            #current_field_ident: Some(#value_initialization_token_stream)
                        }
                    } else {
                        quote::quote! {
                            #current_field_ident: previous_read.#current_field_ident
                        }
                    }
                });
                let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&primary_key_field_type_read_only_is_into_read_read_only_ids_current_element_primary_key_field_ident_clone_token_stream);
                quote::quote! {{
                    let read_only_ids_to_two_dimensional_vec_read_inner_acc = {
                        let mut #acc_snake_case = vec![];
                        #ident_create_defaults_for_column_read_only_ids_to_two_dimensional_vec_read_inner_token_stream
                        #acc_snake_case
                    };
                    if read_only_ids_to_two_dimensional_vec_read_inner_acc.is_empty() {
                        println!(#warning_message_double_quote_token_stream);
                    }
                    else {
                        let current_table = table_update_one_cloned2.clone();
                        let read_only_ids_current_elements = {
                            futures::StreamExt::collect::<Vec<Vec<#ident_read_only_ids_upper_camel_case>>>(
                                futures::StreamExt::buffer_unordered(
                                    futures::stream::iter(
                                        read_only_ids_to_two_dimensional_vec_read_inner_acc
                                        .chunks(25)
                                        .map(Vec::from)
                                        .map(|#element_snake_case| {
                                            let current_table = current_table.clone();
                                            let url_cloned = url.clone();
                                            futures::FutureExt::boxed(async move {
                                                #ident::try_create_many_handle(
                                                    &url_cloned,
                                                    #ident_create_many_parameters_upper_camel_case {
                                                        payload: #ident_create_many_payload_upper_camel_case(#element_snake_case)
                                                    },
                                                    &current_table
                                                ).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98")
                                            })
                                        })
                                    ),
                                    5
                                )
                            )
                            .await
                            .into_iter()
                            .flatten()
                            .collect::<Vec<#ident_read_only_ids_upper_camel_case>>()
                        };
                        assert_eq!(
                            {
                                let mut #acc_snake_case = vec![];
                                for #element_snake_case in &read_only_ids_current_elements {
                                    #acc_snake_case.push(#ident_read_upper_camel_case {
                                        #primary_key_field_ident: <
                                            #primary_key_field_type
                                            as
                                            postgresql_crud::PostgresqlTypeTestCases
                                        >::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                            &#element_snake_case.#primary_key_field_ident
                                        ),
                                        #ident_read_fields_initialization_without_primary_key_after_create_many_token_stream
                                    });
                                }
                                #acc_snake_case.sort_by(|first, second| {
                                    if let (Some(value_first), Some(value_second)) = (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                        value_first.#value_snake_case.cmp(&value_second.#value_snake_case)
                                    }
                                    else {
                                        panic!("must not be what");
                                    }
                                });
                                #acc_snake_case
                            },
                            {
                                let mut #acc_snake_case = generate_try_read_many_order_by_primary_key_with_big_pagination(
                                    &url,
                                    generate_ident_where_many_pripery_key_others_none(
                                        generate_some_postgresql_type_where_try_new_primary_key(
                                            postgresql_crud::LogicalOperator::Or,
                                            {
                                                let mut #acc_snake_case = vec![];
                                                for #element_snake_case in &read_only_ids_current_elements {
                                                    #acc_snake_case.push(#primary_key_field_type_where_token_stream::Equal(
                                                        postgresql_crud::PostgresqlTypeWhereEqual {
                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                            #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(
                                                                <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                                    #primary_key_field_type_read_only_ids_into_read_element_primary_key_field_ident_clone_token_stream
                                                                )
                                                            ),
                                                        }
                                                    ));
                                                }
                                                #acc_snake_case
                                            }
                                        )
                                    ),
                                    #select_default_all_with_max_page_size_clone_token_stream,
                                    &current_table
                                ).await.expect("error 82cb984b-8312-4952-a649-389f7c5adcff");
                                #acc_snake_case.sort_by(|first, second| {
                                    if let (Some(value_first), Some(value_second)) = (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                        value_first.#value_snake_case.cmp(&value_second.#value_snake_case)
                                    }
                                    else {
                                        panic!("must not be what")
                                    }
                                });
                                #acc_snake_case
                            },
                            "error db146190-0496-42a7-93d6-8405eb641954"
                        );
                        for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
                            let current_table = table_update_one_cloned2.clone();
                            let url_cloned = url.clone();
                            let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                            #acc_snake_case.push(futures::FutureExt::boxed(async move {
                                #maybe_previous_read_token_stream
                                let update = <
                                    #field_type
                                    as
                                    postgresql_crud::PostgresqlTypeTestCases
                                >::read_inner_into_update_with_new_or_try_new_unwraped({
                                    let mut local_increment: usize = 0;
                                    let mut option_test_case = None;
                                    for element_0 in <#field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(
                                        &read_only_ids_current_element.#field_ident.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")
                                    ) {
                                        let mut should_break = false;
                                        for element_1 in element_0 {
                                            if local_increment == increment {
                                                option_test_case = Some(element_1);
                                                should_break = true;
                                                break;
                                            }
                                            local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                        }
                                        if should_break {
                                            break;
                                        }
                                    }
                                    option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                });
                                assert_eq!(
                                    #ident_read_only_ids_upper_camel_case {
                                        #primary_key_field_ident: read_only_ids_current_element.#primary_key_field_ident.clone(),
                                        #ident_read_only_ids_upper_fields_initialization_without_primary_key_token_stream
                                    },
                                    #ident::try_update_one_handle(
                                        &url_cloned,
                                        #ident_update_one_parameters_upper_camel_case {
                                            payload: #ident_update_upper_camel_case::try_new(
                                                #primary_key_field_type_as_postgresql_type_update_as_postgresql_type_primary_key_read_only_ids_into_update_token_stream,
                                                #ident_update_parameters_initialization_without_primary_key_token_stream
                                            ).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2")//todo add column ident
                                        },
                                        &current_table
                                    ).await.expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"),
                                    "error 564de31c-3664-4c62-85fc-e03793372f8f"
                                );
                                assert_eq!(
                                    #ident_read_upper_camel_case {
                                        #primary_key_field_ident: Some(#value_initialization_token_stream),
                                        #ident_read_fields_initialization_without_primary_key_after_update_one_token_stream
                                    },
                                    #ident::try_read_one_handle(
                                        &url_cloned,
                                        #ident_read_one_parameters_upper_camel_case {
                                            payload: #ident_read_one_payload_upper_camel_case {
                                                #primary_key_field_ident: #primary_key_field_type_read_only_is_into_read_read_only_ids_current_element_primary_key_field_ident_clone_token_stream,
                                                select: select_default_all_with_max_page_size_cloned
                                            }
                                        },
                                        &current_table
                                    ).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                                    "error d5dec823-b1f9-49b2-9c24-bf788f08cd8c"
                                );
                            }));
                        }
                    }
                };}
            });
            quote::quote! {{
                #update_one_only_one_column_tests_token_stream
            };}
        };
        let delete_many_tests_token_stream = {
            let test_delete_many_by_non_existent_primary_keys_token_stream = {
                let content_token_stream = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_token_stream(&quote::quote! {
                    match #ident::try_delete_many_handle(
                        url,
                        #ident_delete_many_parameters_upper_camel_case {
                            payload: #ident_delete_many_payload_upper_camel_case {
                                where_many: #std_option_option_ident_where_many_upper_camel_case(Some(#ident_where_many_upper_camel_case {
                                    #primary_key_field_ident: generate_some_postgresql_type_where_try_new_primary_key(
                                        postgresql_crud::LogicalOperator::Or,
                                        {
                                            let mut #acc_snake_case = vec![];
                                            for _ in 1..=length {
                                                #acc_snake_case.push(
                                                    #primary_key_field_type_as_postgresql_type_where_token_stream::Equal(
                                                        postgresql_crud::PostgresqlTypeWhereEqual {
                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                            #value_snake_case: #primary_key_field_type_table_type_declaration_token_stream::new(
                                                                uuid::Uuid::new_v4()
                                                            )
                                                        }
                                                    )
                                                );
                                            }
                                            #acc_snake_case
                                        }
                                    ),
                                    #fields_none_initialization_token_stream
                                })),
                            },
                        },
                        current_table
                    )
                    .await {
                        Ok(value) => assert!(value.is_empty(), "error 51d14103-5122-4d96-a45c-4dd958ab3adc"),
                        Err(error) => panic!("error 0d5dec47-8b2e-4f02-909b-3a58b65bc6a5 {error:#?}"),
                    }
                });
                quote::quote! {{
                    let current_table = table_test_read_many_by_equal_to_created_primary_keys_cloned2.clone();
                    async fn generate_test_delete_many_by_non_existent_primary_keys(
                        length: usize,
                        url: &str,
                        select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueEnumVec<#ident_select_upper_camel_case>,
                        current_table: &str,
                        ident_create_default: #ident_create_upper_camel_case,
                        no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row: &str,
                    ){
                        #content_token_stream
                    }
                    let lengths = vec![1,2];
                    for element in lengths {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                        let current_table = current_table.clone();
                        let ident_create_default_cloned = ident_create_default.clone();
                        #acc_snake_case.push(futures::FutureExt::boxed(async move {
                            generate_test_delete_many_by_non_existent_primary_keys(
                                element,
                                &url_cloned,
                                select_default_all_with_max_page_size_cloned,
                                &current_table,
                                ident_create_default_cloned,
                                no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row,
                            ).await;
                        }));
                    };
                }}
            };
            let test_delete_many_by_primary_keys_token_stream = {
                let content_token_stream = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_token_stream(&{
                    quote::quote! {
                        let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                            url,
                            #ident_create_many_parameters_upper_camel_case {
                                payload: #ident_create_many_payload_upper_camel_case({
                                    let mut #acc_snake_case = vec![];
                                    for _ in 1..=length {
                                        #acc_snake_case.push(ident_create_default.clone());
                                    }
                                    #acc_snake_case
                                })
                            },
                            current_table
                        ).await.expect("error b8695890-65fb-469b-a6f9-be481d648eb9");
                        let read_only_ids_from_try_delete_many = #ident::try_delete_many_handle(
                            url,
                            #ident_delete_many_parameters_upper_camel_case {
                                payload: #ident_delete_many_payload_upper_camel_case {
                                    where_many: #std_option_option_ident_where_many_upper_camel_case(Some(#ident_where_many_upper_camel_case {
                                        #primary_key_field_ident: generate_some_postgresql_type_where_try_new_primary_key(
                                            postgresql_crud::LogicalOperator::Or,
                                            {
                                                let mut #acc_snake_case = vec![];
                                                for #element_snake_case in &read_only_ids_from_try_create_many {
                                                    #acc_snake_case.push(
                                                        #primary_key_field_type_as_postgresql_type_where_token_stream::Equal(
                                                            postgresql_crud::PostgresqlTypeWhereEqual {
                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                #value_snake_case: #primary_key_field_type_read_only_ids_into_table_type_declaration_element_primary_key_field_ident_clone_token_stream
                                                            }
                                                        )
                                                    );
                                                }
                                                #acc_snake_case
                                            }
                                        ),
                                        #fields_none_initialization_token_stream
                                    })),
                                },
                            },
                            current_table
                        ).await.expect("error b80b91b8-7de1-4ea2-97cf-1987a5f7cc57");
                        assert_eq!(
                            read_only_ids_from_try_delete_many,
                            {
                                read_only_ids_from_try_create_many.iter().map(|#element_snake_case|
                                    <#primary_key_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(
                                        &#element_snake_case.#primary_key_field_ident
                                    ).expect("error 3ee5ee86-05dc-4dc8-9262-8ffa1855d5e4").#value_snake_case
                                ).collect::<Vec<#primary_key_field_type_as_postgresql_type_read_token_stream>>()
                            },
                            "error db5e88a6-c75b-421b-acfb-56931b97ba3b"
                        );
                        match generate_try_read_many_order_by_primary_key_with_big_pagination(
                            &url,
                            generate_ident_where_many_pripery_key_others_none(
                                generate_some_postgresql_type_where_try_new_primary_key(
                                    postgresql_crud::LogicalOperator::Or,
                                    {
                                        let mut #acc_snake_case = vec![];
                                        for element in read_only_ids_from_try_delete_many {
                                            #acc_snake_case.push(#primary_key_field_type_as_postgresql_type_where_token_stream::Equal(
                                                postgresql_crud::PostgresqlTypeWhereEqual {
                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                    #value_snake_case: #primary_key_field_type_read_into_table_type_declaration_element_primary_key_field_ident_clone_token_stream
                                                }
                                            ));
                                        }
                                        #acc_snake_case
                                    }
                                )
                            ),
                            #select_default_all_with_max_page_size_clone_token_stream,
                            &current_table
                        ).await {
                            Ok(#value_snake_case) => assert!(#value_snake_case.is_empty(), "error 77f038b0-6f39-4b5b-a402-a1b6142acd0d"),
                            Err(#error_snake_case) => panic!("error bcb79917-ee81-416e-82a3-f43a823266a3 {error:#?}")
                        }
                    }
                });
                quote::quote! {{
                    let current_table = table_test_read_many_by_equal_to_created_primary_keys_cloned2.clone();
                    async fn generate_test_delete_many_by_primary_keys(
                        length: usize,
                        url: &str,
                        select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueEnumVec<#ident_select_upper_camel_case>,
                        current_table: &str,
                        ident_create_default: #ident_create_upper_camel_case,
                        no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row: &str,
                    ) {
                        #content_token_stream
                    }
                    let lengths = vec![1,2];
                    for element in lengths {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                        let current_table = current_table.clone();
                        let ident_create_default_cloned = ident_create_default.clone();
                        #acc_snake_case.push(futures::FutureExt::boxed(async move {
                            generate_test_delete_many_by_primary_keys(
                                element,
                                &url_cloned,
                                select_default_all_with_max_page_size_cloned,
                                &current_table,
                                ident_create_default_cloned,
                                no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row,
                            ).await;
                        }));
                    };
                }}
            };
            quote::quote! {{
                #test_delete_many_by_non_existent_primary_keys_token_stream
                #test_delete_many_by_primary_keys_token_stream
            };}
        };
        let delete_one_tests_token_stream = {
            let value_initialization_token_stream = generate_import_path_value_initialization_token_stream(&primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream);
            quote::quote! {{
                let current_table = table_delete_one_cloned2.clone();
                let ident_create_default_cloned = ident_create_default.clone();
                let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_token_stream;
                let url_cloned = url.clone();
                #acc_snake_case.push(futures::FutureExt::boxed(async move {
                    if let Err(#error_snake_case) = #ident::try_delete_one_handle(
                        &url_cloned,
                        #ident_delete_one_parameters_upper_camel_case {
                            payload: #ident_delete_one_payload_upper_camel_case {
                                #primary_key_field_ident: #primary_key_field_type_as_postgresql_type_read_token_stream::new(uuid::Uuid::new_v4())
                            }
                        },
                        &current_table
                    ).await {
                        if let #ident_try_delete_one_error_named_upper_camel_case::#ident_delete_one_error_named_with_serialize_deserialize_upper_camel_case {
                            delete_one_error_named_with_serialize_deserialize,
                            ..
                        } = #error_snake_case {
                            if let #ident_delete_one_error_named_with_serialize_deserialize_upper_camel_case::Postgresql {
                                postgresql,
                                ..
                            } = delete_one_error_named_with_serialize_deserialize {
                                assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error c9261bb8-d391-4c4b-9707-3a2c4278ad90");
                            } else {
                                panic!("error e63b27a3-f3e3-4f19-998a-88ce798b08cc");
                            }
                        } else {
                            panic!("error 47a8e0d9-1f95-4fa7-91dc-a94955195204")
                        }
                    } else {
                        panic!("error 9be62f9f-31d9-493c-bb0f-b83b6ecb0026")
                    }
                    let read_only_ids_returned_from_create_one = #ident::try_create_one_handle(
                        &url_cloned,
                        #ident_create_one_parameters_upper_camel_case {
                            payload: ident_create_default_cloned.clone()
                        },
                        &current_table
                    ).await.expect("error 8be80909-0e8d-42f9-a5c8-fa08244cb592");
                    assert_eq!(
                        #ident_read_upper_camel_case {
                            #primary_key_field_ident: Some(#value_initialization_token_stream),
                            #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_create_token_stream
                        },
                        #ident::try_read_one_handle(
                            &url_cloned,
                            #ident_read_one_parameters_upper_camel_case {
                                payload: #ident_read_one_payload_upper_camel_case {
                                    #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream,
                                    select: #select_default_all_with_max_page_size_cloned_clone_token_stream
                                }
                            },
                            &current_table
                        ).await.expect("error c8c44c89-aeb0-43d3-ae72-02b7a5979f5a"),
                        "error 86ef08ae-4356-4417-9490-1d13eb2af71f"
                    );
                    assert_eq!(
                        #ident::try_delete_one_handle(
                            &url_cloned,
                            #ident_delete_one_parameters_upper_camel_case {
                                payload: #ident_delete_one_payload_upper_camel_case {
                                    #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream
                                }
                            },
                            &current_table
                        ).await.expect("error acab86b7-b199-4732-b8ea-76c00a12abb2"),
                        #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream,
                        "error 99f81971-dc80-46db-b466-4f309b215a8c"
                    );
                    if let Err(#error_snake_case) = #ident::try_read_one_handle(
                        &url_cloned,
                        #ident_read_one_parameters_upper_camel_case {
                            payload: #ident_read_one_payload_upper_camel_case {
                                #primary_key_field_ident: #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_clone_token_stream,
                                select: #select_default_all_with_max_page_size_cloned_clone_token_stream
                            }
                        },
                        &current_table
                    ).await {
                        if let #ident_try_read_one_error_named_upper_camel_case::#ident_read_one_error_named_with_serialize_deserialize_upper_camel_case { read_one_error_named_with_serialize_deserialize, .. } = error {
                            if let #ident_read_one_error_named_with_serialize_deserialize_upper_camel_case::Postgresql {
                                postgresql,
                                ..
                            } = read_one_error_named_with_serialize_deserialize {
                                assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 2c10a1e0-ee7f-4710-9329-5e6ba04a880c");
                            } else {
                                panic!("error f6bb9f64-4453-4eb2-9972-7266270b3972");
                            }
                        } else {
                            panic!("error e494cdfd-1f76-4694-a727-9365e1fdf3c6")
                        }
                    } else {
                        panic!("error baa0a952-679c-4485-8360-279eef9982b4")
                    }
                }));
            };}
        };
        quote::quote! {
            #[cfg(test)]
            mod #ident_tests_snake_case {
                use super::*;
                #[test]
                fn size_of() {
                    assert_eq!(std::mem::size_of::<#ident>(), 0);
                }
                #[test]
                fn crud() {
                    std::thread::Builder::new()
                    .stack_size(16 * 1024 * 1024)
                    .spawn(|| {
                        tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().expect("error 38823c21-1879-449c-9b60-ce7293709959").block_on(async {
                            tracing_subscriber::fmt::init();
                            let no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row = "no rows returned by a query that expected to return at least one row";
                            static #config_upper_case_token_stream: std::sync::OnceLock<#config_path_token_stream> = std::sync::OnceLock::new();
                            //todo maybe refactor
                            let #config_snake_case = #config_upper_case_token_stream.get_or_init(||
                                #config_path_token_stream {
                                    service_socket_address: <config_lib::ServiceSocketAddress as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                        "127.0.0.1:8080".to_string()
                                    ).expect("error b5b3915a-0e18-4815-a614-6b0e9a00d73f").0,
                                    database_url: <config_lib::DatabaseUrl as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                        "postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=10".to_string()
                                    ).expect("error f9c20f05-3cdf-46ae-b6d3-5943c627f0df").0,
                                    timezone: <config_lib::Timezone as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                        "10800".to_string()
                                    ).expect("error d00d8998-52f9-45c1-a4b0-c93bc95a313e").0,
                                    tracing_level: <config_lib::TracingLevel as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                        "error".to_string()
                                    ).expect("error 957178c9-4d92-4110-b524-9dc21d147a7c").0,
                                    source_place_type: <config_lib::SourcePlaceType as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                        "source".to_string()
                                    ).expect("error bec0950e-e9de-42f3-b3a2-67d9d98ae8a6").0,
                                    enable_api_git_commit_check: <config_lib::EnableApiGitCommitCheck as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                        "true".to_string()
                                    ).expect("error 31f02640-d62b-41ca-837d-d61b707d4baf").0,
                                    maximum_size_of_http_body_in_bytes: <config_lib::MaximumSizeOfHttpBodyInBytes as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                        "1048576000".to_string()
                                    ).expect("error 93b2f818-18be-4bb6-8a02-53c6e55ded2d").0,
                                }
                            );
                            let #postgres_pool_snake_case = sqlx::postgres::PgPoolOptions::new()
                            .max_connections(50)
                            .connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&#config_snake_case)))
                            .await.expect("error e3044bb9-7b76-4c0c-bc5f-eb34da05a103");
                            let #url_snake_case = format!("http://{}", app_state::GetServiceSocketAddress::get_service_socket_address(&#config_snake_case));
                            let table = #ident_double_quotes_token_stream;

                            let add_table_postfix = |postfix: &str|{
                                let value = format!("{table}_{postfix}");
                                assert!(value.len() <= 63, "error 77f9bfb7-f7d8-4ba0-96d0-712d4246ecae");
                                value
                            };
                            let table_create_many = add_table_postfix("create_many");
                            let table_create_one = add_table_postfix("create_one");
                            let table_test_read_many_by_non_existent_primary_keys = add_table_postfix("test_read_many_by_non_existent_primary_keys");
                            let table_test_read_many_by_equal_to_created_primary_keys = add_table_postfix("test_read_many_by_equal_to_created_primary_keys");
                            #(#table_field_idents_initialization_vec_token_stream)*
                            let table_read_one = add_table_postfix("read_one");
                            let table_update_many = add_table_postfix("update_many");
                            let table_update_one = add_table_postfix("update_one");
                            let table_delete_many = add_table_postfix("delete_many");
                            let table_delete_one = add_table_postfix("delete_one");

                            let table_create_many_cloned = table_create_many.clone();
                            let table_create_one_cloned = table_create_one.clone();
                            let table_test_read_many_by_non_existent_primary_keys_cloned = table_test_read_many_by_non_existent_primary_keys.clone();
                            let table_test_read_many_by_equal_to_created_primary_keys_cloned = table_test_read_many_by_equal_to_created_primary_keys.clone();
                            #(#table_field_idents_clones_vec_token_stream)*
                            let table_read_one_cloned = table_read_one.clone();
                            let table_update_many_cloned = table_update_many.clone();
                            let table_update_one_cloned = table_update_one.clone();
                            let table_delete_many_cloned = table_delete_many.clone();
                            let table_delete_one_cloned = table_delete_one.clone();

                            let table_create_many_cloned2 = table_create_many.clone();
                            let table_create_one_cloned2 = table_create_one.clone();
                            let table_test_read_many_by_non_existent_primary_keys_cloned2 = table_test_read_many_by_non_existent_primary_keys.clone();
                            let table_test_read_many_by_equal_to_created_primary_keys_cloned2 = table_test_read_many_by_equal_to_created_primary_keys.clone();
                            #(#table_field_idents_clones2_vec_token_stream)*
                            let table_read_one_cloned2 = table_read_one.clone();
                            let table_update_many_cloned2 = table_update_many.clone();
                            let table_update_one_cloned2 = table_update_one.clone();
                            // let table_delete_many_cloned2 = table_delete_many.clone();
                            let table_delete_one_cloned2 = table_delete_one.clone();

                            let drop_all_test_tables = async ||{
                                let _unused = futures::future::try_join_all(
                                    [
                                        &table_create_many,
                                        &table_create_one,
                                        &table_test_read_many_by_non_existent_primary_keys,
                                        &table_test_read_many_by_equal_to_created_primary_keys,
                                        #(#table_field_idents_to_drop_table_if_exists_vec_token_stream)*
                                        &table_read_one,
                                        &table_update_many,
                                        &table_update_one,
                                        &table_delete_many,
                                        &table_delete_one,
                                    ]
                                    .iter()
                                    .map(|table_name|{
                                        let postgres_pool = &postgres_pool;
                                        async move {
                                            sqlx::query(&format!("drop table if exists {table_name}")).execute(postgres_pool).await
                                        }
                                    })
                                )
                                .await
                                .expect("error b9c1eb2e-4ead-449b-abb8-0a160cf68efd");
                            };
                            drop_all_test_tables().await;
                            #ident::prepare_extensions(&#postgres_pool_snake_case).await.expect("error 0633ff48-ebc4-460f-a282-d750511f5d78");
                            //do not make it concurrent. would be postgresql error: "duplicate key value violates unique constraint \"pg_class_relname_nsp_index\""
                            for table_name in [
                                &table_create_many_cloned,
                                &table_create_one_cloned,
                                &table_test_read_many_by_non_existent_primary_keys_cloned,
                                &table_test_read_many_by_equal_to_created_primary_keys_cloned,
                                #(#table_field_idents_for_prepare_postgresql_table_vec_token_stream)*
                                &table_read_one_cloned,
                                &table_update_many_cloned,
                                &table_update_one_cloned,
                                &table_delete_many_cloned,
                                &table_delete_one_cloned
                            ] {
                                #ident::prepare_postgresql_table(
                                    &#postgres_pool_snake_case,
                                    table_name,
                                ).await.expect("error c7952247-dc94-441b-9aef-368b8fdc593c");
                            }
                            let #postgres_pool_for_tokio_spawn_sync_move_snake_case = #postgres_pool_snake_case.clone();
                            let (started_tx, started_rx) = tokio::sync::oneshot::channel();
                            let #underscore_unused_token_stream = tokio::spawn(async move {
                                let #app_state_snake_case = std::sync::Arc::new(server_app_state::ServerAppState {
                                    #postgres_pool_snake_case: #postgres_pool_for_tokio_spawn_sync_move_snake_case.clone(),
                                    #config_snake_case,
                                    project_git_info: &git_info::PROJECT_GIT_INFO,
                                });
                                let tcp_listener = tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&#config_snake_case)).await.expect("error 663ae29e-bc00-4ea1-a7e9-4dddceb5b53a");
                                if let Err(error) = started_tx.send(()) {
                                    panic!("error aa3b8154-1fe2-4d3f-a164-26f9d21245cd {error:#?}");
                                }
                                axum::serve(
                                    tcp_listener,
                                    {
                                        let mut router = axum::Router::new()
                                            .merge(#ident::routes(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state)));
                                        for table_name in [
                                            &table_create_many_cloned,
                                            &table_create_one_cloned,
                                            &table_test_read_many_by_non_existent_primary_keys_cloned,
                                            &table_test_read_many_by_equal_to_created_primary_keys_cloned,
                                            #(#table_field_idents_for_routes_handle_vec_token_stream)*
                                            &table_read_one_cloned,
                                            &table_update_many_cloned,
                                            &table_update_one_cloned,
                                            &table_delete_many_cloned,
                                            &table_delete_one_cloned,
                                        ] {
                                            router = router.merge(#ident::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), table_name));
                                        }
                                        router.into_make_service()
                                    },
                                )
                                .await
                                .unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
                            });
                            started_rx.await.expect("error 87003141-43a4-4975-8ddf-273148add50f");
                            let #select_primary_key_snake_case = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![
                                #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(
                                    #primary_key_field_type_as_postgresql_type_select_token_stream::default(),
                                )
                            ])
                            .expect("error 0776170e-4dd6-4c14-a412-ce10b0c746f1");
                            let #ident_create_default_snake_case = #ident_create_upper_camel_case {
                                #ident_create_default_fields_initialization_without_primary_key_token_stream
                            };
                            #select_default_all_with_max_page_size_not_empty_unique_enum_vec_token_stream
                            #common_read_only_ids_returned_from_create_one_token_stream
                            #generate_ident_where_many_pripery_key_others_none_content_token_stream
                            #generate_some_postgresql_type_where_try_new_primary_key_content_token_stream
                            #generate_some_postgresql_type_where_try_new_or_primary_keys_content_token_stream
                            #generate_try_read_many_order_by_primary_key_asc_with_big_pagination_content_token_stream
                            futures::StreamExt::for_each_concurrent(
                                futures::stream::iter({
                                    let mut #acc_snake_case: Vec<futures::future::BoxFuture<'static, ()>> = vec![];
                                    #create_many_tests_token_stream
                                    #create_one_tests_token_stream
                                    #read_many_tests_token_stream
                                    #read_one_tests_token_stream
                                    #update_many_tests_token_stream
                                    #update_one_tests_token_stream
                                    #delete_many_tests_token_stream
                                    #delete_one_tests_token_stream
                                    #acc_snake_case
                                }),
                                100,
                                async |fut| { fut.await; },
                            )
                            .await;
                            drop_all_test_tables().await;
                        });
                    })
                    .expect("error 4d329978-f5af-424e-8757-e8a32dbeb5a1")
                    .join()
                    .unwrap_or_else(|error|{
                        panic!("error b2f21a5f-d9ce-435c-809f-bd40741c8795 {error:#?}");
                    });
                }
            }
        }
    };
    // if ident == "" {
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlTable",
    //     &ident_tests_token_stream,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::False
    // );
    // }
    let common_token_stream = quote::quote! {
        #impl_ident_token_stream
        #ident_create_token_stream
        #ident_where_many_token_stream
        #std_option_option_ident_where_many_token_stream
        #select_token_stream
        #ident_read_token_stream
        #ident_read_only_ids_token_stream
        // #ident_column_read_permission_token_stream
        #ident_update_token_stream
        #ident_update_for_query_token_stream
    };
    // if ident == "" {
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlTable",
    //     &common_token_stream,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
    // );
    // }
    let generated = quote::quote! {
        impl #ident {
            #(#impl_ident_vec_token_stream)*
        }
        #common_token_stream
        #create_many_token_stream
        #create_one_token_stream
        #read_many_token_stream
        #read_one_token_stream
        #update_many_token_stream
        #update_one_token_stream
        #delete_many_token_stream
        #delete_one_token_stream
        #ident_tests_token_stream
    };
    // if ident == "" {
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     "GeneratePostgresqlTable",
    //     &generated,
    //     &macros_helpers::write_token_stream_into_file::FormatWithRustfmt::True
    // );
    // }
    generated.into()
}
