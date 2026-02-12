pub use postgresql_crud_common_and_macros_common::*;

use error_occurence_lib::code_occurence::CodeOccurence;
use naming::{AscUcc, DescUcc, DisplayToScStr, DisplayToUccStr};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgArguments, PgValueRef},
    query::Query,
    types::Json,
    {Decode, Postgres, Type},
};
use std::{
    error::Error as StdErrorError,
    fmt::{
        Formatter, Result as StdFmtResult, {Debug, Display},
    },
};

macro_rules! trait_alias {
    ($name:ident = $($bounds:tt)+) => {
        pub trait $name: $($bounds)+ {}
        impl<T: $($bounds)+> $name for T {}
    };
}

pub const DEFAULT_PAGINATION_LIMIT: i64 = 5;

trait_alias!(DebugClonePartialEqAlias = Debug + Clone + PartialEq);
trait_alias!(DebugClonePartialEqSerializeAlias = DebugClonePartialEqAlias + serde::Serialize);
trait_alias!(DebugClonePartialEqSerializeDeserializeAlias = DebugClonePartialEqSerializeAlias + for<'__> serde::Deserialize<'__>);
trait_alias!(
    DebugClonePartialEqSerializeDeserializeDefaultSomeOneAlias =
        DebugClonePartialEqSerializeDeserializeAlias + DefaultOptionSomeVecOneEl
);
trait_alias!(SqlxEncodePostgresSqlxTypePostgresAlias = for<'__> sqlx::Encode<'__, Postgres> + Type<Postgres>);
trait_alias!(UtoipaToSchemaAndSchemarsJsonSchemaAlias = for<'__> utoipa::ToSchema<'__> + schemars::JsonSchema);

trait_alias!(
    TableTypeDeclarationAlias = DebugClonePartialEqSerializeDeserializeDefaultSomeOneAlias
);
trait_alias!(CreateAlias = DebugClonePartialEqSerializeDeserializeDefaultSomeOneAlias);
trait_alias!(
    CreateForQueryAlias =
        DebugClonePartialEqSerializeAlias + SqlxEncodePostgresSqlxTypePostgresAlias
);
trait_alias!(SelectAlias = DebugClonePartialEqSerializeDeserializeDefaultSomeOneAlias);
trait_alias!(WhereAlias = DebugClonePartialEqSerializeDeserializeAlias + for<'__> PostgresqlTypeWhereFilter<'__>);
trait_alias!(ReadAlias = DebugClonePartialEqSerializeDeserializeAlias);
trait_alias!(ReadOnlyIdsAlias = DebugClonePartialEqSerializeDeserializeAlias);
trait_alias!(ReadInnerAlias = DebugClonePartialEqAlias);
trait_alias!(UpdateAlias = DebugClonePartialEqSerializeDeserializeDefaultSomeOneAlias);
trait_alias!(UpdateForQueryAlias = DebugClonePartialEqSerializeAlias);
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PostgresqlType {
    //difference between Create and TableTypeDeclaration - Create may not contain gend by postgresql id
    type TableTypeDeclaration: TableTypeDeclarationAlias;
    fn create_table_column_query_part(column: &dyn Display, _: bool) -> impl Display;
    type Create: CreateAlias;
    fn create_query_part(
        value: &Self::Create,
        increment: &mut u64,
    ) -> Result<String, QueryPartErrorNamed>;
    fn create_query_bind(
        value: Self::Create,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Query<'_, Postgres, PgArguments>, String>;
    type Select: SelectAlias;
    fn select_query_part(value: &Self::Select, column: &str)
    -> Result<String, QueryPartErrorNamed>;
    type Where: WhereAlias;
    type Read: ReadAlias + for<'__> Decode<'__, Postgres> + Type<Postgres>;
    fn normalize(value: Self::Read) -> Self::Read;
    type ReadOnlyIds: ReadOnlyIdsAlias;
    fn select_only_ids_query_part(column: &str) -> Result<String, QueryPartErrorNamed>;
    type ReadInner: ReadInnerAlias;
    fn into_inner(value: Self::Read) -> Self::ReadInner;
    type Update: UpdateAlias;
    type UpdateForQuery: UpdateForQueryAlias;
    fn update_query_part(
        value: &Self::UpdateForQuery,
        jsonb_set_accumulator: &str,
        jsonb_set_target: &str,
        jsonb_set_path: &str,
        increment: &mut u64,
    ) -> Result<String, QueryPartErrorNamed>;
    fn update_query_bind(
        value: Self::UpdateForQuery,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Query<'_, Postgres, PgArguments>, String>;
    fn select_only_updated_ids_query_part(
        value: &Self::UpdateForQuery,
        column: &str,
        increment: &mut u64,
    ) -> Result<String, QueryPartErrorNamed>;
    fn select_only_updated_ids_query_bind<'lifetime>(
        value: &'lifetime Self::UpdateForQuery,
        query: Query<'lifetime, Postgres, PgArguments>,
    ) -> Result<Query<'lifetime, Postgres, PgArguments>, String>;
}
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PostgresqlJsonType {
    type TableTypeDeclaration: TableTypeDeclarationAlias + UtoipaToSchemaAndSchemarsJsonSchemaAlias;
    type Create: CreateAlias + UtoipaToSchemaAndSchemarsJsonSchemaAlias;
    type CreateForQuery: CreateForQueryAlias + From<Self::Create>;
    type Select: SelectAlias + UtoipaToSchemaAndSchemarsJsonSchemaAlias;
    fn select_query_part(
        value: &Self::Select,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        //todo remove this coz its used properly now
        column_name_and_maybe_field_getter_for_error_message: &str,
        is_postgresql_type: bool,
    ) -> Result<String, QueryPartErrorNamed>;
    type Where: WhereAlias
        + UtoipaToSchemaAndSchemarsJsonSchemaAlias
        + AllEnumVariantsArrayDefaultOptionSomeVecOneEl
        + error_occurence_lib::ToStdStringString;
    //todo impl get fields from read
    //todo maybe add Decode trait here and Type
    type Read: ReadAlias + UtoipaToSchemaAndSchemarsJsonSchemaAlias + DefaultOptionSomeVecOneEl;
    type ReadOnlyIds: ReadOnlyIdsAlias;
    fn select_only_ids_query_part(
        column_name_and_maybe_field_getter: &str,
    ) -> Result<String, QueryPartErrorNamed>;
    type ReadInner: ReadInnerAlias;
    fn into_inner(value: Self::Read) -> Self::ReadInner;
    type Update: UpdateAlias + UtoipaToSchemaAndSchemarsJsonSchemaAlias;
    type UpdateForQuery: UpdateForQueryAlias + From<Self::Update>;
    fn update_query_part(
        value: &Self::UpdateForQuery,
        jsonb_set_accumulator: &str,
        jsonb_set_target: &str,
        jsonb_set_path: &str,
        increment: &mut u64,
    ) -> Result<String, QueryPartErrorNamed>;
    fn update_query_bind(
        value: Self::UpdateForQuery,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Query<'_, Postgres, PgArguments>, String>;
    fn select_only_updated_ids_query_part(
        value: &Self::UpdateForQuery,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, QueryPartErrorNamed>;
    fn select_only_updated_ids_query_bind<'lifetime>(
        value: &'lifetime Self::UpdateForQuery,
        query: Query<'lifetime, Postgres, PgArguments>,
    ) -> Result<Query<'lifetime, Postgres, PgArguments>, String>;
    fn select_only_created_ids_query_part(
        value: &Self::CreateForQuery,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, QueryPartErrorNamed>;
    fn select_only_created_ids_query_bind<'lifetime>(
        value: &'lifetime Self::CreateForQuery,
        query: Query<'lifetime, Postgres, PgArguments>,
    ) -> Result<Query<'lifetime, Postgres, PgArguments>, String>;
}
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PostgresqlTypePrimaryKey {
    type PostgresqlType: PostgresqlType;
    type TableTypeDeclaration: TableTypeDeclarationAlias + PartialOrd;
    fn read_only_ids_into_table_type_declaration(
        value: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
    ) -> <Self::PostgresqlType as PostgresqlType>::TableTypeDeclaration;
    fn read_only_ids_into_read(
        value: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
    ) -> <Self::PostgresqlType as PostgresqlType>::Read;
    fn read_only_ids_into_update(
        value: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
    ) -> <Self::PostgresqlType as PostgresqlType>::Update;
    fn read_into_table_type_declaration(
        value: <Self::PostgresqlType as PostgresqlType>::Read,
    ) -> <Self::PostgresqlType as PostgresqlType>::TableTypeDeclaration;
}
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PostgresqlTypeNotPrimaryKey {
    type PostgresqlType: PostgresqlType;
    type Create: CreateAlias + SqlxEncodePostgresSqlxTypePostgresAlias;
}
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PostgresqlJsonTypeObjectVecElementId {
    type PostgresqlJsonType: PostgresqlJsonType;
    type CreateForQuery: CreateForQueryAlias
        + From<<Self::PostgresqlJsonType as PostgresqlJsonType>::Create>
        + From<<Self::PostgresqlJsonType as PostgresqlJsonType>::Update>;
    type Update: UpdateAlias
        + UtoipaToSchemaAndSchemarsJsonSchemaAlias
        + error_occurence_lib::ToStdStringString;
    type ReadInner: ReadInnerAlias;
    fn query_bind_string_as_postgresql_text_create_for_query(
        value: <Self::PostgresqlJsonType as PostgresqlJsonType>::CreateForQuery,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Query<'_, Postgres, PgArguments>, String>;
    fn query_bind_string_as_postgresql_text_update_for_query(
        value: <Self::PostgresqlJsonType as PostgresqlJsonType>::UpdateForQuery,
        query: Query<'_, Postgres, PgArguments>,
    ) -> Result<Query<'_, Postgres, PgArguments>, String>;
    fn get_inner(
        value: &<Self::PostgresqlJsonType as PostgresqlJsonType>::CreateForQuery,
    ) -> &Self::ReadInner;
    fn increment_checked_add_one(increment: &mut u64) -> Result<u64, QueryPartErrorNamed>;
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
pub trait PostgresqlTypeTestCases {
    type PostgresqlType: PostgresqlType;
    type Select: SelectAlias + DefaultOptionSomeVecOneElMaxPageSize;
    fn option_vec_create() -> Option<Vec<<Self::PostgresqlType as PostgresqlType>::Create>>;
    fn read_only_ids_to_two_dimensional_vec_read_inner(
        read_only_ids: &<Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
    ) -> Vec<Vec<<Self::PostgresqlType as PostgresqlType>::ReadInner>>;
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        value: <Self::PostgresqlType as PostgresqlType>::ReadInner,
    ) -> <Self::PostgresqlType as PostgresqlType>::Read;
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        value: <Self::PostgresqlType as PostgresqlType>::ReadInner,
    ) -> <Self::PostgresqlType as PostgresqlType>::Update;
    fn update_to_read_only_ids(
        value: &<Self::PostgresqlType as PostgresqlType>::Update,
    ) -> <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds;
    fn read_only_ids_to_option_value_read_default_option_some_vec_one_el(
        value: &<Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
    ) -> Option<Value<<Self::PostgresqlType as PostgresqlType>::Read>>;
    fn previous_read_merged_with_option_update_into_read(
        read: <Self::PostgresqlType as PostgresqlType>::Read,
        option_update: Option<<Self::PostgresqlType as PostgresqlType>::Update>,
    ) -> <Self::PostgresqlType as PostgresqlType>::Read;
    fn read_only_ids_merged_with_create_into_read(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as PostgresqlType>::Read;
    fn read_only_ids_merged_with_create_into_option_value_read(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<Value<<Self::PostgresqlType as PostgresqlType>::Read>>;
    fn read_only_ids_merged_with_create_into_table_type_declaration(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as PostgresqlType>::TableTypeDeclaration;

    //todo add prefix postgresql_type or postgresql_json_type ?
    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as PostgresqlType>::Where;
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> NotEmptyUniqueVec<<Self::PostgresqlType as PostgresqlType>::Where>;
    fn read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlType as PostgresqlType>::Where>>;
    fn create_into_postgresql_type_option_vec_where_dimension_one_equal(
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlType as PostgresqlType>::Where>>;
    fn postgresql_type_option_vec_where_greater_than_test()
    -> Option<NotEmptyUniqueVec<PostgresqlTypeGreaterThanTest<Self::PostgresqlType>>>;
    fn read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than(
        greater_than_variant: PostgresqlTypeGreaterThanVariant,
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        table_type_declaration: <Self::PostgresqlType as PostgresqlType>::TableTypeDeclaration,
    ) -> Option<<Self::PostgresqlType as PostgresqlType>::Where>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlType as PostgresqlType>::Where>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlType as PostgresqlType>::Where>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlType as PostgresqlType>::Where>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlType as PostgresqlType>::Where>>;

    fn create_into_postgresql_json_type_option_vec_where_length_equal(
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlType as PostgresqlType>::Where>>;
    fn create_into_postgresql_json_type_option_vec_where_length_greater_than(
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlType as PostgresqlType>::Where>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<SingleOrMultiple<<Self::PostgresqlType as PostgresqlType>::Where>>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<SingleOrMultiple<<Self::PostgresqlType as PostgresqlType>::Where>>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<SingleOrMultiple<<Self::PostgresqlType as PostgresqlType>::Where>>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<SingleOrMultiple<<Self::PostgresqlType as PostgresqlType>::Where>>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<SingleOrMultiple<<Self::PostgresqlType as PostgresqlType>::Where>>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression(
        read_only_ids: <Self::PostgresqlType as PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as PostgresqlType>::Create,
    ) -> Option<NotEmptyUniqueVec<SingleOrMultiple<<Self::PostgresqlType as PostgresqlType>::Where>>>;
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq)]
pub struct PostgresqlTypeGreaterThanTest<T: PostgresqlType> {
    pub variant: PostgresqlTypeGreaterThanVariant,
    pub create: <T as PostgresqlType>::Create,
    pub greater_than: <T as PostgresqlType>::TableTypeDeclaration,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub struct PostgresqlTypeLengthGreaterThanTest<T: PostgresqlType> {
    pub variant: PostgresqlJsonTypeLengthGreaterThanVariant,
    pub create: <T as PostgresqlType>::Create,
    pub length_greater_than: UnsignedPartOfStdPrimitiveI32,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub struct PostgresqlJsonTypeLengthGreaterThanTest<T: PostgresqlJsonType> {
    pub variant: PostgresqlJsonTypeLengthGreaterThanVariant,
    pub create: <T as PostgresqlJsonType>::Create,
    pub length_greater_than: UnsignedPartOfStdPrimitiveI32,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
pub trait PostgresqlJsonTypeTestCases {
    type PostgresqlJsonType: PostgresqlJsonType;
    type Select: SelectAlias
        + UtoipaToSchemaAndSchemarsJsonSchemaAlias
        + DefaultOptionSomeVecOneElMaxPageSize;
    fn option_vec_create() -> Option<Vec<<Self::PostgresqlJsonType as PostgresqlJsonType>::Create>>;
    fn read_only_ids_to_two_dimensional_vec_read_inner(
        read_only_ids: &<Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
    ) -> Vec<Vec<<Self::PostgresqlJsonType as PostgresqlJsonType>::ReadInner>>;
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        value: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadInner,
    ) -> <Self::PostgresqlJsonType as PostgresqlJsonType>::Read;
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        value: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadInner,
    ) -> <Self::PostgresqlJsonType as PostgresqlJsonType>::Update;
    fn read_only_ids_into_option_value_read_inner(
        value: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
    ) -> Option<Value<<Self::PostgresqlJsonType as PostgresqlJsonType>::ReadInner>>;
    fn update_to_read_only_ids(
        value: &<Self::PostgresqlJsonType as PostgresqlJsonType>::Update,
    ) -> <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds;
    fn read_only_ids_to_option_value_read_default_option_some_vec_one_el(
        value: &<Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
    ) -> Option<Value<<Self::PostgresqlJsonType as PostgresqlJsonType>::Read>>;
    fn previous_read_merged_with_option_update_into_read(
        read: <Self::PostgresqlJsonType as PostgresqlJsonType>::Read,
        option_update: Option<<Self::PostgresqlJsonType as PostgresqlJsonType>::Update>,
    ) -> <Self::PostgresqlJsonType as PostgresqlJsonType>::Read;
    fn read_only_ids_merged_with_create_into_read(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as PostgresqlJsonType>::Read;
    fn read_only_ids_merged_with_create_into_option_value_read(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<Value<<Self::PostgresqlJsonType as PostgresqlJsonType>::Read>>;
    fn read_only_ids_merged_with_create_into_table_type_declaration(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as PostgresqlJsonType>::TableTypeDeclaration;

    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as PostgresqlJsonType>::Where;
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> NotEmptyUniqueVec<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>;
    fn read_only_ids_merged_with_create_into_vec_where_equal_to_json_field(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> NotEmptyUniqueVec<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>>;
    fn create_into_postgresql_json_type_option_vec_where_length_equal(
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>>;
    fn create_into_postgresql_json_type_option_vec_where_length_greater_than(
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>>;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<
        NotEmptyUniqueVec<
            SingleOrMultiple<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>,
        >,
    >;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<
        NotEmptyUniqueVec<
            SingleOrMultiple<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>,
        >,
    >;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<
        NotEmptyUniqueVec<
            SingleOrMultiple<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>,
        >,
    >;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<
        NotEmptyUniqueVec<
            SingleOrMultiple<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>,
        >,
    >;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<
        NotEmptyUniqueVec<
            SingleOrMultiple<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>,
        >,
    >;
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression(
        read_only_ids: <Self::PostgresqlJsonType as PostgresqlJsonType>::ReadOnlyIds,
        create: <Self::PostgresqlJsonType as PostgresqlJsonType>::Create,
    ) -> Option<
        NotEmptyUniqueVec<
            SingleOrMultiple<<Self::PostgresqlJsonType as PostgresqlJsonType>::Where>,
        >,
    >;
}
pub trait PostgresqlTypeWhereFilter<'query_lifetime> {
    fn query_bind(
        self,
        query: Query<'query_lifetime, Postgres, PgArguments>,
    ) -> Result<Query<'query_lifetime, Postgres, PgArguments>, String>;
    fn query_part(
        &self,
        increment: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed>;
}
//todo custom deserialization - must not contain more than one element
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
)]
pub struct NullableJsonObjectPostgresqlTypeWhereFilter<
    T: Debug
        + PartialEq
        + Clone
        + for<'lifetime> PostgresqlTypeWhereFilter<'lifetime>
        + AllEnumVariantsArrayDefaultOptionSomeVecOneEl,
>(pub Option<NotEmptyUniqueVec<T>>);
impl<'query_lifetime, T> PostgresqlTypeWhereFilter<'query_lifetime>
    for NullableJsonObjectPostgresqlTypeWhereFilter<T>
where
    T: Debug
        + PartialEq
        + Clone
        + for<'t_lifetime> PostgresqlTypeWhereFilter<'t_lifetime>
        + AllEnumVariantsArrayDefaultOptionSomeVecOneEl,
{
    fn query_bind(
        self,
        query: Query<'query_lifetime, Postgres, PgArguments>,
    ) -> Result<Query<'query_lifetime, Postgres, PgArguments>, String> {
        match self.0 {
            Some(value) => value.query_bind(query),
            None => Ok(query), //todo maybe wrong
        }
    }
    fn query_part(
        &self,
        increment: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        self.0.as_ref().map_or_else(
            || Ok(format!("{column} = 'null'")),
            |value_b4a9fcfb| {
                value_b4a9fcfb.query_part(increment, column, is_need_to_add_logical_operator)
            },
        )
    }
}
impl<T> error_occurence_lib::ToStdStringString for NullableJsonObjectPostgresqlTypeWhereFilter<T>
where
    T: Debug
        + PartialEq
        + Clone
        + for<'t_lifetime> PostgresqlTypeWhereFilter<'t_lifetime>
        + AllEnumVariantsArrayDefaultOptionSomeVecOneEl,
{
    fn to_std_string_string(&self) -> String {
        format!("{self:#?}")
    }
}
impl<T> AllEnumVariantsArrayDefaultOptionSomeVecOneEl
    for NullableJsonObjectPostgresqlTypeWhereFilter<T>
where
    T: Debug
        + PartialEq
        + Clone
        + for<'t_lifetime> PostgresqlTypeWhereFilter<'t_lifetime>
        + AllEnumVariantsArrayDefaultOptionSomeVecOneEl,
{
    fn all_variants_default_option_some_vec_one_el() -> Vec<Self> {
        vec![Self(Some(
            DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))]
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
)]
pub enum QueryPartErrorNamed {
    CheckedAdd { code_occurence: CodeOccurence },
    WriteIntoBuffer { code_occurence: CodeOccurence },
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct PostgresqlTypeWhere<T> {
    logical_operator: LogicalOperator,
    value: NotEmptyUniqueVec<T>,
}
impl<T: PartialEq + Clone> PostgresqlTypeWhere<T> {
    #[must_use]
    pub const fn get_logical_operator(&self) -> &LogicalOperator {
        &self.logical_operator
    }
    #[must_use]
    pub const fn new(logical_operator: LogicalOperator, value: NotEmptyUniqueVec<T>) -> Self {
        Self {
            logical_operator,
            value,
        }
    }
    pub fn try_new(
        logical_operator: LogicalOperator,
        value: Vec<T>,
    ) -> Result<Self, NotEmptyUniqueVecTryNewErrorNamed<T>> {
        match NotEmptyUniqueVec::try_new(value) {
            Ok(value_56f976af) => Ok(Self {
                logical_operator,
                value: value_56f976af,
            }),
            Err(error) => Err(error),
        }
    }
}

#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: Debug + PartialEq + Clone + _serde::Deserialize<'de>> _serde::Deserialize<'de>
        for PostgresqlTypeWhere<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[expect(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __f: &mut Formatter<'_>) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__f, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => Ok(__Field::__field0),
                        1u64 => Ok(__Field::__field1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => Ok(__Field::__field0),
                        "value" => Ok(__Field::__field1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => Ok(__Field::__field0),
                        b"value" => Ok(__Field::__field1),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de, PostgresqlTypeWhere> {
                marker: _serde::__private228::PhantomData<PostgresqlTypeWhere>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            impl<'de, T: Debug + PartialEq + Clone + _serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = PostgresqlTypeWhere<T>;
                fn expecting(&self, __f: &mut Formatter<'_>) -> _serde::__private228::fmt::Result {
                    Formatter::write_str(__f, "struct PostgresqlTypeWhere")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) =
                        _serde::de::SeqAccess::next_element::<LogicalOperator>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"struct PostgresqlTypeWhere with 2 elements",
                        ));
                    };
                    let Some(__field1) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            1usize,
                            &"struct PostgresqlTypeWhere with 2 elements",
                        ));
                    };
                    match PostgresqlTypeWhere::try_new(__field0, __field1) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: Option<LogicalOperator> = None;
                    let mut __field1: Option<Vec<T>> = None;
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "logical_operator",
                                        ),
                                    );
                                }
                                __field0 = Some(_serde::de::MapAccess::next_value::<
                                    LogicalOperator,
                                >(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    );
                                }
                                __field1 =
                                    Some(_serde::de::MapAccess::next_value::<Vec<T>>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __map,
                                    )?;
                            }
                        }
                    }
                    let __field0_value = match __field0 {
                        Some(value) => value,
                        None => _serde::__private228::de::missing_field("logical_operator")?,
                    };
                    let __field1_value = match __field1 {
                        Some(value) => value,
                        None => _serde::__private228::de::missing_field("value")?,
                    };
                    match PostgresqlTypeWhere::try_new(__field0_value, __field1_value) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeWhere",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<T>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<'query_lifetime, T: PostgresqlTypeWhereFilter<'query_lifetime>>
    PostgresqlTypeWhereFilter<'query_lifetime> for PostgresqlTypeWhere<T>
{
    fn query_bind(
        self,
        mut query: Query<'query_lifetime, Postgres, PgArguments>,
    ) -> Result<Query<'query_lifetime, Postgres, PgArguments>, String> {
        for el_4e314a75 in self.value.0 {
            match PostgresqlTypeWhereFilter::query_bind(el_4e314a75, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(query)
    }
    fn query_part(
        &self,
        increment: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        let mut acc_cc6d18f7 = String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for el_a38b9c67 in &self.value.0 {
            match PostgresqlTypeWhereFilter::query_part(
                el_a38b9c67,
                increment,
                column,
                is_need_to_add_logical_operator_inner_handle,
            ) {
                Ok(value) => {
                    use std::fmt::Write as _;
                    if write!(acc_cc6d18f7, "{value} ").is_err() {
                        return Err(QueryPartErrorNamed::WriteIntoBuffer {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                    is_need_to_add_logical_operator_inner_handle = true;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _: Option<char> = acc_cc6d18f7.pop();
        Ok(format!(
            "{}({acc_cc6d18f7})",
            &self
                .logical_operator
                .to_query_part(is_need_to_add_logical_operator)
        ))
    }
}
impl<T: Debug + PartialEq + Clone + AllEnumVariantsArrayDefaultOptionSomeVecOneEl>
    DefaultOptionSomeVecOneEl for PostgresqlTypeWhere<T>
{
    fn default_option_some_vec_one_el() -> Self {
        Self {
            logical_operator: DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            value: NotEmptyUniqueVec::try_new(
                AllEnumVariantsArrayDefaultOptionSomeVecOneEl::all_variants_default_option_some_vec_one_el()
            ).expect("a918b427-2d74-4096-915c-2f97314cc05f"),
        }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    from_str::FromStr,
)]
pub enum Order {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    #[default]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}
impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
        match self {
            Self::Asc => write!(f, "{AscUcc}"),
            Self::Desc => write!(f, "{DescUcc}"),
        }
    }
}
impl DefaultOptionSomeVecOneEl for Order {
    fn default_option_some_vec_one_el() -> Self {
        Self::default()
    }
}
impl Order {
    #[must_use]
    pub fn to_sc_str(&self) -> String {
        DisplayToScStr::case(&self)
    }
    #[must_use]
    pub fn to_ucc_str(&self) -> String {
        DisplayToUccStr::case(&self)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderBy<ColumnGeneric> {
    pub column: ColumnGeneric,
    pub order: Option<Order>,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
)]
pub struct PaginationBase {
    limit: i64,
    offset: i64,
}
impl PaginationBase {
    #[must_use]
    pub const fn end(&self) -> i64 {
        self.offset
            .checked_add(self.limit)
            .expect("8a297b66-4f42-4b48-8e18-cc1f35302e0a")
    }
    #[must_use]
    pub const fn new_unchecked(limit: i64, offset: i64) -> Self {
        Self { limit, offset }
    }
    #[must_use]
    pub const fn start(&self) -> i64 {
        self.offset
    }
}
impl<'query_lifetime> PostgresqlTypeWhereFilter<'query_lifetime> for PaginationBase {
    fn query_bind(
        self,
        mut query: Query<'query_lifetime, Postgres, PgArguments>,
    ) -> Result<Query<'query_lifetime, Postgres, PgArguments>, String> {
        if let Err(error) = query.try_bind(self.limit) {
            return Err(error.to_string());
        }
        if let Err(error) = query.try_bind(self.offset) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn query_part(
        &self,
        increment: &mut u64,
        _: &dyn Display,
        _: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        let limit_increment = match increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let offset_increment = match increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!(
            "limit ${limit_increment} offset ${offset_increment}"
        ))
    }
}
impl Default for PaginationBase {
    fn default() -> Self {
        Self::new_unchecked(DEFAULT_PAGINATION_LIMIT, 0)
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
)]
pub struct PaginationStartsWithZero(PaginationBase);
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
)]
pub enum PaginationStartsWithZeroTryNewErrorNamed {
    LimitIsLessThanOrEqualToZero {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: i64,
        code_occurence: CodeOccurence,
    },
    OffsetIsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        offset: i64,
        code_occurence: CodeOccurence,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: i64,
        #[eo_to_std_string_string_serialize_deserialize]
        offset: i64,
        code_occurence: CodeOccurence,
    },
}
impl PaginationStartsWithZero {
    #[must_use]
    pub const fn end(&self) -> i64 {
        self.0.end()
    }
    #[must_use]
    pub const fn start(&self) -> i64 {
        self.0.start()
    }
    pub fn try_new(
        limit: i64,
        offset: i64,
    ) -> Result<Self, PaginationStartsWithZeroTryNewErrorNamed> {
        if limit <= 0 || offset < 0 {
            if limit <= 0 {
                Err(
                    PaginationStartsWithZeroTryNewErrorNamed::LimitIsLessThanOrEqualToZero {
                        limit,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    },
                )
            } else {
                Err(
                    PaginationStartsWithZeroTryNewErrorNamed::OffsetIsLessThanZero {
                        offset,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    },
                )
            }
        } else if offset.checked_add(limit).is_some() {
            Ok(Self(PaginationBase::new_unchecked(limit, offset)))
        } else {
            Err(
                PaginationStartsWithZeroTryNewErrorNamed::OffsetPlusLimitIsIntOverflow {
                    limit,
                    offset,
                    code_occurence: error_occurence_lib::code_occurence!(),
                },
            )
        }
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
impl<'de> serde::Deserialize<'de> for PaginationStartsWithZero {
    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[expect(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __f: &mut Formatter<'_>) -> serde::__private228::fmt::Result {
                serde::__private228::Formatter::write_str(__f, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => Ok(__Field::__field0),
                    1u64 => Ok(__Field::__field1),
                    _ => Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "limit" => Ok(__Field::__field0),
                    "offset" => Ok(__Field::__field1),
                    _ => Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"limit" => Ok(__Field::__field0),
                    b"offset" => Ok(__Field::__field1),
                    _ => Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private228::PhantomData<PaginationStartsWithZero>,
            lifetime: serde::__private228::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PaginationStartsWithZero;
            fn expecting(&self, __f: &mut Formatter<'_>) -> serde::__private228::fmt::Result {
                Formatter::write_str(__f, "struct PaginationStartsWithZero")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let Some(__field0) = serde::de::SeqAccess::next_element::<i64>(&mut __seq)? else {
                    return Err(serde::de::Error::invalid_length(
                        0usize,
                        &"struct PaginationStartsWithZero with 2 elements",
                    ));
                };
                let Some(__field1) = serde::de::SeqAccess::next_element::<i64>(&mut __seq)? else {
                    return Err(serde::de::Error::invalid_length(
                        1usize,
                        &"struct PaginationStartsWithZero with 2 elements",
                    ));
                };
                match PaginationStartsWithZero::try_new(__field0, __field1) {
                    Ok(value) => Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))), //todo use serde_json::to_string(&error).unwrap_or_else(|_|"failed to serialize error".into())
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: Option<i64> = None;
                let mut __field1: Option<i64> = None;
                while let Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "limit",
                                ));
                            }
                            __field0 = Some(serde::de::MapAccess::next_value::<i64>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if Option::is_some(&__field1) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "offset",
                                ));
                            }
                            __field1 = Some(serde::de::MapAccess::next_value::<i64>(&mut __map)?);
                        }
                        __Field::__ignore => {
                            let _: serde::de::IgnoredAny =
                                serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                        }
                    }
                }
                let __field0_value = match __field0 {
                    Some(value) => value,
                    None => serde::__private228::de::missing_field("limit")?,
                };
                let __field1_value = match __field1 {
                    Some(value) => value,
                    None => serde::__private228::de::missing_field("offset")?,
                };
                match PaginationStartsWithZero::try_new(__field0_value, __field1_value) {
                    Ok(value) => Ok(value),
                    Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &[&str] = &["limit", "offset"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "PaginationStartsWithZero",
            FIELDS,
            __Visitor {
                marker: serde::__private228::PhantomData::<Self>,
                lifetime: serde::__private228::PhantomData,
            },
        )
    }
}
impl<'query_lifetime> PostgresqlTypeWhereFilter<'query_lifetime> for PaginationStartsWithZero {
    fn query_bind(
        self,
        query: Query<'query_lifetime, Postgres, PgArguments>,
    ) -> Result<Query<'query_lifetime, Postgres, PgArguments>, String> {
        self.0.query_bind(query)
    }
    fn query_part(
        &self,
        increment: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        self.0
            .query_part(increment, column, is_need_to_add_logical_operator)
    }
}
impl DefaultOptionSomeVecOneEl for PaginationStartsWithZero {
    #[inline]
    fn default_option_some_vec_one_el() -> Self {
        Self(PaginationBase::new_unchecked(DEFAULT_PAGINATION_LIMIT, 0))
    }
}
impl DefaultOptionSomeVecOneElMaxPageSize for PaginationStartsWithZero {
    #[inline]
    fn default_option_some_vec_one_el_max_page_size() -> Self {
        Self(PaginationBase::new_unchecked(i32::MAX.into(), 0))
    }
}

//this needed coz serde Option<Option<T>> #[serde(skip_serializing_if = "Option::is_none")] - if both options: inner and parent is null then it skip - its not correct
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
)]
pub struct Value<T> {
    pub value: T,
}

//todo ExactSizeIterator now is not a solution. error[E0658]: use of unstable library feature `exact_size_is_empty`. maybe rewrite it later
pub trait IsStringEmpty {
    fn is_string_empty(&self) -> bool;
}

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
)]
pub enum NotEmptyUniqueVecTryNewErrorNamed<T> {
    IsEmpty {
        code_occurence: CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: T,
        code_occurence: CodeOccurence,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct NotEmptyUniqueVec<T>(Vec<T>);
impl<T: PartialEq + Clone> NotEmptyUniqueVec<T> {
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
    #[must_use]
    pub const fn to_vec(&self) -> &Vec<T> {
        &self.0
    }
    pub fn try_new(value: Vec<T>) -> Result<Self, NotEmptyUniqueVecTryNewErrorNamed<T>> {
        if value.is_empty() {
            return Err(NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        {
            let mut acc_11fac69e = Vec::new();
            for el_db9bd5a0 in &value {
                if acc_11fac69e.contains(&el_db9bd5a0) {
                    return Err(NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                        value: el_db9bd5a0.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
                acc_11fac69e.push(el_db9bd5a0);
            }
        }
        Ok(Self(value))
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: Debug + PartialEq + Clone + _serde::Deserialize<'de>> _serde::Deserialize<'de>
        for NotEmptyUniqueVec<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private228::PhantomData<NotEmptyUniqueVec<T>>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: Debug + PartialEq + Clone + _serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = NotEmptyUniqueVec<T>;
                fn expecting(&self, __f: &mut Formatter<'_>) -> _serde::__private228::fmt::Result {
                    Formatter::write_str(__f, "tuple struct NotEmptyUniqueVec")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<T> = <Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    Ok(NotEmptyUniqueVec(__field0))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct NotEmptyUniqueVec with 1 element",
                        ));
                    };
                    match NotEmptyUniqueVec::try_new(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "NotEmptyUniqueVec",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<T: AllEnumVariantsArrayDefaultOptionSomeVecOneEl> DefaultOptionSomeVecOneEl
    for NotEmptyUniqueVec<T>
{
    fn default_option_some_vec_one_el() -> Self {
        Self(AllEnumVariantsArrayDefaultOptionSomeVecOneEl::all_variants_default_option_some_vec_one_el())
    }
}
impl<T: AllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize>
    DefaultOptionSomeVecOneElMaxPageSize for NotEmptyUniqueVec<T>
{
    fn default_option_some_vec_one_el_max_page_size() -> Self {
        Self(AllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize::all_variants_default_option_some_vec_one_el_max_page_size())
    }
}
impl<T> Default for NotEmptyUniqueVec<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
impl<T> From<NotEmptyUniqueVec<T>> for Vec<T> {
    fn from(value: NotEmptyUniqueVec<T>) -> Self {
        value.0
    }
}
impl<T1> NotEmptyUniqueVec<T1> {
    pub fn from_t1_impl_from_t2<T2: From<T1>>(value: Self) -> NotEmptyUniqueVec<T2> {
        NotEmptyUniqueVec(value.0.into_iter().map(T2::from).collect::<Vec<T2>>())
    }
}
impl<'query_lifetime, T> PostgresqlTypeWhereFilter<'query_lifetime> for NotEmptyUniqueVec<T>
where
    T: Debug
        + PartialEq
        + Clone
        + for<'t_lifetime> PostgresqlTypeWhereFilter<'t_lifetime>
        + AllEnumVariantsArrayDefaultOptionSomeVecOneEl,
{
    fn query_bind(
        self,
        mut query: Query<'query_lifetime, Postgres, PgArguments>,
    ) -> Result<Query<'query_lifetime, Postgres, PgArguments>, String> {
        for el_7f5ffb83 in self.0 {
            match el_7f5ffb83.query_bind(query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(query)
    }
    fn query_part(
        &self,
        increment: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        let mut acc_57b31116 = String::default();
        for (index, value_953208ce) in self.0.iter().enumerate() {
            match value_953208ce.query_part(
                increment,
                column,
                if index == 0 {
                    is_need_to_add_logical_operator
                } else {
                    true
                },
            ) {
                Ok(value) => {
                    acc_57b31116.push_str(&value);
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(acc_57b31116)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct JsonFieldRights {
    can_create: bool,
    can_read: bool,
    can_update: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NonPrimaryKeyPostgresqlTypeReadOnlyIds(pub Value<Option<()>>);
impl Decode<'_, Postgres> for NonPrimaryKeyPostgresqlTypeReadOnlyIds {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        match <Json<Self> as Decode<Postgres>>::decode(value) {
            Ok(decode_value) => Ok(decode_value.0),
            Err(error) => Err(error),
        }
    }
}
impl Type<Postgres> for NonPrimaryKeyPostgresqlTypeReadOnlyIds {
    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        <Json<Self> as Type<Postgres>>::compatible(ty)
    }
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <Json<Self> as Type<Postgres>>::type_info()
    }
}
impl Default for NonPrimaryKeyPostgresqlTypeReadOnlyIds {
    fn default() -> Self {
        Self(Value { value: None })
    }
}
#[derive(Debug, Clone, Copy)]
pub enum EqualOperator {
    Equal,
    IsNull,
}
impl EqualOperator {
    #[must_use]
    pub const fn to_query_str(&self) -> &'static str {
        match &self {
            Self::Equal => "=",
            Self::IsNull => "is null",
        }
    }
}
pub trait PostgresqlTypeEqualOperator {
    fn operator(&self) -> EqualOperator;
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, serde::Serialize, schemars::JsonSchema,
)]
pub struct UnsignedPartOfStdPrimitiveI32(i32); //todo why exactly i32? maybe different types for postgresql type and postgresql json type
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
    schemars::JsonSchema,
)]
pub enum UnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed {
    LessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        value: i32,
        code_occurence: CodeOccurence,
    },
}
impl TryFrom<i32> for UnsignedPartOfStdPrimitiveI32 {
    type Error = UnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value))
        } else {
            Err(Self::Error::LessThanZero {
                value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for UnsignedPartOfStdPrimitiveI32 {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<UnsignedPartOfStdPrimitiveI32>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = UnsignedPartOfStdPrimitiveI32;
                fn expecting(
                    &self,
                    __formatter: &mut Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    Formatter::write_str(__formatter, "tuple struct UnsignedPartOfStdPrimitiveI32")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: i32 = <i32 as _serde::Deserialize>::deserialize(__e)?;
                    match UnsignedPartOfStdPrimitiveI32::try_from(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = _serde::de::SeqAccess::next_element::<i32>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct UnsignedPartOfStdPrimitiveI32 with 1 element",
                        ));
                    };
                    match UnsignedPartOfStdPrimitiveI32::try_from(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "UnsignedPartOfStdPrimitiveI32",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl error_occurence_lib::ToStdStringString for UnsignedPartOfStdPrimitiveI32 {
    fn to_std_string_string(&self) -> String {
        self.0.to_string()
    }
}
impl Type<Postgres> for UnsignedPartOfStdPrimitiveI32 {
    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        <i32 as Type<Postgres>>::compatible(ty)
    }
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <i32 as Type<Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, Postgres> for UnsignedPartOfStdPrimitiveI32 {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn StdErrorError + Send + Sync>> {
        <i32 as sqlx::Encode<Postgres>>::encode_by_ref(&self.0, buf)
    }
}
impl UnsignedPartOfStdPrimitiveI32 {
    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0
    }
}
impl DefaultOptionSomeVecOneEl for UnsignedPartOfStdPrimitiveI32 {
    fn default_option_some_vec_one_el() -> Self {
        Self(0)
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, serde::Serialize, schemars::JsonSchema,
)]
pub struct NotZeroUnsignedPartOfStdPrimitiveI32(UnsignedPartOfStdPrimitiveI32);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
    schemars::JsonSchema,
)]
pub enum NotZeroUnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed {
    IsZero {
        code_occurence: CodeOccurence,
    },
    UnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed {
        #[eo_error_occurence]
        value: UnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed,
        code_occurence: CodeOccurence,
    },
}
impl TryFrom<i32> for NotZeroUnsignedPartOfStdPrimitiveI32 {
    type Error = NotZeroUnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match UnsignedPartOfStdPrimitiveI32::try_from(value) {
            Ok(handle) => {
                if handle.0 == 0 {
                    Err(Self::Error::IsZero {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                } else {
                    Ok(Self(handle))
                }
            }
            Err(error) => Err(
                Self::Error::UnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed {
                    value: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                },
            ),
        }
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for NotZeroUnsignedPartOfStdPrimitiveI32 {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<NotZeroUnsignedPartOfStdPrimitiveI32>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = NotZeroUnsignedPartOfStdPrimitiveI32;
                fn expecting(
                    &self,
                    __formatter: &mut Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    Formatter::write_str(
                        __formatter,
                        "tuple struct NotZeroUnsignedPartOfStdPrimitiveI32",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: i32 = <i32 as _serde::Deserialize>::deserialize(__e)?;
                    match NotZeroUnsignedPartOfStdPrimitiveI32::try_from(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = _serde::de::SeqAccess::next_element::<i32>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct NotZeroUnsignedPartOfStdPrimitiveI32 with 1 element",
                        ));
                    };
                    match NotZeroUnsignedPartOfStdPrimitiveI32::try_from(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "NotZeroUnsignedPartOfStdPrimitiveI32",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl error_occurence_lib::ToStdStringString for NotZeroUnsignedPartOfStdPrimitiveI32 {
    fn to_std_string_string(&self) -> String {
        self.0.to_std_string_string()
    }
}
impl Type<Postgres> for NotZeroUnsignedPartOfStdPrimitiveI32 {
    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        <UnsignedPartOfStdPrimitiveI32 as Type<Postgres>>::compatible(ty)
    }
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <UnsignedPartOfStdPrimitiveI32 as Type<Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, Postgres> for NotZeroUnsignedPartOfStdPrimitiveI32 {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn StdErrorError + Send + Sync>> {
        <UnsignedPartOfStdPrimitiveI32 as sqlx::Encode<Postgres>>::encode_by_ref(&self.0, buf)
    }
}
impl NotZeroUnsignedPartOfStdPrimitiveI32 {
    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0.get()
    }
}
impl DefaultOptionSomeVecOneEl for NotZeroUnsignedPartOfStdPrimitiveI32 {
    fn default_option_some_vec_one_el() -> Self {
        Self(DefaultOptionSomeVecOneEl::default_option_some_vec_one_el())
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
)]
pub enum SingleOrMultiple<T: Debug + PartialEq + Clone> {
    Multiple(NotEmptyUniqueVec<T>),
    Single(T),
}
pub fn increment_checked_add_one_returning_increment(
    increment: &mut u64,
) -> Result<u64, QueryPartErrorNamed> {
    increment.checked_add(1).map_or_else(
        || {
            Err(QueryPartErrorNamed::CheckedAdd {
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        },
        |value_d25735be| {
            *increment = value_d25735be;
            Ok(value_d25735be)
        },
    )
}
#[must_use]
pub const fn std_primitive_i8_test_cases_vec() -> [i8; 3] {
    [i8::MIN, 0, i8::MAX]
}
#[must_use]
pub const fn std_primitive_i16_test_cases_vec() -> [i16; 3] {
    [i16::MIN, 0, i16::MAX]
}
#[must_use]
pub const fn std_primitive_i32_test_cases_vec() -> [i32; 3] {
    [i32::MIN, 0, i32::MAX]
}
#[must_use]
pub const fn std_primitive_i64_test_cases_vec() -> [i64; 3] {
    [i64::MIN, 0, i64::MAX]
}
#[must_use]
pub const fn std_primitive_u8_test_cases_vec() -> [u8; 3] {
    [u8::MIN, 0, u8::MAX]
}
#[must_use]
pub const fn std_primitive_u16_test_cases_vec() -> [u16; 3] {
    [u16::MIN, 0, u16::MAX]
}
#[must_use]
pub const fn std_primitive_u32_test_cases_vec() -> [u32; 3] {
    [u32::MIN, 0, u32::MAX]
}
#[must_use]
pub const fn std_primitive_u64_test_cases_vec() -> [u64; 3] {
    [u64::MIN, 0, u64::MAX]
}
#[must_use]
pub const fn std_primitive_f32_test_cases_vec() -> [f32; 18] {
    [
        f32::EPSILON,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        -1e30,
        -1e-30,
        -16_777_214.0,
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        16_777_214.0,
        1e-30,
        1e30,
    ]
}
#[must_use]
pub const fn std_primitive_f64_test_cases_vec() -> [f64; 18] {
    [
        f64::EPSILON,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        -1e300,
        -1e-300,
        -9_007_199_254_740_990.0,
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        9_007_199_254_740_990.0,
        1e-300,
        1e300,
    ]
}
#[must_use]
pub const fn std_primitive_bool_test_cases_vec() -> [bool; 2] {
    [true, false]
}
#[must_use]
pub fn std_string_string_test_cases_vec() -> [String; 12] {
    #[allow(clippy::non_ascii_literal)]
    [
        String::new(),
        "a".to_owned(),
        "Hello, world!".to_owned(),
        "   ".to_owned(),
        "\n\r\t".to_owned(),
        "1234567890".to_owned(),
        "😀".to_owned(),
        "こんにちは".to_owned(),
        "🌍🚀✨ Rust 💖🦀".to_owned(),
        "a".repeat(1024),
        "line1\nline2\nline3".to_owned(),
        String::from_utf8_lossy(&[0xF0, 0x9F, 0x92, 0x96]).to_string(),
    ]
}
#[must_use]
pub fn uuid_uuid_test_cases_vec() -> [uuid::Uuid; 1] {
    [uuid::Uuid::new_v4()]
}
#[must_use]
pub fn wrap_into_jsonb_build_object(field: &str, value: &str) -> String {
    format!("jsonb_build_object('{field}',{value})||")
}
