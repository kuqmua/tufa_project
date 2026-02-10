use naming::parameter::ErrorSelfSnakeCase;
use naming::parameter::IsSelfUpdateExistSnakeCase;
use naming::parameter::SelfCreateUpperCamelCase;
use naming::parameter::SelfDeleteManyParametersUpperCamelCase;
use naming::parameter::SelfDeleteManyPayloadUpperCamelCase;
use naming::parameter::SelfDeleteOneErrorNamedWithSerializeDeserializeUpperCamelCase;
use naming::parameter::SelfDeleteOneParametersUpperCamelCase;
use naming::parameter::SelfDeleteOnePayloadUpperCamelCase;
use naming::parameter::SelfErrorNamedWithSerializeDeserializeSnakeCase;
use naming::parameter::SelfHandleSnakeCase;
use naming::parameter::SelfPayloadExampleSnakeCase;
use naming::parameter::SelfPreparePostgresqlErrorNamedUpperCamelCase;
use naming::parameter::SelfReadOneErrorNamedWithSerializeDeserializeUpperCamelCase;
use naming::parameter::SelfReadOnlyIdsToTwoDimensionalVecReadInnerAccSnakeCase;
use naming::parameter::SelfReadOnlyIdsUpperCamelCase;
use naming::parameter::SelfReadUpperCamelCase;
use naming::parameter::SelfSelectUpperCamelCase;
use naming::parameter::SelfTableTypeDeclarationUpperCamelCase;
use naming::parameter::SelfTestsSnakeCase;
use naming::parameter::SelfTryDeleteOneErrorNamedUpperCamelCase;
use naming::parameter::SelfTryReadOneErrorNamedUpperCamelCase;
use naming::parameter::SelfUpdateForQueryUpperCamelCase;
use naming::parameter::SelfUpdateManyParametersUpperCamelCase;
use naming::parameter::SelfUpdateManyPayloadUpperCamelCase;
use naming::parameter::SelfUpdateTryNewErrorNamedUpperCamelCase;
use naming::parameter::SelfUpdateUpperCamelCase;
use naming::parameter::SelfWhereManyTryNewErrorNamedUpperCamelCase;
use naming::parameter::SelfWhereManyUpperCamelCase;
use naming::parameter::SelfWhereUpperCamelCase;
use naming::parameter::StdOptionOptionSelfWhereManyUpperCamelCase;
use naming::parameter::TryFromSqlxPostgresPgRowWithNotEmptyUniqueVecSelfSelectSnakeCase;
use naming::parameter::TrySelfHandleSnakeCase;
use naming::parameter::TrySelfSnakeCase;
use naming::parameter::UpdateQueryPartSelfSnakeCase;

use naming::AsRefStrToSnakeCaseStringified;
use naming::AsRefStrToSnakeCaseTokenStream;

use macros_helpers::ErrorOccurenceFieldAttribute;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as StdFmtResult;
use std::iter::once;
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::token::Bracket;
use syn::token::Colon;
use syn::token::Comma;
use syn::token::PathSep;
use syn::token::Pound;
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
#[must_use]
pub fn generate_postgresql_table(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    #[derive(Debug)]
    struct SynVariantWrapper {
        status_code: Option<macros_helpers::StatusCode>,
        variant: syn::Variant,
    }
    impl SynVariantWrapper {
        const fn get_option_status_code(&self) -> Option<&macros_helpers::StatusCode> {
            self.status_code.as_ref()
        }
        const fn get_syn_variant(&self) -> &syn::Variant {
            &self.variant
        }
    }
    enum ShouldAddBorrow {
        False,
        True,
    }
    impl quote::ToTokens for ShouldAddBorrow {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match &self {
                Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
                Self::True => quote::quote! {&}.to_tokens(tokens),
            }
        }
    }
    enum ShouldAddReturn {
        False,
        True,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        Copy,
        naming::AsRefStrEnumWithUnitFieldsToUpperCamelCaseStringified,
        naming::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified,
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
        const fn derive_clone_and_copy(
            self,
        ) -> (macros_helpers::DeriveClone, macros_helpers::DeriveCopy) {
            match self {
                Self::CreateMany
                | Self::CreateOne
                | Self::ReadMany
                | Self::ReadOne
                | Self::UpdateMany
                | Self::UpdateOne
                | Self::DeleteMany => (
                    macros_helpers::DeriveClone::False,
                    macros_helpers::DeriveCopy::False,
                ),
                Self::DeleteOne => (
                    macros_helpers::DeriveClone::True,
                    macros_helpers::DeriveCopy::True,
                ),
            }
        }
        const fn desirable_status_code(self) -> macros_helpers::StatusCode {
            match self {
                Self::CreateMany | Self::CreateOne => macros_helpers::StatusCode::Created201,
                Self::ReadMany
                | Self::ReadOne
                | Self::UpdateMany
                | Self::UpdateOne
                | Self::DeleteMany
                | Self::DeleteOne => macros_helpers::StatusCode::Ok200,
            }
        }
        const fn generate_postgresql_table_attribute_additional_error_variants(
            self,
        ) -> GeneratePostgresqlTableAttribute {
            match self {
                Self::CreateMany => {
                    GeneratePostgresqlTableAttribute::CreateManyAdditionalErrorVariants
                }
                Self::CreateOne => {
                    GeneratePostgresqlTableAttribute::CreateOneAdditionalErrorVariants
                }
                Self::ReadMany => GeneratePostgresqlTableAttribute::ReadManyAdditionalErrorVariants,
                Self::ReadOne => GeneratePostgresqlTableAttribute::ReadOneAdditionalErrorVariants,
                Self::UpdateMany => {
                    GeneratePostgresqlTableAttribute::UpdateManyAdditionalErrorVariants
                }
                Self::UpdateOne => {
                    GeneratePostgresqlTableAttribute::UpdateOneAdditionalErrorVariants
                }
                Self::DeleteMany => {
                    GeneratePostgresqlTableAttribute::DeleteManyAdditionalErrorVariants
                }
                Self::DeleteOne => {
                    GeneratePostgresqlTableAttribute::DeleteOneAdditionalErrorVariants
                }
            }
        }
        const fn generate_postgresql_table_attribute_additional_logic(
            self,
        ) -> GeneratePostgresqlTableAttribute {
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
        const fn http_method(self) -> OperationHttpMethod {
            match self {
                Self::CreateMany | Self::CreateOne | Self::ReadMany | Self::ReadOne => {
                    OperationHttpMethod::Post
                }
                Self::UpdateMany | Self::UpdateOne => OperationHttpMethod::Patch,
                Self::DeleteMany | Self::DeleteOne => OperationHttpMethod::Delete,
            }
        }
        fn operation_error_named_with_serialize_deserialize_snake_case(
            self,
        ) -> SelfErrorNamedWithSerializeDeserializeSnakeCase {
            SelfErrorNamedWithSerializeDeserializeSnakeCase::from_display(&self)
        }
        fn operation_payload_example_snake_case(
            self,
        ) -> impl naming::StdFmtDisplayPlusQuoteToTokens {
            SelfPayloadExampleSnakeCase::from_display(&self)
        }
        fn self_handle_snake_case_ts(self) -> proc_macro2::TokenStream {
            let value = SelfHandleSnakeCase::from_tokens(&self.self_snake_case_ts());
            quote::quote! {#value}
        }
        fn self_snake_case_stringified(self) -> String {
            AsRefStrToSnakeCaseStringified::case(&self.to_string())
        }
        fn self_snake_case_ts(self) -> proc_macro2::TokenStream {
            AsRefStrToSnakeCaseTokenStream::case_or_panic(&self.to_string())
        }
        fn try_self_handle_snake_case_ts(self) -> proc_macro2::TokenStream {
            let value = TrySelfHandleSnakeCase::from_tokens(&self.self_snake_case_ts());
            quote::quote! {#value}
        }
        fn try_self_snake_case_ts(self) -> proc_macro2::TokenStream {
            let value = TrySelfSnakeCase::from_tokens(&self.self_snake_case_ts());
            quote::quote! {#value}
        }
    }
    impl Display for Operation {
        fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
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
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(naming::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified)]
    enum OperationHttpMethod {
        Post,
        Patch,
        Delete,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum ReadManyOrDeleteMany {
        ReadMany,
        DeleteMany,
    }
    enum ReadManyOrReadOne {
        ReadMany,
        ReadOne,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
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
                Self::CreateManyAdditionalErrorVariants => {
                    naming::CreateManyAdditionalErrorVariantsSnakeCase.to_string()
                }
                Self::CreateOneAdditionalErrorVariants => {
                    naming::CreateOneAdditionalErrorVariantsSnakeCase.to_string()
                }
                Self::ReadManyAdditionalErrorVariants => {
                    naming::ReadManyAdditionalErrorVariantsSnakeCase.to_string()
                }
                Self::ReadOneAdditionalErrorVariants => {
                    naming::ReadOneAdditionalErrorVariantsSnakeCase.to_string()
                }
                Self::UpdateManyAdditionalErrorVariants => {
                    naming::UpdateManyAdditionalErrorVariantsSnakeCase.to_string()
                }
                Self::UpdateOneAdditionalErrorVariants => {
                    naming::UpdateOneAdditionalErrorVariantsSnakeCase.to_string()
                }
                Self::DeleteManyAdditionalErrorVariants => {
                    naming::DeleteManyAdditionalErrorVariantsSnakeCase.to_string()
                }
                Self::DeleteOneAdditionalErrorVariants => {
                    naming::DeleteOneAdditionalErrorVariantsSnakeCase.to_string()
                }
                Self::CommonAdditionalErrorVariants => {
                    naming::CommonAdditionalErrorVariantsSnakeCase.to_string()
                }
                Self::CreateManyAdditionalLogic => {
                    naming::CreateManyAdditionalLogicSnakeCase.to_string()
                }
                Self::CreateOneAdditionalLogic => {
                    naming::CreateOneAdditionalLogicSnakeCase.to_string()
                }
                Self::ReadManyAdditionalLogic => {
                    naming::ReadManyAdditionalLogicSnakeCase.to_string()
                }
                Self::ReadOneAdditionalLogic => naming::ReadOneAdditionalLogicSnakeCase.to_string(),
                Self::UpdateManyAdditionalLogic => {
                    naming::UpdateManyAdditionalLogicSnakeCase.to_string()
                }
                Self::UpdateOneAdditionalLogic => {
                    naming::UpdateOneAdditionalLogicSnakeCase.to_string()
                }
                Self::DeleteManyAdditionalLogic => {
                    naming::DeleteManyAdditionalLogicSnakeCase.to_string()
                }
                Self::DeleteOneAdditionalLogic => {
                    naming::DeleteOneAdditionalLogicSnakeCase.to_string()
                }
                Self::CommonAdditionalLogic => naming::CommonAdditionalLogicSnakeCase.to_string(),
            };
            format!("{}::{value}", naming::PostgresqlCrudSnakeCase)
        }
    }
    enum ShouldWrapIntoValue {
        False,
        True,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum CreateOrUpdateOrDeleteMany {
        Create,
        Update,
        Delete,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum CreateOrUpdateOrDeleteOne {
        Create,
        Update,
        Delete,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize)]
    struct GeneratePostgresqlTableConfig {
        create_many_content_write_into_generate_postgresql_table_create_many:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        create_one_content_write_into_generate_postgresql_table_create_one:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        read_many_content_write_into_generate_postgresql_table_read_many:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        read_one_content_write_into_generate_postgresql_table_read_one:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        update_many_content_write_into_generate_postgresql_table_update_many:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        update_one_content_write_into_generate_postgresql_table_update_one:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        delete_many_content_write_into_generate_postgresql_table_delete_many:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        delete_one_content_write_into_generate_postgresql_table_delete_one:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        tests_content_write_into_generate_postgresql_table_tests:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        common_content_write_into_generate_postgresql_table_common:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_generate_postgresql_table:
            macros_helpers::ShouldWriteTokenStreamIntoFile,
    }
    panic_location::panic_location();
    let generate_select_query_part_snake_case = naming::GenerateSelectQueryPartSnakeCase;
    let create_extension_if_not_exists_pg_jsonschema_upper_camel_case =
        naming::CreateExtensionIfNotExistsPgJsonschemaUpperCamelCase;
    let create_extension_if_not_exists_uuid_ossp_upper_camel_case =
        naming::CreateExtensionIfNotExistsUuidOsspUpperCamelCase;
    let prepare_postgresql_upper_camel_case = naming::PreparePostgresqlUpperCamelCase;
    let pool_snake_case = naming::PoolSnakeCase;
    let value_snake_case = naming::ValueSnakeCase;
    let el_snake_case = naming::ElementSnakeCase;
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
    let error_snake_case = naming::ErrorSnakeCase;
    let query_part_snake_case = naming::QueryPartSnakeCase;
    let query_bind_snake_case = naming::QueryBindSnakeCase;
    let order_by_snake_case = naming::OrderBySnakeCase;
    let response_snake_case = naming::ResponseSnakeCase;
    let status_code_snake_case = naming::StatusCodeSnakeCase;
    let body_snake_case = naming::BodySnakeCase;
    let executor_acquire_snake_case = naming::ExecutorAcquireSnakeCase;
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
    let into_serialize_deserialize_version_snake_case =
        naming::IntoSerializeDeserializeVersionSnakeCase;
    let primary_key_snake_case = naming::PrimaryKeySnakeCase;
    let pagination_snake_case = naming::PaginationSnakeCase;
    let config_snake_case = naming::ConfigSnakeCase;
    let postgres_pool_snake_case = naming::PostgresPoolSnakeCase;
    let ident_create_default_snake_case = naming::IdentCreateDefaultSnakeCase;
    let postgres_pool_for_tokio_spawn_sync_move_snake_case =
        naming::PostgresPoolForTokioSpawnSyncMoveSnakeCase;
    let select_primary_key_snake_case = naming::SelectPrimaryKeySnakeCase;
    let update_for_query_vec_snake_case = naming::UpdateForQueryVecSnakeCase;
    let common_read_only_ids_returned_from_create_one_snake_case =
        naming::CommonReadOnlyIdsReturnedFromCreateOneSnakeCase;
    let select_only_updated_ids_query_part_snake_case =
        naming::SelectOnlyUpdatedIdsQueryPartSnakeCase;
    let update_for_query_snake_case = naming::UpdateForQuerySnakeCase;
    let read_only_ids_snake_case = naming::ReadOnlyIdsSnakeCase;
    let create_snake_case = naming::CreateSnakeCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let postgresql_type_upper_camel_case = naming::PostgresqlTypeUpperCamelCase;
    let create_table_column_query_part_snake_case = naming::CreateTableColumnQueryPartSnakeCase;
    let read_only_ids_merged_with_create_into_where_equal_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoWhereEqualSnakeCase;
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSnakeCase;
    let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoOptionVecWhereEqualToJsonFieldSnakeCase;
    let create_into_postgresql_json_type_option_vec_where_length_equal_snake_case =
        naming::CreateIntoPostgresqlJsonTypeOptionVecWhereLengthEqualSnakeCase;
    let create_into_postgresql_json_type_option_vec_where_length_greater_than_snake_case =
        naming::CreateIntoPostgresqlJsonTypeOptionVecWhereLengthGreaterThanSnakeCase;
    let create_into_postgresql_type_option_vec_where_dimension_one_equal_snake_case =
        naming::CreateIntoPostgresqlTypeOptionVecWhereDimensionOneEqualSnakeCase;
    let read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_snake_case = naming::ReadOnlyIdsMergedWithTableTypeDeclarationIntoPostgresqlTypeOptionWhereGreaterThanSnakeCase;
    let default_option_some_vec_one_el_upper_camel_case =
        naming::DefaultOptionSomeVecOneElUpperCamelCase;
    let default_option_some_vec_one_el_snake_case = naming::DefaultOptionSomeVecOneElSnakeCase;
    let read_only_ids_into_table_type_declaration_snake_case =
        naming::ReadOnlyIdsIntoTableTypeDeclarationSnakeCase;
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereGreaterThanSnakeCase;
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereBetweenSnakeCase;
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereInSnakeCase;
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereRegularExpressionSnakeCase;
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereContainsElGreaterThanSnakeCase;
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_snake_case =
        naming::ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereContainsElRegularExpressionSnakeCase;
    let read_only_ids_into_read_snake_case = naming::ReadOnlyIdsIntoReadSnakeCase;
    let read_only_ids_into_update_snake_case = naming::ReadOnlyIdsIntoUpdateSnakeCase;
    let read_into_table_type_declaration_snake_case = naming::ReadIntoTableTypeDeclarationSnakeCase;
    let prepare_postgresql_snake_case = naming::PreparePostgresqlSnakeCase;
    let prepare_extensions_snake_case = naming::PrepareExtensionsSnakeCase;
    let prepare_postgresql_table_snake_case = naming::PreparePostgresqlTableSnakeCase;
    let option_vec_create_snake_case = naming::OptionVecCreateSnakeCase;
    let postgresql_type_option_vec_where_greater_than_test_snake_case =
        naming::PostgresqlTypeOptionVecWhereGreaterThanTestSnakeCase;
    let routes_handle_snake_case = naming::RoutesHandleSnakeCase;
    let routes_snake_case = naming::RoutesSnakeCase;
    let select_query_part_snake_case = naming::SelectQueryPartSnakeCase;
    let from_handle_snake_case = naming::FromHandleSnakeCase;
    let select_only_ids_query_part_snake_case = naming::SelectOnlyIdsQueryPartSnakeCase;
    let error_0_ts = token_patterns::Error0;
    let error_1_ts = token_patterns::Error1;
    let error_2_ts = token_patterns::Error2;
    let error_3_ts = token_patterns::Error3;
    let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
    let derive_debug_this_error_error_occurence =
        token_patterns::DeriveDebugThisErrorErrorOccurence;
    let sqlx_acquire = token_patterns::SqlxAcquire;
    let derive_debug_serde_serialize_serde_deserialize =
        token_patterns::DeriveDebugSerdeSerializeSerdeDeserialize;
    let ref_std_primitive_str = token_patterns::RefStdPrimitiveStr;
    let field_attribute_serde_skip_serializing_if_option_is_none_ts =
        token_patterns::FieldAttributeSerdeSkipSerializingIfOptionIsNone;
    let sqlx_row = token_patterns::SqlxRow;
    let postgresql_crud_default_option_some_vec_one_el_call_ts =
        token_patterns::PostgresqlCrudDefaultOptionSomeVecOneElCall;
    let string_ts = token_patterns::StdStringString;
    let must_use_ts = token_patterns::MustUse;
    let allow_clippy_arbitrary_source_item_ordering_ts =
        token_patterns::AllowClippyArbitrarySourceItemOrdering;
    let import_path = postgresql_crud_macros_common::ImportPath::PostgresqlCrud;
    let return_err_query_part_error_named_write_into_buffer_ts = postgresql_crud_macros_common::generate_return_err_query_part_error_named_write_into_buffer_ts(import_path);

    // let postgresql_crud_all_variants_default_option_some_vec_one_el_call_ts = token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElCall;
    let syn_derive_input: syn::DeriveInput =
        syn::parse2(input).expect("991c614f-5cf9-4a53-873a-5280b62e2dfa");
    let generate_postgresql_table_config = serde_json::from_str::<GeneratePostgresqlTableConfig>(
        &macros_helpers::get_macro_attribute_meta_list_ts(
            &syn_derive_input.attrs,
            &format!(
                "{}::generate_postgresql_table_config",
                import_path.snake_case_std_primitive_str()
            ),
        )
        .to_string(),
    )
    .expect("1b6adf7e-7da9-4e2d-82e6-e8bb48ba8ee6");
    let ident = &syn_derive_input.ident;
    let ident_snake_case_stringified = naming::ToTokensToSnakeCaseStringified::case(&ident);
    let ident_snake_case_double_quotes_ts =
        generate_quotes::double_quotes_ts(&ident_snake_case_stringified);
    let self_table_name_call_ts = quote::quote! {Self::#table_name_snake_case()};
    let (primary_key_field, fields, fields_without_primary_key) =
        if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
            if let syn::Fields::Named(fields_named) = &data_struct.fields {
                let mut option_primary_key_field: Option<macros_helpers::SynFieldWrapper> = None;
                let mut fields = Vec::new();
                let mut fields_without_primary_key = Vec::new();
                for el_2e7b44a3 in &fields_named.named {
                    let field_ident = el_2e7b44a3
                        .ident
                        .clone()
                        .expect("915ef2ce-d4d5-4943-997a-a2a004807452");
                    let field_ident_len = field_ident.to_string().len();
                    let max_postgresql_column_length = 63;
                    //todo write runtime check
                    assert!(
                        field_ident_len <= max_postgresql_column_length,
                        "1266ae5a-aaef-43a7-a724-c9532e01c7e9"
                    );
                    fields.push(macros_helpers::SynFieldWrapper {
                        field_visibility: el_2e7b44a3.vis.clone(),
                        field_ident: field_ident.clone(),
                        field_type: el_2e7b44a3.ty.clone(),
                    });
                    let mut is_primary_key = false;
                    {
                        for el_f4d3785c in &el_2e7b44a3.attrs {
                            if el_f4d3785c.path().segments.len() == 1 {
                                let first_segment_ident = &el_f4d3785c
                                    .path()
                                    .segments
                                    .first()
                                    .expect("a9c3b38b-0a8d-43b1-a33e-bf3858394ea5")
                                    .ident;
                                let generate_postgresql_table_primary_key_snake_case_stringified =
                                    naming::GeneratePostgresqlTablePrimaryKeySnakeCase.to_string();
                                if first_segment_ident
                                    == &generate_postgresql_table_primary_key_snake_case_stringified
                                {
                                    if option_primary_key_field.is_some() {
                                        panic!("1a75cea1-9961-4f01-a54a-5b4acc08547c");
                                    } else {
                                        option_primary_key_field =
                                            Some(macros_helpers::SynFieldWrapper {
                                                field_visibility: el_2e7b44a3.vis.clone(),
                                                field_ident: field_ident.clone(),
                                                field_type: el_2e7b44a3.ty.clone(),
                                            });
                                        is_primary_key = true;
                                    }
                                }
                            }
                        }
                    }
                    if !is_primary_key {
                        fields_without_primary_key.push(macros_helpers::SynFieldWrapper {
                            field_visibility: el_2e7b44a3.vis.clone(),
                            field_ident: field_ident.clone(),
                            field_type: el_2e7b44a3.ty.clone(),
                        });
                    }
                }
                // explicitly not supporting number of columns more than 100 so its less possibility to cause stack overflow or build process exit
                // assert!((fields.len() <= 100), "d9963f32-0811-48d1-bb60-e6b365a529eb");
                (
                    option_primary_key_field.expect("6a529a99-e9ba-43d6-a6d6-511209caa8b6"),
                    fields,
                    fields_without_primary_key,
                )
            } else {
                panic!("7f31872d-1867-48e0-a1ef-0de268f357a2");
            }
        } else {
            panic!("bd4718d0-4e14-4b22-ad88-b3575bb24eab");
        };
    let fields_len = fields.len();
    let fields_len_without_primary_key = fields_without_primary_key.len();
    let primary_key_field_type = &primary_key_field.field_type;
    let primary_key_field_type_where_ts =
        SelfWhereUpperCamelCase::from_type_last_segment(&primary_key_field.field_type);
    //todo must remove this and use trait type instead
    let primary_key_field_type_table_type_declaration_ts =
        SelfTableTypeDeclarationUpperCamelCase::from_type_last_segment(
            &primary_key_field.field_type,
        );
    let generate_as_postgresql_type_ts = |field_type: &dyn quote::ToTokens| {
        quote::quote! {<#field_type as postgresql_crud::PostgresqlType>::}
    };
    let primary_key_field_type_as_postgresql_type_ts =
        generate_as_postgresql_type_ts(&primary_key_field_type);
    let generate_as_postgresql_type_tokens_ts =
        |field_type: &dyn quote::ToTokens, tokens: &dyn quote::ToTokens| {
            let as_postgresql_type_ts = generate_as_postgresql_type_ts(&field_type);
            quote::quote! {#as_postgresql_type_ts #tokens}
        };
    // let generate_as_postgresql_type_table_type_declaration_ts = |field_type: &dyn quote::ToTokens| generate_as_postgresql_type_tokens_ts(&field_type, &naming::TableTypeDeclarationUpperCamelCase);
    // let primary_key_field_type_as_postgresql_type_table_type_declaration_ts = generate_as_postgresql_type_table_type_declaration_ts(&primary_key_field_type);
    let generate_as_postgresql_type_create_ts = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_ts(&field_type, &naming::CreateUpperCamelCase)
    };
    let generate_as_postgresql_type_select_ts = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_ts(&field_type, &naming::SelectUpperCamelCase)
    };
    let primary_key_field_type_as_postgresql_type_select_ts =
        generate_as_postgresql_type_select_ts(&primary_key_field_type);
    let generate_as_postgresql_type_where_ts = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_ts(&field_type, &naming::WhereUpperCamelCase)
    };
    let primary_key_field_type_as_postgresql_type_where_ts =
        generate_as_postgresql_type_where_ts(&primary_key_field_type);
    let generate_as_postgresql_type_read_ts = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_ts(&field_type, &naming::ReadUpperCamelCase)
    };
    let generate_as_postgresql_type_read_only_ids_ts = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_ts(&field_type, &naming::ReadOnlyIdsUpperCamelCase)
    };
    let primary_key_field_type_as_postgresql_type_read_ts =
        generate_as_postgresql_type_read_ts(&primary_key_field_type);
    let generate_as_postgresql_type_update_ts = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_ts(&field_type, &naming::UpdateUpperCamelCase)
    };
    let generate_as_postgresql_type_update_for_query_ts = |field_type: &dyn quote::ToTokens| {
        generate_as_postgresql_type_tokens_ts(&field_type, &naming::UpdateForQueryUpperCamelCase)
    };
    let primary_key_field_type_as_postgresql_type_read_upper_camel_case = quote::quote! {<#primary_key_field_type as postgresql_crud::#postgresql_type_upper_camel_case>::#read_upper_camel_case};
    let ident_read_only_ids_upper_camel_case = SelfReadOnlyIdsUpperCamelCase::from_tokens(&ident);
    let ident_delete_many_parameters_upper_camel_case =
        SelfDeleteManyParametersUpperCamelCase::from_tokens(&ident);
    let ident_delete_many_payload_upper_camel_case =
        SelfDeleteManyPayloadUpperCamelCase::from_tokens(&ident);
    let ident_delete_one_parameters_upper_camel_case =
        SelfDeleteOneParametersUpperCamelCase::from_tokens(&ident);
    let ident_delete_one_payload_upper_camel_case =
        SelfDeleteOnePayloadUpperCamelCase::from_tokens(&ident);
    let ident_try_read_one_error_named_upper_camel_case =
        SelfTryReadOneErrorNamedUpperCamelCase::from_tokens(&ident);
    let ident_read_one_error_named_with_serialize_deserialize_upper_camel_case =
        SelfReadOneErrorNamedWithSerializeDeserializeUpperCamelCase::from_tokens(&ident);
    let ident_try_delete_one_error_named_upper_camel_case =
        SelfTryDeleteOneErrorNamedUpperCamelCase::from_tokens(&ident);
    let ident_delete_one_error_named_with_serialize_deserialize_upper_camel_case =
        SelfDeleteOneErrorNamedWithSerializeDeserializeUpperCamelCase::from_tokens(&ident);
    let std_vec_vec_primary_key_field_type_read_ts =
        postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(
            &primary_key_field_type_as_postgresql_type_read_upper_camel_case,
        );
    let std_vec_vec_ident_read_only_ids_ts =
        postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(
            &ident_read_only_ids_upper_camel_case,
        );
    let primary_key_field_ident = &primary_key_field.field_ident;
    let primary_key_field_ident_upper_camel_case_ts =
        naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&primary_key_field_ident);
    let primary_key_field_type_update_ts =
        &SelfUpdateUpperCamelCase::from_type_last_segment(primary_key_field_type);
    let primary_key_field_type_update_for_query_ts =
        &SelfUpdateForQueryUpperCamelCase::from_type_last_segment(primary_key_field_type);
    let ident_select_upper_camel_case = SelfSelectUpperCamelCase::from_tokens(&ident);
    let generate_from_handle_ts =
        |ident_ts: &dyn quote::ToTokens, content_ts: &dyn quote::ToTokens| {
            quote::quote! {
                fn #from_handle_snake_case(#value_snake_case: #ident_ts) -> Self {
                    #content_ts
                }
            }
        };
    let generate_select_postgresql_crud_not_empty_unique_vec_ident_select_ts =
        |should_add_borrow: &ShouldAddBorrow| {
            quote::quote! {#select_snake_case: #should_add_borrow postgresql_crud::NotEmptyUniqueVec<#ident_select_upper_camel_case>}
        };
    let select_borrow_postgresql_crud_not_empty_unique_vec_ident_select_ts =
        generate_select_postgresql_crud_not_empty_unique_vec_ident_select_ts(
            &ShouldAddBorrow::True,
        );
    let select_postgresql_crud_not_empty_unique_vec_ident_select_ts =
        generate_select_postgresql_crud_not_empty_unique_vec_ident_select_ts(
            &ShouldAddBorrow::False,
        );
    let pub_select_postgresql_crud_not_empty_unique_vec_ident_select_ts = {
        quote::quote! {pub #select_postgresql_crud_not_empty_unique_vec_ident_select_ts}
    };
    let generate_fields_named_with_comma_ts = |function: &dyn Fn(
        &macros_helpers::SynFieldWrapper,
    )
        -> proc_macro2::TokenStream|
     -> proc_macro2::TokenStream {
        let fields_ts = fields.iter().map(function);
        quote::quote! {#(#fields_ts),*}
    };
    let generate_fields_named_without_comma_ts = |function: &dyn Fn(
        &macros_helpers::SynFieldWrapper,
    )
        -> proc_macro2::TokenStream|
     -> proc_macro2::TokenStream {
        let fields_ts = fields.iter().map(function);
        quote::quote! {#(#fields_ts)*}
    };
    let generate_fields_named_without_primary_key_with_comma_ts = |function: &dyn Fn(&macros_helpers::SynFieldWrapper) -> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_ts = fields_without_primary_key.iter().map(function);
        quote::quote! {#(#fields_ts),*}
    };
    let generate_fields_named_without_primary_key_without_comma_ts = |function: &dyn Fn(&macros_helpers::SynFieldWrapper) -> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_ts = fields_without_primary_key.iter().map(function);
        quote::quote! {#(#fields_ts)*}
    };
    let none_ts = quote::quote! {None};
    let fields_named_with_comma_none_ts = generate_fields_named_with_comma_ts(
        &|_: &macros_helpers::SynFieldWrapper| -> proc_macro2::TokenStream { none_ts.clone() },
    );
    let fields_named_without_primary_key_with_comma_none_ts =
        generate_fields_named_without_primary_key_with_comma_ts(
            &|_: &macros_helpers::SynFieldWrapper| -> proc_macro2::TokenStream { none_ts.clone() },
        );
    let mut impl_ident_vec_ts = Vec::new();
    let impl_ident_ts = {
        let ident_prepare_postgresql_error_named_upper_camel_case =
            SelfPreparePostgresqlErrorNamedUpperCamelCase::from_tokens(&ident);
        let content_ts = quote::quote! {
            #[eo_to_std_string_string]
            error: sqlx::Error,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        };
        let ident_prepare_postgresql_error_named_ts =
            macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_thiserror_error()
                .derive_error_occurence_lib_error_occurence()
                .build_enum(
                    &ident_prepare_postgresql_error_named_upper_camel_case,
                    &quote::quote! {{
                        #create_extension_if_not_exists_pg_jsonschema_upper_camel_case {
                            #content_ts
                        },
                        #create_extension_if_not_exists_uuid_ossp_upper_camel_case {
                            #content_ts
                        },
                        #prepare_postgresql_upper_camel_case {
                            #content_ts
                        },
                    }},
                );
        let pub_fn_table_ts = quote::quote! {
            #must_use_ts
            pub const fn #table_name_snake_case() -> &'static str {
                #ident_snake_case_double_quotes_ts
            }
        };
        let fn_primary_key_ts = {
            let primary_key_field_ident_double_quotes_ts =
                generate_quotes::double_quotes_ts(&primary_key_field_ident);
            quote::quote! {
                const fn #primary_key_snake_case() -> &'static str {
                    #primary_key_field_ident_double_quotes_ts
                }
            }
        };
        let pub_async_fn_prepare_extensions_ts = quote::quote! {
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
        let pub_async_fn_prepare_postgresql_table_ts = {
            let prepare_postgresql_double_quotes_ts = generate_quotes::double_quotes_ts(&format!(
                "create table if not exists {{table}} ({})",
                fields.iter().map(|_| "{}").collect::<Vec<&str>>().join(",")
            ));
            let serde_json_to_string_schemars_schema_for_generic_unwrap_ts = {
                let generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_ts =
                    |field_type: &syn::Type, field_ident: &syn::Ident, is_primary_key: bool| {
                        let is_primary_key_ts: &dyn quote::ToTokens = if is_primary_key {
                            &naming::TrueSnakeCase
                        } else {
                            &naming::FalseSnakeCase
                        };
                        let field_ident_double_quotes_ts =
                            generate_quotes::double_quotes_ts(&field_ident);
                        let field_type_postgresql_type_ts =
                            generate_as_postgresql_type_ts(&field_type);
                        quote::quote! {
                            #field_type_postgresql_type_ts #create_table_column_query_part_snake_case(&#field_ident_double_quotes_ts, #is_primary_key_ts)
                        }
                    };
                once(
                    generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_ts(
                        primary_key_field_type,
                        &primary_key_field.field_ident,
                        true,
                    ),
                )
                .chain(fields_without_primary_key.iter().map(|el_e48e222c| {
                    generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_ts(
                        &el_e48e222c.field_type,
                        &el_e48e222c.field_ident,
                        false,
                    )
                })).collect::<Vec<proc_macro2::TokenStream>>()
            };
            quote::quote! {
                pub async fn #prepare_postgresql_table_snake_case(#pool_snake_case: &sqlx::Pool<sqlx::Postgres>, table: &str) -> Result<(), #ident_prepare_postgresql_error_named_upper_camel_case> {
                    if let Err(error) = sqlx::query(&format!(
                        #prepare_postgresql_double_quotes_ts,
                        #(#serde_json_to_string_schemars_schema_for_generic_unwrap_ts),*
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
        let pub_async_fn_prepare_postgresql_ts = quote::quote! {
            pub async fn #prepare_postgresql_snake_case(#pool_snake_case: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #ident_prepare_postgresql_error_named_upper_camel_case> {
                Self::#prepare_extensions_snake_case(#pool_snake_case).await?;
                Self::#prepare_postgresql_table_snake_case(#pool_snake_case, #ident_snake_case_double_quotes_ts).await?;
                Ok(())
            }
        };
        let pub_fn_allow_methods_ts = {
            let http_method_ts = quote::quote! {http::Method};
            quote::quote! {
                #must_use_ts
                pub const fn allow_methods() -> [#http_method_ts;4] {[
                    #http_method_ts::GET,
                    #http_method_ts::POST,
                    #http_method_ts::PATCH,
                    #http_method_ts::DELETE
                ]}
            }
        };
        let fn_generate_select_query_part_ts = {
            let variants_ts = generate_fields_named_with_comma_ts(
                &|element: &macros_helpers::SynFieldWrapper| {
                    let field_ident_upper_camel_case_ts =
                        naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(
                            &element.field_ident,
                        );
                    let initialization_ts = {
                        let field_ident_string_double_quotes_ts =
                            generate_quotes::double_quotes_ts(&element.field_ident);
                        let as_postgresql_crud_postgresql_type_postgresql_type_ts =
                            generate_as_postgresql_type_ts(&element.field_type);
                        quote::quote! {
                            => match #as_postgresql_crud_postgresql_type_postgresql_type_ts #select_query_part_snake_case(
                                #column_snake_case,
                                #field_ident_string_double_quotes_ts
                            ) {
                                Ok(value_820e1163) => value_820e1163,
                                Err(#error_snake_case) => {
                                    return Err(#error_snake_case);
                                }
                            }
                        }
                    };
                    quote::quote! {#ident_select_upper_camel_case::#field_ident_upper_camel_case_ts(#column_snake_case) #initialization_ts}
                },
            );
            let std_option_option_std_primitive_char_ts =
                postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(
                    &token_patterns::StdPrimitiveChar,
                );
            quote::quote! {
                fn #generate_select_query_part_snake_case(#select_borrow_postgresql_crud_not_empty_unique_vec_ident_select_ts) -> Result<#string_ts, #import_path ::#query_part_error_named_upper_camel_case> {
                    let mut acc_37c883c3 = #string_ts::default();
                    for el_78d2ec39 in #select_snake_case.to_vec() {
                        acc_37c883c3.push_str(&match el_78d2ec39 {
                            #variants_ts
                        });
                        acc_37c883c3.push(',');
                    }
                    let _: #std_option_option_std_primitive_char_ts = acc_37c883c3.pop();
                    Ok(acc_37c883c3)
                }
            }
        };
        impl_ident_vec_ts.push(quote::quote! {
            #pub_fn_table_ts
            #fn_primary_key_ts
            #pub_async_fn_prepare_extensions_ts
            #pub_async_fn_prepare_postgresql_table_ts
            #pub_async_fn_prepare_postgresql_ts
            #pub_fn_allow_methods_ts
            #fn_generate_select_query_part_ts
        });
        quote::quote! {
            #ident_prepare_postgresql_error_named_ts
        }
    };
    let wrap_into_axum_response_ts =
        |axum_json_content_ts: &dyn quote::ToTokens,
         status_code_ts: &dyn quote::ToTokens,
         should_add_return: &ShouldAddReturn| {
            let return_content_ts = match should_add_return {
                ShouldAddReturn::False => quote::quote! {response},
                ShouldAddReturn::True => quote::quote! {return response;},
            };
            quote::quote! {
                let mut response = axum::response::IntoResponse::into_response(
                    axum::Json(#axum_json_content_ts)
                );
                *response.status_mut() = #status_code_ts;
                #return_content_ts
            }
        };
    let generate_ident_operation_error_named_upper_camel_case = |operation: &Operation| {
        format!("{ident}{operation}ErrorNamed")
            .parse::<proc_macro2::TokenStream>()
            .expect("79ab147e-f603-4cb7-81af-2e35344780fe")
    };
    let generate_ident_operation_response_variants_upper_camel_case = |operation: &Operation| {
        format!("{ident}{operation}ResponseVariants")
            .parse::<proc_macro2::TokenStream>()
            .expect("f386c0d4-6704-475a-8045-91431f1da815")
    };
    let generate_initialization_ts = |syn_variant_wrapper: &SynVariantWrapper,
                                      file: &'static str,
                                      line: u32,
                                      column: u32|
     -> proc_macro2::TokenStream {
        let variant_ident = &syn_variant_wrapper.variant.ident;
        let fields_ts = if let syn::Fields::Named(value) = &syn_variant_wrapper.variant.fields {
            value.named.iter().enumerate().map(|(index, element)| {
                let field_ident = &element.ident;
                if *field_ident
                    .as_ref()
                    .expect("edbbd08a-ab4b-4553-ae90-e97c714f7908")
                    == naming::CodeOccurenceSnakeCase.to_string()
                {
                    macros_helpers::generate_field_code_occurence_new_ts(file, line, column)
                } else {
                    let error_increment_snake_case = ErrorSelfSnakeCase::from_display(&index);
                    quote::quote! {#field_ident: #error_increment_snake_case}
                }
            })
        } else {
            panic!("10773d36-d47f-4563-98f2-118d9ac519fc");
        };
        quote::quote! {
            #variant_ident {
                #(#fields_ts),*
            }
        }
    };
    let generate_operation_error_initialization_eprintln_response_creation_ts =
        |operation: &Operation,
         syn_variant_wrapper: &SynVariantWrapper,
         file: &'static str,
         line: u32,
         column: u32| {
            let ident_operation_error_named_upper_camel_case =
                generate_ident_operation_error_named_upper_camel_case(operation);
            let ident_operation_response_variants_upper_camel_case =
                generate_ident_operation_response_variants_upper_camel_case(operation);
            let syn_variant_initialization_ts =
                generate_initialization_ts(syn_variant_wrapper, file, line, column);
            let status_code_ts = syn_variant_wrapper
                .get_option_status_code()
                .expect("81efa954-1175-4f89-a343-c79f8184e19b")
                .to_axum_http_status_code_ts();
            let wraped_into_axum_response_ts = wrap_into_axum_response_ts(
                &quote::quote! {#ident_operation_response_variants_upper_camel_case::#from_handle_snake_case(#error_snake_case)},
                &status_code_ts,
                &ShouldAddReturn::True,
            );
            quote::quote! {
                let #error_snake_case = #ident_operation_error_named_upper_camel_case::#syn_variant_initialization_ts;
                // eprintln!("{error}");
                #wraped_into_axum_response_ts
            }
        };
    let new_syn_variant_wrapper = |variant_name: &dyn Display,
                                   status_code: Option<macros_helpers::StatusCode>,
                                   current_fields: Vec<(
        ErrorOccurenceFieldAttribute,
        &dyn Display,
        Punctuated<syn::PathSegment, PathSep>,
    )>|
     -> SynVariantWrapper {
        SynVariantWrapper {
            variant: syn::Variant {
                attrs: {
                    let mut attributes = Vec::new();
                    if let Some(value) = status_code.as_ref() {
                        let mut segments = Punctuated::new();
                        segments.push(syn::PathSegment {
                            ident: proc_macro2::Ident::new(
                                &AsRefStrToSnakeCaseStringified::case(value),
                                proc_macro2::Span::call_site(),
                            ),
                            arguments: syn::PathArguments::None,
                        });
                        attributes.push(syn::Attribute {
                            pound_token: Pound {
                                spans: [proc_macro2::Span::call_site()],
                            },
                            style: syn::AttrStyle::Outer,
                            bracket_token: Bracket::default(),
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
                    brace_token: Brace::default(),
                    named: {
                        let mut handle = current_fields.into_iter().fold(Punctuated::new(), |mut acc_37be2059, element| {
                            acc_37be2059.push_value(syn::Field {
                                attrs: vec![syn::Attribute {
                                    pound_token: Pound { spans: [proc_macro2::Span::call_site()] },
                                    style: syn::AttrStyle::Outer,
                                    bracket_token: Bracket::default(),
                                    meta: syn::Meta::Path(syn::Path {
                                        leading_colon: None,
                                        segments: {
                                            let mut handle = Punctuated::new();
                                            handle.push(syn::PathSegment {
                                                ident: proc_macro2::Ident::new(macros_helpers::AttributeIdentStringified::attribute_ident_stringified(&element.0), proc_macro2::Span::call_site()),
                                                arguments: syn::PathArguments::None,
                                            });
                                            handle
                                        },
                                    }),
                                }],
                                vis: syn::Visibility::Inherited,
                                mutability: syn::FieldMutability::None,
                                ident: Some(syn::Ident::new(&element.1.to_string(), proc_macro2::Span::call_site())),
                                colon_token: Some(Colon { spans: [proc_macro2::Span::call_site()] }),
                                ty: syn::Type::Path(syn::TypePath {
                                    qself: None,
                                    path: syn::Path { leading_colon: None, segments: element.2 },
                                }),
                            });
                            acc_37be2059.push_punct(Comma { spans: [proc_macro2::Span::call_site()] });
                            acc_37be2059
                        });
                        handle.push_value(macros_helpers::code_occurence_syn_field());
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
        Some(macros_helpers::StatusCode::BadRequest400),
        vec![(
            ErrorOccurenceFieldAttribute::EoErrorOccurence,
            &naming::ErrorSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated(&[
                &postgresql_crud_snake_case.to_string(),
                &query_part_error_named_upper_camel_case.to_string(),
            ]),
        )],
    );
    let generate_select_query_part_parameters_payload_select_ts = |operation: &Operation| {
        let content_ts_59c8df3f =
            generate_operation_error_initialization_eprintln_response_creation_ts(
                operation,
                &query_part_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
            );
        quote::quote! {
            match Self::#generate_select_query_part_snake_case(&#parameters_snake_case.#payload_snake_case.#select_snake_case) {
                Ok(value_357219fb) => value_357219fb,
                Err(#error_0_ts) => {
                    #content_ts_59c8df3f
                }
            }
        }
    };
    let ident_read_upper_camel_case = SelfReadUpperCamelCase::from_tokens(&ident);
    let generate_value_declaration_ts = |content_ts: &dyn quote::ToTokens| {
        quote::quote! {#postgresql_crud_snake_case::#value_upper_camel_case<#content_ts>}
    };
    let generate_import_path_value_initialization_ts = |content_ts: &dyn quote::ToTokens| {
        postgresql_crud_macros_common::generate_value_initialization_ts(&import_path, &content_ts)
    };
    let generate_impl_postgresql_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts =
        |current_ident: &dyn quote::ToTokens, content_ts: &dyn quote::ToTokens| {
            postgresql_crud_macros_common::generate_impl_postgresql_crud_default_option_some_vec_one_el_ts(
        &current_ident,
        &proc_macro2::TokenStream::new(),
        &content_ts
    )
        };
    let ident_create_upper_camel_case = SelfCreateUpperCamelCase::from_tokens(&ident);
    let ident_create_ts = {
        let ident_create_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .derive_utoipa_to_schema()
            .build_struct(&ident_create_upper_camel_case, &{
                let content_ts = generate_fields_named_without_primary_key_with_comma_ts(
                    &|element: &macros_helpers::SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let el_syn_field_ty_as_postgresql_type_create_ts =
                            generate_as_postgresql_type_create_ts(&element.field_type);
                        quote::quote! {
                            pub #field_ident: #el_syn_field_ty_as_postgresql_type_create_ts
                        }
                    },
                );
                quote::quote! {{#content_ts}}
            });
        let impl_ident_create_ts = {
            let primary_key_field_type_as_default_option_some_vec_one_el_call_ts = {
                let primary_key_field_type_as_postgresql_type_create_ts =
                    generate_as_postgresql_type_create_ts(&primary_key_field_type);
                quote::quote! {
                    <
                        #primary_key_field_type_as_postgresql_type_create_ts as #postgresql_crud_snake_case::#default_option_some_vec_one_el_upper_camel_case
                    >::#default_option_some_vec_one_el_snake_case()
                }
            };
            let fn_create_query_part_ts = {
                let generate_match_as_postgresql_crud_postgresql_type_postgresql_type_create_query_part_ts =
                    |field_type: &syn::Type, content_ts: &dyn quote::ToTokens| {
                        let as_postgresql_crud_postgresql_type_postgresql_type_ts =
                            generate_as_postgresql_type_ts(&field_type);
                        let if_write_is_err_ts = macros_helpers::generate_if_write_is_err_ts(
                            &quote::quote! {acc_a097110b, "{value_c3f0b59a},"},
                            &return_err_query_part_error_named_write_into_buffer_ts,
                        );
                        quote::quote! {
                            match #as_postgresql_crud_postgresql_type_postgresql_type_ts #create_query_part_snake_case(
                                &#content_ts,
                                #increment_snake_case
                            ) {
                                Ok(value_c3f0b59a) => {
                                    #if_write_is_err_ts
                                }
                                Err(#error_0_ts) => {
                                    return Err(#error_0_ts);
                                }
                            }
                        }
                    };
                let primary_key_content_ts = generate_match_as_postgresql_crud_postgresql_type_postgresql_type_create_query_part_ts(primary_key_field_type, &primary_key_field_type_as_default_option_some_vec_one_el_call_ts);
                let column_increments_ts =
                    generate_fields_named_without_primary_key_without_comma_ts(
                        &|element: &macros_helpers::SynFieldWrapper| {
                            generate_match_as_postgresql_crud_postgresql_type_postgresql_type_create_query_part_ts(&element.field_type, &{
                        let el_field_ident = &element.field_ident;
                        quote::quote! {self.#el_field_ident}
                    })
                        },
                    );
                quote::quote! {
                    fn #create_query_part_snake_case(&self, #increment_snake_case: &mut u64) -> Result<#string_ts, postgresql_crud::#query_part_error_named_upper_camel_case> {
                        let mut acc_a097110b = String::default();
                        #primary_key_content_ts
                        #column_increments_ts
                        let _: Option<char> = acc_a097110b.pop();
                        Ok(acc_a097110b)
                    }
                }
            };
            let fn_create_query_bind_ts = {
                let generate_query_as_postgresql_crud_postgresql_type_postgresql_type_create_query_bind_ts =
                    |field_type: &syn::Type, content_ts: &dyn quote::ToTokens| {
                        let as_postgresql_crud_postgresql_type_postgresql_type_ts =
                            generate_as_postgresql_type_ts(&field_type);
                        quote::quote! {
                            match #as_postgresql_crud_postgresql_type_postgresql_type_ts #create_query_bind_snake_case(
                                #content_ts,
                                #query_snake_case
                            ) {
                                Ok(value_3c55d2e1) => {
                                    #query_snake_case = value_3c55d2e1;
                                },
                                Err(#error_0_ts) => {
                                    return Err(#error_0_ts);
                                }
                            }
                        }
                    };
                let primary_key_content_ts = generate_query_as_postgresql_crud_postgresql_type_postgresql_type_create_query_bind_ts(primary_key_field_type, &primary_key_field_type_as_default_option_some_vec_one_el_call_ts);
                let binded_query_modifications_ts =
                    generate_fields_named_without_primary_key_without_comma_ts(
                        &|element: &macros_helpers::SynFieldWrapper| {
                            generate_query_as_postgresql_crud_postgresql_type_postgresql_type_create_query_bind_ts(&element.field_type, &{
                        let field_ident = &element.field_ident;
                        quote::quote! {self.#field_ident}
                    })
                        },
                    );
                quote::quote! {
                    fn #create_query_bind_snake_case(self, mut #query_snake_case: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
                        sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                        String
                    > {
                        #primary_key_content_ts
                        #binded_query_modifications_ts
                        Ok(#query_snake_case)
                    }
                }
            };
            quote::quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                impl #ident_create_upper_camel_case {
                    #fn_create_query_part_ts
                    #fn_create_query_bind_ts
                }
            }
        };
        let impl_postgresql_crud_default_option_some_vec_one_el_for_ident_create_ts =
            generate_impl_postgresql_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                &ident_create_upper_camel_case,
                &{
                    let fields_initialiation_without_primary_key_with_default_option_some_vec_one_el_ts =
                        generate_fields_named_without_primary_key_with_comma_ts(
                            &|element: &macros_helpers::SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                quote::quote! {#field_ident: #postgresql_crud_default_option_some_vec_one_el_call_ts}
                            },
                        );
                    quote::quote! {
                        Self{#fields_initialiation_without_primary_key_with_default_option_some_vec_one_el_ts}
                    }
                },
            );
        quote::quote! {
            #ident_create_ts
            #impl_ident_create_ts
            #impl_postgresql_crud_default_option_some_vec_one_el_for_ident_create_ts
        }
    };
    let ident_where_many_upper_camel_case = SelfWhereManyUpperCamelCase::from_tokens(&ident);
    let ident_where_many_try_new_error_named_upper_camel_case =
        SelfWhereManyTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
    let ident_where_many_ts = {
        let fields_declaration_ts = generate_fields_named_with_comma_ts(
            &|element: &macros_helpers::SynFieldWrapper| -> proc_macro2::TokenStream {
                let field_ident = &element.field_ident;
                let el_syn_field_ty_as_postgresql_type_where_ts =
                    generate_as_postgresql_type_where_ts(&element.field_type);
                let std_option_option_postgresql_type_where_syn_field_ty_as_postgresql_type_where_ts =
                    postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(
                        &quote::quote! {postgresql_crud::PostgresqlTypeWhere<#el_syn_field_ty_as_postgresql_type_where_ts>},
                    );
                quote::quote! {
                    #field_ident: #std_option_option_postgresql_type_where_syn_field_ty_as_postgresql_type_where_ts
                }
            },
        );
        let ident_where_many_ts = {
            let content_ts_2ecd6da8 = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_serde_serialize()
                .derive_utoipa_to_schema()
                .build_struct(
                    &ident_where_many_upper_camel_case,
                    &quote::quote! {{#fields_declaration_ts}},
                );
            quote::quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                #content_ts_2ecd6da8
            }
        };
        let ident_where_many_try_new_error_named_ts =
            macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_thiserror_error()
                .derive_error_occurence_lib_error_occurence()
                .build_enum(
                    &ident_where_many_try_new_error_named_upper_camel_case,
                    &quote::quote! {{
                        #no_fields_provided_upper_camel_case {
                            #[eo_to_std_string_string]
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        }
                    }},
                );
        let impl_pub_try_new_for_ident_where_many_ts =
            macros_helpers::generate_impl_pub_try_new_for_ident_ts(
                &ident_where_many_upper_camel_case,
                &fields_declaration_ts,
                &ident_where_many_try_new_error_named_upper_camel_case,
                &{
                    let generate_fields_ts = |should_add_borrow: ShouldAddBorrow| {
                        generate_fields_named_with_comma_ts(
                            &|element: &macros_helpers::SynFieldWrapper| -> proc_macro2::TokenStream {
                                let field_ident = &element.field_ident;
                                quote::quote! {#should_add_borrow #field_ident}
                            },
                        )
                    };
                    let fields_ts = generate_fields_ts(ShouldAddBorrow::True);
                    let fields_inialization_ts = generate_fields_ts(ShouldAddBorrow::False);
                    quote::quote! {
                        if matches!((#fields_ts), (#fields_named_with_comma_none_ts)) {
                            return Err(#ident_where_many_try_new_error_named_upper_camel_case::#no_fields_provided_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        Ok(Self {#fields_inialization_ts})
                    }
                },
            );
        let impl_serde_deserialize_for_ident_where_many_ts =
            postgresql_crud_macros_common::generate_impl_serde_deserialize_for_struct_ts(
                &ident_where_many_upper_camel_case,
                &fields
                    .iter()
                    .map(|el_3e09c5fb| (&el_3e09c5fb.field_ident, &el_3e09c5fb.field_type))
                    .collect::<Vec<(&syn::Ident, &syn::Type)>>(),
                fields_len,
                &|_: &syn::Ident, syn_type: &syn::Type| {
                    let syn_type_as_postgresql_type_where_ts =
                        generate_as_postgresql_type_where_ts(&syn_type);
                    postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(
                        &quote::quote! {postgresql_crud::PostgresqlTypeWhere<#syn_type_as_postgresql_type_where_ts>},
                    )
                },
            );
        let impl_postgresql_crud_default_option_some_vec_one_el_for_ident_where_many_ts =
            generate_impl_postgresql_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                &ident_where_many_upper_camel_case,
                &{
                    let fields_ts = generate_fields_named_without_comma_ts(
                        &|el_0fd667f6: &macros_helpers::SynFieldWrapper| {
                            let field_ident = &el_0fd667f6.field_ident;
                            quote::quote! {
                                #field_ident: Some(
                                    #postgresql_crud_default_option_some_vec_one_el_call_ts
                                ),
                            }
                        },
                    );
                    quote::quote! {Self{#fields_ts}}
                },
            );
        quote::quote! {
            #ident_where_many_ts
            #ident_where_many_try_new_error_named_ts
            #impl_pub_try_new_for_ident_where_many_ts
            #impl_serde_deserialize_for_ident_where_many_ts
            #impl_postgresql_crud_default_option_some_vec_one_el_for_ident_where_many_ts
        }
    };

    let std_option_option_ident_where_many_upper_camel_case =
        StdOptionOptionSelfWhereManyUpperCamelCase::from_tokens(&ident);
    let std_option_option_ident_where_many_ts = {
        let std_option_option_ident_where_many_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .derive_utoipa_to_schema()
            .build_struct(
                &std_option_option_ident_where_many_upper_camel_case,
                &{
                    let std_option_option_ident_read_only_ids_standart_not_null_ts = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(
                        &ident_where_many_upper_camel_case
                    );
                    quote::quote!{(pub #std_option_option_ident_read_only_ids_standart_not_null_ts);}
                }
            );
        let impl_postgresql_type_where_filter_for_std_option_option_ident_where_many_ts =
            postgresql_crud_macros_common::impl_postgresql_type_where_filter_for_ident_ts(
                &quote::quote! {<'lifetime>},
                &std_option_option_ident_where_many_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &postgresql_crud_macros_common::IncrementParameterUnderscore::False,
                &postgresql_crud_macros_common::ColumnParameterUnderscore::True,
                &postgresql_crud_macros_common::IsNeedToAddLogicalOperatorUnderscore::True,
                &{
                    let additional_parameters_modification_ts = fields.iter().enumerate().map(|(index, element)| {
                    let field_ident = &element.field_ident;
                    let field_ident_double_quotes_ts = generate_quotes::double_quotes_ts(&field_ident);
                    let maybe_is_first_push_to_additional_parameters_already_happend_true_ts = if index == fields_len_without_primary_key {
                        proc_macro2::TokenStream::new()
                    } else {
                        quote::quote! {is_first_push_to_additional_parameters_already_happend = true;}
                    };
                    quote::quote! {
                        if let Some(value_da0f0616) = &#value_snake_case.#field_ident {
                            match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                                value_da0f0616,
                                increment,
                                &#field_ident_double_quotes_ts,
                                is_first_push_to_additional_parameters_already_happend,
                            ) {
                                Ok(value_9e3f8fdd) => {
                                    #additional_parameters_snake_case.push_str(&value_9e3f8fdd);
                                    #maybe_is_first_push_to_additional_parameters_already_happend_true_ts
                                }
                                Err(#error_0_ts) => {
                                    return Err(#error_0_ts);
                                }
                            }
                        }
                    }
                });
                    quote::quote! {
                        Ok(match &self.0 {
                            Some(value) => {
                                let mut #additional_parameters_snake_case = #string_ts::from("where");
                                let mut is_first_push_to_additional_parameters_already_happend = false;
                                #(#additional_parameters_modification_ts)*
                                #additional_parameters_snake_case
                            },
                            None => #string_ts::default()
                        })
                    }
                },
                &postgresql_crud_macros_common::IsQueryBindMutable::True,
                &{
                    let binded_query_modifications_ts = generate_fields_named_without_comma_ts(
                        &|element: &macros_helpers::SynFieldWrapper| {
                            let field_ident = &element.field_ident;
                            quote::quote! {
                                if let Some(value_b12d6fe0) = value_27176ffb.#field_ident {
                                    match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value_b12d6fe0, #query_snake_case) {
                                        Ok(value_edaee3b2) => {
                                            #query_snake_case = value_edaee3b2;
                                        },
                                        Err(#error_0_ts) => {
                                            return Err(#error_0_ts);
                                        }
                                    }
                                }
                            }
                        },
                    );
                    quote::quote! {
                        if let Some(value_27176ffb) = self.0 {
                            #binded_query_modifications_ts
                        }
                        Ok(#query_snake_case)
                    }
                },
                &postgresql_crud_macros_common::ImportPath::PostgresqlCrud,
            );
        let impl_postgresql_crud_default_option_some_vec_one_el_for_std_option_option_ident_where_many_ts =
            generate_impl_postgresql_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                &std_option_option_ident_where_many_upper_camel_case,
                &quote::quote! {Self(Some(#postgresql_crud_default_option_some_vec_one_el_call_ts))},
            );
        quote::quote! {
            #std_option_option_ident_where_many_ts
            #impl_postgresql_type_where_filter_for_std_option_option_ident_where_many_ts
            #impl_postgresql_crud_default_option_some_vec_one_el_for_std_option_option_ident_where_many_ts
        }
    };
    let pub_where_many_std_option_option_ident_where_many_ts = quote::quote! {pub #where_many_snake_case: #std_option_option_ident_where_many_upper_camel_case};
    let where_many_postgresql_crud_default_option_some_vec_one_el_call_ts = quote::quote! {
        #where_many_snake_case: #postgresql_crud_default_option_some_vec_one_el_call_ts
    };
    let generate_read_or_delete_many_additional_paramaters_initialization_ts =
        |read_many_or_delete_many: &ReadManyOrDeleteMany| {
            let content_ts_b34ec240 =
                generate_operation_error_initialization_eprintln_response_creation_ts(
                    &Operation::from(read_many_or_delete_many),
                    &query_part_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            quote::quote! {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                    &#parameters_snake_case.#payload_snake_case.#where_many_snake_case,
                    &mut #increment_snake_case,
                    &"",//useless //todo check if can be optimized
                    false//useless
                ) {
                    Ok(value_d1627695) => value_d1627695,
                    Err(#error_0_ts) => {
                        #content_ts_b34ec240
                    }
                }
            }
        };
    let macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize =
        ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize;
    let string_syn_punctuated_punctuated =
        macros_helpers::generate_simple_syn_punctuated_punctuated(&["String"]);
    let try_bind_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::TryBindUpperCamelCase,
        Some(macros_helpers::StatusCode::InternalServerError500),
        vec![(macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize, &naming::TryBindSnakeCase, string_syn_punctuated_punctuated.clone())],
    );
    let generate_query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_ts =
        |operation: &Operation| {
            let content_ts_818208f4 =
                generate_operation_error_initialization_eprintln_response_creation_ts(
                    operation,
                    &try_bind_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            quote::quote! {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(#parameters_snake_case.#payload_snake_case.#where_many_snake_case, #query_snake_case) {
                    Ok(value_03a58371) => {
                        #query_snake_case = value_03a58371;
                    },
                    Err(#error_0_ts) => {
                        #content_ts_818208f4
                    },
                }
            }
        };
    let try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_ident_select_snake_case =
        TryFromSqlxPostgresPgRowWithNotEmptyUniqueVecSelfSelectSnakeCase::from_display(&ident);
    let sqlx_error_syn_punctuated_punctuated =
        macros_helpers::generate_simple_syn_punctuated_punctuated(&["sqlx", "Error"]);
    let macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string =
        ErrorOccurenceFieldAttribute::EoToStdStringString;
    let postgresql_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::PostgresqlUpperCamelCase,
        Some(macros_helpers::StatusCode::InternalServerError500),
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
            &naming::PostgresqlSnakeCase,
            sqlx_error_syn_punctuated_punctuated.clone(),
        )],
    );
    let generate_match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_ident_select_ts =
        |read_many_or_read_one: &ReadManyOrReadOne| {
            let content_ts_995d3d1d =
                generate_operation_error_initialization_eprintln_response_creation_ts(
                    &Operation::from(read_many_or_read_one),
                    &postgresql_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            quote::quote! {
                match #ident_read_upper_camel_case::#try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_ident_select_snake_case(
                    &value_b27d7d79,
                    &#parameters_snake_case.#payload_snake_case.#select_snake_case
                ) {
                    Ok(value_90535a1d) => value_90535a1d,
                    Err(#error_0_ts) => {
                        #content_ts_995d3d1d
                    }
                }
            }
        };
    let select_ts = {
        let ident_select_ts = {
            let content_ts_179037cd = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_partial_eq()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .build_enum(
                &ident_select_upper_camel_case,
                &{
                    let variants = generate_fields_named_with_comma_ts(&|element: &macros_helpers::SynFieldWrapper| {
                        let serialize_deserialize_ident_ts = generate_quotes::double_quotes_ts(&element.field_ident);
                        let field_ident_upper_camel_case_ts = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                        let el_syn_field_ty_as_postgresql_type_select_ts = generate_as_postgresql_type_select_ts(&element.field_type);
                        quote::quote! {
                            #[serde(rename(serialize = #serialize_deserialize_ident_ts, deserialize = #serialize_deserialize_ident_ts))]
                            #field_ident_upper_camel_case_ts(#el_syn_field_ty_as_postgresql_type_select_ts)
                        }
                    });
                    quote::quote!{{#variants}}
                }
            );
            quote::quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                #content_ts_179037cd
            }
        };
        let impl_std_fmt_display_for_ident_select_ts =
            macros_helpers::generate_impl_std_fmt_display_ts(
                &proc_macro2::TokenStream::new(),
                &ident_select_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {write!(f, "{}", serde_json::to_string(&self).unwrap_or_else(|el_2636212f|format!("cannot serialize into json: {el_2636212f:?}")))},
            );
        let impl_error_occurence_lib_to_std_string_string_for_ident_select_ts =
            macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_ts(
                &proc_macro2::TokenStream::new(),
                &ident_select_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &quote::quote! {format!("{self}")},
            );
        let impl_postgresql_crud_all_variants_default_option_some_vec_one_el_for_ident_select_ts = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_variants_default_option_some_vec_one_el_ts(&ident_select_upper_camel_case, &{
            let elements_ts = generate_fields_named_with_comma_ts(&|el_5282570d: &macros_helpers::SynFieldWrapper| {
                let field_ident_upper_camel_case_ts = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&el_5282570d.field_ident);
                quote::quote! {
                    Self::#field_ident_upper_camel_case_ts(#postgresql_crud_default_option_some_vec_one_el_call_ts)
                }
            });
            quote::quote! {vec![#elements_ts]}
        });
        quote::quote! {
            #ident_select_ts
            #impl_std_fmt_display_for_ident_select_ts
            #impl_error_occurence_lib_to_std_string_string_for_ident_select_ts
            #impl_postgresql_crud_all_variants_default_option_some_vec_one_el_for_ident_select_ts
        }
    };
    let select_postgresql_crud_default_option_some_vec_one_el_call_ts = quote::quote! {
        #select_snake_case: #postgresql_crud_default_option_some_vec_one_el_call_ts
    };
    let ident_read_ts = {
        let ident_read_ts = {
            let content_ts_f80f1f3e = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_partial_eq()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .build_struct(
                &ident_read_upper_camel_case,
                &{
                    let field_option_primary_key_ts = {
                        let std_option_option_value_primary_key_field_type_as_postgresql_type_read_ts = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&generate_value_declaration_ts(&generate_as_postgresql_type_read_ts(&primary_key_field_type)));
                        quote::quote! {
                            #field_attribute_serde_skip_serializing_if_option_is_none_ts
                            pub #primary_key_field_ident: #std_option_option_value_primary_key_field_type_as_postgresql_type_read_ts
                        }
                    };
                    let fields_options_without_primary_key_ts = generate_fields_named_without_primary_key_with_comma_ts(&|element: &macros_helpers::SynFieldWrapper| -> proc_macro2::TokenStream {
                        let field_visibility = &element.field_visibility;
                        let field_ident = &element.field_ident;
                        let std_option_option_value_field_type_as_postgresql_type_read_ts = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&generate_value_declaration_ts(&generate_as_postgresql_type_read_ts(&element.field_type)));
                        quote::quote! {
                            #field_attribute_serde_skip_serializing_if_option_is_none_ts
                            #field_visibility #field_ident: #std_option_option_value_field_type_as_postgresql_type_read_ts
                        }
                    });
                    quote::quote!{{
                        #field_option_primary_key_ts,
                        #fields_options_without_primary_key_ts
                    }}
                }
            );
            quote::quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                #content_ts_f80f1f3e
            }
        };
        let impl_ident_read_ts = {
            let fn_try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_ident_select_ts = {
                let declaration_primary_key_ts = {
                    let std_option_option_value_primary_key_field_type_as_primary_key_ts = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&generate_value_declaration_ts(&primary_key_field_type_as_postgresql_type_read_upper_camel_case));
                    quote::quote! {
                        let mut #primary_key_field_ident: #std_option_option_value_primary_key_field_type_as_primary_key_ts = None;
                    }
                };
                let declaration_without_primary_key_ts =
                    generate_fields_named_without_primary_key_without_comma_ts(
                        &|element: &macros_helpers::SynFieldWrapper| {
                            let field_ident = &element.field_ident;
                            let std_option_option_value_field_type_as_postgresql_type_read_ts = postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&generate_value_declaration_ts(&generate_as_postgresql_type_read_ts(&element.field_type)));
                            quote::quote! {
                                let mut #field_ident: #std_option_option_value_field_type_as_postgresql_type_read_ts = None;
                            }
                        },
                    );
                //todo reuse code?
                let assignment_variant_primary_key_ts = {
                    let primary_key_field_ident_string_double_quotes_ts =
                        generate_quotes::double_quotes_ts(&primary_key_field_ident);
                    quote::quote! {
                        #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_ts(_) => match sqlx::Row::try_get::<
                            #primary_key_field_type_as_postgresql_type_read_upper_camel_case,
                            #ref_std_primitive_str
                        >(
                            value,
                            #primary_key_field_ident_string_double_quotes_ts
                        ) {
                            Ok(value_dccdf117) => {
                                #primary_key_field_ident = Some(#import_path::#value_upper_camel_case { value: value_dccdf117});
                            },
                            Err(#error_0_ts) => {
                                return Err(#error_0_ts);
                            }
                        }
                    }
                };
                let assignment_variants_without_primary_key_ts = fields_without_primary_key
                    .iter()
                    .map(|el_3ce946f9| {
                        let field_ident = &el_3ce946f9.field_ident;
                        let field_ident_upper_camel_case_ts = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&el_3ce946f9.field_ident);
                        let field_ident_string_double_quotes_ts = generate_quotes::double_quotes_ts(&el_3ce946f9.field_ident);
                        let el_syn_field_ty_as_postgresql_type_read_ts = generate_as_postgresql_type_read_ts(&el_3ce946f9.field_type);
                        quote::quote! {
                            #ident_select_upper_camel_case::#field_ident_upper_camel_case_ts(_) => match sqlx::Row::try_get::<
                                #el_syn_field_ty_as_postgresql_type_read_ts,
                                #ref_std_primitive_str
                            >(
                                value,
                                #field_ident_string_double_quotes_ts
                            ) {
                                Ok(value_09b0fc09) => {
                                    #field_ident = Some(#import_path::#value_upper_camel_case { value: value_09b0fc09});
                                },
                                Err(#error_0_ts) => {
                                    return Err(#error_0_ts);
                                }
                            }
                        }
                    })
                    .collect::<Vec<proc_macro2::TokenStream>>();
                let fields_initiation_ts = &fields
                    .iter()
                    .map(|el_2bfe6afc| &el_2bfe6afc.field_ident)
                    .collect::<Vec<&syn::Ident>>();
                quote::quote! {
                    fn #try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_ident_select_snake_case(
                        #value_snake_case: &sqlx::postgres::PgRow,
                        #select_borrow_postgresql_crud_not_empty_unique_vec_ident_select_ts
                    ) -> Result<Self, sqlx::Error> {
                        #declaration_primary_key_ts
                        #declaration_without_primary_key_ts
                        for el_dca9f0b7 in #select_snake_case.to_vec() {
                            match el_dca9f0b7 {
                                #assignment_variant_primary_key_ts,
                                #(#assignment_variants_without_primary_key_ts),*
                            }
                        }
                        Ok(Self {#(#fields_initiation_ts),*})
                    }
                }
            };
            quote::quote! {
                impl #ident_read_upper_camel_case {
                    #fn_try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_ident_select_ts
                }
            }
        };
        quote::quote! {
            #ident_read_ts
            #impl_ident_read_ts
        }
    };
    let ident_read_only_ids_ts = {
        let ident_read_only_ids_ts = {
            let content_ts_472e3ebf = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_partial_eq()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .build_struct(
                &ident_read_only_ids_upper_camel_case,
                &{
                    enum WrapIntoOption {
                        False,
                        True,
                    }
                    let generate_field_ts = |field_ident: &dyn quote::ToTokens, field_type: &dyn quote::ToTokens, wrap_into_option: &WrapIntoOption| {
                        let field_type_ts = match &wrap_into_option {
                            WrapIntoOption::False => generate_as_postgresql_type_read_only_ids_ts(&field_type),
                            WrapIntoOption::True => postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&generate_as_postgresql_type_read_only_ids_ts(&field_type)),
                        };
                        quote::quote! {
                            pub #field_ident: #field_type_ts
                        }
                    };
                    let primary_key_ts = generate_field_ts(&primary_key_field_ident, &primary_key_field_type, &WrapIntoOption::False);
                    let content_ts = generate_fields_named_without_primary_key_with_comma_ts(
                        &|element: &macros_helpers::SynFieldWrapper| generate_field_ts(
                            &element.field_ident,
                            &element.field_type,
                            &WrapIntoOption::True
                        )
                    );
                    quote::quote!{{
                        #primary_key_ts,
                        #content_ts
                    }}
                }
            );
            quote::quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                #content_ts_472e3ebf
            }
        };
        let impl_sqlx_row_for_ident_read_only_ids_ts = {
            let undescore_underscore_row = quote::quote! {__row};
            let where_field_types_ts = generate_fields_named_with_comma_ts(
                &|element: &macros_helpers::SynFieldWrapper| {
                    let field_type = &element.field_type;
                    let el_syn_field_ty_as_postgresql_type_read_only_ids_ts =
                        generate_as_postgresql_type_read_only_ids_ts(&field_type);
                    quote::quote! {
                        #el_syn_field_ty_as_postgresql_type_read_only_ids_ts: ::sqlx::decode::Decode<'lifetime, R::Database>
                    }
                },
            );
            let primary_key_ts = {
                let el_syn_field_ty_as_postgresql_type_read_only_ids_ts =
                    generate_as_postgresql_type_read_only_ids_ts(&primary_key_field_type);
                let field_ident_double_quotes_ts =
                    generate_quotes::double_quotes_ts(&primary_key_field_ident);
                quote::quote! {
                    let #primary_key_field_ident = match sqlx::Row::try_get::<#el_syn_field_ty_as_postgresql_type_read_only_ids_ts, &str>(
                        #undescore_underscore_row,
                        #field_ident_double_quotes_ts
                    ) {
                        Ok(value_283179dd) => value_283179dd,
                        Err(#error_0_ts) => {
                            return Err(#error_0_ts);
                        }
                    };
                }
            };
            let fields_initialization_ts =
                generate_fields_named_without_primary_key_without_comma_ts(
                    &|element: &macros_helpers::SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let field_type = &element.field_type;
                        let field_ident_double_quotes_ts =
                            generate_quotes::double_quotes_ts(&quote::quote! {#field_ident});
                        let el_syn_field_ty_as_postgresql_type_read_only_ids_ts =
                            generate_as_postgresql_type_read_only_ids_ts(&field_type);
                        quote::quote! {
                            let #field_ident = sqlx::Row::try_get::<
                                #el_syn_field_ty_as_postgresql_type_read_only_ids_ts,
                                &str
                            >(#undescore_underscore_row, #field_ident_double_quotes_ts).ok();
                        }
                    },
                );
            let self_fields_ts = generate_fields_named_with_comma_ts(
                &|element: &macros_helpers::SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    quote::quote! {#field_ident}
                },
            );
            quote::quote! {
                impl<'lifetime, R: ::sqlx::Row<Database = sqlx::Postgres>> ::sqlx::FromRow<'lifetime, R> for #ident_read_only_ids_upper_camel_case
                where
                    &'lifetime ::std::primitive::str: ::sqlx::ColumnIndex<R>,
                    #where_field_types_ts
                {
                    fn from_row(#undescore_underscore_row: &'lifetime R) -> ::sqlx::Result<Self> {
                        #primary_key_ts
                        #fields_initialization_ts
                        Ok(Self { #self_fields_ts })
                    }
                }
            }
        };
        quote::quote! {
            #ident_read_only_ids_ts
            #impl_sqlx_row_for_ident_read_only_ids_ts
        }
    };
    // println!("{ident_read_only_ids_ts}");
    let generate_ident_try_operation_error_named_upper_camel_case = |operation: &Operation| {
        format!("{ident}Try{operation}ErrorNamed")
            .parse::<proc_macro2::TokenStream>()
            .expect("6a5468b2-c8d6-4c5e-88a6-adce2bfe7467")
    };
    let ident_try_read_many_error_named_upper_camel_case =
        generate_ident_try_operation_error_named_upper_camel_case(&Operation::ReadMany);
    let generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case =
        |operation: &Operation| {
            format!("{ident}{operation}ErrorNamedWithSerializeDeserialize")
                .parse::<proc_macro2::TokenStream>()
                .expect("f9e053d1-c5ce-4a8e-ac79-6cc30ba19bb9")
        };
    let postgresql_crud_order_by_ts =
        quote::quote! {#postgresql_crud_snake_case::#order_by_upper_camel_case};
    let ident_update_upper_camel_case = SelfUpdateUpperCamelCase::from_tokens(&ident);
    let ident_update_many_parameters_upper_camel_case =
        SelfUpdateManyParametersUpperCamelCase::from_tokens(&ident);
    let ident_update_many_payload_upper_camel_case =
        SelfUpdateManyPayloadUpperCamelCase::from_tokens(&ident);
    let ident_update_try_new_error_named_upper_camel_case =
        SelfUpdateTryNewErrorNamedUpperCamelCase::from_tokens(&ident);
    let ident_update_for_query_upper_camel_case =
        SelfUpdateForQueryUpperCamelCase::from_tokens(&ident);
    let update_query_part_primary_key_snake_case = naming::UpdateQueryPartPrimaryKeySnakeCase;
    let ident_update_ts = {
        let generate_option_value_field_type_as_postgresql_type_update_ts =
            |syn_type: &syn::Type| {
                let path_value_ts = {
                    let value = format!(
                        "{}::{}",
                        naming::PostgresqlCrudSnakeCase,
                        naming::ValueUpperCamelCase
                    );
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .expect("dbdbb7f2-7801-49e7-917e-f7daa9b4dd91")
                };
                let syn_type_as_postgresql_type_update_ts =
                    generate_as_postgresql_type_update_ts(&syn_type);
                postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(
                    &quote::quote! {#path_value_ts<#syn_type_as_postgresql_type_update_ts>},
                )
            };
        let fields_declaration_ts = {
            let fields_named_without_primary_key_ts =
                generate_fields_named_without_primary_key_with_comma_ts(
                    &|element: &macros_helpers::SynFieldWrapper| -> proc_macro2::TokenStream {
                        let field_ident = &element.field_ident;
                        let option_value_field_type_as_postgresql_type_update_ts =
                            generate_option_value_field_type_as_postgresql_type_update_ts(
                                &element.field_type,
                            );
                        quote::quote! {
                            #field_ident: #option_value_field_type_as_postgresql_type_update_ts
                        }
                    },
                );
            quote::quote! {
                #primary_key_field_ident: #primary_key_field_type_update_ts,
                #fields_named_without_primary_key_ts
            }
        };
        let ident_update_ts = {
            let content_ts_a09c0471 = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_serde_serialize()
                .derive_utoipa_to_schema()
                .build_struct(
                    &ident_update_upper_camel_case,
                    &quote::quote! {{#fields_declaration_ts}},
                );
            quote::quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                #content_ts_a09c0471
            }
        };
        let ident_update_try_new_error_named_ts =
            macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_thiserror_error()
                .derive_error_occurence_lib_error_occurence()
                .build_enum(
                    &ident_update_try_new_error_named_upper_camel_case,
                    &quote::quote! {{
                        #no_fields_provided_upper_camel_case {
                            #[eo_to_std_string_string]
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        }
                    }},
                );
        let impl_pub_try_new_for_ident_update_ts =
            macros_helpers::generate_impl_pub_try_new_for_ident_ts(
                &ident_update_upper_camel_case,
                &fields_declaration_ts,
                &ident_update_try_new_error_named_upper_camel_case,
                &{
                    let (left_ts, right_ts) = {
                        let maybe_wrap_into_braces_handle_ts =
                            |content_ts: &dyn quote::ToTokens| {
                                postgresql_crud_macros_common::maybe_wrap_into_braces_ts(
                                    content_ts,
                                    fields_len_without_primary_key > 1,
                                )
                            };
                        (
                            maybe_wrap_into_braces_handle_ts(
                                &generate_fields_named_without_primary_key_with_comma_ts(
                                    &|element: &macros_helpers::SynFieldWrapper| -> proc_macro2::TokenStream {
                                        let field_ident = &element.field_ident;
                                        quote::quote! {&#field_ident}
                                    },
                                ),
                            ),
                            maybe_wrap_into_braces_handle_ts(
                                &fields_named_without_primary_key_with_comma_none_ts,
                            ),
                        )
                    };
                    let fields_inialization_ts = generate_fields_named_with_comma_ts(
                        &|element: &macros_helpers::SynFieldWrapper| -> proc_macro2::TokenStream {
                            let field_ident = &element.field_ident;
                            quote::quote! {#field_ident}
                        },
                    );
                    quote::quote! {
                        if matches!(#left_ts, #right_ts) {
                            return Err(#ident_update_try_new_error_named_upper_camel_case::#no_fields_provided_upper_camel_case {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        Ok(Self {#fields_inialization_ts})
                    }
                },
            );
        let impl_serde_deserialize_for_ident_update_ts =
            postgresql_crud_macros_common::generate_impl_serde_deserialize_for_struct_ts(
                &ident_update_upper_camel_case,
                &fields
                    .iter()
                    .map(|el_2bfe6afc| (&el_2bfe6afc.field_ident, &el_2bfe6afc.field_type))
                    .collect::<Vec<(&syn::Ident, &syn::Type)>>(),
                fields_len,
                &|syn_ident: &syn::Ident, syn_type: &syn::Type| {
                    if syn_ident == primary_key_field_ident {
                        quote::quote! {#primary_key_field_type_update_ts}
                    } else {
                        generate_option_value_field_type_as_postgresql_type_update_ts(syn_type)
                    }
                },
            );
        let impl_postgresql_crud_default_option_some_vec_one_el_for_ident_update_ts =
            generate_impl_postgresql_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                &ident_update_upper_camel_case,
                &{
                    let primary_key_field_with_default_option_some_vec_one_el_ts = {
                        quote::quote! {
                            #primary_key_field_ident: #postgresql_crud_default_option_some_vec_one_el_call_ts
                        }
                    };
                    let fields_without_primary_key_with_default_option_some_vec_one_el_ts =
                        generate_fields_named_without_primary_key_with_comma_ts(
                            &|element: &macros_helpers::SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                quote::quote! {
                                    #field_ident: Some(postgresql_crud::Value{
                                        #value_snake_case: #postgresql_crud_default_option_some_vec_one_el_call_ts
                                    })
                                }
                            },
                        );
                    quote::quote! {Self{
                        #primary_key_field_with_default_option_some_vec_one_el_ts,
                        #fields_without_primary_key_with_default_option_some_vec_one_el_ts
                    }}
                },
            );
        quote::quote! {
            #ident_update_ts
            #ident_update_try_new_error_named_ts
            #impl_pub_try_new_for_ident_update_ts
            #impl_serde_deserialize_for_ident_update_ts
            #impl_postgresql_crud_default_option_some_vec_one_el_for_ident_update_ts
        }
    };
    let ident_update_for_query_ts = {
        let ident_update_for_query_ts = {
            let content_ts_50ae0c5f = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_serde_serialize()
            .derive_utoipa_to_schema()
            .build_struct(
                &ident_update_for_query_upper_camel_case,
                &{
                    let fields_named_without_primary_key_ts = generate_fields_named_without_primary_key_with_comma_ts(&|element: &macros_helpers::SynFieldWrapper| -> proc_macro2::TokenStream {
                        let field_ident = &element.field_ident;
                        let option_value_field_type_as_postgresql_type_update_for_query_ts = {
                            let path_value_ts = {
                                let value = format!("{}::{}", naming::PostgresqlCrudSnakeCase, naming::ValueUpperCamelCase);
                                value.parse::<proc_macro2::TokenStream>().expect("2b09d4ae-757c-4c6b-a9ba-edd2ea95128a")
                            };
                            let syn_type_as_postgresql_type_update_for_query_ts = generate_as_postgresql_type_update_for_query_ts(&element.field_type);
                            postgresql_crud_macros_common::generate_std_option_option_tokens_declaration_ts(&quote::quote! {#path_value_ts<#syn_type_as_postgresql_type_update_for_query_ts>})
                        };
                        quote::quote! {#field_ident: #option_value_field_type_as_postgresql_type_update_for_query_ts}
                    });
                    quote::quote!{{
                        #primary_key_field_ident: #primary_key_field_type_update_for_query_ts,
                        #fields_named_without_primary_key_ts
                    }}
                }
            );
            quote::quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                #content_ts_50ae0c5f
            }
        };
        let impl_ident_update_for_query_ts = {
            let update_query_part_primary_key_ts = {
                quote::quote! {
                    fn #update_query_part_primary_key_snake_case(&self, #increment_snake_case: &mut u64) -> Result<#string_ts, #postgresql_crud_snake_case::#query_part_error_named_upper_camel_case> {
                        match #primary_key_field_type_as_postgresql_type_ts #update_query_part_snake_case(
                            &self.#primary_key_field_ident,
                            "",
                            #ident::#primary_key_snake_case(),
                            "",
                            #increment_snake_case,
                        ) {
                            Ok(value_snake_case) => Ok(value_snake_case),
                            Err(#error_0_ts) => Err(#error_0_ts)
                        }
                    }
                }
            };
            let update_query_part_fields_ts =
                generate_fields_named_without_primary_key_without_comma_ts(
                    &|element: &macros_helpers::SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let field_ident_double_quotes_ts =
                            generate_quotes::double_quotes_ts(&field_ident);
                        let update_query_part_field_ident_snake_case =
                            UpdateQueryPartSelfSnakeCase::from_tokens(&field_ident);
                        let field_type_as_postgresql_crud_postgresql_type_postgresql_type_ts =
                            generate_as_postgresql_type_ts(&element.field_type);
                        quote::quote! {
                            fn #update_query_part_field_ident_snake_case(
                                #value_snake_case: &postgresql_crud::Value<#field_type_as_postgresql_crud_postgresql_type_postgresql_type_ts #update_for_query_upper_camel_case>,
                                #increment_snake_case: &mut u64
                            ) -> Result<#string_ts, #postgresql_crud_snake_case::#query_part_error_named_upper_camel_case> {
                                match #field_type_as_postgresql_crud_postgresql_type_postgresql_type_ts #update_query_part_snake_case(
                                    &#value_snake_case.#value_snake_case,
                                    #field_ident_double_quotes_ts,
                                    #field_ident_double_quotes_ts,
                                    "",
                                    #increment_snake_case
                                ) {
                                    Ok(value_f75dfd93) => Ok(value_f75dfd93),
                                    Err(#error_0_ts) => Err(#error_0_ts),
                                }
                            }
                        }
                    },
                );
            let select_only_updated_ids_query_part_ts = {
                let primary_key_content_ts = {
                    let primary_key_field_ident_double_quotes_ts =
                        generate_quotes::double_quotes_ts(&primary_key_field_ident);
                    quote::quote! {
                        acc_88c91f52.push_str(&match <#primary_key_field_type as postgresql_crud::PostgresqlType>::#select_only_updated_ids_query_part_snake_case(
                            &self.#primary_key_field_ident,
                            #primary_key_field_ident_double_quotes_ts,
                            increment,
                        ){
                            Ok(value) => value,
                            Err(error) => {
                                return Err(error);
                            }
                        });
                    }
                };
                let content_ts = fields_without_primary_key.iter().map(|element: &macros_helpers::SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_ident_double_quotes_ts = generate_quotes::double_quotes_ts(&field_ident);
                    let field_type_as_postgresql_crud_postgresql_type_postgresql_type_ts = generate_as_postgresql_type_ts(&element.field_type);
                    quote::quote! {
                        if let Some(value_90f79b11) = &self.#field_ident {
                            acc_88c91f52.push_str(&match #field_type_as_postgresql_crud_postgresql_type_postgresql_type_ts #select_only_updated_ids_query_part_snake_case(
                                &value_90f79b11.#value_snake_case,
                                #field_ident_double_quotes_ts,
                                increment,
                            ){
                                Ok(value_47a6f597) => value_47a6f597,
                                Err(#error_snake_case) => {
                                    return Err(#error_snake_case);
                                }
                            });
                        }
                    }
                });
                quote::quote! {
                    fn #select_only_updated_ids_query_part_snake_case(&self, #increment_snake_case: &mut u64) -> Result<#string_ts, postgresql_crud::QueryPartErrorNamed> {
                        let mut acc_88c91f52 = String::new();
                        #primary_key_content_ts
                        #(#content_ts)*
                        let _: Option<char> = acc_88c91f52.pop();
                        Ok(acc_88c91f52)
                    }
                }
            };
            let update_handle_ts = generate_from_handle_ts(&ident_update_upper_camel_case, &{
                let primary_key_field_type_as_postgresql_type_update_for_query_ts =
                    generate_as_postgresql_type_update_for_query_ts(&primary_key_field_type);
                let fields_named_without_primary_key_ts =
                    generate_fields_named_without_primary_key_with_comma_ts(
                        &|element: &macros_helpers::SynFieldWrapper| -> proc_macro2::TokenStream {
                            let field_ident = &element.field_ident;
                            let value_initialization_ts =
                                generate_import_path_value_initialization_ts(&{
                                    let field_type_as_postgresql_type_update_for_query_ts =
                                        generate_as_postgresql_type_update_for_query_ts(
                                            &element.field_type,
                                        );
                                    quote::quote! {
                                         #field_type_as_postgresql_type_update_for_query_ts::from(value_0e64c53a.#value_snake_case)
                                    }
                                });
                            quote::quote! {
                                #field_ident: value.#field_ident.map(|value_0e64c53a| #value_initialization_ts)
                            }
                        },
                    );
                quote::quote! {
                    Self {
                        #primary_key_field_ident: #primary_key_field_type_as_postgresql_type_update_for_query_ts::from(#value_snake_case.#primary_key_field_ident),
                        #fields_named_without_primary_key_ts
                    }
                }
            });
            quote::quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                impl #ident_update_for_query_upper_camel_case {
                    #update_query_part_primary_key_ts
                    #update_query_part_fields_ts
                    #select_only_updated_ids_query_part_ts
                    #update_handle_ts
                }
            }
        };
        quote::quote! {
            #ident_update_for_query_ts
            #impl_ident_update_for_query_ts
        }
    };
    let generate_match_update_query_part_primary_key_ts =
        |operation: &Operation, content_ts: &dyn quote::ToTokens| {
            let content_ts_75b4019b =
                generate_operation_error_initialization_eprintln_response_creation_ts(
                    operation,
                    &query_part_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            quote::quote! {
                match #content_ts.#update_query_part_primary_key_snake_case(&mut #increment_snake_case) {
                    Ok(value_f269a3b2) => value_f269a3b2,
                    Err(#error_0_ts) => {
                        #content_ts_75b4019b
                    }
                }
            }
        };
    let row_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::RowAndRollbackUpperCamelCase,
        Some(macros_helpers::StatusCode::InternalServerError500),
        vec![
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &naming::RowSnakeCase, sqlx_error_syn_punctuated_punctuated.clone()),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &rollback_snake_case, sqlx_error_syn_punctuated_punctuated),
        ],
    );
    let sqlx_query_sqlx_postgres_ts = quote::quote! {sqlx::query::<sqlx::Postgres>};
    let (
        postgresql_crud_postgresql_type_where_filter_query_part_ts,
        postgresql_crud_postgresql_type_where_filter_query_bind_ts,
    ) = {
        let postgresql_crud_postgresql_type_where_filter_ts =
            quote::quote! {#postgresql_crud_snake_case::PostgresqlTypeWhereFilter::};
        (
            quote::quote! {#postgresql_crud_postgresql_type_where_filter_ts #query_part_snake_case},
            quote::quote! {#postgresql_crud_postgresql_type_where_filter_ts #query_bind_snake_case},
        )
    };

    let std_vec_vec_struct_options_ident_ts =
        postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(
            &ident_read_upper_camel_case,
        );
    let not_unique_field_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NotUniqueFieldUpperCamelCase,
        Some(macros_helpers::StatusCode::BadRequest400),
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize,
            &naming::NotUniqueFieldSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated(&[&ident_select_upper_camel_case.to_string()]),
        )],
    );
    let serde_json_to_string_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::SerdeJsonToStringUpperCamelCase,
        None,
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
            &naming::SerdeJsonToStringSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
        )],
    );
    let failed_to_get_response_text_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::FailedToGetResponseTextUpperCamelCase,
        Some(macros_helpers::StatusCode::BadRequest400),
        vec![
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &status_code_snake_case, macros_helpers::generate_simple_syn_punctuated_punctuated(&["reqwest", "StatusCode"])),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
                &naming::HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated(&["reqwest", "header", "HeaderMap"]),
            ),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &naming::ReqwestSnakeCase, macros_helpers::generate_simple_syn_punctuated_punctuated(&["reqwest", "Error"])),
        ],
    );
    let deserialize_response_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::DeserializeResponseUpperCamelCase,
        None,
        vec![
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &status_code_snake_case, macros_helpers::generate_simple_syn_punctuated_punctuated(&["reqwest", "StatusCode"])),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
                &naming::HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated(&["reqwest", "header", "HeaderMap"]),
            ),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize, &naming::ResponseTextSnakeCase, string_syn_punctuated_punctuated),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string, &naming::SerdeSnakeCase, macros_helpers::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"])),
        ],
    );
    let reqwest_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::ReqwestUpperCamelCase,
        None,
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
            &naming::ReqwestSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated(&["reqwest", "Error"]),
        )],
    );
    let check_body_size_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::CheckBodySizeUpperCamelCase,
        Some(macros_helpers::StatusCode::BadRequest400),
        vec![(
            ErrorOccurenceFieldAttribute::EoErrorOccurence,
            &naming::CheckBodySizeSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated(&[
                &postgresql_crud_snake_case.to_string(),
                "check_body_size",
                &naming::ErrorNamedUpperCamelCase.to_string(),
            ]),
        )],
    );
    let serde_json_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::SerdeJsonUpperCamelCase,
        Some(macros_helpers::StatusCode::BadRequest400),
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
            &naming::SerdeJsonSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
        )],
    );
    let header_content_type_application_json_not_found_syn_variant_wrapper =
        new_syn_variant_wrapper(
            &naming::HeaderContentTypeApplicationJsonNotFoundUpperCamelCase,
            Some(macros_helpers::StatusCode::BadRequest400),
            Vec::<(
                ErrorOccurenceFieldAttribute,
                &'static dyn Display,
                Punctuated<syn::PathSegment, PathSep>,
            )>::default(),
        );
    let common_http_request_syn_variants = {
        vec![
            serde_json_to_string_syn_variant_wrapper
                .get_syn_variant()
                .clone(),
            failed_to_get_response_text_syn_variant_wrapper
                .get_syn_variant()
                .clone(),
            deserialize_response_syn_variant_wrapper
                .get_syn_variant()
                .clone(),
            reqwest_syn_variant_wrapper.get_syn_variant().clone(),
        ]
    };
    let generate_additional_error_variants =
        |current_syn_derive_input: &syn::DeriveInput,
         generate_postgresql_table_attribute: GeneratePostgresqlTableAttribute|
         -> Vec<syn::Variant> {
            let generate_postgresql_table_attribute_stringified =
                generate_postgresql_table_attribute.to_string();
            let common_additional_error_variants_attribute_ts =
                macros_helpers::get_macro_attribute_meta_list_ts(
                    &current_syn_derive_input.attrs,
                    &generate_postgresql_table_attribute.generate_path_to_attribute(),
                );
            let value: syn::DeriveInput =
                syn::parse2((*common_additional_error_variants_attribute_ts).clone())
                    .expect("1b80783d-f818-4ba3-ba17-3c7831d16530");
            let value_ident_stringified = value.ident.to_string();
            assert!(
                value_ident_stringified == generate_postgresql_table_attribute_stringified,
                "8a66c852-43a7-4c1c-ad6a-ac0a232d2215"
            );
            let variants = if let syn::Data::Enum(data_enum) = value.data {
                data_enum.variants
            } else {
                panic!("f3ddc78c-27f7-443c-a892-e64b9654a425");
            };
            variants.into_iter().collect()
        };
    let common_additional_error_variants = generate_additional_error_variants(
        &syn_derive_input,
        GeneratePostgresqlTableAttribute::CommonAdditionalErrorVariants,
    );
    let common_route_syn_variants = {
        let mut acc_94f701ab = vec![
            check_body_size_syn_variant_wrapper.get_syn_variant(),
            postgresql_syn_variant_wrapper.get_syn_variant(),
            serde_json_syn_variant_wrapper.get_syn_variant(),
            header_content_type_application_json_not_found_syn_variant_wrapper.get_syn_variant(),
        ];
        for el_af152d67 in &common_additional_error_variants {
            acc_94f701ab.push(el_af152d67);
        }
        acc_94f701ab
    };
    let common_additional_logic_ts = macros_helpers::get_macro_attribute_meta_list_ts(
        &syn_derive_input.attrs,
        &GeneratePostgresqlTableAttribute::CommonAdditionalLogic.generate_path_to_attribute(),
    );
    let generate_pub_handle_ts = |is_pub: bool| {
        if is_pub {
            quote::quote! {pub}
        } else {
            proc_macro2::TokenStream::new()
        }
    };
    let generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_ts =
        |primary_key_type_ts: &dyn quote::ToTokens| {
            let is_pub = true;
            let pub_handle_ts = generate_pub_handle_ts(is_pub);
            quote::quote! {#pub_handle_ts #primary_key_field_ident: #primary_key_type_ts}
        };
    let generate_match_postgres_transaction_rollback_await_ts =
        |operation: &Operation,
         postgresql_file: &'static str,
         postgresql_line: u32,
         postgresql_column: u32,
         row_and_rollback_file: &'static str,
         row_and_rollback_line: u32,
         row_and_rollback_column: u32| {
            let content_ts_91f19090 =
                generate_operation_error_initialization_eprintln_response_creation_ts(
                    operation,
                    &postgresql_syn_variant_wrapper,
                    postgresql_file,
                    postgresql_line,
                    postgresql_column,
                );
            let row_and_rollback_syn_variant_error_initialization_eprintln_response_creation_ts =
                generate_operation_error_initialization_eprintln_response_creation_ts(
                    operation,
                    &row_and_rollback_syn_variant_wrapper,
                    row_and_rollback_file,
                    row_and_rollback_line,
                    row_and_rollback_column,
                );
            quote::quote! {{
                if let Err(#error_1_ts) = #executor_snake_case.#rollback_snake_case().await {
                    #row_and_rollback_syn_variant_error_initialization_eprintln_response_creation_ts
                }
                #content_ts_91f19090
            }}
        };
    let generate_drop_rows_match_postgres_transaction_rollback_await_handle_ts =
        |operation: &Operation,
         postgresql_file: &'static str,
         postgresql_line: u32,
         postgresql_column: u32,
         row_and_rollback_file: &'static str,
         row_and_rollback_line: u32,
         row_and_rollback_column: u32| {
            let match_postgres_transaction_rollback_await_ts =
                generate_match_postgres_transaction_rollback_await_ts(
                    operation,
                    postgresql_file,
                    postgresql_line,
                    postgresql_column,
                    row_and_rollback_file,
                    row_and_rollback_line,
                    row_and_rollback_column,
                );
            quote::quote! {
                drop(#rows_snake_case);
                #match_postgres_transaction_rollback_await_ts
            }
        };
    let wrap_into_value_ts = |content_ts: &dyn quote::ToTokens| {
        quote::quote! {
            let #value_snake_case = {
                #content_ts
            };
        }
    };
    let generate_fetch_ts =
        |executor_name_ts: &dyn quote::ToTokens,
         value_handle_ts: &dyn quote::ToTokens,
         try_next_error_initialization_ts: &dyn quote::ToTokens,
         should_wrap_into_value: &ShouldWrapIntoValue| {
            let content_ts = quote::quote! {
                let mut #rows_snake_case = #binded_query_snake_case.fetch(#executor_name_ts.as_mut());
                let mut acc_d16ac269 = Vec::new();
                while let Some(value_d9cc2c36) = match #postgresql_crud_snake_case::TryStreamExt::try_next(&mut #rows_snake_case).await {
                    Ok(value_19f3d6e1) => match value_19f3d6e1 {
                        Some(value_b27d7d79) => #value_handle_ts,
                        None => None,
                    },
                    Err(#error_0_ts) => {
                        #try_next_error_initialization_ts
                    }
                }
                {
                    acc_d16ac269.push(value_d9cc2c36);
                }
                acc_d16ac269
            };
            match should_wrap_into_value {
                ShouldWrapIntoValue::False => content_ts,
                ShouldWrapIntoValue::True => wrap_into_value_ts(&content_ts),
            }
        };
    let generate_fetch_one_ts =
        |executor_name_ts: &dyn quote::ToTokens,
         value_handle_ts: &dyn quote::ToTokens,
         fetch_one_error_initialization_ts: &dyn quote::ToTokens| {
            quote::quote! {
                match #binded_query_snake_case.fetch_one(#executor_name_ts.as_mut()).await {
                    Ok(value_b27d7d79) => {
                        #value_handle_ts
                    },
                    Err(#error_0_ts) => {
                        #fetch_one_error_initialization_ts
                    }
                }
            }
        };
    let generate_sqlx_row_try_get_primary_key_ts =
        |sqlx_row_try_get_type_ts: &dyn quote::ToTokens,
         ok_ts: &dyn quote::ToTokens,
         err_ts: &dyn quote::ToTokens| {
            quote::quote! {
                match #sqlx_row::try_get::<
                    #sqlx_row_try_get_type_ts,
                    #ref_std_primitive_str
                >(&value_b27d7d79, Self::#primary_key_snake_case()) {
                    Ok(value_69ecb6a9) => #ok_ts,
                    Err(#error_0_ts) => {
                        #err_ts
                    }
                }
            }
        };
    let wrap_content_into_postgresql_transaction_begin_commit_value_ts =
        |operation: &Operation, content_ts: &dyn quote::ToTokens| {
            let postgres_transaction_begin_ts = {
                let content_ts_efebc55b =
                    generate_operation_error_initialization_eprintln_response_creation_ts(
                        operation,
                        &postgresql_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                quote::quote! {
                    let mut #executor_snake_case = match #sqlx_acquire::#begin_snake_case(#executor_acquire_snake_case).await {
                        Ok(value_1aaca28f) => value_1aaca28f,
                        Err(#error_0_ts) => {
                            #content_ts_efebc55b
                        }
                    };
                }
            };
            let postgres_transaction_commit_ts = {
                let postgresql_syn_variant_error_initialization_eprintln_response_creation_ts =
                    generate_operation_error_initialization_eprintln_response_creation_ts(
                        operation,
                        &postgresql_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                quote::quote! {
                    if let Err(#error_0_ts) = #executor_snake_case.#commit_snake_case().await {
                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_ts
                    }
                }
            };
            quote::quote! {
                #postgres_transaction_begin_ts
                #content_ts
                #postgres_transaction_commit_ts
                #value_snake_case
            }
        };
    let generate_error_occurence_variant_ts =
        |error_variant: &syn::Variant| -> proc_macro2::TokenStream {
            let variant_ident = &error_variant.ident;
            let syn::Fields::Named(fields_named) = &error_variant.fields else {
                panic!("2acd4725-3fc0-4903-812c-d75f22e3f713");
            };
            let fields_mapped_into_ts = fields_named.named.iter().map(|field| {
                let field_ident = field
                    .ident
                    .as_ref()
                    .expect("a21dc807-77ad-4b05-a9ae-e1f17216b490");
                let error_occurence_attribute = if *field_ident
                    == *naming::CodeOccurenceSnakeCase.to_string()
                {
                    proc_macro2::TokenStream::new()
                } else {
                    let mut error_occurence_attribute: Option<ErrorOccurenceFieldAttribute> = None;
                    for el_1c83e302 in &field.attrs {
                        if el_1c83e302.path().segments.len() == 1 {
                            let segment = el_1c83e302
                                .path()
                                .segments
                                .first()
                                .expect("5bd7ed8d-d28d-4614-8f3d-add4046df76a");
                            if let Ok(value) = {
                                <ErrorOccurenceFieldAttribute as FromStr>::from_str(
                                    &segment.ident.to_string(),
                                )
                            } {
                                if error_occurence_attribute.is_some() {
                                    panic!("9a469d36-1b5b-4f2e-8ed4-c8713c60fb39")
                                } else {
                                    error_occurence_attribute = Some(value);
                                }
                            }
                        }
                    }
                    error_occurence_attribute
                        .expect("d1003b2e-b108-4807-a769-9c2611284713")
                        .to_attribute_view_ts()
                };
                let field_type = &field.ty;
                quote::quote! {
                    #error_occurence_attribute
                    #field_ident: #field_type
                }
            });
            quote::quote! {
                #variant_ident {
                    #(#fields_mapped_into_ts),*
                }
            }
        };
    let generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_ts =
        |operation: &Operation,
         desirable_type_ts: &dyn quote::ToTokens,
         type_variants_from_request_response_syn_variants: &Vec<syn::Variant>|
         -> proc_macro2::TokenStream {
            let ident_operation_response_variants_upper_camel_case =
                generate_ident_operation_response_variants_upper_camel_case(operation);
            let ident_try_operation_logic_response_variants_ts = {
                let variants_ts = type_variants_from_request_response_syn_variants.iter().map(
                    macros_helpers::generate_serialize_deserialize_version_of_named_syn_variant,
                );
                quote::quote! {
                    #allow_clippy_arbitrary_source_item_ordering_ts
                    #derive_debug_serde_serialize_serde_deserialize
                    pub enum #ident_operation_response_variants_upper_camel_case {
                        #desirable_upper_camel_case(#desirable_type_ts),
                        #(#variants_ts),*
                    }
                }
            };
            let ident_operation_error_named_upper_camel_case =
                generate_ident_operation_error_named_upper_camel_case(operation);
            let impl_ident_operation_response_variants_ts = {
                let from_handle_ts = generate_from_handle_ts(
                    &ident_operation_error_named_upper_camel_case,
                    &{
                        let variants_ts = type_variants_from_request_response_syn_variants.iter().map(|el_d80f0707| {
                    let variant_ident = &el_d80f0707.ident;
                    let syn::Fields::Named(fields_named) = &el_d80f0707.fields else {
                        panic!("10764d2b-48db-47c4-bc8c-12e13d6b9621");
                    };
                    let fields_mapped_into_ts = {
                        let fields_ts = fields_named.named.iter().map(|field| &field.ident);
                        quote::quote! {#(#fields_ts),*}
                    };
                    let ident_operation_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case(operation);
                    quote::quote! {
                        #ident_operation_error_named_with_serialize_deserialize_upper_camel_case::#variant_ident {
                            #fields_mapped_into_ts
                        } => Self::#variant_ident {
                            #fields_mapped_into_ts
                        }
                    }
                });
                        quote::quote! {
                            match #value_snake_case.#into_serialize_deserialize_version_snake_case() {
                                #(#variants_ts),*
                            }
                        }
                    },
                );
                quote::quote! {
                    impl #ident_operation_response_variants_upper_camel_case {
                        #from_handle_ts
                    }
                }
            };
            let ident_operation_error_named_ts = {
                let variants_ts = type_variants_from_request_response_syn_variants
                    .iter()
                    .map(generate_error_occurence_variant_ts);
                quote::quote! {
                    #allow_clippy_arbitrary_source_item_ordering_ts
                    #derive_debug_this_error_error_occurence
                    pub enum #ident_operation_error_named_upper_camel_case {
                        #(#variants_ts),*
                    }
                }
            };
            quote::quote! {
                #ident_try_operation_logic_response_variants_ts
                #impl_ident_operation_response_variants_ts
                #ident_operation_error_named_ts
            }
        };
    let generate_ident_operation_payload_upper_camel_case = |operation: &Operation| match &operation
    {
        Operation::CreateOne => quote::quote! {#ident_create_upper_camel_case},
        Operation::UpdateOne => quote::quote! {#ident_update_upper_camel_case},
        Operation::CreateMany
        | Operation::ReadMany
        | Operation::ReadOne
        | Operation::UpdateMany
        | Operation::DeleteMany
        | Operation::DeleteOne => format!("{ident}{operation}{}", naming::PayloadUpperCamelCase)
            .parse::<proc_macro2::TokenStream>()
            .expect("c042f504-5275-4388-80cc-a141c066daf8"),
    };
    let generate_ident_operation_parameters_upper_camel_case = |operation: &Operation| {
        format!("{ident}{operation}Parameters")
            .parse::<proc_macro2::TokenStream>()
            .expect("c1203fc6-3dbd-4a37-a407-f9913aa7964d")
    };
    let generate_parameters_pattern_ts = |operation: &Operation,
                                          payload_ts: proc_macro2::TokenStream|
     -> proc_macro2::TokenStream {
        let parameters_ts = {
            let (derive_clone, derive_copy) = operation.derive_clone_and_copy();
            let content_ts_0d032fce = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone_if(derive_clone)
                .derive_copy_if(derive_copy)
                .build_struct(
                    &generate_ident_operation_parameters_upper_camel_case(operation),
                    &{
                        let ident_operation_payload_upper_camel_case =
                            generate_ident_operation_payload_upper_camel_case(operation);
                        quote::quote! {{
                            pub #payload_snake_case: #ident_operation_payload_upper_camel_case,
                        }}
                    },
                );
            quote::quote! {
                #allow_clippy_arbitrary_source_item_ordering_ts
                #content_ts_0d032fce
            }
        };
        quote::quote! {
            #payload_ts
            #parameters_ts
        }
    };
    let generate_parameters_payload_and_default_ts =
        |operation: &Operation,
         declaration_ts: &dyn quote::ToTokens,
         default_init_content_ts: &dyn quote::ToTokens| {
            let ident_operation_payload_upper_camel_case =
                generate_ident_operation_payload_upper_camel_case(operation);
            let ident_operation_payload_ts = {
                let (derive_clone, derive_copy) = operation.derive_clone_and_copy();
                let content_ts_ec5b096c =
                    macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                        .make_pub()
                        .derive_debug()
                        .derive_clone_if(derive_clone)
                        .derive_copy_if(derive_copy)
                        .derive_serde_serialize()
                        .derive_serde_deserialize()
                        .derive_utoipa_to_schema()
                        .build_struct(&ident_operation_payload_upper_camel_case, &declaration_ts);
                quote::quote! {
                    #allow_clippy_arbitrary_source_item_ordering_ts
                    #content_ts_ec5b096c
                }
            };
            let impl_postgresql_crud_default_option_some_vec_one_el_for_operation_payload_ts = generate_impl_postgresql_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(&ident_operation_payload_upper_camel_case, &quote::quote! {Self #default_init_content_ts});
            quote::quote! {
                #ident_operation_payload_ts
                #impl_postgresql_crud_default_option_some_vec_one_el_for_operation_payload_ts
            }
        };
    let generate_type_variants_from_request_response_syn_variants =
        |syn_variants: &Vec<&syn::Variant>, operation: &Operation| -> Vec<syn::Variant> {
            let mut type_variants_from_request_response_syn_variants = Vec::new();
            for el_21f2d46c in syn_variants {
                type_variants_from_request_response_syn_variants.push((*el_21f2d46c).clone());
            }
            for el_60533068 in generate_additional_error_variants(
                &syn_derive_input,
                operation.generate_postgresql_table_attribute_additional_error_variants(),
            ) {
                type_variants_from_request_response_syn_variants.push(el_60533068.clone());
            }
            type_variants_from_request_response_syn_variants
        };
    let generate_ident_try_operation_error_named_ts = |
        operation: &Operation,
        syn_variants: &Vec<syn::Variant>
    | -> proc_macro2::TokenStream {
        let ident_try_operation_error_named_upper_camel_case =
            generate_ident_try_operation_error_named_upper_camel_case(operation);
        let variants = syn_variants
        .iter()
        .cloned()
        .chain(once({
            let ident_operation_error_named_with_serialize_deserialize_upper_camel_case =
                generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case(operation);
            new_syn_variant_wrapper(
                &ident_operation_error_named_with_serialize_deserialize_upper_camel_case,
                None,
                vec![(
                    macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string,
                    &operation.operation_error_named_with_serialize_deserialize_snake_case(),
                    macros_helpers::generate_simple_syn_punctuated_punctuated(&[
                        &ident_operation_error_named_with_serialize_deserialize_upper_camel_case.to_string(),
                    ]),
                )],
            )
            .get_syn_variant()
            .clone()
        }))
        .collect::<Vec<syn::Variant>>();
        let variants_ts = variants.iter().map(generate_error_occurence_variant_ts);
        quote::quote! {
            #allow_clippy_arbitrary_source_item_ordering_ts
            #derive_debug_thiserror_error_occurence
            pub enum #ident_try_operation_error_named_upper_camel_case {
                #(#variants_ts),*
            }
        }
    };
    let std_sync_arc_combination_of_app_state_logic_traits_ts = quote::quote! {std::sync::Arc<dyn #postgresql_crud_snake_case::CombinationOfAppStateLogicTraits>};
    let generate_operation_ts = |
        operation: &Operation,
        current_additional_logic_ts: &dyn quote::ToTokens,
        parameters_logic_ts: &dyn quote::ToTokens,
        expected_updated_primary_keys_ts: &dyn quote::ToTokens,
        query_string_ts: &dyn quote::ToTokens,
        binded_query_ts: &dyn quote::ToTokens,
        postgresql_logic_ts: &dyn quote::ToTokens
    | -> proc_macro2::TokenStream {
        let operation_handle_snake_case_ts = operation.self_handle_snake_case_ts();
        let operation_snake_case_ts = operation.self_snake_case_ts();
        let request_parts_preparation_ts = {
            let header_content_type_application_json_not_found_syn_variant_wrapper_error_initialization_eprintln_response_creation_ts =
                &generate_operation_error_initialization_eprintln_response_creation_ts(
                    operation,
                    &header_content_type_application_json_not_found_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            let check_body_size_syn_variant_wrapper_error_initialization_eprintln_response_creation_ts =
                &generate_operation_error_initialization_eprintln_response_creation_ts(
                    operation,
                    &check_body_size_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            quote::quote! {
                let (parts, #body_snake_case) = #request_snake_case.into_parts();
                let headers = parts.headers;
                if !matches!(
                    headers.get(http::header::CONTENT_TYPE),
                    Some(value) if value == http::header::HeaderValue::from_static("application/json")
                ) {
                    #header_content_type_application_json_not_found_syn_variant_wrapper_error_initialization_eprintln_response_creation_ts
                }
                //todo
                // match axum::body::HttpBody::size_hint(&#body_snake_case).exact() {
                //     Some(value) => {
                //         println!(
                //             "HttpBody::size_hint {value} byte or {} kilobyte or {} megabyte",
                //             value
                //                 .checked_div(1024)
                //                 .expect("111fd01a-cfef-47f0-bc0b-661da0d8371b"),
                //             value
                //                 .checked_div(1_048_576)
                //                 .expect("efbe0db4-2196-4998-b11f-8844ce5fcf18"), //(1024*1024)
                //         );
                //     }
                //     None => {
                //         println!("HttpBody::size_hint is None");
                //     }
                // }
                let body_bytes = match #postgresql_crud_snake_case::check_body_size::check_body_size(#body_snake_case, *#app_state_snake_case.get_maximum_size_of_http_body_in_bytes()).await {
                    Ok(value_cfac9140) => value_cfac9140,
                    Err(#error_0_ts) => {
                        #check_body_size_syn_variant_wrapper_error_initialization_eprintln_response_creation_ts
                    }
                };
            }
        };
        let additional_validators_ts = {
            let operation_additional_logic_ts = macros_helpers::get_macro_attribute_meta_list_ts(
                &syn_derive_input.attrs,
                &operation
                    .generate_postgresql_table_attribute_additional_logic()
                    .generate_path_to_attribute(),
            );
            quote::quote! {
                #current_additional_logic_ts
                #operation_additional_logic_ts
            }
        };
        let acquire_pool_and_connection_ts = {
            let postgresql_syn_variant_wrapper_error_initialization_eprintln_response_creation_ts =
                generate_operation_error_initialization_eprintln_response_creation_ts(
                    operation,
                    &postgresql_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            quote::quote! {
                let mut #pool_connection_snake_case = match #app_state_snake_case.get_postgres_pool().acquire().await {
                    Ok(value_4535ee48) => value_4535ee48,
                    Err(#error_0_ts) => {
                        #postgresql_syn_variant_wrapper_error_initialization_eprintln_response_creation_ts
                    }
                };
                let #executor_acquire_snake_case = match sqlx::Acquire::acquire(&mut #pool_connection_snake_case).await {
                    Ok(value_61ae8f84) => value_61ae8f84,
                    Err(#error_0_ts) => {
                        #postgresql_syn_variant_wrapper_error_initialization_eprintln_response_creation_ts
                    }
                };
            }
        };
        let wraped_into_axum_response_ts = wrap_into_axum_response_ts(
            &{
                let ident_operation_response_variants_upper_camel_case =
                    generate_ident_operation_response_variants_upper_camel_case(operation);
                quote::quote! {#ident_operation_response_variants_upper_camel_case::#desirable_upper_camel_case(#value_snake_case)}
            },
            &operation
                .desirable_status_code()
                .to_axum_http_status_code_ts(),
            &ShouldAddReturn::False,
        );
        quote::quote! {
            #[allow(clippy::single_call_fn)]
            async fn #operation_handle_snake_case_ts(
                #app_state_snake_case: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_ts>,
                #request_snake_case: axum::extract::Request,
                #table_snake_case: &str,
            ) -> axum::response::Response {
                #request_parts_preparation_ts
                #additional_validators_ts
                #parameters_logic_ts
                #expected_updated_primary_keys_ts
                let #query_string_snake_case = #query_string_ts;
                // println!("{}", #query_string_snake_case);
                let #binded_query_snake_case = {
                    #binded_query_ts
                };
                #acquire_pool_and_connection_ts
                let #value_snake_case = {
                    #postgresql_logic_ts
                };
                #wraped_into_axum_response_ts
            }
            #[allow(clippy::single_call_fn)]
            pub async fn #operation_snake_case_ts(
                #app_state_snake_case: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_ts>,
                #request_snake_case: axum::extract::Request,
            ) -> axum::response::Response {
                Self::#operation_handle_snake_case_ts(#app_state_snake_case, #request_snake_case, #self_table_name_call_ts).await
            }
        }
    };
    let generate_parameters_logic_ts = |operation: &Operation| -> proc_macro2::TokenStream {
        let ident_operation_payload_upper_camel_case =
            generate_ident_operation_payload_upper_camel_case(operation);
        let serde_json_syn_variant_wrapper_error_initialization_eprintln_response_creation_ts =
            generate_operation_error_initialization_eprintln_response_creation_ts(
                operation,
                &serde_json_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
            );
        let ident_operation_parameters_upper_camel_case =
            generate_ident_operation_parameters_upper_camel_case(operation);
        //todo in case of large type there is a stackoverflow. for example it was a 3.5md json file generated by create_many_payload_example. 3400 fields = success. 16000 = stackoverflow
        quote::quote! {
            let #parameters_snake_case = #ident_operation_parameters_upper_camel_case {
                #payload_snake_case: match serde_json::from_slice::<#ident_operation_payload_upper_camel_case>(
                    &#body_bytes_snake_case,
                ) {
                    Ok(value_9e6fcd2d) => value_9e6fcd2d,
                    Err(#error_0_ts) => {
                        #serde_json_syn_variant_wrapper_error_initialization_eprintln_response_creation_ts
                    }
                },
            };
        }
    };
    let generate_try_operation_ts = |operation: &Operation, type_variants_from_request_response_syn_variants: &[syn::Variant], result_ok_type_ts: &dyn quote::ToTokens, desirable_from_or_try_from_desirable_with_serialize_deserialize_ts: &dyn quote::ToTokens| {
        let try_operation_snake_case_ts = operation.try_self_snake_case_ts();
        let try_operation_handle_snake_case_ts = operation.try_self_handle_snake_case_ts();
        let ident_try_operation_error_named_upper_camel_case = generate_ident_try_operation_error_named_upper_camel_case(operation);
        let ident_operation_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(operation);
        let payload_ts = {
            let serde_json_to_string_syn_variant_initialization_ts = generate_initialization_ts(&serde_json_to_string_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #payload_snake_case = {
                    match serde_json::to_string(&#parameters_snake_case.#payload_snake_case) {
                        Ok(value_1772a83e) => value_1772a83e,
                        Err(#error_0_ts) => {
                            return Err(#ident_try_operation_error_named_upper_camel_case::#serde_json_to_string_syn_variant_initialization_ts);
                        }
                    }
                };
            }
        };
        let url_ts = {
            let format_handle_ts = generate_quotes::double_quotes_ts(&format!("{{endpoint_location}}/{{table}}/{}", operation.self_snake_case_stringified()));
            quote::quote! {let #url_snake_case = format!(#format_handle_ts);}
        };
        let future_ts = {
            let operation_http_method_snake_case_ts = AsRefStrToSnakeCaseTokenStream::case_or_panic(&operation.http_method());
            let commit_header_addition_ts = quote::quote! {
                .header(
                    &"commit".to_owned(),
                    git_info::PROJECT_GIT_INFO.commit,
                )
            };
            let application_json_double_quotes_ts = generate_quotes::double_quotes_ts(&"application/json");
            let content_type_application_json_header_addition_ts = quote::quote! {
                .header(reqwest::header::CONTENT_TYPE, #application_json_double_quotes_ts)
            };
            quote::quote! {
                let #future_snake_case = reqwest::Client::new()
                    .#operation_http_method_snake_case_ts(&#url_snake_case)
                    #commit_header_addition_ts
                    #content_type_application_json_header_addition_ts
                    .#body_snake_case(#payload_snake_case)
                    .send();
            }
        };
        let response_ts = {
            let reqwest_syn_variant_initialization_ts = generate_initialization_ts(&reqwest_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #response_snake_case = match #future_snake_case.await {
                    Ok(value_180559e9) => value_180559e9,
                    Err(#error_0_ts) => {
                        return Err(#ident_try_operation_error_named_upper_camel_case::#reqwest_syn_variant_initialization_ts);
                    }
                };
            }
        };
        let error_0_response_status_ts = quote::quote! {
            let #error_0_ts = #response_snake_case.status();
        };
        let headers_ts = quote::quote! {
            let #error_1_ts = #response_snake_case.headers().clone();
        };
        let response_text_ts = {
            let failed_to_get_response_text_syn_variant_initialization_ts = generate_initialization_ts(&failed_to_get_response_text_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #error_2_ts = match #response_snake_case.text().await {
                    Ok(value_6a62b2b9) => value_6a62b2b9,
                    Err(#error_2_ts) => {
                        return Err(#ident_try_operation_error_named_upper_camel_case::#failed_to_get_response_text_syn_variant_initialization_ts);
                    }
                };
            }
        };
        let ident_operation_response_variants_upper_camel_case = generate_ident_operation_response_variants_upper_camel_case(operation);
        let expected_response_ts = {
            let deserialize_response_syn_variant_initialization_ts = generate_initialization_ts(&deserialize_response_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #expected_response_snake_case = match serde_json::from_str::<#ident_operation_response_variants_upper_camel_case>(&#error_2_ts) {
                    Ok(value_563d2a75) => value_563d2a75,
                    Err(#error_3_ts) => {
                        return Err(#ident_try_operation_error_named_upper_camel_case::#deserialize_response_syn_variant_initialization_ts);
                    }
                };
            }
        };
        let try_operation_logic_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_operation_error_named_with_serialize_deserialize_upper_camel_case(operation);
        let operation_error_named_with_serialize_deserialize_snake_case = &operation.operation_error_named_with_serialize_deserialize_snake_case();
        let try_operation_logic_error_named_with_serialize_deserialize_ts = {
            let try_operation_logic_response_variants_to_try_operation_logic_error_named_with_serialize_deserialize = type_variants_from_request_response_syn_variants.iter().map(|el_f83d5272| {
                let variant_ident = &el_f83d5272.ident;
                let fields_idents_ts = if let syn::Fields::Named(fields_named) = &el_f83d5272.fields {
                    let fields_idents = fields_named.named.iter().map(|field| &field.ident);
                    quote::quote! {#(#fields_idents),*}
                } else {
                    panic!("8dcafc1c-2fc8-4d85-b0ba-cc7725b313a0");
                };
                quote::quote! {
                    #ident_operation_response_variants_upper_camel_case::#variant_ident {
                        #fields_idents_ts
                    } => #try_operation_logic_error_named_with_serialize_deserialize_upper_camel_case::#variant_ident { #fields_idents_ts }
                }
            });
            quote::quote! {
                let #operation_error_named_with_serialize_deserialize_snake_case = match #expected_response_snake_case {
                    #ident_operation_response_variants_upper_camel_case::#desirable_upper_camel_case(#value_snake_case) => {
                        return Ok(#desirable_from_or_try_from_desirable_with_serialize_deserialize_ts);
                    },
                    #(#try_operation_logic_response_variants_to_try_operation_logic_error_named_with_serialize_deserialize),*
                };
            }
        };
        let return_error_ts = {
            let field_code_occurence_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_ts = macros_helpers::generate_field_code_occurence_new_ts(file!(), line!(), column!());
            quote::quote! {
                Err(#ident_try_operation_error_named_upper_camel_case::#try_operation_logic_error_named_with_serialize_deserialize_upper_camel_case {
                    #operation_error_named_with_serialize_deserialize_snake_case,
                    #field_code_occurence_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_ts,
                })
            }
        };
        quote::quote! {
            #[allow(clippy::single_call_fn)]
            async fn #try_operation_handle_snake_case_ts(
                #endpoint_location_snake_case: #ref_std_primitive_str,
                #parameters_snake_case: #ident_operation_parameters_upper_camel_case,
                #table_snake_case: &str,
            ) -> Result<#result_ok_type_ts, #ident_try_operation_error_named_upper_camel_case> {
                #payload_ts
                #url_ts
                #future_ts
                #response_ts
                #error_0_response_status_ts
                #headers_ts
                #response_text_ts
                #expected_response_ts
                #try_operation_logic_error_named_with_serialize_deserialize_ts
                #return_error_ts
            }
            pub async fn #try_operation_snake_case_ts(
                #endpoint_location_snake_case: #ref_std_primitive_str,
                #parameters_snake_case: #ident_operation_parameters_upper_camel_case
            ) -> Result<#result_ok_type_ts, #ident_try_operation_error_named_upper_camel_case> {
                Self::#try_operation_handle_snake_case_ts(
                    #endpoint_location_snake_case,
                    #parameters_snake_case,
                    #self_table_name_call_ts
                ).await
            }
        }
    };
    let generate_match_ident_read_only_ids_as_from_row_from_row_ts =
        |content_ts: &dyn quote::ToTokens| {
            quote::quote! {
                match <#ident_read_only_ids_upper_camel_case as sqlx::FromRow<'_, sqlx::postgres::PgRow>>::from_row(&value_b27d7d79) {
                    Ok(value_33759463) => value_33759463,
                    Err(#error_0_ts) => #content_ts
                }
            }
        };
    let generate_create_update_delete_many_fetch_ts =
        |create_or_update_or_delete_many: &CreateOrUpdateOrDeleteMany| {
            let current_operation = Operation::from(create_or_update_or_delete_many);
            generate_fetch_ts(
                &executor_snake_case,
                &match &create_or_update_or_delete_many {
                    CreateOrUpdateOrDeleteMany::Create | CreateOrUpdateOrDeleteMany::Update => {
                        let content_ts = generate_match_ident_read_only_ids_as_from_row_from_row_ts(
                            &{
                                let content_ts = generate_drop_rows_match_postgres_transaction_rollback_await_handle_ts(&current_operation, file!(), line!(), column!(), file!(), line!(), column!());
                                quote::quote! {{#content_ts}}
                            },
                        );
                        quote::quote! {Some(#content_ts)}
                    }
                    CreateOrUpdateOrDeleteMany::Delete => generate_sqlx_row_try_get_primary_key_ts(
                        &primary_key_field_type_as_postgresql_type_read_upper_camel_case,
                        &quote::quote! {Some(value_69ecb6a9)},
                        &generate_drop_rows_match_postgres_transaction_rollback_await_handle_ts(
                            &current_operation,
                            file!(),
                            line!(),
                            column!(),
                            file!(),
                            line!(),
                            column!(),
                        ),
                    ),
                },
                &generate_drop_rows_match_postgres_transaction_rollback_await_handle_ts(
                    &current_operation,
                    file!(),
                    line!(),
                    column!(),
                    file!(),
                    line!(),
                    column!(),
                ),
                &ShouldWrapIntoValue::True,
            )
        };
    let generate_create_update_delete_one_fetch_ts =
        |create_or_update_or_delete_one: &CreateOrUpdateOrDeleteOne| {
            let current_operation = Operation::from(create_or_update_or_delete_one);
            wrap_into_value_ts(&generate_fetch_one_ts(
                &executor_snake_case,
                &generate_sqlx_row_try_get_primary_key_ts(
                    &quote::quote! {#primary_key_field_type_as_postgresql_type_read_upper_camel_case},
                    &quote::quote! {value_69ecb6a9},
                    &generate_match_postgres_transaction_rollback_await_ts(
                        &current_operation,
                        file!(),
                        line!(),
                        column!(),
                        file!(),
                        line!(),
                        column!(),
                    ),
                ),
                &generate_match_postgres_transaction_rollback_await_ts(
                    &current_operation,
                    file!(),
                    line!(),
                    column!(),
                    file!(),
                    line!(),
                    column!(),
                ),
            ))
        };
    let generate_operation_payload_example_ts = |operation: &Operation| {
        let operation_payload_example_snake_case = operation.operation_payload_example_snake_case();
        let wraped_into_axum_response_ts = wrap_into_axum_response_ts(
            &{
                let ident_operation_payload_upper_camel_case =
                    generate_ident_operation_payload_upper_camel_case(operation);
                quote::quote! {<#ident_operation_payload_upper_camel_case as postgresql_crud::#default_option_some_vec_one_el_upper_camel_case>::#default_option_some_vec_one_el_snake_case()}
            },
            &quote::quote! {http::StatusCode::OK},
            &ShouldAddReturn::False,
        );
        quote::quote! {
            #must_use_ts
            pub fn #operation_payload_example_snake_case() -> axum::response::Response {
                #wraped_into_axum_response_ts
            }
        }
    };
    let increment_initialization_ts = quote::quote! {let mut #increment_snake_case: u64 = 0;};
    let column_names = {
        let mut value = fields
            .iter()
            .fold(String::default(), |mut acc_b2600a1f, element| {
                use std::fmt::Write as _;
                assert!(
                    write!(acc_b2600a1f, "{}", &element.field_ident).is_ok(),
                    "b9fe50dc-69a2-4af1-801d-69b7839a1471"
                );
                acc_b2600a1f.push(',');
                acc_b2600a1f
            });
        let _: Option<char> = value.pop();
        value
    };
    let column_names_double_quotes_ts = generate_quotes::double_quotes_ts(&column_names);
    let generate_select_only_ids_query_part_ts = |operation: &Operation| {
        let select_only_ids_query_part_initialization_ts = fields.iter().map(|element: &macros_helpers::SynFieldWrapper| {
            let field_ident = &element.field_ident;
            let field_ident_double_quotes_ts = generate_quotes::double_quotes_ts(&field_ident);
            let field_type_as_postgresql_crud_postgresql_type_postgresql_type_ts = generate_as_postgresql_type_ts(&element.field_type);
            let content_ts_00878df8 = generate_operation_error_initialization_eprintln_response_creation_ts(operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                match #field_type_as_postgresql_crud_postgresql_type_postgresql_type_ts #select_only_ids_query_part_snake_case(#field_ident_double_quotes_ts) {
                    Ok(value_aa341baf) => {
                        acc_a35168d8.push_str(&value_aa341baf);
                    },
                    Err(#error_0_ts) => {
                        #content_ts_00878df8
                    }
                }
            }
        });
        quote::quote! {
            {
                let mut acc_a35168d8 = #string_ts::new();
                #(#select_only_ids_query_part_initialization_ts)*
                let _: Option<char> = acc_a35168d8.pop();
                acc_a35168d8
            }
        }
    };
    let generate_write_into_buffer_query_part_syn_variant_error_initialization_eprintln_response_creation_ts =
        |operation: &Operation| {
            let query_part_error_named_write_into_buffer_ts =
                postgresql_crud_macros_common::generate_query_part_error_named_write_into_buffer_ts(
                    import_path,
                );
            let content_ts_fa8795ea =
                generate_operation_error_initialization_eprintln_response_creation_ts(
                    operation,
                    &query_part_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            quote::quote! {
                let #error_0_ts = #query_part_error_named_write_into_buffer_ts;
                #content_ts_fa8795ea
            }
        };
    let create_many_ts = {
        let operation = Operation::CreateMany;
        let type_variants_from_request_response_syn_variants =
            generate_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = generate_parameters_pattern_ts(
            &operation,
            generate_parameters_payload_and_default_ts(
                &operation,
                &{
                    let std_vec_vec_ident_create_ts =
                        postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(
                            &ident_create_upper_camel_case,
                        );
                    quote::quote! {(pub #std_vec_vec_ident_create_ts);}
                },
                &quote::quote! {(vec![#postgresql_crud_default_option_some_vec_one_el_call_ts])},
            ),
        );
        let operation_ts = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_ts(&operation, &std_vec_vec_ident_read_only_ids_ts, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_ts = generate_parameters_logic_ts(&operation);
                let query_string_ts = {
                    let if_write_is_err_ts = macros_helpers::generate_if_write_is_err_ts(
                        &quote::quote! {
                            acc_8a58994e,
                            "({value_f4fdd10d}),"
                        },
                        &generate_write_into_buffer_query_part_syn_variant_error_initialization_eprintln_response_creation_ts(
                            &operation
                        )
                    );
                    let content_ts_4b2a4911 =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &query_part_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    let select_only_ids_query_part_ts =
                        generate_select_only_ids_query_part_ts(&operation);
                    quote::quote! {#postgresql_crud_snake_case::generate_create_many_query_string(
                        #table_snake_case,
                        #column_names_double_quotes_ts,
                        &{
                            #increment_initialization_ts
                            let mut acc_8a58994e = #string_ts::default();
                            for el_1651705d in &#parameters_snake_case.#payload_snake_case.0 {
                                match el_1651705d.#create_query_part_snake_case(&mut #increment_snake_case) {
                                    Ok(value_f4fdd10d) => {
                                        #if_write_is_err_ts
                                    },
                                    Err(#error_0_ts) => {
                                        #content_ts_4b2a4911
                                    }
                                }
                            }
                            let _: Option<char> = acc_8a58994e.pop();
                            acc_8a58994e
                        },
                        &#select_only_ids_query_part_ts
                    )}
                };
                let binded_query_ts = {
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_ts =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &try_bind_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    quote::quote! {
                        let mut #query_snake_case = sqlx::query::<sqlx::Postgres>(&#query_string_snake_case);
                        for el_7f862135 in #parameters_snake_case.#payload_snake_case.0 {
                            match el_7f862135.#create_query_bind_snake_case(#query_snake_case) {
                                Ok(value_011a3eb4) => {
                                    #query_snake_case = value_011a3eb4;
                                },
                                Err(#error_0_ts) => {
                                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_ts
                                }
                            }
                        }
                        #query_snake_case
                    }
                };
                let postgresql_logic_ts =
                    wrap_content_into_postgresql_transaction_begin_commit_value_ts(
                        &operation,
                        &generate_create_update_delete_many_fetch_ts(
                            &CreateOrUpdateOrDeleteMany::Create,
                        ),
                    );
                impl_ident_vec_ts.push(generate_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &proc_macro2::TokenStream::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &postgresql_logic_ts,
                ));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts
            }
        };
        let try_operation_ts = {
            let try_operation_error_named_ts = generate_ident_try_operation_error_named_ts(
                &operation,
                &common_http_request_syn_variants,
            );
            impl_ident_vec_ts.push(generate_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_ident_read_only_ids_ts,
                &value_snake_case,
            ));
            quote::quote! {
                #try_operation_error_named_ts
            }
        };
        impl_ident_vec_ts.push(generate_operation_payload_example_ts(&operation));
        quote::quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config
            .create_many_content_write_into_generate_postgresql_table_create_many,
        "generate_postgresql_table_create_many",
        &create_many_ts,
        &macros_helpers::FormatWithCargofmt::True,
    );
    let create_one_ts = {
        let operation = Operation::CreateOne;
        let type_variants_from_request_response_syn_variants =
            generate_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts =
            generate_parameters_pattern_ts(&operation, proc_macro2::TokenStream::new());
        let operation_ts = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_ts(&operation, &ident_read_only_ids_upper_camel_case, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_ts = generate_parameters_logic_ts(&operation);
                let query_string_ts = {
                    let content_ts_cfcf1c2a =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &query_part_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    let select_only_ids_query_part_ts =
                        generate_select_only_ids_query_part_ts(&operation);
                    quote::quote! {
                        #postgresql_crud_snake_case::generate_create_one_query_string(
                            #table_snake_case,
                            #column_names_double_quotes_ts,
                            &match #parameters_snake_case.#payload_snake_case.#create_query_part_snake_case(&mut 0) {
                                Ok(value_3267d57d) => value_3267d57d,
                                Err(#error_0_ts) => {
                                    #content_ts_cfcf1c2a
                                }
                            },
                            &#select_only_ids_query_part_ts
                        )
                    }
                };
                let binded_query_ts = {
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_ts =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &try_bind_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_ts(&#query_string_snake_case);
                        match #parameters_snake_case.#payload_snake_case.#create_query_bind_snake_case(#query_snake_case) {
                            Ok(value_06f852cd) => {
                                #query_snake_case = value_06f852cd;
                            },
                            Err(#error_0_ts) => {
                                #postgresql_syn_variant_error_initialization_eprintln_response_creation_ts
                            }
                        }
                        #query_snake_case
                    }
                };
                let postgresql_logic_ts =
                    wrap_content_into_postgresql_transaction_begin_commit_value_ts(
                        &operation,
                        // &generate_create_update_delete_one_fetch_ts(&CreateOrUpdateOrDeleteOne::Create)
                        &{
                            let current_operation =
                                Operation::from(&CreateOrUpdateOrDeleteOne::Create);
                            wrap_into_value_ts(&generate_fetch_one_ts(
                                &executor_snake_case,
                                &generate_match_ident_read_only_ids_as_from_row_from_row_ts(&{
                                    let content_ts =
                                        generate_match_postgres_transaction_rollback_await_ts(
                                            &current_operation,
                                            file!(),
                                            line!(),
                                            column!(),
                                            file!(),
                                            line!(),
                                            column!(),
                                        );
                                    quote::quote! {{#content_ts}}
                                }),
                                &generate_match_postgres_transaction_rollback_await_ts(
                                    &current_operation,
                                    file!(),
                                    line!(),
                                    column!(),
                                    file!(),
                                    line!(),
                                    column!(),
                                ),
                            ))
                        },
                    );
                impl_ident_vec_ts.push(generate_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &proc_macro2::TokenStream::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &postgresql_logic_ts,
                ));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts
            }
        };
        let try_operation_ts = {
            let try_operation_error_named_ts = generate_ident_try_operation_error_named_ts(
                &operation,
                &common_http_request_syn_variants,
            );
            impl_ident_vec_ts.push(generate_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &ident_read_only_ids_upper_camel_case,
                &value_snake_case,
            ));
            quote::quote! {
                #try_operation_error_named_ts
            }
        };
        impl_ident_vec_ts.push(generate_operation_payload_example_ts(&operation));
        quote::quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config
            .create_one_content_write_into_generate_postgresql_table_create_one,
        "generate_postgresql_table_create_one",
        &create_one_ts,
        &macros_helpers::FormatWithCargofmt::True,
    );
    let read_many_ts = {
        let operation = Operation::ReadMany;
        let type_variants_from_request_response_syn_variants =
            generate_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(not_unique_field_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = generate_parameters_pattern_ts(
            &operation,
            generate_parameters_payload_and_default_ts(
                &operation,
                &quote::quote! {{
                    #pub_where_many_std_option_option_ident_where_many_ts,
                    #pub_select_postgresql_crud_not_empty_unique_vec_ident_select_ts,
                    pub #order_by_snake_case: #postgresql_crud_order_by_ts<#ident_select_upper_camel_case>,
                    pub #pagination_snake_case: postgresql_crud::PaginationStartsWithZero,
                }},
                &quote::quote! {{
                    #where_many_postgresql_crud_default_option_some_vec_one_el_call_ts,
                    #select_postgresql_crud_default_option_some_vec_one_el_call_ts,
                    #order_by_snake_case: postgresql_crud::OrderBy {
                        #column_snake_case: #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_ts(
                            #postgresql_crud_default_option_some_vec_one_el_call_ts
                        ),
                        #order_snake_case: Some(
                            #postgresql_crud_default_option_some_vec_one_el_call_ts
                        ),
                    },
                    #pagination_snake_case: #postgresql_crud_default_option_some_vec_one_el_call_ts,
                }},
            ),
        );
        let operation_ts = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_ts(&operation, &std_vec_vec_struct_options_ident_ts, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_ts = generate_parameters_logic_ts(&operation);
                let query_string_ts = {
                    let select_query_part_parameters_payload_select_ts =
                        generate_select_query_part_parameters_payload_select_ts(&operation);
                    let additional_paramaters_initialization_ts =
                        generate_read_or_delete_many_additional_paramaters_initialization_ts(
                            &ReadManyOrDeleteMany::ReadMany,
                        );
                    let additional_parameters_order_by_handle_ts =
                        generate_quotes::double_quotes_ts(&format!(
                            "{{}}{order_snake_case} {by_snake_case} {{}} {{}}"
                        ));
                    let content_ts_0ec756e2 =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &query_part_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    let order_by_column_match_ts = generate_fields_named_with_comma_ts(
                        &|element: &macros_helpers::SynFieldWrapper| {
                            let field_ident_upper_camel_case =
                                naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(
                                    &element.field_ident,
                                );
                            let field_ident_double_quotes_ts =
                                generate_quotes::double_quotes_ts(&element.field_ident);
                            quote::quote! {
                                #ident_select_upper_camel_case::#field_ident_upper_camel_case(_) => #field_ident_double_quotes_ts
                            }
                        },
                    );
                    let if_write_is_err_curly_braces_0_ts = macros_helpers::generate_if_write_is_err_curly_braces_ts(
                        &quote::quote! {
                            #additional_parameters_snake_case,
                            #additional_parameters_order_by_handle_ts,
                            #prefix_snake_case,
                            &match &#parameters_snake_case.#payload_snake_case.#order_by_snake_case.#column_snake_case {
                                #order_by_column_match_ts
                            },
                            #parameters_snake_case.#payload_snake_case.#order_by_snake_case.#order_snake_case.as_ref().map_or_else(
                                || postgresql_crud::Order::default().to_snake_case_stringified(),
                                #import_path::Order::to_snake_case_stringified
                            )
                        },
                        &generate_write_into_buffer_query_part_syn_variant_error_initialization_eprintln_response_creation_ts(&operation),
                    );
                    let if_write_is_err_curly_braces_1_ts = macros_helpers::generate_if_write_is_err_curly_braces_ts(
                        &quote::quote! {
                            #additional_parameters_snake_case,
                            "{prefix}{}",
                            match #postgresql_crud_postgresql_type_where_filter_query_part_ts(
                                &#parameters_snake_case.#payload_snake_case.pagination,
                                &mut #increment_snake_case,
                                &"",
                                bool::default()
                            ) {
                                Ok(value_742be6cf) => value_742be6cf,
                                Err(#error_0_ts) => {
                                    #content_ts_0ec756e2
                                },
                            }
                        },
                        &generate_write_into_buffer_query_part_syn_variant_error_initialization_eprintln_response_creation_ts(&operation)
                    );
                    quote::quote! {#postgresql_crud_snake_case::generate_read_many_query_string(
                        #table_snake_case,
                        &#select_query_part_parameters_payload_select_ts,
                        &{
                            #increment_initialization_ts
                            let mut #additional_parameters_snake_case = #additional_paramaters_initialization_ts;
                            let #prefix_snake_case = if additional_parameters.is_empty() {""} else {" "};
                            #if_write_is_err_curly_braces_0_ts
                            #if_write_is_err_curly_braces_1_ts
                            #additional_parameters_snake_case
                        }
                    )}
                };
                let binded_query_ts = {
                    let query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_ts = generate_query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_ts(&operation);
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_ts =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &try_bind_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_ts(&#query_string_snake_case);
                        #query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_ts
                        match #postgresql_crud_postgresql_type_where_filter_query_bind_ts(
                            #parameters_snake_case.#payload_snake_case.pagination,
                            #query_snake_case,
                        ) {
                            Ok(value_9f7e487b) => {
                                #query_snake_case = value_9f7e487b;
                            },
                            Err(#error_0_ts) => {
                                #postgresql_syn_variant_error_initialization_eprintln_response_creation_ts
                            }
                        }
                        #query_snake_case
                    }
                };
                let postgresql_logic_ts = {
                    let fetch_ts = generate_fetch_ts(
                        &executor_acquire_snake_case,
                        &{
                            let match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_ident_select_ts = generate_match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_ident_select_ts(&ReadManyOrReadOne::ReadMany);
                            quote::quote! {Some(#match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_ident_select_ts)}
                        },
                        &generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &postgresql_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        ),
                        &ShouldWrapIntoValue::False,
                    );
                    quote::quote! {{
                        #fetch_ts
                    }}
                };
                impl_ident_vec_ts.push(generate_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &proc_macro2::TokenStream::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &postgresql_logic_ts,
                ));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts
            }
        };
        let try_operation_ts = {
            let try_operation_error_named_ts =
                generate_ident_try_operation_error_named_ts(&operation, &{
                    let mut value = common_http_request_syn_variants.clone();
                    value.push(
                        not_unique_field_syn_variant_wrapper
                            .get_syn_variant()
                            .clone(),
                    );
                    value
                });
            impl_ident_vec_ts.push(generate_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_struct_options_ident_ts,
                &quote::quote! {
                    #value_snake_case
                    .into_iter()
                    .fold(Vec::new(), |mut acc_4adf5a80, el_6a197212| {
                        acc_4adf5a80.push(el_6a197212);
                        acc_4adf5a80
                    })
                },
            ));
            quote::quote! {
                #try_operation_error_named_ts
            }
        };
        impl_ident_vec_ts.push(generate_operation_payload_example_ts(&operation));
        quote::quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config
            .read_many_content_write_into_generate_postgresql_table_read_many,
        "generate_postgresql_table_read_many",
        &read_many_ts,
        &macros_helpers::FormatWithCargofmt::True,
    );
    let read_one_ts = {
        let operation = Operation::ReadOne;
        let type_variants_from_request_response_syn_variants =
            generate_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(not_unique_field_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = generate_parameters_pattern_ts(
            &operation,
            generate_parameters_payload_and_default_ts(
                &operation,
                &{
                    let pub_handle_primary_key_field_ident_primary_key_inner_type_handle_ts = generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_ts(&SelfReadUpperCamelCase::from_type_last_segment(primary_key_field_type));
                    quote::quote! {{
                        #pub_handle_primary_key_field_ident_primary_key_inner_type_handle_ts,
                        #pub_select_postgresql_crud_not_empty_unique_vec_ident_select_ts,
                    }}
                },
                &quote::quote! {{
                    #primary_key_field_ident: #postgresql_crud_default_option_some_vec_one_el_call_ts,
                    #select_postgresql_crud_default_option_some_vec_one_el_call_ts
                }},
            ),
        );
        let operation_ts = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts = generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_ts(&operation, &ident_read_upper_camel_case, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_ts = generate_parameters_logic_ts(&operation);
                let query_string_ts = {
                    let select_query_part_parameters_payload_select_ts =
                        generate_select_query_part_parameters_payload_select_ts(&operation);
                    let content_ts_1ead7cf9 =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &query_part_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    quote::quote! {#postgresql_crud_snake_case::generate_read_one_query_string(
                        #table_snake_case,
                        &#select_query_part_parameters_payload_select_ts,
                        &match #postgresql_crud_postgresql_type_where_filter_query_part_ts(
                            &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            &mut 0,
                            &Self::#primary_key_snake_case(),
                            false
                        ) {
                            Ok(value_be9e7b7d) => value_be9e7b7d,
                            Err(#error_0_ts) => {
                                #content_ts_1ead7cf9
                            }
                        }
                    )}
                };
                let binded_query_ts = {
                    let binded_query_modifications_ts = {
                        let postgresql_syn_variant_error_initialization_eprintln_response_creation_ts =
                            generate_operation_error_initialization_eprintln_response_creation_ts(
                                &operation,
                                &try_bind_syn_variant_wrapper,
                                file!(),
                                line!(),
                                column!(),
                            );
                        quote::quote! {
                            match #postgresql_crud_postgresql_type_where_filter_query_bind_ts(#parameters_snake_case.#payload_snake_case.#primary_key_field_ident, #query_snake_case) {
                                Ok(value_80ee6983) => {
                                    #query_snake_case = value_80ee6983;
                                },
                                Err(#error_0_ts) => {
                                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_ts
                                }
                            }
                        }
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_ts(&#query_string_snake_case);
                        #binded_query_modifications_ts
                        #query_snake_case
                    }
                };
                let postgresql_logic_ts = generate_fetch_one_ts(
                    &executor_acquire_snake_case,
                    &generate_match_ident_read_try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_ident_select_ts(&ReadManyOrReadOne::ReadOne),
                    &generate_operation_error_initialization_eprintln_response_creation_ts(&operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!()),
                );
                impl_ident_vec_ts.push(generate_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &proc_macro2::TokenStream::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &postgresql_logic_ts,
                ));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts
            }
        };
        let try_operation_ts = {
            let try_operation_error_named_ts =
                generate_ident_try_operation_error_named_ts(&operation, &{
                    let mut value = common_http_request_syn_variants.clone();
                    value.push(
                        not_unique_field_syn_variant_wrapper
                            .get_syn_variant()
                            .clone(),
                    );
                    value
                });
            impl_ident_vec_ts.push(generate_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &ident_read_upper_camel_case,
                &value_snake_case,
            ));
            quote::quote! {
                #try_operation_error_named_ts
            }
        };
        impl_ident_vec_ts.push(generate_operation_payload_example_ts(&operation));
        quote::quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config
            .read_one_content_write_into_generate_postgresql_table_read_one,
        "generate_postgresql_table_read_one",
        &read_one_ts,
        &macros_helpers::FormatWithCargofmt::True,
    );
    //todo update not only with array of objects with ids but with WHERE and one object
    let update_many_ts = {
        let operation = Operation::UpdateMany;
        let type_variants_from_request_response_syn_variants =
            generate_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = generate_parameters_pattern_ts(&operation, {
            let ident_operation_payload_upper_camel_case =
                generate_ident_operation_payload_upper_camel_case(&operation);
            let std_vec_vec_ident_update_ts =
                postgresql_crud_macros_common::generate_std_vec_vec_tokens_declaration_ts(
                    &ident_update_upper_camel_case,
                );
            let ident_operation_payload_vec_ts =
                macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_serde_serialize()
                    .derive_utoipa_to_schema()
                    .build_struct(
                        &ident_operation_payload_upper_camel_case,
                        &quote::quote! {(#std_vec_vec_ident_update_ts);},
                    );
            let ident_operation_payload_try_new_error_named_upper_camel_case =
                format!("{ident}{operation}PayloadTryNewErrorNamed")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("3da248bb-84ba-48c9-9b7c-e0853198e0aa");
            let not_unique_primary_key_upper_camel_case = naming::NotUniquePrimaryKeyUpperCamelCase;
            let not_unique_primary_key_snake_case = naming::NotUniquePrimaryKeySnakeCase;
            let ident_operation_payload_try_new_error_named_ts = macros_helpers::StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_thiserror_error()
                .derive_error_occurence_lib_error_occurence()
                .build_enum(
                    &ident_operation_payload_try_new_error_named_upper_camel_case,
                    &quote::quote!{{
                        #not_unique_primary_key_upper_camel_case {
                            #[eo_to_std_string_string]
                            #not_unique_primary_key_snake_case: #primary_key_field_type_update_ts,
                            #[eo_to_std_string_string]
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                        }
                    }}
                );
            let impl_pub_try_new_for_ident_operation_payload_ts =
                macros_helpers::generate_impl_pub_try_new_for_ident_ts(
                    &generate_ident_operation_payload_upper_camel_case(&operation),
                    &quote::quote! {#value_snake_case: #std_vec_vec_ident_update_ts},
                    &ident_operation_payload_try_new_error_named_upper_camel_case,
                    &quote::quote! {
                        let mut acc_6bf275fc = Vec::new();
                        for el_35facc3a in &#value_snake_case {
                            if acc_6bf275fc.contains(&&el_35facc3a.#primary_key_field_ident) {
                                return Err(#ident_operation_payload_try_new_error_named_upper_camel_case::#not_unique_primary_key_upper_camel_case {
                                    #not_unique_primary_key_snake_case: el_35facc3a.#primary_key_field_ident,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                            acc_6bf275fc.push(&el_35facc3a.#primary_key_field_ident);
                        }
                        Ok(Self(#value_snake_case))
                    },
                );
            let impl_serde_deserialize_for_ident_update_many_payload_ts = {
                let tuple_struct_ident_operation_payload_double_quotes_ts =
                    generate_quotes::double_quotes_ts(&format!(
                        "tuple struct {ident_operation_payload_upper_camel_case}"
                    ));
                let tuple_struct_ident_operation_payload_with_1_el_double_quotes_ts =
                    generate_quotes::double_quotes_ts(&format!(
                        "tuple struct {ident_operation_payload_upper_camel_case} with 1 element"
                    ));
                let match_ident_update_many_payload_try_new_field0_ts =
                    postgresql_crud_macros_common::generate_match_try_new_in_deserialize_ts(
                        &ident_operation_payload_upper_camel_case,
                        &quote::quote! {__field0},
                    );
                let ident_operation_payload_double_quotes_ts =
                    generate_quotes::double_quotes_ts(&ident_operation_payload_upper_camel_case);
                quote::quote! {
                    #[allow(unused_qualifications)]
                    #[allow(clippy::absolute_paths)]
                    #allow_clippy_arbitrary_source_item_ordering_ts
                    const _: () = {
                        #[allow(unused_extern_crates, clippy::useless_attribute, clippy::arbitrary_source_item_ordering)]
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
                                    marker: _serde::__private228::PhantomData<#ident_operation_payload_upper_camel_case>,
                                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = #ident_operation_payload_upper_camel_case;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private228::Formatter<'_>,
                                    ) -> _serde::__private228::fmt::Result {
                                        _serde::__private228::Formatter::write_str(
                                            __formatter,
                                            #tuple_struct_ident_operation_payload_double_quotes_ts,
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
                                        let __field0: #std_vec_vec_ident_update_ts = <#std_vec_vec_ident_update_ts as _serde::Deserialize>::deserialize(__e)?;
                                        #match_ident_update_many_payload_try_new_field0_ts
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let Some(__field0) = _serde::de::SeqAccess::next_element::<#std_vec_vec_ident_update_ts>(&mut __seq)? else {
                                            return Err(_serde::de::Error::invalid_length(0usize, &#tuple_struct_ident_operation_payload_with_1_el_double_quotes_ts));
                                        };
                                        #match_ident_update_many_payload_try_new_field0_ts
                                    }
                                }
                                _serde::Deserializer::deserialize_newtype_struct(
                                    __deserializer,
                                    #ident_operation_payload_double_quotes_ts,
                                    __Visitor {
                                        marker: _serde::__private228::PhantomData::<Self>,
                                        lifetime: _serde::__private228::PhantomData,
                                    },
                                )
                            }
                        }
                    };
                }
            };
            let impl_postgresql_crud_default_option_some_vec_one_el_for_operation_payload_ts = generate_impl_postgresql_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                &ident_operation_payload_upper_camel_case,
                &quote::quote! {
                    Self(vec![#postgresql_crud_default_option_some_vec_one_el_call_ts])
                },
            );
            quote::quote! {
                #ident_operation_payload_vec_ts
                #ident_operation_payload_try_new_error_named_ts
                #impl_pub_try_new_for_ident_operation_payload_ts
                #impl_serde_deserialize_for_ident_update_many_payload_ts
                #impl_postgresql_crud_default_option_some_vec_one_el_for_operation_payload_ts
            }
        });
        let operation_ts = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_ts(&operation, &std_vec_vec_ident_read_only_ids_ts, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_ts = {
                    let parameters_logic_ts = generate_parameters_logic_ts(&operation);
                    quote::quote! {
                        #parameters_logic_ts
                        let #update_for_query_vec_snake_case = #parameters_snake_case.#payload_snake_case.0.into_iter()
                        .map(#ident_update_for_query_upper_camel_case::#from_handle_snake_case)
                        .collect::<Vec<#ident_update_for_query_upper_camel_case>>();
                    }
                };
                let query_string_ts = {
                    let content_ts_1b64e228 =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &query_part_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    let generate_match_update_query_part_primary_key_operation_ts =
                        |content_ts: &dyn quote::ToTokens| {
                            generate_match_update_query_part_primary_key_ts(&operation, &content_ts)
                        };
                    let fields_named_without_primary_key_update_assignment_ts =
                        generate_fields_named_without_primary_key_without_comma_ts(
                            &|element: &macros_helpers::SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let field_ident_double_quotes_ts =
                                    generate_quotes::double_quotes_ts(&field_ident);
                                let is_field_ident_update_exists_snake_case =
                                    IsSelfUpdateExistSnakeCase::from_tokens(&field_ident);
                                let update_query_part_field_ident_snake_case =
                                    UpdateQueryPartSelfSnakeCase::from_tokens(&field_ident);
                                let generate_when_column_id_then_value_update_many_query_part_snake_case = naming::GenerateWhenColumnIdThenValueUpdateManyQueryPartSnakeCase;
                                quote::quote! {
                                    {
                                        let mut #is_field_ident_update_exists_snake_case = false;
                                        for el_a72f3eac in &#update_for_query_vec_snake_case {
                                            if el_a72f3eac.#field_ident.is_some() {
                                                #is_field_ident_update_exists_snake_case = true;
                                                break;
                                            }
                                        }
                                        if #is_field_ident_update_exists_snake_case {
                                            acc_b86a253a.push_str(&
                                                postgresql_crud::generate_column_equals_case_acc_else_column_end_comma_update_many_query_part(
                                                    #field_ident_double_quotes_ts,
                                                    &{
                                                        let mut acc_8ad06c8c = #string_ts::default();
                                                        for el_defbc401 in &#update_for_query_vec_snake_case {
                                                            if let Some(value_3ea04126) = &el_defbc401.#field_ident {
                                                                acc_8ad06c8c.push_str(&#postgresql_crud_snake_case::#generate_when_column_id_then_value_update_many_query_part_snake_case(
                                                                    Self::#primary_key_snake_case(),
                                                                    &match el_defbc401.#update_query_part_primary_key_snake_case(&mut #increment_snake_case) {
                                                                        Ok(value_00890100) => value_00890100,
                                                                        Err(#error_0_ts) => {
                                                                            #content_ts_1b64e228
                                                                        }
                                                                    },
                                                                    &match #ident_update_for_query_upper_camel_case::#update_query_part_field_ident_snake_case(value_3ea04126, &mut #increment_snake_case) {
                                                                        Ok(value_8797585c) => value_8797585c,
                                                                        Err(#error_0_ts) => {
                                                                            #content_ts_1b64e228
                                                                        }
                                                                    },
                                                                ));
                                                            }
                                                        }
                                                        acc_8ad06c8c
                                                    }
                                                )
                                            );
                                        }
                                    }
                                }
                            },
                        );
                    let if_write_is_err_ts = macros_helpers::generate_if_write_is_err_ts(
                        &{
                            let match_update_query_part_primary_key_operation_ts = generate_match_update_query_part_primary_key_operation_ts(
                                &quote::quote!{el_9b2b56f8}
                            );
                            quote::quote! {
                                acc_a95eb175,
                                "{},",
                                #match_update_query_part_primary_key_operation_ts
                            }
                        },
                        &generate_write_into_buffer_query_part_syn_variant_error_initialization_eprintln_response_creation_ts(
                            &operation
                        )
                    );
                    quote::quote! {
                        {
                            #increment_initialization_ts
                            let elements = {
                                let mut acc_b86a253a = #string_ts::default();
                                #fields_named_without_primary_key_update_assignment_ts
                                let _: Option<char> = acc_b86a253a.pop();
                                acc_b86a253a
                            };
                            let primary_keys = {
                                let mut acc_a95eb175 = #string_ts::default();
                                for el_9b2b56f8 in &#update_for_query_vec_snake_case {
                                    #if_write_is_err_ts
                                }
                                let _: Option<char> = acc_a95eb175.pop();
                                acc_a95eb175
                            };
                            let return_columns = {
                                let mut acc_fd44b0aa = String::new();
                                for el_bcf0dde4 in &#update_for_query_vec_snake_case {
                                    match el_bcf0dde4.select_only_updated_ids_query_part(&mut #increment_snake_case) {
                                        Ok(value_4f536654) => {
                                            acc_fd44b0aa.push_str(&value_4f536654);
                                        },
                                        Err(#error_0_ts) => {
                                            #content_ts_1b64e228
                                        }
                                    }
                                }
                                acc_fd44b0aa
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
                let binded_query_ts = {
                    let postgresql_syn_variant_error_initialization_eprintln_response_creation_ts =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &try_bind_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    let fields_named_without_primary_key_update_assignment_ts =
                        generate_fields_named_without_primary_key_without_comma_ts(
                            &|element: &macros_helpers::SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let as_postgresql_crud_postgresql_type_postgresql_type_ts =
                                    generate_as_postgresql_type_ts(&element.field_type);
                                quote::quote! {
                                    for el_4b24f8f0 in &#update_for_query_vec_snake_case {
                                        if let Some(value_2edaa480) = &el_4b24f8f0.#field_ident {
                                            if let Err(error_696908ba) = #query_snake_case.try_bind(el_4b24f8f0.#primary_key_field_ident) {
                                                let #error_0_ts = error_696908ba.to_string();
                                                #postgresql_syn_variant_error_initialization_eprintln_response_creation_ts
                                            }
                                            match #as_postgresql_crud_postgresql_type_postgresql_type_ts #update_query_bind_snake_case(
                                                value_2edaa480.#value_snake_case.clone(),
                                                #query_snake_case,
                                            ) {
                                                Ok(value_600e67dc) => {
                                                    #query_snake_case = value_600e67dc;
                                                },
                                                Err(#error_0_ts) => {
                                                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_ts
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        );
                    let primary_key_update_assignment_ts = quote::quote! {
                        for el_323f7dfc in &#update_for_query_vec_snake_case {
                            match #primary_key_field_type_as_postgresql_type_ts #update_query_bind_snake_case(
                                el_323f7dfc.#primary_key_field_ident,
                                #query_snake_case,
                            ) {
                                Ok(value_c40a4522) => {
                                    #query_snake_case = value_c40a4522;
                                },
                                Err(#error_0_ts) => {
                                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_ts
                                }
                            }
                        }
                    };
                    let binded_query_select_only_updated_ids_query_bind_ts =
                        generate_fields_named_without_primary_key_without_comma_ts(
                            &|element: &macros_helpers::SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let as_postgresql_crud_postgresql_type_postgresql_type_ts =
                                    generate_as_postgresql_type_ts(&element.field_type);
                                quote::quote! {
                                    for el_a1660ed1 in &#update_for_query_vec_snake_case {
                                        if let Some(value_47030ac2) = &el_a1660ed1.#field_ident {
                                            match #as_postgresql_crud_postgresql_type_postgresql_type_ts select_only_updated_ids_query_bind(
                                                &value_47030ac2.#value_snake_case,
                                                #query_snake_case
                                            ) {
                                                Ok(value_c5b79b95) => {
                                                    #query_snake_case = value_c5b79b95;
                                                },
                                                Err(#error_0_ts) => {
                                                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_ts
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        );
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_ts(&#query_string_snake_case);
                        #fields_named_without_primary_key_update_assignment_ts
                        #primary_key_update_assignment_ts
                        #binded_query_select_only_updated_ids_query_bind_ts
                        #query_snake_case
                    }
                };
                let postgresql_logic_ts =
                    wrap_content_into_postgresql_transaction_begin_commit_value_ts(
                        &operation,
                        &generate_create_update_delete_many_fetch_ts(
                            &CreateOrUpdateOrDeleteMany::Update,
                        ),
                    );
                impl_ident_vec_ts.push(generate_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &proc_macro2::TokenStream::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &postgresql_logic_ts,
                ));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts
            }
        };
        let try_operation_ts = {
            let try_operation_error_named_ts = generate_ident_try_operation_error_named_ts(
                &operation,
                &common_http_request_syn_variants,
            );
            impl_ident_vec_ts.push(generate_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_ident_read_only_ids_ts,
                &value_snake_case,
            ));
            quote::quote! {
                #try_operation_error_named_ts
            }
        };
        impl_ident_vec_ts.push(generate_operation_payload_example_ts(&operation));
        quote::quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config
            .update_many_content_write_into_generate_postgresql_table_update_many,
        "generate_postgresql_table_update_many",
        &update_many_ts,
        &macros_helpers::FormatWithCargofmt::True,
    );
    let update_one_ts = {
        let operation = Operation::UpdateOne;
        let type_variants_from_request_response_syn_variants =
            generate_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts =
            generate_parameters_pattern_ts(&operation, proc_macro2::TokenStream::new());
        let operation_ts = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_ts(&operation, &ident_read_only_ids_upper_camel_case, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_ts = {
                    let parameters_logic_ts = generate_parameters_logic_ts(&operation);
                    quote::quote! {
                        #parameters_logic_ts
                        let #update_for_query_snake_case = #ident_update_for_query_upper_camel_case::#from_handle_snake_case(#parameters_snake_case.#payload_snake_case);
                    }
                };
                let query_string_ts = {
                    let additional_parameters_modification_ts =
                        generate_fields_named_without_primary_key_without_comma_ts(
                            &|element: &macros_helpers::SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let field_ident_double_quotes_ts =
                                    generate_quotes::double_quotes_ts(&field_ident);
                                let content_ts_9ec6b359 = generate_operation_error_initialization_eprintln_response_creation_ts(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                                let generate_column_queals_value_comma_update_one_query_part_snake_case = naming::GenerateColumnQuealsValueCommaUpdateOneQueryPartSnakeCase;
                                let update_query_part_field_ident_snake_case =
                                    UpdateQueryPartSelfSnakeCase::from_tokens(&field_ident);
                                quote::quote! {
                                    if let Some(value_2d144436) = &#update_for_query_snake_case.#field_ident {
                                        acc_683e37b8.push_str(&#postgresql_crud_snake_case::#generate_column_queals_value_comma_update_one_query_part_snake_case(
                                            #field_ident_double_quotes_ts,
                                            &match #ident_update_for_query_upper_camel_case::#update_query_part_field_ident_snake_case(value_2d144436, &mut #increment_snake_case) {
                                                Ok(value_1ec12051) => value_1ec12051,
                                                Err(#error_0_ts) => {
                                                    #content_ts_9ec6b359
                                                }
                                            }
                                        ));
                                    }
                                }
                            },
                        );
                    let primary_key_query_part_snake_case = naming::PrimaryKeyQueryPartSnakeCase;
                    let additional_parameters_primary_key_modification_ts =
                        generate_match_update_query_part_primary_key_ts(
                            &operation,
                            &quote::quote! {#update_for_query_snake_case},
                        );
                    let content_ts_255ad2f1 =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &query_part_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    quote::quote! {
                        {
                            #increment_initialization_ts
                            let #columns_snake_case = {
                                let mut acc_683e37b8 = #string_ts::default();
                                #additional_parameters_modification_ts
                                let _: Option<char> = acc_683e37b8.pop();
                                acc_683e37b8
                            };
                            let #primary_key_query_part_snake_case = #additional_parameters_primary_key_modification_ts;
                            let return_columns = match #update_for_query_snake_case.select_only_updated_ids_query_part(&mut #increment_snake_case) {
                                Ok(value_7f0d86a1) => value_7f0d86a1,
                                Err(#error_0_ts) => {
                                    #content_ts_255ad2f1
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
                let binded_query_ts = {
                    let content_ts_1bdf01cd =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &try_bind_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    let binded_query_modifications_ts =
                        generate_fields_named_without_primary_key_without_comma_ts(
                            &|element: &macros_helpers::SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let as_postgresql_crud_postgresql_type_postgresql_type_ts =
                                    generate_as_postgresql_type_ts(&element.field_type);
                                quote::quote! {
                                    if let Some(value_ed87c152) = &#update_for_query_snake_case.#field_ident {
                                        match #as_postgresql_crud_postgresql_type_postgresql_type_ts #update_query_bind_snake_case(
                                            value_ed87c152.#value_snake_case.clone(),//todo is there a way to remove .clone here?
                                            #query_snake_case
                                        ) {
                                            Ok(value_c3c1b857) => {
                                                #query_snake_case = value_c3c1b857;
                                            }
                                            Err(#error_0_ts) => {
                                                #content_ts_1bdf01cd
                                            }
                                        }
                                    }
                                }
                            },
                        );
                    let binded_query_primary_key_modification_ts = quote::quote! {
                        match #primary_key_field_type_as_postgresql_type_ts #update_query_bind_snake_case(
                            #update_for_query_snake_case.#primary_key_field_ident,
                            #query_snake_case,
                        ) {
                            Ok(value_d64bac39) => {
                                #query_snake_case = value_d64bac39;
                            },
                            Err(#error_0_ts) => {
                                #content_ts_1bdf01cd
                            }
                        }
                    };
                    let binded_query_select_only_updated_ids_query_bind_ts =
                        generate_fields_named_without_primary_key_without_comma_ts(
                            &|element: &macros_helpers::SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let as_postgresql_crud_postgresql_type_postgresql_type_ts =
                                    generate_as_postgresql_type_ts(&element.field_type);
                                quote::quote! {
                                    if let Some(value_b2902425) = &#update_for_query_snake_case.#field_ident {
                                        match #as_postgresql_crud_postgresql_type_postgresql_type_ts select_only_updated_ids_query_bind(
                                            &value_b2902425.#value_snake_case,
                                            #query_snake_case
                                        ) {
                                            Ok(value_cc6145f8) => {
                                                #query_snake_case = value_cc6145f8;
                                            },
                                            Err(#error_0_ts) => {
                                                #content_ts_1bdf01cd
                                            }
                                        }
                                    }
                                }
                            },
                        );
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_ts(&#query_string_snake_case);
                        #binded_query_modifications_ts
                        #binded_query_primary_key_modification_ts
                        #binded_query_select_only_updated_ids_query_bind_ts
                        #query_snake_case
                    }
                };
                let postgresql_logic_ts =
                    wrap_content_into_postgresql_transaction_begin_commit_value_ts(
                        &operation,
                        // &generate_create_update_delete_one_fetch_ts(&CreateOrUpdateOrDeleteOne::Update)
                        &{
                            let current_operation =
                                Operation::from(&CreateOrUpdateOrDeleteOne::Update);
                            wrap_into_value_ts(&generate_fetch_one_ts(
                                &executor_snake_case,
                                &generate_match_ident_read_only_ids_as_from_row_from_row_ts(
                                    &generate_match_postgres_transaction_rollback_await_ts(
                                        &current_operation,
                                        file!(),
                                        line!(),
                                        column!(),
                                        file!(),
                                        line!(),
                                        column!(),
                                    ),
                                ),
                                &generate_match_postgres_transaction_rollback_await_ts(
                                    &current_operation,
                                    file!(),
                                    line!(),
                                    column!(),
                                    file!(),
                                    line!(),
                                    column!(),
                                ),
                            ))
                        },
                    );
                impl_ident_vec_ts.push(generate_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &proc_macro2::TokenStream::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &postgresql_logic_ts,
                ));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts
            }
        };
        let try_operation_ts = {
            let try_operation_error_named_ts = generate_ident_try_operation_error_named_ts(
                &operation,
                &common_http_request_syn_variants,
            );
            impl_ident_vec_ts.push(generate_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &ident_read_only_ids_upper_camel_case,
                &value_snake_case,
            ));
            quote::quote! {
                #try_operation_error_named_ts
            }
        };
        impl_ident_vec_ts.push(generate_operation_payload_example_ts(&operation));
        quote::quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config
            .update_one_content_write_into_generate_postgresql_table_update_one,
        "generate_postgresql_table_update_one",
        &update_one_ts,
        &macros_helpers::FormatWithCargofmt::True,
    );
    //todo return deleted rows ids vec
    let delete_many_ts = {
        let operation = Operation::DeleteMany;
        let type_variants_from_request_response_syn_variants =
            generate_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = generate_parameters_pattern_ts(
            &operation,
            generate_parameters_payload_and_default_ts(
                &operation,
                &quote::quote! {{#pub_where_many_std_option_option_ident_where_many_ts}},
                &quote::quote! {{#where_many_postgresql_crud_default_option_some_vec_one_el_call_ts}},
            ),
        );
        let operation_ts = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_ts(&operation, &std_vec_vec_primary_key_field_type_read_ts, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_ts = generate_parameters_logic_ts(&operation);
                let query_string_ts = {
                    let additional_paramaters_initialization_ts =
                        generate_read_or_delete_many_additional_paramaters_initialization_ts(
                            &ReadManyOrDeleteMany::DeleteMany,
                        );
                    quote::quote! {#postgresql_crud_snake_case::generate_delete_many_query_string(
                        #table_snake_case,
                        &{
                            #increment_initialization_ts
                            #additional_paramaters_initialization_ts
                        },
                        Self::#primary_key_snake_case(),
                    )}
                };
                let binded_query_ts = {
                    let query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_ts = generate_query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_ts(&operation);
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_ts(&#query_string_snake_case);
                        #query_postgresql_type_where_filter_query_bind_parameters_payload_where_many_query_ts
                        #query_snake_case
                    }
                };
                let postgresql_logic_ts =
                    wrap_content_into_postgresql_transaction_begin_commit_value_ts(
                        &operation,
                        &generate_create_update_delete_many_fetch_ts(
                            &CreateOrUpdateOrDeleteMany::Delete,
                        ),
                    );
                impl_ident_vec_ts.push(generate_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &proc_macro2::TokenStream::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &postgresql_logic_ts,
                ));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts
            }
        };
        let try_operation_ts = {
            let try_operation_error_named_ts = generate_ident_try_operation_error_named_ts(
                &operation,
                &common_http_request_syn_variants,
            );
            impl_ident_vec_ts.push(generate_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_field_type_read_ts,
                &value_snake_case,
            ));
            quote::quote! {
                #try_operation_error_named_ts
            }
        };
        impl_ident_vec_ts.push(generate_operation_payload_example_ts(&operation));
        quote::quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config
            .delete_many_content_write_into_generate_postgresql_table_delete_many,
        "generate_postgresql_table_delete_many",
        &delete_many_ts,
        &macros_helpers::FormatWithCargofmt::True,
    );
    let delete_one_ts = {
        let operation = Operation::DeleteOne;
        let type_variants_from_request_response_syn_variants =
            generate_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = generate_parameters_pattern_ts(
            &operation,
            generate_parameters_payload_and_default_ts(
                &operation,
                &{
                    let content_ts = generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_ts(&SelfReadUpperCamelCase::from_type_last_segment(primary_key_field_type));
                    quote::quote! {{#content_ts}}
                },
                &{
                    let primary_key_field_with_default_option_some_vec_one_el_ts = {
                        quote::quote! {
                            #primary_key_field_ident: #postgresql_crud_default_option_some_vec_one_el_call_ts
                        }
                    };
                    quote::quote! {{#primary_key_field_with_default_option_some_vec_one_el_ts}}
                },
            ),
        );
        let operation_ts = {
            let try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts =
                generate_ident_try_operation_logic_response_variants_ident_operation_error_named_convert_ts(&operation, &primary_key_field_type_as_postgresql_type_read_upper_camel_case, &type_variants_from_request_response_syn_variants);
            {
                let parameters_logic_ts = generate_parameters_logic_ts(&operation);
                let query_string_ts = quote::quote! {#postgresql_crud_snake_case::generate_delete_one_query_string(
                    #table_snake_case,
                    Self::#primary_key_snake_case(),
                )};
                let binded_query_ts = {
                    let content_ts_1319f705 =
                        generate_operation_error_initialization_eprintln_response_creation_ts(
                            &operation,
                            &try_bind_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_ts(&#query_string_snake_case);
                        match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(
                            #parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            #query_snake_case
                        ) {
                            Ok(value_3099ea0f) => {
                                #query_snake_case = value_3099ea0f;
                            },
                            Err(#error_0_ts) => {
                                #content_ts_1319f705
                            }
                        }
                        #query_snake_case
                    }
                };
                let postgresql_logic_ts =
                    wrap_content_into_postgresql_transaction_begin_commit_value_ts(
                        &operation,
                        &generate_create_update_delete_one_fetch_ts(
                            &CreateOrUpdateOrDeleteOne::Delete,
                        ),
                    );
                impl_ident_vec_ts.push(generate_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &proc_macro2::TokenStream::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &postgresql_logic_ts,
                ));
            };
            quote::quote! {
                #try_operation_logic_response_variants_impl_std_convert_from_try_operation_logic_error_named_for_try_operation_logic_response_variants_try_operation_logic_error_named_ts
            }
        };
        let try_operation_ts = {
            let try_operation_error_named_ts = generate_ident_try_operation_error_named_ts(
                &operation,
                &common_http_request_syn_variants,
            );
            impl_ident_vec_ts.push(generate_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_as_postgresql_type_read_upper_camel_case,
                &value_snake_case,
            ));
            quote::quote! {
                #try_operation_error_named_ts
            }
        };
        impl_ident_vec_ts.push(generate_operation_payload_example_ts(&operation));
        quote::quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config
            .delete_one_content_write_into_generate_postgresql_table_delete_one,
        "generate_postgresql_table_delete_one",
        &delete_one_ts,
        &macros_helpers::FormatWithCargofmt::True,
    );
    impl_ident_vec_ts.push({
        let routes_handle_ts = {
            let operation_routes_ts = [
                Operation::CreateMany,
                Operation::CreateOne,
                Operation::ReadMany,
                Operation::ReadOne,
                Operation::UpdateMany,
                Operation::UpdateOne,
                Operation::DeleteMany,
                Operation::DeleteOne
            ].into_iter().map(|operation: Operation|{
                let method_ts = match &operation {
                    Operation::CreateMany |
                    Operation::CreateOne |
                    Operation::ReadMany |
                    Operation::ReadOne => quote::quote!{post},
                    Operation::UpdateMany |
                    Operation::UpdateOne => quote::quote!{patch},
                    Operation::DeleteMany |
                    Operation::DeleteOne => quote::quote!{delete},
                };
                let operation_snake_case_ts = operation.self_handle_snake_case_ts();
                let operation_payload_example_snake_case =
                    operation.operation_payload_example_snake_case();
                let (
                    slash_operation_double_quotes_ts,
                    slash_operation_payload_example_double_quotes_ts
                ) = {
                    let generate_slash_route_double_quotes_ts = |
                        value: &dyn Display
                    | generate_quotes::double_quotes_ts(&format!("/{value}"));
                    (
                        generate_slash_route_double_quotes_ts(&operation.self_snake_case_stringified()),
                        generate_slash_route_double_quotes_ts(&operation_payload_example_snake_case)
                    )
                };
                quote::quote!{
                    .route(#slash_operation_double_quotes_ts, axum::routing::#method_ts({
                        let table_owned = table.to_owned();
                        async move |
                            app_state_99328dfe: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>,
                            request: axum::extract::Request
                        | Self::#operation_snake_case_ts(app_state_99328dfe, request, &table_owned).await
                    }))
                    .route(#slash_operation_payload_example_double_quotes_ts, axum::routing::get(async move||Self::#operation_payload_example_snake_case()))
                }
            });
            quote::quote!{
                #[allow(clippy::single_call_fn)]
                fn #routes_handle_snake_case(#app_state_snake_case: #std_sync_arc_combination_of_app_state_logic_traits_ts, #table_snake_case: &str) -> axum::Router {
                    axum::Router::new().nest(
                        &format!("/{table}"),
                        axum::Router::new()
                        #(#operation_routes_ts)*
                        .with_state(#app_state_snake_case)
                    )
                }
            }
        };
        let routes_ts = quote::quote!{
            pub fn #routes_snake_case(#app_state_snake_case: #std_sync_arc_combination_of_app_state_logic_traits_ts) -> axum::Router {
                Self::#routes_handle_snake_case(#app_state_snake_case, #self_table_name_call_ts)
            }
        };
        quote::quote! {
            #routes_handle_ts
            #routes_ts
        }
    });
    let ident_tests_ts = {
        let ident_tests_snake_case = SelfTestsSnakeCase::from_display(&ident);
        let ident_double_quotes_ts =
            generate_quotes::double_quotes_ts(&naming::DisplayToSnakeCaseStringified::case(&ident));
        let ident_create_many_parameters_upper_camel_case =
            generate_ident_operation_parameters_upper_camel_case(&Operation::CreateMany);
        let ident_read_many_parameters_upper_camel_case =
            generate_ident_operation_parameters_upper_camel_case(&Operation::ReadMany);
        let ident_create_many_payload_upper_camel_case =
            generate_ident_operation_payload_upper_camel_case(&Operation::CreateMany);
        let ident_read_many_payload_upper_camel_case =
            generate_ident_operation_payload_upper_camel_case(&Operation::ReadMany);
        let ident_create_one_parameters_upper_camel_case =
            generate_ident_operation_parameters_upper_camel_case(&Operation::CreateOne);
        let ident_read_one_parameters_upper_camel_case =
            generate_ident_operation_parameters_upper_camel_case(&Operation::ReadOne);
        let ident_read_one_payload_upper_camel_case =
            generate_ident_operation_payload_upper_camel_case(&Operation::ReadOne);
        let ident_update_one_parameters_upper_camel_case =
            generate_ident_operation_parameters_upper_camel_case(&Operation::UpdateOne);
        let config_path_ts = quote::quote! {server_config::Config};
        let underscore_unused_ts = quote::quote! {_unused};
        //todo maybe remove it?\
        let generate_some_postgresql_type_where_try_new_ts =
            |logical_operator_ts: &dyn quote::ToTokens, content_ts: &dyn quote::ToTokens| {
                quote::quote! {
                    Some(
                        #import_path::PostgresqlTypeWhere::try_new(
                            #logical_operator_ts,
                            #content_ts
                        ).expect("6b0491b2-1555-4f1c-81f7-5b22d7d353fb"),
                    )
                }
            };
        let generate_some_postgresql_type_where_try_new_and_ts =
            |content_ts: &dyn quote::ToTokens| {
                generate_some_postgresql_type_where_try_new_ts(
                    &quote::quote! {#import_path::LogicalOperator::And},
                    content_ts,
                )
            };
        let generate_postgresql_type_where_try_new_primary_key_content_ts = quote::quote! {
            #import_path::PostgresqlTypeWhere::try_new(
                logical_operator,
                vec
            ).expect("fd20ad6d-fb4c-4da7-96b5-43fce0cdb94c")
        };
        let ident_create_default_fields_initialization_without_primary_key_ts =
            generate_fields_named_without_primary_key_with_comma_ts(
                &|element: &macros_helpers::SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type_as_postgresql_type_create_ts =
                        generate_as_postgresql_type_create_ts(&element.field_type);
                    quote::quote! {
                        #field_ident: <#field_type_as_postgresql_type_create_ts as postgresql_crud::DefaultOptionSomeVecOneEl>::default_option_some_vec_one_el()
                    }
                },
            );
        let fields_none_initialization_ts = generate_fields_named_without_primary_key_with_comma_ts(
            &|element: &macros_helpers::SynFieldWrapper| {
                let field_ident = &element.field_ident;
                quote::quote! {#field_ident: None}
            },
        );
        //todo instead of first dropping table - check if its not exists. if exists test must fail
        let select_default_all_with_max_page_size_not_empty_unique_vec_ts = {
            let content_ts = generate_fields_named_with_comma_ts(
                &|element: &macros_helpers::SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let field_ident_upper_camel_case =
                        naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&field_ident);
                    let upper_camel_case =
                        naming::DefaultOptionSomeVecOneElMaxPageSizeUpperCamelCase;
                    let snake_case = naming::DefaultOptionSomeVecOneElMaxPageSizeSnakeCase;
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
                },
            );
            quote::quote! {
                let select_default_all_with_max_page_size = postgresql_crud::NotEmptyUniqueVec::try_new(vec![
                    #content_ts
                ]).expect("5e82ac66-c7d7-4e1c-8800-b2bb75b0d0cc");
            }
        };
        let generate_primary_key_field_type_as_postgresql_type_primary_key_method_call_ts =
            |method_ts: &dyn quote::ToTokens, parameters_ts: &dyn quote::ToTokens| {
                quote::quote! {
                    <
                        #primary_key_field_type
                        as
                        postgresql_crud::PostgresqlTypePrimaryKey
                    >::#method_ts(
                        #parameters_ts
                    )
                }
            };
        let primary_key_field_type_read_into_table_type_declaration_el_primary_key_field_ident_clone_ts =
            generate_primary_key_field_type_as_postgresql_type_primary_key_method_call_ts(
                &read_into_table_type_declaration_snake_case,
                &quote::quote! {el_adcc8db3},
            );
        let (
            primary_key_field_type_read_only_ids_into_read_el_fdc88812_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_el_43ab7fb5_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_el_bf356906_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_el_80a93892_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_el_adf2b4c4_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
            primary_key_field_type_read_only_is_into_read_read_only_ids_current_el_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts,
        ) = {
            let generate_read_only_ids_into_read_ts = |content_ts: &dyn quote::ToTokens| {
                generate_primary_key_field_type_as_postgresql_type_primary_key_method_call_ts(
                    &read_only_ids_into_read_snake_case,
                    &content_ts,
                )
            };
            (
                generate_read_only_ids_into_read_ts(
                    &quote::quote! {el_fdc88812.#primary_key_field_ident},
                ),
                generate_read_only_ids_into_read_ts(
                    &quote::quote! {el_43ab7fb5.#primary_key_field_ident},
                ),
                generate_read_only_ids_into_read_ts(
                    &quote::quote! {el_bf356906.#primary_key_field_ident},
                ),
                generate_read_only_ids_into_read_ts(
                    &quote::quote! {el_80a93892.#primary_key_field_ident},
                ),
                generate_read_only_ids_into_read_ts(
                    &quote::quote! {el_adf2b4c4.#primary_key_field_ident},
                ),
                generate_read_only_ids_into_read_ts(
                    &quote::quote! {read_only_ids_from_try_create_one.#primary_key_field_ident},
                ),
                generate_read_only_ids_into_read_ts(
                    &quote::quote! {read_only_ids_current_element.#primary_key_field_ident},
                ),
                generate_read_only_ids_into_read_ts(
                    &quote::quote! {read_only_ids_returned_from_create_one.#primary_key_field_ident},
                ),
            )
        };
        let primary_key_field_type_as_postgresql_type_update_as_postgresql_type_primary_key_read_only_ids_into_update_ts = {
            let method_call_ts =
                generate_primary_key_field_type_as_postgresql_type_primary_key_method_call_ts(
                    &read_only_ids_into_update_snake_case,
                    &quote::quote! {read_only_ids_current_element.#primary_key_field_ident},
                );
            quote::quote! {
                <
                    #primary_key_field_type
                    as
                    postgresql_crud::PostgresqlType
                >::Update::from(#method_call_ts)
            }
        };
        let (
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_and_create_ts,
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_from_try_create_one_ident_create_ts,
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_create_ts,
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_clone_ident_create_clone_ts,
        ) = {
            enum ShouldAddDotClone {
                False,
                True,
            }
            let generate_field_ident_read_only_ids_merged_with_create_into_option_value_read_ts =
                |read_only_ids_content_ts: &dyn quote::ToTokens,
                 create_content_ts: &dyn quote::ToTokens,
                 should_add_dot_clone: &ShouldAddDotClone| {
                    generate_fields_named_without_primary_key_with_comma_ts(
                        &|element: &macros_helpers::SynFieldWrapper| {
                            let current_field_ident = &element.field_ident;
                            let current_field_type = &element.field_type;
                            let maybe_dot_clone_ts = match &should_add_dot_clone {
                                ShouldAddDotClone::False => proc_macro2::TokenStream::new(),
                                ShouldAddDotClone::True => quote::quote! {.clone()},
                            };
                            quote::quote! {
                                #current_field_ident: <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(
                                    #read_only_ids_content_ts.#current_field_ident #maybe_dot_clone_ts.expect("f967434c-f45a-47f9-a289-36ef99d80e33"),
                                    #create_content_ts.#current_field_ident #maybe_dot_clone_ts
                                )
                            }
                        },
                    )
                };
            (
                generate_field_ident_read_only_ids_merged_with_create_into_option_value_read_ts(
                    &read_only_ids_snake_case,
                    &create_snake_case,
                    &ShouldAddDotClone::False,
                ),
                generate_field_ident_read_only_ids_merged_with_create_into_option_value_read_ts(
                    &quote::quote! {read_only_ids_from_try_create_one},
                    &quote::quote! {ident_create},
                    &ShouldAddDotClone::False,
                ),
                generate_field_ident_read_only_ids_merged_with_create_into_option_value_read_ts(
                    &quote::quote! {read_only_ids_returned_from_create_one},
                    &quote::quote! {ident_create_default},
                    &ShouldAddDotClone::False,
                ),
                generate_field_ident_read_only_ids_merged_with_create_into_option_value_read_ts(
                    &quote::quote! {read_only_ids_returned_from_create_one},
                    &quote::quote! {ident_create},
                    &ShouldAddDotClone::True,
                ),
            )
        };
        let std_option_option_ident_where_many_content_ts =
            generate_fields_named_without_primary_key_with_comma_ts(
                &|element: &macros_helpers::SynFieldWrapper| {
                    let current_field_ident = &element.field_ident;
                    quote::quote! {
                        #current_field_ident: None
                    }
                },
            );
        let select_default_all_with_max_page_size_clone_ts =
            quote::quote! {select_default_all_with_max_page_size.clone()};

        let common_read_only_ids_returned_from_create_one_ts = {
            let primary_key_read_ts = quote::quote! {primary_key_read};
            let primary_key_read_clone_ts = quote::quote! {primary_key_read.clone()};
            let value_initialization_ts =
                generate_import_path_value_initialization_ts(&primary_key_read_clone_ts);
            quote::quote! {
                let #common_read_only_ids_returned_from_create_one_snake_case = {
                    let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one_default(&#url_snake_case, &table_initialization).await;
                    let primary_key_read = #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts;
                    assert_eq!(
                        #ident_read_upper_camel_case {
                            #primary_key_field_ident: Some(#value_initialization_ts),
                            #fields_none_initialization_ts
                        },
                        generate_ident_try_read_one_handle_primary_key(
                            &#url_snake_case,
                            #primary_key_read_clone_ts,
                            #select_primary_key_snake_case.clone(),
                            &table_initialization
                        )
                        .await
                        .expect("36b95e96-8f97-4088-86e3-c521adf8b199"),
                        "3d9f2ec0-e374-48d2-a36b-486f5598b0b4"
                    );
                    assert_eq!(
                        generate_try_delete_one_handle(
                            &url,
                            #primary_key_read_clone_ts,
                            &table_initialization,
                        ).await.expect("4d96d385-1ff8-4cc4-a8af-b2c8c6118ad4"),
                        #primary_key_read_clone_ts,
                        "26e2058b-4bc1-42da-8f35-0ab993904de5"
                    );
                    generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                        &url,
                        #primary_key_read_ts,
                        #select_default_all_with_max_page_size_clone_ts,
                        &table_initialization,
                    ).await;
                    read_only_ids_from_try_create_one
                };
            }
        };
        let generate_ident_create_content_ts =
            |field_ident: &syn::Ident, content_ts: &dyn quote::ToTokens| {
                generate_fields_named_without_primary_key_with_comma_ts(
                    &|element: &macros_helpers::SynFieldWrapper| {
                        let current_field_ident = &element.field_ident;
                        let current_field_type = &element.field_type;
                        if field_ident == current_field_ident {
                            quote::quote! {
                                #current_field_ident: #content_ts
                            }
                        } else {
                            quote::quote! {
                                #current_field_ident: <
                                    <#current_field_type as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultOptionSomeVecOneEl
                                >::default_option_some_vec_one_el()
                            }
                        }
                    },
                )
            };
        let generate_ident_create_content_el_id_ts =
            |field_ident: &syn::Ident, el_ts: &dyn quote::ToTokens| {
                generate_ident_create_content_ts(field_ident, &el_ts)
            };
        let generate_ident_create_content_el_ts = |field_ident: &syn::Ident| {
            generate_ident_create_content_ts(field_ident, &el_snake_case)
        };
        let generate_table_test_name_field_ident_ts =
            |test_name: &str, field_ident: &syn::Ident| {
                format!("table_{test_name}_{field_ident}")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("eb30c1e4-d208-4fe5-bb73-0c8cbac8b1fd")
            };
        let mut table_field_idents_initialization_vec_ts = Vec::new();
        let mut table_test_name_field_idents_vec_ts = Vec::new();
        let mut fill_table_field_idents_vec_ts = |test_names: Vec<&str>| {
            for el_8f39799f in test_names {
                let generate_initialization_variable_name_ts = |field_ident: &syn::Ident| {
                    format!("table_{el_8f39799f}_{field_ident}")
                        .parse::<proc_macro2::TokenStream>()
                        .expect("2003ad9f-013a-48ba-b0ef-d2d48774d60c")
                };
                table_field_idents_initialization_vec_ts.push(generate_fields_named_without_primary_key_without_comma_ts(&|el_51b56762: &macros_helpers::SynFieldWrapper| {
                    let field_ident = &el_51b56762.field_ident;
                    let initialization_variable_name_ts = generate_initialization_variable_name_ts(field_ident);
                    let format_content_ts = generate_quotes::double_quotes_ts(&format!("{el_8f39799f}_{field_ident}"));
                    quote::quote! {
                        let #initialization_variable_name_ts = add_table_postfix(#format_content_ts);
                    }
                }));
                table_test_name_field_idents_vec_ts.push(
                    generate_fields_named_without_primary_key_without_comma_ts(
                        &|el_785024b5: &macros_helpers::SynFieldWrapper| {
                            let field_ident = &el_785024b5.field_ident;
                            let initialization_variable_name_ts =
                                generate_initialization_variable_name_ts(field_ident);
                            quote::quote! {&#initialization_variable_name_ts,}
                        },
                    ),
                );
            }
        };
        let table_read_only_ids_merged_with_create_into_where_equal_name =
            "8e427ad7_5231_4f1e_8579_2e1aaa5da988";
        let table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name =
            "eb24448c_fa63_4259_bb05_3215802a78f6";
        let table_read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_name =
            "9ac6d79a_2673_4c07_be4a_01c5c20ff1ab";
        let table_create_into_postgresql_type_option_vec_where_dimension_one_equal_name =
            "72940b0e_cd26_493f_9ec1_2d999d9a4401";
        let table_read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_name =
            "5a52af33_a590_403b_808e_961df6d7e7aa";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_name =
            "1f388ef8_dc28_489d_bed9_ca4e7f640dd5";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_name =
            "581c947f_9b0f_452f_8e52_524088bbb2e7";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_name =
            "de556c26_9297_4adb_9483_22d474cf1e7d";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_name =
            "35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d";
        let table_create_into_postgresql_json_type_option_vec_where_length_equal_name =
            "1ce53b67_1e94_413e_83cf_c6d7094289a8";
        let table_create_into_postgresql_json_type_option_vec_where_length_greater_than_name =
            "6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_name =
            "35a01678_f7e2_482d_9803_c3b5a23d36ad";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_name =
            "33a3706a_ef28_4c80_88e0_b8e7fb720de2";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_name =
            "a3e2165c_e030_4b31_ab3d_dcd29f27f90b";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_name =
            "427ac837_383b_4af1_b956_3e64a78e1449";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_name =
            "fe3267a0_f49a_42ce_8e51_2a10e5360eb8";
        let table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_name =
            "b4504737_4463_4e47_bb30_9512275c66b1";

        fill_table_field_idents_vec_ts(vec![
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
            &table_create_into_postgresql_json_type_option_vec_where_length_greater_than_name,
            &table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_name,
            &table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_name,
            &table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_name,
            &table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_name,
            &table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_name,
            &table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_name,
        ]);
        let select_default_all_with_max_page_size_cloned_clone_ts =
            quote::quote! {select_default_all_with_max_page_size_cloned.clone()};
        let read_only_ids_to_two_dimensional_vec_read_inner_acc_fields_ts =
            generate_fields_named_without_primary_key_without_comma_ts(
                &|element: &macros_helpers::SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_ident_read_only_ids_to_two_dimensional_vec_read_inner_acc_snake_case =
                        SelfReadOnlyIdsToTwoDimensionalVecReadInnerAccSnakeCase::from_tokens(
                            &field_ident,
                        );
                    let ident_create_defaults_for_column_read_only_ids_to_two_dimensional_vec_read_inner_ts =
                        generate_fields_named_without_primary_key_without_comma_ts(
                            &|el_0dfa76d6: &macros_helpers::SynFieldWrapper| {
                                let current_field_ident = &el_0dfa76d6.field_ident;
                                let current_field_type = &el_0dfa76d6.field_type;
                                if field_ident == current_field_ident {
                                    quote::quote! {
                                        if let Some(value_a5f7e6cd) = &common_read_only_ids_returned_from_create_one.#current_field_ident {
                                            for el_b3522b7d in <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(value_a5f7e6cd) {
                                                for _ in el_b3522b7d {
                                                    acc_458cda9e.push(ident_create_default.clone());
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    proc_macro2::TokenStream::new()
                                }
                            },
                        );
                    quote::quote! {
                        let #field_ident_read_only_ids_to_two_dimensional_vec_read_inner_acc_snake_case = {
                            let mut acc_458cda9e = Vec::new();
                            #ident_create_defaults_for_column_read_only_ids_to_two_dimensional_vec_read_inner_ts
                            acc_458cda9e
                        };
                    }
                },
            );
        let generate_read_only_ids_current_elements_ts = {
            let ident_read_fields_initialization_without_primary_key_ts =
                generate_fields_named_without_primary_key_with_comma_ts(
                    &|syn_field_wrapper: &macros_helpers::SynFieldWrapper| {
                        let current_field_ident = &syn_field_wrapper.field_ident;
                        let current_field_type = &syn_field_wrapper.field_type;
                        let value_initialization_ts = generate_import_path_value_initialization_ts(
                            &postgresql_crud_default_option_some_vec_one_el_call_ts,
                        );
                        quote::quote! {
                            #current_field_ident: el_f108da5a.#current_field_ident.as_ref().map_or_else(
                                || Some(#value_initialization_ts),
                                <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_option_some_vec_one_el
                            )
                        }
                    },
                );
            quote::quote! {
                async fn generate_read_only_ids_current_elements(
                    url: &str,
                    current_table: &str,
                    select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueVec<#ident_select_upper_camel_case>,
                    read_only_ids_to_two_dimensional_vec_read_inner_acc: Vec<#ident_create_upper_camel_case>
                ) -> Vec<#ident_read_only_ids_upper_camel_case> {
                    let read_only_ids_current_elements = futures::StreamExt::collect::<Vec<Vec<#ident_read_only_ids_upper_camel_case>>>(
                        futures::StreamExt::buffer_unordered(
                            futures::stream::iter(
                                read_only_ids_to_two_dimensional_vec_read_inner_acc
                                .chunks(25)
                                .map(Vec::from)
                                .map(|el_8e425cb1| futures::FutureExt::boxed(async move { #ident::try_create_many_handle(
                                    url,
                                    #ident_create_many_parameters_upper_camel_case {
                                        payload: #ident_create_many_payload_upper_camel_case(el_8e425cb1)
                                    },
                                    current_table
                                ).await.expect("38a24e7a-5b0e-4237-b686-3f03ab332efd") }))
                            ),
                            5
                        )
                    )
                    .await
                    .into_iter()
                    .flatten()
                    .collect::<Vec<#ident_read_only_ids_upper_camel_case>>();
                    assert_eq!(
                        itertools::Itertools::sorted_by(
                            read_only_ids_current_elements.iter().map(|el_f108da5a| {
                                #ident_read_upper_camel_case {
                                    #primary_key_field_ident: <
                                        #primary_key_field_type as postgresql_crud::PostgresqlTypeTestCases
                                    >::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                        &el_f108da5a.#primary_key_field_ident
                                    ),
                                    #ident_read_fields_initialization_without_primary_key_ts
                                }
                            }),
                            |first, second| match (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                (Some(first_handle), Some(second_handle)) => first_handle.#value_snake_case.cmp(&second_handle.#value_snake_case),
                                _ => panic!("0f1d45ed-b6e3-4fbd-bd41-bcbe61739f83"),
                            }
                        ).collect::<Vec<#ident_read_upper_camel_case>>(),
                        itertools::Itertools::sorted_by(
                            generate_try_read_many_order_by_primary_key_with_big_pagination(
                                url,
                                generate_ident_where_many_pripery_key_others_none(
                                    Some(
                                        generate_postgresql_type_where_try_new_primary_key(
                                            postgresql_crud::LogicalOperator::Or,
                                            read_only_ids_current_elements.iter().map(|el_43ab7fb5| #primary_key_field_type_where_ts::Equal(
                                                postgresql_crud::PostgresqlTypeWhereEqual {
                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                    #value_snake_case: #primary_key_field_type_table_type_declaration_ts::new(
                                                        <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                            #primary_key_field_type_read_only_ids_into_read_el_43ab7fb5_primary_key_field_ident_ts
                                                        )
                                                    )
                                                }
                                            )).collect()
                                        )
                                    )
                                ),
                                #select_default_all_with_max_page_size_clone_ts,
                                current_table
                            )
                            .await
                            .expect("097d5e7d-41c6-41f4-8847-720647f2d6ea")
                            .into_iter(),
                            |first, second| match (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                (Some(first_handle), Some(second_handle)) => first_handle.#value_snake_case.cmp(&second_handle.#value_snake_case),
                                _ => panic!("51e477ea-0a01-46f0-89fb-967bb8e4e131"),
                            }
                        )
                        .collect::<Vec<#ident_read_upper_camel_case>>(),
                        "50198a7f-e65c-4e4e-8d7f-9881cfd42453"
                    );
                    read_only_ids_current_elements
                }
            }
        };
        let create_many_tests_ts = {
            let create_many_tests_ts = generate_fields_named_without_primary_key_without_comma_ts(
                &|element: &macros_helpers::SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let ident_create_content_ts = generate_ident_create_content_el_id_ts(
                        field_ident,
                        &quote::quote! {el_03a4f4ee},
                    );
                    quote::quote! {
                        for el_fce0969c in <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#option_vec_create_snake_case().unwrap_or(Vec::new())
                            .chunks(10)
                            .map(Vec::from)
                        {
                            let table_create_many_cloned = table_create_many.clone();
                            let url_cloned = url.clone();
                            let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                            acc_9189f86e.push(futures::FutureExt::boxed(async move {
                                let ident_vec_create = {
                                    let mut acc_92d248f7 = Vec::new();
                                    for el_03a4f4ee in el_fce0969c {
                                        acc_92d248f7.push(#ident_create_upper_camel_case {
                                            #ident_create_content_ts
                                        });
                                    }
                                    acc_92d248f7
                                };
                                let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                                    &url_cloned,
                                    #ident_create_many_parameters_upper_camel_case {
                                        #payload_snake_case: #ident_create_many_payload_upper_camel_case(ident_vec_create.clone())
                                    },
                                    &table_create_many_cloned.clone()
                                ).await.expect("5eecedc4-bb02-454a-acd9-0af758f30b2e");
                                assert_eq!(
                                    generate_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create(
                                        read_only_ids_from_try_create_many.clone(),
                                        ident_vec_create.clone()
                                    ),
                                    generate_try_read_many_order_by_primary_key_with_big_pagination(
                                        &url_cloned,
                                        generate_ident_where_many_pripery_key_others_none(
                                            Some(
                                                generate_postgresql_type_where_try_new_primary_key(
                                                    postgresql_crud::LogicalOperator::Or,
                                                    {
                                                        let mut acc_1381c719 = Vec::new();
                                                        for el_bf356906 in &read_only_ids_from_try_create_many {
                                                            acc_1381c719.push(#primary_key_field_type_as_postgresql_type_where_ts::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                //todo must use trait type instead
                                                                #value_snake_case: #primary_key_field_type_table_type_declaration_ts::new(<#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                                    #primary_key_field_type_read_only_ids_into_read_el_bf356906_primary_key_field_ident_ts
                                                                )),
                                                            }));
                                                        }
                                                        acc_1381c719
                                                    }
                                                )
                                            )
                                        ),
                                        #select_default_all_with_max_page_size_cloned_clone_ts,
                                        &table_create_many_cloned
                                    ).await.expect("bdb72341-016f-4d85-8ce8-abe7e97666ca"),
                                    "d19bbbf6-f64c-4151-8b5b-998a93e13af5"
                                );
                                let read_only_ids_from_try_delete_many = itertools::Itertools::sorted(
                                    #ident::try_delete_many_handle(
                                        &url_cloned,
                                        #ident_delete_many_parameters_upper_camel_case {
                                            //todo rewrite it using new\try_new?
                                            payload: #ident_delete_many_payload_upper_camel_case {
                                                where_many: #std_option_option_ident_where_many_upper_camel_case(Some(
                                                    #ident_where_many_upper_camel_case {
                                                        #primary_key_field_ident: Some(generate_postgresql_type_where_try_new_or_primary_keys(
                                                            &read_only_ids_from_try_create_many
                                                        )),
                                                        #std_option_option_ident_where_many_content_ts
                                                    }
                                                ))
                                            }
                                        },
                                        &table_create_many_cloned
                                    )
                                    .await
                                    .expect("716e470e-d738-4642-adfc-df1f9b945d27")
                                    .into_iter()
                                ).collect::<Vec<<#primary_key_field_type as postgresql_crud::PostgresqlType>::Read>>();
                                assert_eq!(
                                    read_only_ids_from_try_delete_many,
                                    itertools::Itertools::sorted(
                                        read_only_ids_from_try_create_many
                                        .into_iter()
                                        .map(|el_80a93892| {
                                            #primary_key_field_type_read_only_ids_into_read_el_80a93892_primary_key_field_ident_ts
                                        })
                                    ).collect::<Vec<#primary_key_field_type_as_postgresql_type_read_ts>>(),
                                    "f58f5572-4286-4a74-8006-0507339910d4"
                                );
                                assert!(
                                    generate_try_read_many_order_by_primary_key_with_big_pagination(
                                        &url_cloned,
                                        generate_ident_where_many_pripery_key_others_none(
                                            Some(
                                                generate_postgresql_type_where_try_new_primary_key(
                                                    postgresql_crud::LogicalOperator::Or,
                                                    {
                                                        let mut acc_87ea12c9 = Vec::new();
                                                        for el_a37bca54 in &read_only_ids_from_try_delete_many {
                                                            acc_87ea12c9.push(#primary_key_field_type_where_ts::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                #value_snake_case: #primary_key_field_type_table_type_declaration_ts::new(
                                                                    <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(el_a37bca54.clone())
                                                                ),
                                                            }));
                                                        }
                                                        acc_87ea12c9
                                                    }
                                                )
                                            )
                                        ),
                                        #select_default_all_with_max_page_size_cloned_clone_ts,
                                        &table_create_many_cloned
                                    ).await
                                    .expect("24ab86d6-15c9-47f1-a43f-c5fac4b38188")
                                    .is_empty(),
                                    "4e88679a-0d23-418f-8767-4e9b7531429c"
                                );
                            }));
                        }
                    }
                },
            );
            quote::quote! {#create_many_tests_ts}
        };
        let create_one_tests_ts = {
            let create_one_tests_ts = generate_fields_named_without_primary_key_without_comma_ts(
                &|element: &macros_helpers::SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let ident_create_content_ts = generate_ident_create_content_el_id_ts(
                        field_ident,
                        &quote::quote! {el_7632d698},
                    );
                    let value_initialization_ts = generate_import_path_value_initialization_ts(&primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts);
                    quote::quote! {
                        for el_7632d698 in <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#option_vec_create_snake_case().unwrap_or(Vec::new()) {
                            let table_create_one_cloned = table_create_one.clone();
                            let url_cloned = url.clone();
                            let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                            acc_9189f86e.push(futures::FutureExt::boxed(async move {
                                let ident_create = #ident_create_upper_camel_case {
                                    #ident_create_content_ts
                                };
                                let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one(
                                    &url_cloned,
                                    ident_create.clone(),
                                    &table_create_one_cloned
                                ).await;
                                assert_eq!(
                                    #ident_read_upper_camel_case {
                                        #primary_key_field_ident: Some(#value_initialization_ts),
                                        #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_from_try_create_one_ident_create_ts
                                    },
                                    generate_ident_try_read_one_handle_primary_key(
                                        &url_cloned,
                                        #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                                        #select_default_all_with_max_page_size_cloned_clone_ts,
                                        &table_create_one_cloned
                                    )
                                    .await
                                    .expect("f8e1cb88-93ef-440d-9888-49fef60182d6"),
                                    "5f2adbed-f716-440e-a990-4f1c258808b1"
                                );
                                assert_eq!(
                                    generate_try_delete_one_handle(
                                        &url_cloned,
                                        #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                                        &table_create_one_cloned
                                    ).await.expect("20d5a40a-8467-481c-9715-f9b8fef63fbd"),
                                    #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                                    "4f563faf-1d9b-4ef3-8636-f93fde8ef235"
                                );
                                generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                                    &url_cloned,
                                    #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                                    select_default_all_with_max_page_size_cloned,
                                    &table_create_one_cloned,
                                ).await;
                            }));
                        }
                    }
                },
            );
            quote::quote! {#create_one_tests_ts}
        };
        let add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts =
            |content_ts: &dyn quote::ToTokens| {
                quote::quote! {
                    let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one_default(
                        &url_cloned,
                        &current_table
                    ).await;
                    #content_ts
                    let _: #primary_key_field_type_as_postgresql_type_read_ts = generate_try_delete_one_handle(
                        &url_cloned,
                        #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                        &current_table
                    ).await.expect("93b4bf61-3a98-42ea-ab66-015c5d211622");
                    generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                        &url_cloned,
                        #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                        select_default_all_with_max_page_size_cloned,
                        &current_table,
                    ).await;
                }
            };
        let read_many_tests_ts = {
            //todo additional read_many checks
            let test_read_many_by_non_existent_primary_keys_ts = {
                let content_ts = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&{
                    quote::quote! {
                        assert!(
                            generate_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                generate_ident_where_many_pripery_key_others_none(
                                    Some(
                                        generate_postgresql_type_where_try_new_primary_key(
                                            postgresql_crud::LogicalOperator::Or,
                                            std::iter::repeat_with(|| #primary_key_field_type_as_postgresql_type_where_ts::Equal(
                                                postgresql_crud::PostgresqlTypeWhereEqual {
                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                    #value_snake_case: #primary_key_field_type_table_type_declaration_ts::new(
                                                        uuid::Uuid::new_v4()
                                                    )
                                                }
                                            ))
                                            .take(el_30614c66)
                                            .collect::<Vec<_>>()
                                        )
                                    )
                                ),
                                select_default_all_with_max_page_size_cloned.clone(),
                                &current_table
                            ).await
                            .expect("e661c49b-2288-4548-8783-35495e193976")
                            .is_empty(),
                            "06df4025-e2d1-4128-b819-c06613c6ae3f"
                        );
                    }
                });
                quote::quote! {
                    for el_30614c66 in [1,2] {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                        let current_table = table_test_read_many_by_non_existent_primary_keys.clone();
                        acc_9189f86e.push(futures::FutureExt::boxed(async move {
                            #content_ts
                        }));
                    }
                }
            };
            let test_read_many_by_equal_to_created_primary_keys_ts = {
                let content_ts = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&{
                    quote::quote! {
                        let ident_vec_create = std::iter::repeat_n(
                            ident_create_default_cloned.clone(),//todo maybe remove
                            el_a636d084
                        ).collect::<Vec<#ident_create_upper_camel_case>>();
                        let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                            &url_cloned,
                            #ident_create_many_parameters_upper_camel_case {
                                payload: #ident_create_many_payload_upper_camel_case(ident_vec_create.clone())
                            },
                            &current_table
                        ).await.expect("d775179f-f7b1-41d3-9c83-4ca8bd1abeec");
                        assert_eq!(
                            generate_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create(
                                read_only_ids_from_try_create_many.clone(),
                                ident_vec_create.clone()
                            ),
                            generate_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                generate_ident_where_many_pripery_key_others_none(
                                    Some(
                                        generate_postgresql_type_where_try_new_primary_key(
                                            postgresql_crud::LogicalOperator::Or,
                                            read_only_ids_from_try_create_many.iter().map(|el_adf2b4c4| {
                                                #primary_key_field_type_where_ts::Equal(
                                                    postgresql_crud::PostgresqlTypeWhereEqual {
                                                        logical_operator: postgresql_crud::LogicalOperator::Or,
                                                        #value_snake_case: #primary_key_field_type_table_type_declaration_ts::new(
                                                            <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                                #primary_key_field_type_read_only_ids_into_read_el_adf2b4c4_primary_key_field_ident_ts
                                                            )
                                                        ),
                                                    },
                                                )
                                            }).collect()
                                        )
                                    )
                                ),
                                select_default_all_with_max_page_size_cloned.clone(),
                                &current_table
                            ).await.expect("b8efe770-153c-4e4a-ab0e-6484a8dc5343"),
                            "error 3b2cf1f5-2c4e-4908-ba66-f4af84fe0893"
                        );
                        let read_only_ids_from_try_delete_many = itertools::Itertools::sorted(
                            #ident::try_delete_many_handle(
                                &url_cloned,
                                #ident_delete_many_parameters_upper_camel_case {
                                    payload: #ident_delete_many_payload_upper_camel_case {
                                        where_many: #std_option_option_ident_where_many_upper_camel_case(Some(
                                            #ident_where_many_upper_camel_case {
                                                #primary_key_field_ident: Some(generate_postgresql_type_where_try_new_or_primary_keys(&read_only_ids_from_try_create_many)),
                                                #std_option_option_ident_where_many_content_ts
                                            }
                                        )),
                                    },
                                },
                                &current_table
                            )
                            .await
                            .expect("d5c23a9d-eb02-44e4-8654-e2a3d7752f51")
                            .into_iter()
                        ).collect::<Vec<<#primary_key_field_type as postgresql_crud::PostgresqlType>::Read>>();
                        assert_eq!(
                            read_only_ids_from_try_delete_many,
                            itertools::Itertools::sorted(
                                read_only_ids_from_try_create_many
                                .into_iter()
                                .map(|el_fdc88812| {
                                    #primary_key_field_type_read_only_ids_into_read_el_fdc88812_primary_key_field_ident_ts
                                }).collect::<Vec<<#primary_key_field_type as postgresql_crud::PostgresqlType>::Read>>()
                                .into_iter()
                            ).collect::<Vec<<#primary_key_field_type as postgresql_crud::PostgresqlType>::Read>>(),
                            "ebbbea6e-c402-4637-9bab-02678c11926c"
                        );
                        assert!(
                            generate_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                generate_ident_where_many_pripery_key_others_none(
                                    Some(
                                        generate_postgresql_type_where_try_new_primary_key(
                                            postgresql_crud::LogicalOperator::Or,
                                            read_only_ids_from_try_delete_many
                                            .iter()
                                            .map(|el_1e9c87ce| #primary_key_field_type_where_ts::Equal(
                                                postgresql_crud::PostgresqlTypeWhereEqual {
                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                    #value_snake_case: #primary_key_field_type_table_type_declaration_ts::new(
                                                        <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                            el_1e9c87ce.clone()
                                                        )
                                                    ),
                                                },
                                            ))
                                            .collect()
                                        )
                                    )
                                ),
                                select_default_all_with_max_page_size_cloned.clone(),
                                &current_table
                            ).await
                            .expect("1f079962-06af-4d21-a837-c88b0e7db265")
                            .is_empty(),
                            "d79c0af3-5e2e-4891-a7ff-d1007b573e77"
                        );
                    }
                });
                quote::quote! {
                    for el_a636d084 in [1,2] {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                        let current_table = table_test_read_many_by_equal_to_created_primary_keys.clone();
                        let ident_create_default_cloned = ident_create_default.clone();
                        acc_9189f86e.push(futures::FutureExt::boxed(async move {
                            #content_ts
                        }));
                    }
                }
            };
            let generate_read_only_ids_merged_with_create_into_where_assert_eq_ts =
                |ident_where_many_try_new_parameters_content_ts: &dyn quote::ToTokens| {
                    quote::quote! {
                        assert_eq!(
                            vec![
                                #ident_read_upper_camel_case {
                                    #primary_key_field_ident: <
                                        #primary_key_field_type
                                        as
                                        postgresql_crud::PostgresqlTypeTestCases
                                    >::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                        &read_only_ids_returned_from_create_one.#primary_key_field_ident
                                    ),
                                    #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_clone_ident_create_clone_ts
                                }
                            ],
                            generate_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                #ident_where_many_upper_camel_case::try_new(#ident_where_many_try_new_parameters_content_ts).expect("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a"),
                                #select_default_all_with_max_page_size_cloned_clone_ts,
                                &current_table
                            ).await.expect("c3e316c0-e6da-4790-a97b-4ff09fe87a0f"),
                            "ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                        );
                    }
                };
            let generate_option_vec_create_call_unwrap_or_vec_ts =
                |_: &syn::Ident, field_type: &syn::Type| {
                    quote::quote! {
                        <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#option_vec_create_snake_case().unwrap_or(Vec::new())
                    }
                };
            let generate_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_ts =
                |field_ident: &syn::Ident, field_type: &syn::Type| {
                    quote::quote! {
                        <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#option_vec_create_snake_case()
                        .filter(|el_bba28182| !el_bba28182.is_empty())
                        .unwrap_or_else(|| vec![ident_create_default.#field_ident.clone()])
                    }
                };
            let generate_postgresql_type_option_vec_where_greater_than_test_unwrap_or_else_vec_call_ts =
                |_: &syn::Ident, field_type: &syn::Type| {
                    quote::quote! {
                        <#field_type as #import_path::PostgresqlTypeTestCases>::#postgresql_type_option_vec_where_greater_than_test_snake_case()
                        .map_or_else(Vec::new, Into::into)
                    }
                };
            let generate_read_test_ts =
                |test_name: &str,
                 generate_method_call_ts: &dyn Fn(
                    &syn::Ident,
                    &syn::Type,
                ) -> proc_macro2::TokenStream,
                 generate_create_content_ts: &dyn Fn(&syn::Ident) -> proc_macro2::TokenStream,
                 generate_content_ts: &dyn Fn(
                    &macros_helpers::SynFieldWrapper,
                ) -> proc_macro2::TokenStream| {
                    generate_fields_named_without_primary_key_without_comma_ts(
                        &|element: &macros_helpers::SynFieldWrapper| {
                            let field_ident = &element.field_ident;
                            let field_type = &element.field_type;
                            let method_call_ts = generate_method_call_ts(field_ident, field_type);
                            let table_test_name_field_ident_ts =
                                generate_table_test_name_field_ident_ts(test_name, field_ident);
                            let ident_create_content_ts = generate_create_content_ts(field_ident);
                            let content_ts = generate_content_ts(element);
                            quote::quote! {
                                for #el_snake_case in #method_call_ts {
                                    let current_table = #table_test_name_field_ident_ts.clone();
                                    let url_cloned = url.clone();
                                    let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                                    acc_9189f86e.push(futures::FutureExt::boxed(async move {
                                        let ident_create = #ident_create_upper_camel_case {
                                            #ident_create_content_ts
                                        };
                                        let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one(
                                            &url_cloned,
                                            ident_create.clone(),
                                            &current_table
                                        ).await;
                                        #content_ts
                                        let read_only_ids_from_try_delete_many = itertools::Itertools::sorted(
                                            #ident::try_delete_many_handle(
                                                &url_cloned,
                                                #ident_delete_many_parameters_upper_camel_case {
                                                    payload: #ident_delete_many_payload_upper_camel_case {
                                                        where_many: #std_option_option_ident_where_many_upper_camel_case(Some(
                                                            #ident_where_many_upper_camel_case {
                                                                #primary_key_field_ident: Some(
                                                                    generate_postgresql_type_where_try_new_primary_key(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![
                                                                            #primary_key_field_type_where_ts::Equal(
                                                                                postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    #value_snake_case: #primary_key_field_type_table_type_declaration_ts::new(
                                                                                        <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                                                            #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts
                                                                                        )
                                                                                    )
                                                                                }
                                                                            )
                                                                        ]
                                                                    )
                                                                ),
                                                                #std_option_option_ident_where_many_content_ts
                                                            }
                                                        )),
                                                    },
                                                },
                                                &current_table
                                            )
                                            .await
                                            .expect("338bcf89-0c3d-49d7-ac51-b73af98a32b0")
                                            .into_iter()
                                        ).collect::<Vec<<#primary_key_field_type as postgresql_crud::PostgresqlType>::Read>>();
                                        assert_eq!(
                                            read_only_ids_from_try_delete_many,
                                            vec![#primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts],
                                            "9fc29fa5-caba-403d-99da-ca9107d0c2e9"
                                        );
                                        assert!(
                                            generate_try_read_many_order_by_primary_key_with_big_pagination(
                                                &url_cloned,
                                                generate_ident_where_many_pripery_key_others_none(
                                                    Some(
                                                        generate_postgresql_type_where_try_new_primary_key(
                                                            postgresql_crud::LogicalOperator::Or,
                                                            vec![
                                                                #primary_key_field_type_where_ts::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                    #value_snake_case: #primary_key_field_type_table_type_declaration_ts::new(
                                                                        <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                                            #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts
                                                                        )
                                                                    )
                                                                })
                                                            ]
                                                        )
                                                    )
                                                ),
                                                #select_default_all_with_max_page_size_cloned_clone_ts,
                                                &current_table
                                            ).await
                                            .expect("1817b67a-c6c5-4fea-8ca7-23581c1888a3")
                                            .is_empty(),
                                            "38187925-c136-41de-940d-eba75efc3a39"
                                        );
                                    }));
                                }
                            }
                        },
                    )
                };
            let some_primary_key_where_initialization_ts = quote::quote! {
                Some(
                    generate_postgresql_type_where_try_new_primary_key(
                        postgresql_crud::LogicalOperator::And,
                        vec![
                            <#primary_key_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                read_only_ids_returned_from_create_one.#primary_key_field_ident,
                                #postgresql_crud_default_option_some_vec_one_el_call_ts
                            )
                        ]
                    )
                )
            };
            let (
                read_only_ids_merged_with_create_into_where_equal_ts,
                read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
            ) = {
                let generate_test_read_many_by_equal_one_column_value_ts = |test_name: &str, equal_or_equal_using_fields: &postgresql_crud_macros_common::EqualOrEqualUsingFields| {
                    generate_read_test_ts(
                        test_name,
                        &generate_option_vec_create_call_unwrap_or_vec_ts,
                        &generate_ident_create_content_el_ts,
                        &|element: &macros_helpers::SynFieldWrapper| {
                            let field_ident = &element.field_ident;
                            generate_read_only_ids_merged_with_create_into_where_assert_eq_ts(&generate_fields_named_with_comma_ts(&|el_ac437d52: &macros_helpers::SynFieldWrapper| {
                                let current_field_ident = &el_ac437d52.field_ident;
                                let current_field_type = &el_ac437d52.field_type;
                                if current_field_ident == primary_key_field_ident {
                                    some_primary_key_where_initialization_ts.clone()
                                } else if current_field_ident == field_ident {
                                    let method_content_ts = {
                                        let method_ts: &dyn quote::ToTokens = match &equal_or_equal_using_fields {
                                            postgresql_crud_macros_common::EqualOrEqualUsingFields::Equal => &read_only_ids_merged_with_create_into_where_equal_snake_case,
                                            postgresql_crud_macros_common::EqualOrEqualUsingFields::EqualUsingFields => &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_snake_case
                                        };
                                        quote::quote!{
                                            <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::#method_ts(
                                                read_only_ids_returned_from_create_one.#current_field_ident.clone().expect("11c3740b-7c3c-4dd5-b468-71bfa2f10892"),
                                                ident_create.#current_field_ident.clone()
                                            )
                                        }
                                    };
                                    match &equal_or_equal_using_fields {
                                        postgresql_crud_macros_common::EqualOrEqualUsingFields::Equal => generate_some_postgresql_type_where_try_new_and_ts(&quote::quote!{
                                            vec![#method_content_ts]
                                        }),
                                        postgresql_crud_macros_common::EqualOrEqualUsingFields::EqualUsingFields => quote::quote!{
                                            Some(#import_path::PostgresqlTypeWhere::new(
                                                #import_path::LogicalOperator::And,
                                                #method_content_ts
                                            ))
                                        }
                                    }
                                } else {
                                    none_ts.clone()
                                }
                            }))
                        }
                    )
                };
                (
                    generate_test_read_many_by_equal_one_column_value_ts(table_read_only_ids_merged_with_create_into_where_equal_name, &postgresql_crud_macros_common::EqualOrEqualUsingFields::Equal),
                    generate_test_read_many_by_equal_one_column_value_ts(table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name, &postgresql_crud_macros_common::EqualOrEqualUsingFields::EqualUsingFields),
                )
            };
            let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts = generate_read_test_ts(table_read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_name, &generate_option_vec_create_call_unwrap_or_vec_ts, &generate_ident_create_content_el_ts, &|element: &macros_helpers::SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_type = &element.field_type;
                let assert_eq_ts = generate_read_only_ids_merged_with_create_into_where_assert_eq_ts(&generate_fields_named_with_comma_ts(&|el_97d8a089: &macros_helpers::SynFieldWrapper| {
                    let current_field_ident = &el_97d8a089.field_ident;
                    if current_field_ident == primary_key_field_ident {
                        some_primary_key_where_initialization_ts.clone()
                    } else if current_field_ident == field_ident {
                        generate_some_postgresql_type_where_try_new_and_ts(&quote::quote! {vec![el_48a3d976]})
                    } else {
                        none_ts.clone()
                    }
                }));
                quote::quote! {
                    if let Some(value_d5cd3c70) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_snake_case(
                        read_only_ids_returned_from_create_one.#field_ident.clone().expect("65cef584-1427-482f-9c42-574630badedf"),
                        ident_create.#field_ident.clone()
                    ) {
                        for el_48a3d976 in value_d5cd3c70.into_vec() {
                            #assert_eq_ts
                        }
                    }
                }
            });
            let create_into_postgresql_type_option_vec_where_dimension_one_equal_ts =
                generate_read_test_ts(
                    table_create_into_postgresql_type_option_vec_where_dimension_one_equal_name,
                    &generate_option_vec_create_call_unwrap_or_vec_ts,
                    &generate_ident_create_content_el_ts,
                    &|element: &macros_helpers::SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let field_type = &element.field_type;
                        let assert_eq_ts =
                            generate_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                                &generate_fields_named_with_comma_ts(
                                    &|el_483e5312: &macros_helpers::SynFieldWrapper| {
                                        let current_field_ident = &el_483e5312.field_ident;
                                        if primary_key_field_ident == current_field_ident {
                                            some_primary_key_where_initialization_ts.clone()
                                        } else if current_field_ident == field_ident {
                                            generate_some_postgresql_type_where_try_new_and_ts(
                                                &quote::quote! {vec![el_39d1fb5d]},
                                            )
                                        } else {
                                            none_ts.clone()
                                        }
                                    },
                                ),
                            );
                        quote::quote! {
                            if let Some(value_b02d763d) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#create_into_postgresql_type_option_vec_where_dimension_one_equal_snake_case(
                                ident_create.#field_ident.clone()
                            ) {
                                for el_39d1fb5d in value_b02d763d.into_vec() {
                                    #assert_eq_ts
                                }
                            }
                        }
                    },
                );
            let read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_ts = generate_read_test_ts(
                table_read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_name,
                &generate_postgresql_type_option_vec_where_greater_than_test_unwrap_or_else_vec_call_ts,
                &|field_ident: &syn::Ident| {
                    generate_ident_create_content_ts(
                        field_ident,
                        &quote::quote! {#el_snake_case.#create_snake_case},
                    )
                },
                &|element: &macros_helpers::SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let assert_eq_ts = generate_read_only_ids_merged_with_create_into_where_assert_eq_ts(&generate_fields_named_with_comma_ts(&|el_a8bfc0c0: &macros_helpers::SynFieldWrapper| {
                        let current_field_ident = &el_a8bfc0c0.field_ident;
                        if current_field_ident == primary_key_field_ident {
                            some_primary_key_where_initialization_ts.clone()
                        } else if current_field_ident == field_ident {
                            generate_some_postgresql_type_where_try_new_and_ts(&quote::quote! {vec![value_60baba1f]})
                        } else {
                            none_ts.clone()
                        }
                    }));
                    quote::quote! {
                        if let Some(value_60baba1f) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_snake_case(
                            #el_snake_case.variant,
                            read_only_ids_returned_from_create_one.#field_ident.clone().expect("c8d34556-5a81-4c63-8e26-c79021eb876c"),
                            #el_snake_case.greater_than,
                        ) {
                            #assert_eq_ts
                        }
                    }
                },
            );
            let (
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts,
            ) = {
                //todo if vec_create is empty then do different logic (for uuid). now uuid tested using one default case
                let generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_ts =
                    |test_name: &str, dimension: &postgresql_crud_macros_common::Dimension| {
                        let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case = dimension.read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case();
                        generate_read_test_ts(test_name, &generate_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_ts, &generate_ident_create_content_el_ts, &|element: &macros_helpers::SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let field_type = &element.field_type;
                        let assert_eq_ts = generate_read_only_ids_merged_with_create_into_where_assert_eq_ts(&generate_fields_named_with_comma_ts(&|el_a9b23eca: &macros_helpers::SynFieldWrapper| {
                            let current_field_ident = &el_a9b23eca.field_ident;
                            if current_field_ident == primary_key_field_ident {
                                some_primary_key_where_initialization_ts.clone()
                            } else if current_field_ident == field_ident {
                                generate_some_postgresql_type_where_try_new_and_ts(&quote::quote! {vec![el_3efa0bb4]})
                            } else {
                                none_ts.clone()
                            }
                        }));
                        quote::quote! {
                            if let Some(value_bb67b871) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_snake_case(
                                read_only_ids_returned_from_create_one.#field_ident.clone().expect("2ed000a5-cf70-4df1-903a-c1f6d224e926"),
                                ident_create.#field_ident.clone()
                            ) {
                                for el_3efa0bb4 in value_bb67b871.into_vec() {
                                    #assert_eq_ts
                                }
                            }
                        }
                    })
                    };
                (
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_ts(table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_name, &postgresql_crud_macros_common::Dimension::One),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_ts(table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_name, &postgresql_crud_macros_common::Dimension::Two),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_ts(table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_name, &postgresql_crud_macros_common::Dimension::Three),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_ts(table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_name, &postgresql_crud_macros_common::Dimension::Four),
                )
            };
            let create_into_postgresql_json_type_option_vec_where_length_equal_ts = generate_read_test_ts(
                table_create_into_postgresql_json_type_option_vec_where_length_equal_name,
                &generate_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_ts,
                &generate_ident_create_content_el_ts,
                &|element: &macros_helpers::SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let assert_eq_ts = generate_read_only_ids_merged_with_create_into_where_assert_eq_ts(&generate_fields_named_with_comma_ts(&|el_94f00070: &macros_helpers::SynFieldWrapper| {
                        let current_field_ident = &el_94f00070.field_ident;
                        if current_field_ident == primary_key_field_ident {
                            some_primary_key_where_initialization_ts.clone()
                        } else if current_field_ident == field_ident {
                            generate_some_postgresql_type_where_try_new_and_ts(&quote::quote! {vec![el_c09ef321]})
                        } else {
                            none_ts.clone()
                        }
                    }));
                    quote::quote! {
                        if let Some(value_f825e068) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#create_into_postgresql_json_type_option_vec_where_length_equal_snake_case(
                            ident_create.#field_ident.clone()
                        ) {
                            for el_c09ef321 in value_f825e068.into_vec() {
                                #assert_eq_ts
                            }
                        }
                    }
                },
            );
            let create_into_postgresql_json_type_option_vec_where_length_greater_than_ts = generate_read_test_ts(
                table_create_into_postgresql_json_type_option_vec_where_length_greater_than_name,
                &generate_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_ts,
                &generate_ident_create_content_el_ts,
                &|element: &macros_helpers::SynFieldWrapper|{
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let assert_eq_ts = generate_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                        &generate_fields_named_with_comma_ts(&|el_c927ab80: &macros_helpers::SynFieldWrapper|{
                            let current_field_ident = &el_c927ab80.field_ident;
                            if current_field_ident == primary_key_field_ident {
                                some_primary_key_where_initialization_ts.clone()
                            }
                            else if current_field_ident == field_ident {
                                generate_some_postgresql_type_where_try_new_and_ts(&quote::quote!{vec![el_527b546b]})
                            } else {
                                none_ts.clone()
                            }
                        })
                    );
                    quote::quote!{
                        if let Some(value_cd4aa374) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#create_into_postgresql_json_type_option_vec_where_length_greater_than_snake_case(
                            ident_create.#field_ident.clone()
                        ) {
                            for el_527b546b in value_cd4aa374.into_vec() {
                                #assert_eq_ts
                            }
                        }
                    }
                }
            );
            let (
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts,
                read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts,
            ) = {
                let generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_ts =
                    |table_name: &str, method_ts: &dyn quote::ToTokens| {
                        generate_read_test_ts(
                    table_name,
                    &generate_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_ts,
                    &generate_ident_create_content_el_ts,
                    &|element: &macros_helpers::SynFieldWrapper|{
                        let field_ident = &element.field_ident;
                        let field_type = &element.field_type;
                        let assert_eq_ts = generate_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                            &generate_fields_named_with_comma_ts(&|el_16b8a9cc: &macros_helpers::SynFieldWrapper|{
                                let current_field_ident = &el_16b8a9cc.field_ident;
                                if current_field_ident == primary_key_field_ident {
                                    some_primary_key_where_initialization_ts.clone()
                                }
                                else if current_field_ident == field_ident {
                                    generate_some_postgresql_type_where_try_new_and_ts(&quote::quote!{match el_feacc53b {
                                        #import_path::SingleOrMultiple::Multiple(multiple) => multiple.into_vec().into_iter().collect(),
                                        #import_path::SingleOrMultiple::Single(single) => std::iter::once(single).collect(),
                                    }})
                                } else {
                                    none_ts.clone()
                                }
                            })
                        );
                        quote::quote!{
                            if let Some(value_0b85c066) = <#field_type as postgresql_crud::PostgresqlTypeTestCases>::#method_ts(
                                read_only_ids_returned_from_create_one.#field_ident.clone().expect("df01c8ac-63e3-42f7-aae4-018c7958c00d"),
                                ident_create.#field_ident.clone()
                            ) {
                                for el_feacc53b in value_0b85c066.into_vec() {
                                    #assert_eq_ts
                                }
                            }
                        }
                    }
                )
                    };
                (
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_name,
                        &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_snake_case
                    ),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_name,
                        &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_snake_case
                    ),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_name,
                        &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_snake_case
                    ),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_name,
                        &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_snake_case
                    ),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_name,
                        &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_snake_case
                    ),
                    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_name,
                        &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_snake_case
                    )
                )
            };
            quote::quote! {
                #test_read_many_by_non_existent_primary_keys_ts
                #test_read_many_by_equal_to_created_primary_keys_ts
                #read_only_ids_merged_with_create_into_where_equal_ts
                #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts
                #read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts
                #create_into_postgresql_type_option_vec_where_dimension_one_equal_ts
                #read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_ts
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts
                #create_into_postgresql_json_type_option_vec_where_length_equal_ts
                #create_into_postgresql_json_type_option_vec_where_length_greater_than_ts
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts
                #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts
            }
        };
        let read_one_tests_ts = quote::quote! {
            acc_9189f86e.push({
                let table_read_one_cloned = table_read_one.clone();
                let url_cloned = url.clone();
                let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                futures::FutureExt::boxed(async move {
                    generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                        &url_cloned,
                        #primary_key_field_type_as_postgresql_type_read_ts::new(uuid::Uuid::new_v4()),
                        #select_default_all_with_max_page_size_cloned_clone_ts,
                        &table_read_one_cloned,
                    ).await;
                })
            });
        };
        let update_many_tests_ts = {
            //todo add test for trying to update empty vec
            let update_many_only_one_column_tests_ts =
                generate_fields_named_without_primary_key_without_comma_ts(
                    &|el_94a9ca95: &macros_helpers::SynFieldWrapper| {
                        let field_ident = &el_94a9ca95.field_ident;
                        let field_type = &el_94a9ca95.field_type;
                        let is_fields_without_primary_key_len_greater_than_one =
                            fields_without_primary_key.len() > 1;
                        let maybe_previous_read_ts =
                            if is_fields_without_primary_key_len_greater_than_one {
                                quote::quote! {
                                    let previous_read = itertools::Itertools::sorted_by(
                                        generate_try_read_many_order_by_primary_key_with_big_pagination(
                                            &url_cloned,
                                            generate_ident_where_many_pripery_key_others_none(
                                                Some(
                                                    generate_postgresql_type_where_try_new_primary_key(
                                                        postgresql_crud::LogicalOperator::Or,
                                                        vec![
                                                            #primary_key_field_type_as_postgresql_type_where_ts::Equal(
                                                                postgresql_crud::PostgresqlTypeWhereEqual {
                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                    value: #primary_key_field_type_table_type_declaration_ts::new(
                                                                        #primary_key_field_type_as_postgresql_type_ts into_inner(
                                                                            #primary_key_field_type_read_only_is_into_read_read_only_ids_current_el_primary_key_field_ident_ts
                                                                        )
                                                                    ),
                                                                }
                                                            )
                                                        ]
                                                    )
                                                )
                                            ),
                                            #select_default_all_with_max_page_size_cloned_clone_ts,
                                            &table_update_many_cloned
                                        )
                                        .await
                                        .expect("540ec737-dea7-4d50-a42a-45ea1f81d6c1")
                                        .into_iter(),
                                        |first, second| {
                                            match (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                                (Some(first_handle), Some(second_handle)) => first_handle.#value_snake_case.cmp(&second_handle.#value_snake_case),
                                                _ => panic!("99ba9dc3-ca32-4462-b9b4-b1202265beee"),
                                            }
                                        }
                                    );
                                }
                            } else {
                                proc_macro2::TokenStream::new()
                            };
                        let field_ident_read_only_ids_to_two_dimensional_vec_read_inner_acc_snake_case =
                            SelfReadOnlyIdsToTwoDimensionalVecReadInnerAccSnakeCase::from_tokens(
                                &field_ident,
                            );
                        let ident_read_only_ids_upper_fields_initialization_without_primary_key_ts =
                            generate_fields_named_without_primary_key_with_comma_ts(
                                &|syn_field_wrapper: &macros_helpers::SynFieldWrapper| {
                                    let current_field_ident = &syn_field_wrapper.field_ident;
                                    let current_field_type = &syn_field_wrapper.field_type;
                                    if field_ident == current_field_ident {
                                        quote::quote! {#current_field_ident: Some(<#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update))}
                                    } else {
                                        quote::quote! {#current_field_ident: None}
                                    }
                                },
                            );
                        let ident_update_parameters_initialization_without_primary_key_ts =
                            generate_fields_named_without_primary_key_with_comma_ts(
                                &|syn_field_wrapper: &macros_helpers::SynFieldWrapper| {
                                    let current_field_ident = &syn_field_wrapper.field_ident;
                                    if field_ident == current_field_ident {
                                        let value_initialization_ts =
                                            generate_import_path_value_initialization_ts(
                                                &quote::quote! {
                                                    #update_snake_case.clone()
                                                },
                                            );
                                        quote::quote! {Some(#value_initialization_ts)}
                                    } else {
                                        none_ts.clone()
                                    }
                                },
                            );
                        let ident_read_fields_initialization_without_primary_key_after_update_one_ts =
                            generate_fields_named_without_primary_key_with_comma_ts(
                                &|syn_field_wrapper: &macros_helpers::SynFieldWrapper| {
                                    let current_field_ident = &syn_field_wrapper.field_ident;
                                    if field_ident == current_field_ident {
                                        let value_initialization_ts =
                                            generate_import_path_value_initialization_ts(&{
                                                let current_field_type =
                                                    &syn_field_wrapper.field_type;
                                                quote::quote! {
                                                    <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::previous_read_merged_with_option_update_into_read(
                                                        <#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                            &read_only_ids_current_element.#current_field_ident.clone().expect("96213542-59eb-4767-b120-d9ba575087c8")
                                                        ).expect("bf0d6f55-7457-4ec1-8b79-50efad297ccb").#value_snake_case,
                                                        Some(#update_snake_case.clone())
                                                    )
                                                }
                                            });
                                        quote::quote! {
                                            #current_field_ident: Some(#value_initialization_ts)
                                        }
                                    } else {
                                        quote::quote! {
                                            #current_field_ident: el_a6bc6b2f.#current_field_ident
                                        }
                                    }
                                },
                            );
                        let expected_read_many_ts =
                            if is_fields_without_primary_key_len_greater_than_one {
                                let value_initialization_ts = generate_import_path_value_initialization_ts(&primary_key_field_type_read_only_is_into_read_read_only_ids_current_el_primary_key_field_ident_ts);
                                quote::quote! {
                                    previous_read.into_iter().map(|el_a6bc6b2f| #ident_read_upper_camel_case {
                                        #primary_key_field_ident: Some(#value_initialization_ts),
                                        #ident_read_fields_initialization_without_primary_key_after_update_one_ts
                                    }).collect::<Vec<#ident_read_upper_camel_case>>()
                                }
                            } else {
                                let value_initialization_ts = generate_import_path_value_initialization_ts(&primary_key_field_type_read_only_is_into_read_read_only_ids_current_el_primary_key_field_ident_ts);
                                quote::quote! {
                                    vec![
                                        #ident_read_upper_camel_case {
                                            #primary_key_field_ident: Some(#value_initialization_ts),
                                            #ident_read_fields_initialization_without_primary_key_after_update_one_ts
                                        }
                                    ]
                                }
                            };
                        quote::quote! {
                            for (index_7f181188, read_only_ids_current_element) in generate_read_only_ids_current_elements(
                                &url,
                                &table_update_many,
                                #select_default_all_with_max_page_size_clone_ts,
                                #field_ident_read_only_ids_to_two_dimensional_vec_read_inner_acc_snake_case.clone()
                            ).await.into_iter().enumerate() {
                                let table_update_many_cloned = table_update_many.clone();
                                let url_cloned = url.clone();
                                let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                                acc_9189f86e.push(futures::FutureExt::boxed(async move {
                                    #maybe_previous_read_ts
                                    let update = <
                                        #field_type
                                        as
                                        postgresql_crud::PostgresqlTypeTestCases
                                    >::read_inner_into_update_with_new_or_try_new_unwraped({
                                        let mut index_e0c50b3e: usize = 0;
                                        let mut option_test_case = None;
                                        for el_76abae3a in <#field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(
                                            &read_only_ids_current_element.#field_ident.clone().expect("af7d979d-0d43-47e9-bbf6-07282cae7eff")
                                        ) {
                                            let mut should_break = false;
                                            for el_72f5ad12 in el_76abae3a {
                                                if index_e0c50b3e == index_7f181188 {
                                                    option_test_case = Some(el_72f5ad12);
                                                    should_break = true;
                                                    break;
                                                }
                                                index_e0c50b3e = index_e0c50b3e.checked_add(1).expect("0968dda6-4c3a-42b6-8aa1-83058a928dde");
                                            }
                                            if should_break {
                                                break;
                                            }
                                        }
                                        option_test_case.expect("769983ba-3992-4298-97ab-9a3721c32359")
                                    });
                                    assert_eq!(
                                        vec![
                                            #ident_read_only_ids_upper_camel_case {
                                                #primary_key_field_ident: read_only_ids_current_element.#primary_key_field_ident,
                                                #ident_read_only_ids_upper_fields_initialization_without_primary_key_ts
                                            }
                                        ],
                                        #ident::try_update_many_handle(
                                            &url_cloned,
                                            #ident_update_many_parameters_upper_camel_case {
                                                payload: #ident_update_many_payload_upper_camel_case::try_new(vec![
                                                    #ident_update_upper_camel_case::try_new(
                                                        #primary_key_field_type_as_postgresql_type_update_as_postgresql_type_primary_key_read_only_ids_into_update_ts,
                                                        #ident_update_parameters_initialization_without_primary_key_ts
                                                    ).expect("42dc87b3-2ac4-4e66-8287-91aeb13f0ee8")
                                                ]).expect("69e1bd8a-fe78-4301-85ca-f4f3958d7493")
                                            },
                                            &table_update_many_cloned
                                        ).await.expect("d2de0bd6-1b01-4ef2-b074-a60878241b52"),
                                        "34bfb3c7-7a53-479e-9d4f-0856003573e1"
                                    );
                                    assert_eq!(
                                        {
                                            #expected_read_many_ts
                                        },
                                        itertools::Itertools::sorted_by(
                                            generate_try_read_many_order_by_primary_key_with_big_pagination(
                                                &url_cloned,
                                                generate_ident_where_many_pripery_key_others_none(
                                                    Some(
                                                        generate_postgresql_type_where_try_new_primary_key(
                                                            postgresql_crud::LogicalOperator::Or,
                                                            vec![
                                                                #primary_key_field_type_where_ts::Equal(
                                                                    postgresql_crud::PostgresqlTypeWhereEqual {
                                                                        logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                        #value_snake_case: #primary_key_field_type_table_type_declaration_ts::new(
                                                                            <#primary_key_field_type as postgresql_crud::PostgresqlType>::into_inner(
                                                                                #primary_key_field_type_read_only_is_into_read_read_only_ids_current_el_primary_key_field_ident_ts
                                                                            )
                                                                        ),
                                                                    }
                                                                )
                                                            ]
                                                        )
                                                    )
                                                ),
                                                select_default_all_with_max_page_size_cloned,
                                                &table_update_many_cloned
                                            )
                                            .await
                                            .expect("25c561e2-6b39-4982-8fe7-4473d12b3271")
                                            .into_iter(),
                                            |first, second| match (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                                (Some(first_handle), Some(second_handle)) => first_handle.#value_snake_case.cmp(&second_handle.#value_snake_case),
                                                _ => panic!("3c827ad6-30bb-49db-8f49-8c903a236040"),
                                            }
                                        ).collect::<Vec<#ident_read_upper_camel_case>>(),
                                        "ae2a2da5-3697-4fd7-9ad2-4a535618fbc3"
                                    );
                                }));
                            }
                        }
                    },
                );
            quote::quote! {#update_many_only_one_column_tests_ts}
        };
        let update_one_tests_ts = {
            let update_one_only_one_column_tests_ts =
                generate_fields_named_without_primary_key_without_comma_ts(
                    &|el_d82ff77f: &macros_helpers::SynFieldWrapper| {
                        let field_ident = &el_d82ff77f.field_ident;
                        let field_type = &el_d82ff77f.field_type;
                        let maybe_previous_read_ts = if fields_without_primary_key.len() > 1 {
                            quote::quote! {
                                let previous_read = generate_ident_try_read_one_handle_primary_key(
                                    &url_cloned,
                                    #primary_key_field_type_read_only_is_into_read_read_only_ids_current_el_primary_key_field_ident_ts,
                                    #select_default_all_with_max_page_size_cloned_clone_ts,
                                    &table_update_one_cloned
                                )
                                .await.expect("e6998b47-c593-449e-89fd-3888de9f84a6");
                            }
                        } else {
                            proc_macro2::TokenStream::new()
                        };
                        let field_ident_read_only_ids_to_two_dimensional_vec_read_inner_acc_snake_case =
                            SelfReadOnlyIdsToTwoDimensionalVecReadInnerAccSnakeCase::from_tokens(
                                &field_ident,
                            );
                        let ident_read_only_ids_upper_fields_initialization_without_primary_key_ts =
                            generate_fields_named_without_primary_key_with_comma_ts(
                                &|element: &macros_helpers::SynFieldWrapper| {
                                    let current_field_ident = &element.field_ident;
                                    let current_field_type = &element.field_type;
                                    if field_ident == current_field_ident {
                                        quote::quote! {#current_field_ident: Some(<#current_field_type as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update))}
                                    } else {
                                        quote::quote! {#current_field_ident: None}
                                    }
                                },
                            );
                        let ident_update_parameters_initialization_without_primary_key_ts =
                            generate_fields_named_without_primary_key_with_comma_ts(
                                &|element: &macros_helpers::SynFieldWrapper| {
                                    let current_field_ident = &element.field_ident;
                                    if field_ident == current_field_ident {
                                        let value_initialization_ts =
                                            generate_import_path_value_initialization_ts(
                                                &quote::quote! {#update_snake_case.clone()},
                                            );
                                        quote::quote! {Some(#value_initialization_ts)}
                                    } else {
                                        none_ts.clone()
                                    }
                                },
                            );
                        let ident_read_fields_initialization_without_primary_key_after_update_one_ts =
                            generate_fields_named_without_primary_key_with_comma_ts(
                                &|element: &macros_helpers::SynFieldWrapper| {
                                    let current_field_ident = &element.field_ident;
                                    let current_field_type = &element.field_type;
                                    if field_ident == current_field_ident {
                                        let value_initialization_ts =
                                            generate_import_path_value_initialization_ts(
                                                &quote::quote! {
                                                    <
                                                        #current_field_type
                                                        as
                                                        postgresql_crud::PostgresqlTypeTestCases
                                                    >::previous_read_merged_with_option_update_into_read(
                                                        <
                                                            #current_field_type
                                                            as
                                                            postgresql_crud::PostgresqlTypeTestCases
                                                        >::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                            &read_only_ids_current_element.#current_field_ident.clone().expect("4f19d0d2-a23f-4b77-a2bc-c7b04db7a129")
                                                        ).expect("c7685b19-9bca-47bc-a3a5-8fc543b174a5").#value_snake_case,
                                                        Some(#update_snake_case.clone())
                                                    )
                                                },
                                            );
                                        quote::quote! {
                                            #current_field_ident: Some(#value_initialization_ts)
                                        }
                                    } else {
                                        quote::quote! {
                                            #current_field_ident: previous_read.#current_field_ident
                                        }
                                    }
                                },
                            );
                        let value_initialization_ts = generate_import_path_value_initialization_ts(&primary_key_field_type_read_only_is_into_read_read_only_ids_current_el_primary_key_field_ident_ts);
                        quote::quote! {
                            for (index_26824592, read_only_ids_current_element) in generate_read_only_ids_current_elements(
                                &url,
                                &table_update_one,
                                #select_default_all_with_max_page_size_clone_ts,
                                #field_ident_read_only_ids_to_two_dimensional_vec_read_inner_acc_snake_case
                            ).await.into_iter().enumerate() {
                                let table_update_one_cloned = table_update_one.clone();
                                let url_cloned = url.clone();
                                let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                                acc_9189f86e.push(futures::FutureExt::boxed(async move {
                                    #maybe_previous_read_ts
                                    let update = <
                                        #field_type
                                        as
                                        postgresql_crud::PostgresqlTypeTestCases
                                    >::read_inner_into_update_with_new_or_try_new_unwraped({
                                        let mut index_e0d2f9db: usize = 0;
                                        let mut option_test_case = None;
                                        for el_3a9a65ee in <#field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(
                                            &read_only_ids_current_element.#field_ident.clone().expect("c4d98a71-f30f-410e-b410-a75f4672f2f7")
                                        ) {
                                            let mut should_break = false;
                                            for el_bb734c11 in el_3a9a65ee {
                                                if index_e0d2f9db == index_26824592 {
                                                    option_test_case = Some(el_bb734c11);
                                                    should_break = true;
                                                    break;
                                                }
                                                index_e0d2f9db = index_e0d2f9db.checked_add(1).expect("326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                            }
                                            if should_break {
                                                break;
                                            }
                                        }
                                        option_test_case.expect("bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                    });
                                    assert_eq!(
                                        #ident_read_only_ids_upper_camel_case {
                                            #primary_key_field_ident: read_only_ids_current_element.#primary_key_field_ident,
                                            #ident_read_only_ids_upper_fields_initialization_without_primary_key_ts
                                        },
                                        #ident::try_update_one_handle(
                                            &url_cloned,
                                            #ident_update_one_parameters_upper_camel_case {
                                                payload: #ident_update_upper_camel_case::try_new(
                                                    #primary_key_field_type_as_postgresql_type_update_as_postgresql_type_primary_key_read_only_ids_into_update_ts,
                                                    #ident_update_parameters_initialization_without_primary_key_ts
                                                ).expect("0e5d65a5-12c8-4c48-a24c-0f1fe376ada2")//todo add column ident
                                            },
                                            &table_update_one_cloned
                                        ).await.expect("4d755542-e3e3-4c68-a3a2-beb654cf5b80"),
                                        "564de31c-3664-4c62-85fc-e03793372f8f"
                                    );
                                    assert_eq!(
                                        #ident_read_upper_camel_case {
                                            #primary_key_field_ident: Some(#value_initialization_ts),
                                            #ident_read_fields_initialization_without_primary_key_after_update_one_ts
                                        },
                                        generate_ident_try_read_one_handle_primary_key(
                                            &url_cloned,
                                            #primary_key_field_type_read_only_is_into_read_read_only_ids_current_el_primary_key_field_ident_ts,
                                            select_default_all_with_max_page_size_cloned,
                                            &table_update_one_cloned
                                        )
                                        .await.expect("75894c76-88f5-406e-ab46-c27b1c7d4212"),
                                        "d5dec823-b1f9-49b2-9c24-bf788f08cd8c"
                                    );
                                }));
                            }
                        }
                    },
                );
            quote::quote! {#update_one_only_one_column_tests_ts}
        };
        let delete_many_tests_ts = {
            let test_delete_many_by_non_existent_primary_keys_ts = {
                let content_ts = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&quote::quote! {
                    assert!(
                        #ident::try_delete_many_handle(
                            &url_cloned,
                            #ident_delete_many_parameters_upper_camel_case {
                                payload: #ident_delete_many_payload_upper_camel_case {
                                    where_many: #std_option_option_ident_where_many_upper_camel_case(Some(#ident_where_many_upper_camel_case {
                                        #primary_key_field_ident: Some(
                                            generate_postgresql_type_where_try_new_primary_key(
                                                postgresql_crud::LogicalOperator::Or,
                                                std::iter::repeat_with(|| #primary_key_field_type_as_postgresql_type_where_ts::Equal(
                                                    postgresql_crud::PostgresqlTypeWhereEqual {
                                                        logical_operator: postgresql_crud::LogicalOperator::Or,
                                                        value: #primary_key_field_type_table_type_declaration_ts::new(
                                                            uuid::Uuid::new_v4()
                                                        )
                                                    }
                                                ))
                                                .take(el_39819198)
                                                .collect()
                                            )
                                        ),
                                        #fields_none_initialization_ts
                                    })),
                                },
                            },
                            &current_table
                        )
                        .await
                        .expect("0d5dec47-8b2e-4f02-909b-3a58b65bc6a5")
                        .is_empty(),
                        "51d14103-5122-4d96-a45c-4dd958ab3adc"
                    );
                });
                quote::quote! {
                    for el_39819198 in [1,2] {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                        let current_table = table_test_read_many_by_equal_to_created_primary_keys.clone();
                        acc_9189f86e.push(futures::FutureExt::boxed(async move {
                            #content_ts
                        }));
                    };
                }
            };
            let test_delete_many_by_primary_keys_ts = {
                let content_ts = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&{
                    let primary_key_field_type_read_only_ids_into_table_type_declaration_el_primary_key_field_ident_clone_ts =
                        generate_primary_key_field_type_as_postgresql_type_primary_key_method_call_ts(
                            &read_only_ids_into_table_type_declaration_snake_case,
                            &quote::quote! {el_3bb88958.#primary_key_field_ident},
                        );
                    quote::quote! {
                        let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                            &url_cloned,
                            #ident_create_many_parameters_upper_camel_case {
                                payload: #ident_create_many_payload_upper_camel_case(
                                    std::iter::repeat_n(ident_create_default_cloned, el_56409d32).collect()
                                )
                            },
                            &current_table
                        ).await.expect("b8695890-65fb-469b-a6f9-be481d648eb9");
                        let read_only_ids_from_try_delete_many = #ident::try_delete_many_handle(
                            &url_cloned,
                            #ident_delete_many_parameters_upper_camel_case {
                                payload: #ident_delete_many_payload_upper_camel_case {
                                    where_many: #std_option_option_ident_where_many_upper_camel_case(Some(#ident_where_many_upper_camel_case {
                                        #primary_key_field_ident: Some(
                                            generate_postgresql_type_where_try_new_primary_key(
                                                postgresql_crud::LogicalOperator::Or,//here
                                                read_only_ids_from_try_create_many.iter().map(|el_3bb88958| #primary_key_field_type_as_postgresql_type_where_ts::Equal(
                                                    postgresql_crud::PostgresqlTypeWhereEqual {
                                                        logical_operator: postgresql_crud::LogicalOperator::Or,
                                                        #value_snake_case: #primary_key_field_type_read_only_ids_into_table_type_declaration_el_primary_key_field_ident_clone_ts,
                                                    }
                                                )).collect()
                                            )
                                        ),
                                        #fields_none_initialization_ts
                                    })),
                                },
                            },
                            &current_table
                        ).await.expect("b80b91b8-7de1-4ea2-97cf-1987a5f7cc57");
                        assert_eq!(
                            read_only_ids_from_try_delete_many,
                            {
                                read_only_ids_from_try_create_many.iter().map(|el_ba0f6b1c|
                                    <#primary_key_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                        &el_ba0f6b1c.#primary_key_field_ident
                                    ).expect("3ee5ee86-05dc-4dc8-9262-8ffa1855d5e4").#value_snake_case
                                ).collect::<Vec<#primary_key_field_type_as_postgresql_type_read_ts>>()
                            },
                            "db5e88a6-c75b-421b-acfb-56931b97ba3b"
                        );
                        assert!(
                            generate_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                generate_ident_where_many_pripery_key_others_none(
                                    Some(
                                        generate_postgresql_type_where_try_new_primary_key(
                                            postgresql_crud::LogicalOperator::Or,
                                            read_only_ids_from_try_delete_many.into_iter().map(|el_adcc8db3| #primary_key_field_type_as_postgresql_type_where_ts::Equal(
                                                postgresql_crud::PostgresqlTypeWhereEqual {
                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                    #value_snake_case: #primary_key_field_type_read_into_table_type_declaration_el_primary_key_field_ident_clone_ts,
                                                }
                                            )).collect()
                                        )
                                    )
                                ),
                                select_default_all_with_max_page_size_cloned.clone(),
                                &current_table
                            ).await
                            .expect("bcb79917-ee81-416e-82a3-f43a823266a3")
                            .is_empty(),
                            "77f038b0-6f39-4b5b-a402-a1b6142acd0d"
                        );
                    }
                });
                quote::quote! {
                    for el_56409d32 in [1,2] {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                        //todo is table name correct?
                        let current_table = table_test_read_many_by_equal_to_created_primary_keys.clone();
                        let ident_create_default_cloned = ident_create_default.clone();
                        acc_9189f86e.push(futures::FutureExt::boxed(async move {
                            #content_ts
                        }));
                    };
                }
            };
            quote::quote! {
                #test_delete_many_by_non_existent_primary_keys_ts
                #test_delete_many_by_primary_keys_ts
            }
        };
        let delete_one_tests_ts = {
            let value_initialization_ts = generate_import_path_value_initialization_ts(&primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts);
            quote::quote! {
                acc_9189f86e.push({
                    let table_delete_one_cloned = table_delete_one.clone();
                    let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                    futures::FutureExt::boxed(async move {
                        if let Err(#error_snake_case) = generate_try_delete_one_handle(
                            &url,
                            #primary_key_field_type_as_postgresql_type_read_ts::new(uuid::Uuid::new_v4()),
                            &table_delete_one_cloned
                        ).await {
                            if let #ident_try_delete_one_error_named_upper_camel_case::#ident_delete_one_error_named_with_serialize_deserialize_upper_camel_case {
                                delete_one_error_named_with_serialize_deserialize,
                                ..
                            } = #error_snake_case {
                                if let #ident_delete_one_error_named_with_serialize_deserialize_upper_camel_case::Postgresql {
                                    postgresql,
                                    ..
                                } = delete_one_error_named_with_serialize_deserialize {
                                    assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row(), "c9261bb8-d391-4c4b-9707-3a2c4278ad90");
                                } else {
                                    panic!("e63b27a3-f3e3-4f19-998a-88ce798b08cc");
                                }
                            } else {
                                panic!("47a8e0d9-1f95-4fa7-91dc-a94955195204")
                            }
                        } else {
                            panic!("9be62f9f-31d9-493c-bb0f-b83b6ecb0026")
                        }
                        let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one_default(&url, &table_delete_one_cloned).await;
                        assert_eq!(
                            #ident_read_upper_camel_case {
                                #primary_key_field_ident: Some(#value_initialization_ts),
                                #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_create_ts
                            },
                            generate_ident_try_read_one_handle_primary_key(
                                &url,
                                #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts,
                                #select_default_all_with_max_page_size_cloned_clone_ts,
                                &table_delete_one_cloned
                            )
                            .await.expect("c8c44c89-aeb0-43d3-ae72-02b7a5979f5a"),
                            "86ef08ae-4356-4417-9490-1d13eb2af71f"
                        );
                        assert_eq!(
                            generate_try_delete_one_handle(
                                &url,
                                #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts,
                                &table_delete_one_cloned
                            ).await.expect("7e1d1a70-8f93-43b9-9cfe-37fc240ca7ba"),
                            #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts,
                            "99f81971-dc80-46db-b466-4f309b215a8c"
                        );
                        generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                            &url,
                            #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts,
                            #select_default_all_with_max_page_size_cloned_clone_ts,
                            &table_delete_one_cloned,
                        ).await;
                    })
                });
            }
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
                    fn generate_ident_where_many_pripery_key_others_none(
                        option_postgresql_type_where: Option<#import_path::PostgresqlTypeWhere<#primary_key_field_type_as_postgresql_type_where_ts>>,
                    ) -> #ident_where_many_upper_camel_case {
                        #ident_where_many_upper_camel_case::try_new(
                            option_postgresql_type_where,
                            #fields_named_without_primary_key_with_comma_none_ts
                        )
                        .expect("5fb2b219-8bd7-4edd-9722-b475826707f5")
                    }
                    fn generate_postgresql_type_where_try_new_primary_key(
                        logical_operator: #import_path::LogicalOperator,
                        vec: Vec<#primary_key_field_type_where_ts>
                    ) -> #import_path::PostgresqlTypeWhere<#primary_key_field_type_as_postgresql_type_where_ts> {
                        #generate_postgresql_type_where_try_new_primary_key_content_ts
                    }
                    fn generate_postgresql_type_where_try_new_or_primary_keys(
                        vec_read_only_ids: &[#ident_read_only_ids_upper_camel_case]
                    ) -> #import_path::PostgresqlTypeWhere<#primary_key_field_type_as_postgresql_type_where_ts> {
                        generate_postgresql_type_where_try_new_primary_key(
                            #import_path::LogicalOperator::Or,
                            vec_read_only_ids.iter().map(|el_9530b728| #primary_key_field_type_where_ts::Equal(#import_path::PostgresqlTypeWhereEqual {
                                logical_operator: #import_path::LogicalOperator::Or,
                                value: #primary_key_field_type_table_type_declaration_ts::new(
                                    #primary_key_field_type_as_postgresql_type_ts into_inner(
                                        <#primary_key_field_type as #import_path::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                            el_9530b728.#primary_key_field_ident
                                        ),
                                    )
                                ),
                            })).collect()
                        )
                    }
                    async fn generate_try_read_many_order_by_primary_key_with_big_pagination(
                        endpoint_location: &str,
                        current_ident_where_many: #ident_where_many_upper_camel_case,
                        select: #import_path::NotEmptyUniqueVec<#ident_select_upper_camel_case>,
                        table: &str
                    ) -> Result<Vec<#ident_read_upper_camel_case>, #ident_try_read_many_error_named_upper_camel_case> {
                        #ident::try_read_many_handle(
                            endpoint_location,
                            #ident_read_many_parameters_upper_camel_case {
                                payload: #ident_read_many_payload_upper_camel_case {
                                    where_many: #std_option_option_ident_where_many_upper_camel_case(Some(
                                        current_ident_where_many
                                    )),
                                    select,
                                    order_by: #import_path::OrderBy {
                                        column: #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_ts(
                                            #primary_key_field_type_as_postgresql_type_select_ts::default()
                                        ),
                                        order: Some(#import_path::Order::Asc)
                                    },
                                    pagination: #import_path::PaginationStartsWithZero::try_new(10000, 0).expect("b0cdf0cb-1e31-4a7e-9e53-d2ff71efb983"),
                                }
                            },
                            table
                        )
                        .await
                    }
                    async fn generate_ident_try_read_one_handle_primary_key(
                        url: &str,
                        primary_key_column: #primary_key_field_type_as_postgresql_type_read_ts,
                        select: #import_path::NotEmptyUniqueVec<#ident_select_upper_camel_case>,
                        table: &str,
                    ) -> Result<#ident_read_upper_camel_case, #ident_try_read_one_error_named_upper_camel_case> {
                        #ident::try_read_one_handle(
                            url,
                            #ident_read_one_parameters_upper_camel_case {
                                payload: #ident_read_one_payload_upper_camel_case {
                                    primary_key_column,
                                    select,
                                },
                            },
                            table,
                        )
                        .await
                    }
                    async fn generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                        url: &str,
                        primary_key_column: #primary_key_field_type_as_postgresql_type_read_ts,
                        select: #import_path::NotEmptyUniqueVec<#ident_select_upper_camel_case>,
                        table: &str,
                    ) {
                        if let Err(#error_snake_case) = generate_ident_try_read_one_handle_primary_key(
                            url,
                            primary_key_column,
                            select,
                            table
                        ).await {
                            if let #ident_try_read_one_error_named_upper_camel_case::#ident_read_one_error_named_with_serialize_deserialize_upper_camel_case {
                                read_one_error_named_with_serialize_deserialize,
                                ..
                            } = error {
                                if let #ident_read_one_error_named_with_serialize_deserialize_upper_camel_case::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                    assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row(), "58b9a6a4-cf9b-49f3-a20f-7007deea40fd");
                                } else {
                                    panic!("0ad0117b-a2e0-4629-99d0-71935cd93d15");
                                }
                            } else {
                                panic!("c6695392-4b5f-4482-86aa-b2f19c33a746")
                            }
                        } else {
                            panic!("67e43b7a-d3ec-4a3b-a3f1-8c11499fd090")
                        }
                    }
                    fn ident_create_default() -> #ident_create_upper_camel_case {
                        #ident_create_upper_camel_case {
                            #ident_create_default_fields_initialization_without_primary_key_ts
                        }
                    }
                    async fn generate_read_only_ids_from_try_create_one(
                        #url_snake_case: &str,
                        #payload_snake_case: #ident_create_upper_camel_case,
                        table: &str,
                    ) -> #ident_read_only_ids_upper_camel_case {
                        #ident::try_create_one_handle(
                            #url_snake_case,
                            #ident_create_one_parameters_upper_camel_case {
                                #payload_snake_case
                            },
                            table
                        ).await.expect("32e30b87-b46a-4f39-aeb0-39694fc52d30")
                    }
                    async fn generate_read_only_ids_from_try_create_one_default(
                        #url_snake_case: &str,
                        table: &str,
                    ) -> #ident_read_only_ids_upper_camel_case {
                        generate_read_only_ids_from_try_create_one(
                            #url_snake_case,
                            ident_create_default(),
                            table
                        ).await
                    }
                    async fn generate_try_delete_one_handle(
                        #url_snake_case: &str,
                        #primary_key_field_ident: #primary_key_field_type_as_postgresql_type_read_ts,
                        table: &str,
                    ) -> Result<#primary_key_field_type_as_postgresql_type_read_ts, #ident_try_delete_one_error_named_upper_camel_case> {
                        #ident::try_delete_one_handle(
                            #url_snake_case,
                            #ident_delete_one_parameters_upper_camel_case {
                                payload: #ident_delete_one_payload_upper_camel_case {
                                    #primary_key_field_ident
                                }
                            },
                            table
                        ).await
                    }
                    fn no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row() -> &'static str {
                        "no rows returned by a query that expected to return at least one row"
                    }
                    fn generate_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create(
                        read_only_ids_from_try_create_many: Vec<#ident_read_only_ids_upper_camel_case>,
                        ident_vec_create: Vec<#ident_create_upper_camel_case>
                    ) -> Vec<#ident_read_upper_camel_case> {
                        let mut acc_1debe8fb = Vec::new();
                        assert_eq!(read_only_ids_from_try_create_many.len(), ident_vec_create.len(), "88fb286c-a440-441f-9e36-83454d0c9f75");
                        for (read_only_ids, create) in read_only_ids_from_try_create_many.into_iter().zip(ident_vec_create.into_iter()) {
                            acc_1debe8fb.push(#ident_read_upper_camel_case {
                                #primary_key_field_ident: <#primary_key_field_type as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                    &read_only_ids.#primary_key_field_ident
                                ),
                                #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_and_create_ts
                            });
                        }
                        acc_1debe8fb.sort_by(|first, second| {
                            if let (Some(first_handle), Some(second_handle)) = (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                first_handle.#value_snake_case.cmp(&second_handle.#value_snake_case)
                            } else {
                                panic!("d760ffa3-e9df-4628-92cd-d52c1ae1f91a");
                            }
                        });
                        acc_1debe8fb
                    }
                    #generate_read_only_ids_current_elements_ts
                    tracing_subscriber::fmt::init();
                    tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().expect("38823c21-1879-449c-9b60-ce7293709959").block_on(async {
                        //todo maybe refactor
                        let #config_snake_case = #config_path_ts {
                            service_socket_address: <config_lib::ServiceSocketAddress as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "127.0.0.1:8080".to_owned()
                            ).expect("b5b3915a-0e18-4815-a614-6b0e9a00d73f").0,
                            database_url: <config_lib::DatabaseUrl as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=10".to_owned()
                            ).expect("f9c20f05-3cdf-46ae-b6d3-5943c627f0df").0,
                            timezone: <config_lib::Timezone as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "10800".to_owned()
                            ).expect("d00d8998-52f9-45c1-a4b0-c93bc95a313e").0,
                            tracing_level: <config_lib::TracingLevel as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "error".to_owned()
                            ).expect("957178c9-4d92-4110-b524-9dc21d147a7c").0,
                            source_place_type: <config_lib::SourcePlaceType as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "source".to_owned()
                            ).expect("bec0950e-e9de-42f3-b3a2-67d9d98ae8a6").0,
                            enable_api_git_commit_check: <config_lib::EnableApiGitCommitCheck as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "true".to_owned()
                            ).expect("31f02640-d62b-41ca-837d-d61b707d4baf").0,
                            maximum_size_of_http_body_in_bytes: <config_lib::MaximumSizeOfHttpBodyInBytes as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "1048576000".to_owned()
                            ).expect("93b2f818-18be-4bb6-8a02-53c6e55ded2d").0,
                        };
                        let #postgres_pool_snake_case = sqlx::postgres::PgPoolOptions::new()
                        .max_connections(50)
                        .connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&#config_snake_case)))
                        .await.expect("e3044bb9-7b76-4c0c-bc5f-eb34da05a103");
                        let #url_snake_case = format!("http://{}", app_state::GetServiceSocketAddress::get_service_socket_address(&#config_snake_case));
                        let table = #ident_double_quotes_ts;

                        let add_table_postfix = |postfix: &str|{
                            let value = format!("{table}_{postfix}");
                            assert!(value.len() <= 63, "77f9bfb7-f7d8-4ba0-96d0-712d4246ecae");
                            value
                        };
                        let table_initialization = add_table_postfix("initialization");
                        let table_create_many = add_table_postfix("create_many");
                        let table_create_one = add_table_postfix("create_one");
                        let table_test_read_many_by_non_existent_primary_keys = add_table_postfix("test_read_many_by_non_existent_primary_keys");
                        let table_test_read_many_by_equal_to_created_primary_keys = add_table_postfix("test_read_many_by_equal_to_created_primary_keys");
                        #(#table_field_idents_initialization_vec_ts)*
                        let table_read_one = add_table_postfix("read_one");
                        let table_update_many = add_table_postfix("update_many");
                        let table_update_one = add_table_postfix("update_one");
                        let table_delete_many = add_table_postfix("delete_many");
                        let table_delete_one = add_table_postfix("delete_one");
                        let table_names = [
                            &table_initialization,
                            &table_create_many,
                            &table_create_one,
                            &table_test_read_many_by_non_existent_primary_keys,
                            &table_test_read_many_by_equal_to_created_primary_keys,
                            #(#table_test_name_field_idents_vec_ts)*
                            &table_read_one,
                            &table_update_many,
                            &table_update_one,
                            &table_delete_many,
                            &table_delete_one,
                        ];
                        let drop_all_test_tables = async ||{
                            let _unused = futures::future::try_join_all(
                                table_names
                                .iter()
                                .map(|table_name|{
                                    let postgres_pool_3b948340 = &postgres_pool;
                                    async move {
                                        sqlx::query(&format!("drop table if exists {table_name}")).execute(postgres_pool_3b948340).await
                                    }
                                })
                            )
                            .await
                            .expect("b9c1eb2e-4ead-449b-abb8-0a160cf68efd");
                        };
                        drop_all_test_tables().await;
                        #ident::prepare_extensions(&#postgres_pool_snake_case).await.expect("0633ff48-ebc4-460f-a282-d750511f5d78");
                        //do not make it concurrent. would be postgresql error: "duplicate key value violates unique constraint \"pg_class_relname_nsp_index\""
                        for el_dac43b91 in table_names {
                            #ident::prepare_postgresql_table(
                                &#postgres_pool_snake_case,
                                el_dac43b91,
                            ).await.expect("c7952247-dc94-441b-9aef-368b8fdc593c");
                        }
                        let #postgres_pool_for_tokio_spawn_sync_move_snake_case = #postgres_pool_snake_case.clone();
                        let table_names_cloned = table_names.iter().map(|el_26b304d1| (*el_26b304d1).to_owned()).collect::<Vec<String>>();
                        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
                        let #underscore_unused_ts = tokio::spawn(async move {
                            let tcp_listener = tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&#config_snake_case)).await.expect("663ae29e-bc00-4ea1-a7e9-4dddceb5b53a");
                            let #app_state_snake_case = std::sync::Arc::new(server_app_state::ServerAppState {
                                #postgres_pool_snake_case: #postgres_pool_for_tokio_spawn_sync_move_snake_case.clone(),
                                #config_snake_case,
                                project_git_info: &git_info::PROJECT_GIT_INFO,
                            });
                            started_tx.send(()).expect("431a6f8d-3fbb-4eb2-86f6-1b9cfd57e32e");
                            axum::serve(
                                tcp_listener,
                                {
                                    let mut router = axum::Router::new()
                                        .merge(#ident::routes(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state)));
                                    for el_ef09f2b0 in table_names_cloned {
                                        router = router.merge(#ident::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &el_ef09f2b0));
                                    }
                                    router.into_make_service()
                                },
                            )
                            .await
                            .expect("71c1bc30-2f27-4fb4-8545-bc1bf21bc1ea");
                        });
                        started_rx.await.expect("87003141-43a4-4975-8ddf-273148add50f");
                        let #select_primary_key_snake_case = postgresql_crud::NotEmptyUniqueVec::try_new(vec![
                            #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_ts(
                                #primary_key_field_type_as_postgresql_type_select_ts::default(),
                            )
                        ])
                        .expect("0776170e-4dd6-4c14-a412-ce10b0c746f1");
                        let #ident_create_default_snake_case = ident_create_default();
                        #select_default_all_with_max_page_size_not_empty_unique_vec_ts
                        #common_read_only_ids_returned_from_create_one_ts
                        #read_only_ids_to_two_dimensional_vec_read_inner_acc_fields_ts
                        futures::StreamExt::for_each_concurrent(
                            futures::stream::iter({
                                let mut acc_9189f86e: Vec<futures::future::BoxFuture<'static, ()>> = Vec::new();
                                #create_many_tests_ts
                                #create_one_tests_ts
                                #read_many_tests_ts
                                #read_one_tests_ts
                                #update_many_tests_ts
                                #update_one_tests_ts
                                #delete_many_tests_ts
                                #delete_one_tests_ts
                                acc_9189f86e
                            }),
                            100,
                            async |fut| { fut.await; },
                        )
                        .await;
                        drop_all_test_tables().await;
                    });
                }
            }
        }
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config.tests_content_write_into_generate_postgresql_table_tests,
        "generate_postgresql_table_tests",
        &ident_tests_ts,
        &macros_helpers::FormatWithCargofmt::True,
    );
    let common_ts = quote::quote! {
        #impl_ident_ts
        #ident_create_ts
        #ident_where_many_ts
        #std_option_option_ident_where_many_ts
        #select_ts
        #ident_read_ts
        #ident_read_only_ids_ts
        // #ident_column_read_permission_ts
        #ident_update_ts
        #ident_update_for_query_ts
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config.common_content_write_into_generate_postgresql_table_common,
        "generate_postgresql_table_common",
        &common_ts,
        &macros_helpers::FormatWithCargofmt::True,
    );
    let generated = quote::quote! {
        #allow_clippy_arbitrary_source_item_ordering_ts
        impl #ident {
            #(#impl_ident_vec_ts)*
        }
        #common_ts
        #create_many_ts
        #create_one_ts
        #read_many_ts
        #read_one_ts
        #update_many_ts
        #update_one_ts
        #delete_many_ts
        #delete_one_ts
        #ident_tests_ts
    };
    macros_helpers::maybe_write_ts_into_file(
        generate_postgresql_table_config.whole_content_write_into_generate_postgresql_table,
        "generate_postgresql_table",
        &generated,
        &macros_helpers::FormatWithCargofmt::True,
    );
    generated
}
