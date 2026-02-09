#[derive(Debug, Clone, Copy)]
# [postgresql_crud :: postgresql_json_object_type_config { { "postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_json_object_types" : "False" , "whole_content_write_into_generate_postgresql_json_object_type" : "False" , "variant" : { "not_null_or_nullable" : "Nullable" , "pattern" : "Array" , "trait_gen" : "PostgresqlTypeAndPostgresqlJsonType" } } }]
pub struct ObjectExample {
    pub field_0: postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber,
    pub field_1: postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber,
    pub field_2: postgresql_crud::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
}
#[derive(Debug, Clone, Copy)]
pub struct ObjectExampleAsNotNullJsonbObject;
#[derive(Debug, Clone, Copy)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithId;
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
impl ObjectExampleWithIdAsNotNullJsonbObjectWithId {
    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids: ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIds,
        create: ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate,
    ) -> ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere {
        ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere :: Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration :: new (< postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids . 0. value . id , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ()) , < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids . 0. value . field_0 , create . field_0) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids . 0. value . field_1 , create . field_1) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids . 0. value . field_2 , create . field_2)) , })
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids: ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIds,
        create: ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate,
    ) -> postgresql_crud::NotEmptyUniqueVec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere>
    {
        postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere :: Id (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids . 0. value . id , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ()) ,) ,) , ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids . 0. value . field_0 , create . field_0) ,) ,) , ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids . 0. value . field_1 , create . field_1) ,) ,) , ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids . 0. value . field_2 , create . field_2) ,) ,)]) . expect ("5473d8c4-2b10-4ba8-8a4a-704fde84f6ff")
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_to_json_field(
        read_only_ids: ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIds,
        create: ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate,
    ) -> postgresql_crud::NotEmptyUniqueVec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere>
    {
        postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere :: Id (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: Or , < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids . 0. value . id , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ()) ,) ,) , ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: Or , < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids . 0. value . field_0 , create . field_0) ,) ,) , ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: Or , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids . 0. value . field_1 , create . field_1) ,) ,) , ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: Or , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids . 0. value . field_2 , create . field_2) ,) ,)]) . expect ("221a4c55-5467-44f1-94bb-b748a92cfada")
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration { field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration }
impl ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration {
    #[must_use]
    pub const fn new(
        field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration,
        field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration,
        field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration,
    ) -> Self {
        Self {
            field_0,
            field_1,
            field_2,
        }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration
{
    fn default_option_some_vec_one_el() -> Self {
        Self {
            field_0: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            field_1: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            field_2: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        }
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration { id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration , field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration }
impl ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    #[must_use]
    pub const fn new(
        id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration,
        field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration,
        field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration,
        field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration,
    ) -> Self {
        Self {
            id,
            field_0,
            field_1,
            field_2,
        }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration
{
    fn default_option_some_vec_one_el() -> Self {
        Self {
            id: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            field_0: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            field_1: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            field_2: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        }
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectExampleAsNotNullJsonbObjectCreate { field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create }
impl ObjectExampleAsNotNullJsonbObjectCreate {
    #[must_use]
    pub const fn new(
        field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create,
        field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create,
        field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create,
    ) -> Self {
        Self {
            field_0,
            field_1,
            field_2,
        }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl for ObjectExampleAsNotNullJsonbObjectCreate {
    fn default_option_some_vec_one_el() -> Self {
        Self {
            field_0: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            field_1: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            field_2: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        }
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectCreate {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectCreate {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate { field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create }
impl ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate {
    #[must_use]
    pub const fn new(
        field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create,
        field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create,
        field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Create,
    ) -> Self {
        Self {
            field_0,
            field_1,
            field_2,
        }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate
{
    fn default_option_some_vec_one_el() -> Self {
        Self {
            field_0: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            field_1: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            field_2: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        }
    }
}
impl std::fmt::Display for ObjectExampleAsNotNullJsonbObjectCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for ObjectExampleAsNotNullJsonbObjectCreate {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl std::fmt::Display for ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate
{
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct ObjectExampleAsNotNullJsonbObjectCreateForQuery { field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery }
impl From<ObjectExampleAsNotNullJsonbObjectCreate>
    for ObjectExampleAsNotNullJsonbObjectCreateForQuery
{
    fn from(value: ObjectExampleAsNotNullJsonbObjectCreate) -> Self {
        Self { field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery :: from (value . field_0) , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery :: from (value . field_1) , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery :: from (value . field_2) }
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectCreateForQuery {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectCreateForQuery {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreateForQuery { id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery , field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery }
impl From<ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate>
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreateForQuery
{
    fn from(value: ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate) -> Self {
        Self { id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery :: new (uuid :: Uuid :: new_v4 ()) , field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery :: from (value . field_0) , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery :: from (value . field_1) , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery :: from (value . field_2) }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectExampleAsNotNullJsonbObjectSelect(
    postgresql_crud::NotEmptyUniqueVec<ObjectExampleAsNotNullJsonbObjectSelectElement>,
);
impl ObjectExampleAsNotNullJsonbObjectSelect {
    #[must_use]
    pub const fn new(
        value: postgresql_crud::NotEmptyUniqueVec<ObjectExampleAsNotNullJsonbObjectSelectElement>,
    ) -> Self {
        Self(value)
    }
    fn select_query_part(
        &self,
        column_name_and_maybe_field_getter: &str,
        column_name_and_maybe_field_getter_for_error_message: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc_ac57d097 = String::default();
        for el_0127bf54 in self.0.to_vec() {
            if { use std :: fmt :: Write as _ ; write ! (acc_ac57d097 , "{}||" , match el_0127bf54 { ObjectExampleAsNotNullJsonbObjectSelectElement :: Field0 (value_3c8acf6a) => match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "field_0" , column_name_and_maybe_field_getter , column_name_and_maybe_field_getter_for_error_message , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } , ObjectExampleAsNotNullJsonbObjectSelectElement :: Field1 (value_3c8acf6a) => match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "field_1" , column_name_and_maybe_field_getter , column_name_and_maybe_field_getter_for_error_message , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } , ObjectExampleAsNotNullJsonbObjectSelectElement :: Field2 (value_3c8acf6a) => match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "field_2" , column_name_and_maybe_field_getter , column_name_and_maybe_field_getter_for_error_message , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } }) } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
        }
        let _: Option<char> = acc_ac57d097.pop();
        let _: Option<char> = acc_ac57d097.pop();
        Ok(acc_ac57d097)
    }
    fn select_query_part_postgresql_type(
        &self,
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        self.select_query_part(column, column)
    }
}
impl sqlx::Type<sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectSelect {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl for ObjectExampleAsNotNullJsonbObjectSelect {
    fn default_option_some_vec_one_el() -> Self {
        Self(postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el())
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneElMaxPageSize
    for ObjectExampleAsNotNullJsonbObjectSelect
{
    fn default_option_some_vec_one_el_max_page_size() -> Self {
        Self (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ())
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub enum ObjectExampleAsNotNullJsonbObjectSelectElement {
    # [serde (rename (serialize = "field_0" , deserialize = "field_0"))] Field0 (< postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Select) , # [serde (rename (serialize = "field_1" , deserialize = "field_1"))] Field1 (< postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Select) , # [serde (rename (serialize = "field_2" , deserialize = "field_2"))] Field2 (< postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Select) }
impl error_occurence_lib::ToStdStringString for ObjectExampleAsNotNullJsonbObjectSelectElement {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultOptionSomeVecOneEl
    for ObjectExampleAsNotNullJsonbObjectSelectElement
{
    fn all_variants_default_option_some_vec_one_el() -> Vec<Self> {
        vec![
            Self::Field0(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::Field1(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::Field2(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
        ]
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize
    for ObjectExampleAsNotNullJsonbObjectSelectElement
{
    fn all_variants_default_option_some_vec_one_el_max_page_size() -> Vec<Self> {
        vec ! [Self :: Field0 (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ()) , Self :: Field1 (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ()) , Self :: Field2 (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ())]
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelect(
    postgresql_crud::NotEmptyUniqueVec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement>,
);
impl ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelect {
    #[must_use]
    pub const fn new(
        value: postgresql_crud::NotEmptyUniqueVec<
            ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement,
        >,
    ) -> Self {
        Self(value)
    }
}
impl sqlx::Type<sqlx::Postgres> for ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelect {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelect
{
    fn default_option_some_vec_one_el() -> Self {
        Self(postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el())
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneElMaxPageSize
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelect
{
    fn default_option_some_vec_one_el_max_page_size() -> Self {
        Self (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ())
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub enum ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement {
    # [serde (rename (serialize = "id" , deserialize = "id"))] Id (< postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Select) , # [serde (rename (serialize = "field_0" , deserialize = "field_0"))] Field0 (< postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Select) , # [serde (rename (serialize = "field_1" , deserialize = "field_1"))] Field1 (< postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Select) , # [serde (rename (serialize = "field_2" , deserialize = "field_2"))] Field2 (< postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Select) }
impl error_occurence_lib::ToStdStringString
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement
{
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultOptionSomeVecOneEl
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement
{
    fn all_variants_default_option_some_vec_one_el() -> Vec<Self> {
        vec![
            Self::Id(postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el()),
            Self::Field0(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::Field1(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::Field2(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
        ]
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement
{
    fn all_variants_default_option_some_vec_one_el_max_page_size() -> Vec<Self> {
        vec ! [Self :: Id (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ()) , Self :: Field0 (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ()) , Self :: Field1 (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ()) , Self :: Field2 (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ())]
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub enum ObjectExampleAsNotNullJsonbObjectWhere {
    Field0 (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Where >) , Field1 (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Where >) , Field2 (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Where >) , Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual < < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration >) , }
#[allow(clippy::arbitrary_source_item_ordering)]
impl<'lifetime> postgresql_crud::PostgresqlTypeWhereFilter<'lifetime>
    for ObjectExampleAsNotNullJsonbObjectWhere
{
    fn query_part(
        &self,
        increment: &mut u64,
        column: &dyn std::fmt::Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Field0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                &format!("{column}->'field_0'"),
                is_need_to_add_logical_operator,
            ),
            Self::Field1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                &format!("{column}->'field_1'"),
                is_need_to_add_logical_operator,
            ),
            Self::Field2(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                &format!("{column}->'field_2'"),
                is_need_to_add_logical_operator,
            ),
            Self::Equal(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                &column,
                is_need_to_add_logical_operator,
            ),
        }
    }
    fn query_bind(
        self,
        query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        match self {
            Self::Field0(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::Field1(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::Field2(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::Equal(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
        }
    }
}
impl error_occurence_lib::ToStdStringString for ObjectExampleAsNotNullJsonbObjectWhere {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultOptionSomeVecOneEl
    for ObjectExampleAsNotNullJsonbObjectWhere
{
    fn all_variants_default_option_some_vec_one_el() -> Vec<Self> {
        vec![
            Self::Field0(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::Field1(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::Field2(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::Equal(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
        ]
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub enum ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere {
    Id (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Where >) , Field0 (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Where >) , Field1 (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Where >) , Field2 (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Where >) , Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual < ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration >) , }
#[allow(clippy::arbitrary_source_item_ordering)]
impl<'lifetime> postgresql_crud::PostgresqlTypeWhereFilter<'lifetime>
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere
{
    fn query_part(
        &self,
        increment: &mut u64,
        column: &dyn std::fmt::Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                &format!("{column}->'id'"),
                is_need_to_add_logical_operator,
            ),
            Self::Field0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                &format!("{column}->'field_0'"),
                is_need_to_add_logical_operator,
            ),
            Self::Field1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                &format!("{column}->'field_1'"),
                is_need_to_add_logical_operator,
            ),
            Self::Field2(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                &format!("{column}->'field_2'"),
                is_need_to_add_logical_operator,
            ),
            Self::Equal(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                &column,
                is_need_to_add_logical_operator,
            ),
        }
    }
    fn query_bind(
        self,
        query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        match self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field0(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::Field1(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::Field2(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::Equal(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
        }
    }
}
impl error_occurence_lib::ToStdStringString for ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultOptionSomeVecOneEl
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdWhere
{
    fn all_variants_default_option_some_vec_one_el() -> Vec<Self> {
        vec![
            Self::Id(postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el()),
            Self::Field0(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::Field1(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::Field2(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::Equal(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
        ]
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema,
)]
pub struct ObjectExampleAsNotNullJsonbObjectRead { # [serde (skip_serializing_if = "Option::is_none")] field_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > , # [serde (skip_serializing_if = "Option::is_none")] field_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > , # [serde (skip_serializing_if = "Option::is_none")] field_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > }
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum ObjectExampleAsNotNullJsonbObjectReadTryFromErrorNamed {
    AllFieldsAreNone {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl ObjectExampleAsNotNullJsonbObjectRead {
    pub fn try_new(
        field_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > >,
        field_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > >,
        field_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > >,
    ) -> Result<Self, ObjectExampleAsNotNullJsonbObjectReadTryFromErrorNamed> {
        if matches!((&field_0, &field_1, &field_2), (None, None, None)) {
            return Err(
                ObjectExampleAsNotNullJsonbObjectReadTryFromErrorNamed::AllFieldsAreNone {
                    code_occurence: error_occurence_lib::code_occurence!(),
                },
            );
        }
        Ok(Self {
            field_0,
            field_1,
            field_2,
        })
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ObjectExampleAsNotNullJsonbObjectRead {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => Ok(__Field::__field0),
                        1u64 => Ok(__Field::__field1),
                        2u64 => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "field_0" => Ok(__Field::__field0),
                        "field_1" => Ok(__Field::__field1),
                        "field_2" => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"field_0" => Ok(__Field::__field0),
                        b"field_1" => Ok(__Field::__field1),
                        b"field_2" => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
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
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<ObjectExampleAsNotNullJsonbObjectRead>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ObjectExampleAsNotNullJsonbObjectRead;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct ObjectExampleAsNotNullJsonbObjectRead",
                    )
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some (__field0_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct ObjectExampleAsNotNullJsonbObjectRead with 3 elements")) ; } ;
                    let Some (__field1_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct ObjectExampleAsNotNullJsonbObjectRead with 3 elements")) ; } ;
                    let Some (__field2_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct ObjectExampleAsNotNullJsonbObjectRead with 3 elements")) ; } ;
                    match ObjectExampleAsNotNullJsonbObjectRead::try_new(
                        __field0_handle,
                        __field1_handle,
                        __field2_handle,
                    ) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0 : Option < Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > = None ;
                    let mut __field1 : Option < Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > = None ;
                    let mut __field2 : Option < Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > = None ;
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "field_0",
                                    ));
                                }
                                __field0 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __map) ? ,) ;
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "field_1",
                                    ));
                                }
                                __field1 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __map) ? ,) ;
                            }
                            __Field::__field2 => {
                                if Option::is_some(&__field2) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "field_2",
                                    ));
                                }
                                __field2 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __map) ? ,) ;
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __map,
                                    )?;
                            }
                        }
                    }
                    let __field0_handle = match __field0 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("field_0")?,
                    };
                    let __field1_handle = match __field1 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("field_1")?,
                    };
                    let __field2_handle = match __field2 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("field_2")?,
                    };
                    match ObjectExampleAsNotNullJsonbObjectRead::try_new(
                        __field0_handle,
                        __field1_handle,
                        __field2_handle,
                    ) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["field_0", "field_1", "field_2"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ObjectExampleAsNotNullJsonbObjectRead",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultOptionSomeVecOneEl for ObjectExampleAsNotNullJsonbObjectRead {
    fn default_option_some_vec_one_el() -> Self {
        Self {
            field_0: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            }),
            field_1: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            }),
            field_2: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            }),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectRead {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectRead {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema,
)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead { # [serde (skip_serializing_if = "Option::is_none")] id : Option < postgresql_crud :: Value < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Read > > , # [serde (skip_serializing_if = "Option::is_none")] field_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > , # [serde (skip_serializing_if = "Option::is_none")] field_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > , # [serde (skip_serializing_if = "Option::is_none")] field_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > }
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed {
    AllFieldsAreNone {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead {
    pub fn try_new(
        id : Option < postgresql_crud :: Value < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Read > >,
        field_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > >,
        field_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > >,
        field_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > >,
    ) -> Result<Self, ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed> {
        if matches!(
            (&id, &field_0, &field_1, &field_2),
            (None, None, None, None)
        ) {
            return Err (ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed :: AllFieldsAreNone { code_occurence : error_occurence_lib :: code_occurence ! () }) ;
        }
        Ok(Self {
            id,
            field_0,
            field_1,
            field_2,
        })
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => Ok(__Field::__field0),
                        1u64 => Ok(__Field::__field1),
                        2u64 => Ok(__Field::__field2),
                        3u64 => Ok(__Field::__field3),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => Ok(__Field::__field0),
                        "field_0" => Ok(__Field::__field1),
                        "field_1" => Ok(__Field::__field2),
                        "field_2" => Ok(__Field::__field3),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => Ok(__Field::__field0),
                        b"field_0" => Ok(__Field::__field1),
                        b"field_1" => Ok(__Field::__field2),
                        b"field_2" => Ok(__Field::__field3),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
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
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<
                    ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead,
                >,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead",
                    )
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some (__field0_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: Value < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead with 4 elements")) ; } ;
                    let Some (__field1_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead with 4 elements")) ; } ;
                    let Some (__field2_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead with 4 elements")) ; } ;
                    let Some (__field3_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead with 4 elements")) ; } ;
                    match ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead::try_new(
                        __field0_handle,
                        __field1_handle,
                        __field2_handle,
                        __field3_handle,
                    ) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0 : Option < Option < postgresql_crud :: Value < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Read > > > = None ;
                    let mut __field1 : Option < Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > = None ;
                    let mut __field2 : Option < Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > = None ;
                    let mut __field3 : Option < Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > = None ;
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "id",
                                    ));
                                }
                                __field0 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: Value < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __map) ? ,) ;
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "field_0",
                                    ));
                                }
                                __field1 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __map) ? ,) ;
                            }
                            __Field::__field2 => {
                                if Option::is_some(&__field2) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "field_1",
                                    ));
                                }
                                __field2 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __map) ? ,) ;
                            }
                            __Field::__field3 => {
                                if Option::is_some(&__field3) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "field_2",
                                    ));
                                }
                                __field3 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read > > > (& mut __map) ? ,) ;
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __map,
                                    )?;
                            }
                        }
                    }
                    let __field0_handle = match __field0 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("id")?,
                    };
                    let __field1_handle = match __field1 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("field_0")?,
                    };
                    let __field2_handle = match __field2 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("field_1")?,
                    };
                    let __field3_handle = match __field3 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("field_2")?,
                    };
                    match ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead::try_new(
                        __field0_handle,
                        __field1_handle,
                        __field2_handle,
                        __field3_handle,
                    ) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["id", "field_0", "field_1", "field_2"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead
{
    fn default_option_some_vec_one_el() -> Self {
        Self {
            id: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            }),
            field_0: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            }),
            field_1: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            }),
            field_2: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            }),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ObjectExampleAsNotNullJsonbObjectReadOnlyIdsHandle { field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ObjectExampleAsNotNullJsonbObjectReadOnlyIds(
    postgresql_crud::Value<ObjectExampleAsNotNullJsonbObjectReadOnlyIdsHandle>,
);
impl sqlx::Decode<'_, sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for ObjectExampleAsNotNullJsonbObjectReadOnlyIds {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle { id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds , field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIds(
    pub postgresql_crud::Value<ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle>,
);
impl sqlx::Decode<'_, sqlx::Postgres> for ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectExampleAsNotNullJsonbObjectReadInner { field_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadInner > > , field_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadInner > > , field_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadInner > > }
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadInner { id : Option < postgresql_crud :: Value < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: ReadInner > > , field_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadInner > > , field_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadInner > > , field_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: ReadInner > > }
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectExampleAsNotNullJsonbObjectUpdate(
    postgresql_crud::NotEmptyUniqueVec<ObjectExampleAsNotNullJsonbObjectUpdateElement>,
);
impl ObjectExampleAsNotNullJsonbObjectUpdate {
    #[must_use]
    pub const fn new(
        value: postgresql_crud::NotEmptyUniqueVec<ObjectExampleAsNotNullJsonbObjectUpdateElement>,
    ) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl for ObjectExampleAsNotNullJsonbObjectUpdate {
    fn default_option_some_vec_one_el() -> Self {
        Self(postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el())
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub enum ObjectExampleAsNotNullJsonbObjectUpdateElement {
    # [serde (rename (serialize = "field_0" , deserialize = "field_0"))] Field0 (postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Update >) , # [serde (rename (serialize = "field_1" , deserialize = "field_1"))] Field1 (postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Update >) , # [serde (rename (serialize = "field_2" , deserialize = "field_2"))] Field2 (postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Update >) }
impl postgresql_crud::AllEnumVariantsArrayDefaultOptionSomeVecOneEl
    for ObjectExampleAsNotNullJsonbObjectUpdateElement
{
    fn all_variants_default_option_some_vec_one_el() -> Vec<Self> {
        vec![
            Self::Field0(postgresql_crud::Value {
                value: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            }),
            Self::Field1(postgresql_crud::Value {
                value: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            }),
            Self::Field2(postgresql_crud::Value {
                value: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            }),
        ]
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement { id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Update , fields : < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: Update }
impl ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    #[must_use]
    pub const fn new(
        id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Update,
        fields: <ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update,
    ) -> Self {
        Self { id, fields }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement
{
    fn default_option_some_vec_one_el() -> Self {
        Self {
            id: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            fields: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct ObjectExampleAsNotNullJsonbObjectUpdateForQuery(
    postgresql_crud::NotEmptyUniqueVec<ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement>,
);
impl ObjectExampleAsNotNullJsonbObjectUpdateForQuery {
    #[allow(clippy::single_call_fn)]
    fn select_only_updated_ids_query_part(
        &self,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc_8e628eaf = String::default();
        for el_0963b7df in self.0.to_vec() {
            match & el_0963b7df { ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field0 (value) => { match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_part (& value . value , "field_0" , column_name_and_maybe_field_getter , increment) { Ok (mut value_c3ae3be4) => { let _ : Option < char > = value_c3ae3be4 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_8e628eaf , "jsonb_build_object({value_c3ae3be4})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field1 (value) => { match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_part (& value . value , "field_1" , column_name_and_maybe_field_getter , increment) { Ok (mut value_c3ae3be4) => { let _ : Option < char > = value_c3ae3be4 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_8e628eaf , "jsonb_build_object({value_c3ae3be4})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field2 (value) => { match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_part (& value . value , "field_2" , column_name_and_maybe_field_getter , increment) { Ok (mut value_c3ae3be4) => { let _ : Option < char > = value_c3ae3be4 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_8e628eaf , "jsonb_build_object({value_c3ae3be4})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } } } }
        }
        let _: Option<char> = acc_8e628eaf.pop();
        let _: Option<char> = acc_8e628eaf.pop();
        Ok(acc_8e628eaf)
    }
}
impl From<<ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update>
    for <ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::UpdateForQuery
{
    fn from(
        value: <ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update,
    ) -> Self {
        Self(postgresql_crud::NotEmptyUniqueVec::from_t1_impl_from_t2(
            value.0,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub enum ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement {
    # [serde (rename (serialize = "field_0" , deserialize = "field_0"))] Field0 (postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery >) , # [serde (rename (serialize = "field_1" , deserialize = "field_1"))] Field1 (postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery >) , # [serde (rename (serialize = "field_2" , deserialize = "field_2"))] Field2 (postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery >) }
impl From<ObjectExampleAsNotNullJsonbObjectUpdateElement>
    for ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement
{
    fn from(value: ObjectExampleAsNotNullJsonbObjectUpdateElement) -> Self {
        match value { ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field0 (value_121f1c54) => Self :: Field0 (postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery :: from (value_121f1c54 . value) }) , ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field1 (value_121f1c54) => Self :: Field1 (postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery :: from (value_121f1c54 . value) }) , ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field2 (value_121f1c54) => Self :: Field2 (postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery :: from (value_121f1c54 . value) }) }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateForQueryElement { id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery , fields : < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery }
impl ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateForQueryElement {
    #[must_use]
    pub const fn new(
        id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery,
        fields : < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery,
    ) -> Self {
        Self { id, fields }
    }
}
impl From<ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement>
    for ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateForQueryElement
{
    fn from(value: ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement) -> Self {
        Self { id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery :: from (value . id) , fields : < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery :: from (value . fields) , }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlJsonType for ObjectExampleAsNotNullJsonbObject {
    type TableTypeDeclaration = ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = ObjectExampleAsNotNullJsonbObjectCreate;
    type CreateForQuery = ObjectExampleAsNotNullJsonbObjectCreateForQuery;
    type Select = ObjectExampleAsNotNullJsonbObjectSelect;
    fn select_query_part(
        value: &Self::Select,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        column_name_and_maybe_field_getter_for_error_message: &str,
        is_postgresql_type: bool,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_query_part(
            &if is_postgresql_type {
                column_name_and_maybe_field_getter.to_owned()
            } else {
                format!("{column_name_and_maybe_field_getter}->'{field_ident}'")
            },
            &format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}"),
        ) {
            Ok(value_156121ad) => Ok(if is_postgresql_type {
                value_156121ad
            } else {
                format!(
                    "jsonb_build_object('{field_ident}',jsonb_build_object('value',{value_156121ad}))"
                )
            }),
            Err(error) => Err(error),
        }
    }
    type Where = ObjectExampleAsNotNullJsonbObjectWhere;
    type Read = ObjectExampleAsNotNullJsonbObjectRead;
    type ReadOnlyIds = ObjectExampleAsNotNullJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(
        column_name_and_maybe_field_getter: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok({
            let mut acc_2912b128 = String::default();
            if { use std :: fmt :: Write as _ ; write ! (acc_2912b128 , "jsonb_build_object('field_0',{})||" , match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_ids_query_part (& format ! ("{column_name_and_maybe_field_getter}->'field_0'")) { Ok (value_2317e0af) => value_2317e0af , Err (error) => { return Err (error) ; } }) } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
            if { use std :: fmt :: Write as _ ; write ! (acc_2912b128 , "jsonb_build_object('field_1',{})||" , match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_ids_query_part (& format ! ("{column_name_and_maybe_field_getter}->'field_1'")) { Ok (value_2317e0af) => value_2317e0af , Err (error) => { return Err (error) ; } }) } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
            if { use std :: fmt :: Write as _ ; write ! (acc_2912b128 , "jsonb_build_object('field_2',{})||" , match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_ids_query_part (& format ! ("{column_name_and_maybe_field_getter}->'field_2'")) { Ok (value_2317e0af) => value_2317e0af , Err (error) => { return Err (error) ; } }) } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
            let _: Option<char> = acc_2912b128.pop();
            let _: Option<char> = acc_2912b128.pop();
            format!("jsonb_build_object('value',{acc_2912b128})")
        })
    }
    type ReadInner = ObjectExampleAsNotNullJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        ObjectExampleAsNotNullJsonbObjectReadInner { field_0 : value . field_0 . map (| value_6e5af985 | postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (value_6e5af985 . value) }) , field_1 : value . field_1 . map (| value_6e5af985 | postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (value_6e5af985 . value) }) , field_2 : value . field_2 . map (| value_6e5af985 | postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (value_6e5af985 . value) }) }
    }
    type Update = ObjectExampleAsNotNullJsonbObjectUpdate;
    type UpdateForQuery = ObjectExampleAsNotNullJsonbObjectUpdateForQuery;
    fn update_query_part(
        value: &Self::UpdateForQuery,
        jsonb_set_accumulator: &str,
        jsonb_set_target: &str,
        jsonb_set_path: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut std_option_option_object_acc = format!(
            "case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end"
        );
        let generate_jsonb_set_target =
            |value_12d082b5: &str| format!("{jsonb_set_target}->'{value_12d082b5}'");
        for el_157f4b80 in value.0.to_vec() {
            match el_157f4b80 { ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field0 (value_3b3fae4c) => { match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: update_query_part (& value_3b3fae4c . value , & std_option_option_object_acc , & generate_jsonb_set_target ("field_0") , "field_0" , increment ,) { Ok (value_5edc1648) => { std_option_option_object_acc = value_5edc1648 ; } Err (error) => { return Err (error) ; } } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field1 (value_3b3fae4c) => { match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: update_query_part (& value_3b3fae4c . value , & std_option_option_object_acc , & generate_jsonb_set_target ("field_1") , "field_1" , increment ,) { Ok (value_5edc1648) => { std_option_option_object_acc = value_5edc1648 ; } Err (error) => { return Err (error) ; } } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field2 (value_3b3fae4c) => { match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: update_query_part (& value_3b3fae4c . value , & std_option_option_object_acc , & generate_jsonb_set_target ("field_2") , "field_2" , increment ,) { Ok (value_5edc1648) => { std_option_option_object_acc = value_5edc1648 ; } Err (error) => { return Err (error) ; } } } }
        }
        if jsonb_set_path.is_empty() {
            Ok(std_option_option_object_acc)
        } else {
            Ok(format!(
                "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',{std_option_option_object_acc})"
            ))
        }
    }
    fn update_query_bind(
        value: Self::UpdateForQuery,
        mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        for el_f14dcf6b in value.0.into_vec() {
            match el_f14dcf6b { ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field0 (value_b27f5b09) => { match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: update_query_bind (value_b27f5b09 . value , query) { Ok (value_a4870bad) => { query = value_a4870bad ; } , Err (error) => { return Err (error) ; } } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field1 (value_b27f5b09) => { match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: update_query_bind (value_b27f5b09 . value , query) { Ok (value_a4870bad) => { query = value_a4870bad ; } , Err (error) => { return Err (error) ; } } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field2 (value_b27f5b09) => { match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: update_query_bind (value_b27f5b09 . value , query) { Ok (value_a4870bad) => { query = value_a4870bad ; } , Err (error) => { return Err (error) ; } } } }
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(
        value: &Self::UpdateForQuery,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_only_updated_ids_query_part(
            &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
            increment,
        ) {
            Ok(value_e137951b) => Ok(format!(
                "'{field_ident}',jsonb_build_object('value',{value_e137951b}),"
            )),
            Err(error) => Err(error),
        }
    }
    fn select_only_updated_ids_query_bind<'lifetime>(
        value: &'lifetime Self::UpdateForQuery,
        mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        for el_31dd08ee in value.0.to_vec() {
            match el_31dd08ee { ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field0 (value_b79c2851) => { match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_bind (& value_b79c2851 . value , query) { Ok (value_e8914f75) => { query = value_e8914f75 ; } , Err (error) => { return Err (error) ; } } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field1 (value_b79c2851) => { match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_bind (& value_b79c2851 . value , query) { Ok (value_e8914f75) => { query = value_e8914f75 ; } , Err (error) => { return Err (error) ; } } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field2 (value_b79c2851) => { match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_bind (& value_b79c2851 . value , query) { Ok (value_e8914f75) => { query = value_e8914f75 ; } , Err (error) => { return Err (error) ; } } } }
        }
        Ok(query)
    }
    fn select_only_created_ids_query_part(
        value: &Self::CreateForQuery,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!(
            "'{field_ident}',jsonb_build_object('value',{}),",
            {
                let mut acc_0fe559fa = String::new();
                match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& value . field_0 , "field_0" , & format ! ("{column_name_and_maybe_field_getter}->'field_0'") , increment) { Ok (mut value_cddc8a0a) => { let _ : Option < char > = value_cddc8a0a . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0fe559fa , "jsonb_build_object({value_cddc8a0a})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } }
                match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& value . field_1 , "field_1" , & format ! ("{column_name_and_maybe_field_getter}->'field_1'") , increment) { Ok (mut value_cddc8a0a) => { let _ : Option < char > = value_cddc8a0a . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0fe559fa , "jsonb_build_object({value_cddc8a0a})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } }
                match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& value . field_2 , "field_2" , & format ! ("{column_name_and_maybe_field_getter}->'field_2'") , increment) { Ok (mut value_cddc8a0a) => { let _ : Option < char > = value_cddc8a0a . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0fe559fa , "jsonb_build_object({value_cddc8a0a})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } }
                let _: Option<char> = acc_0fe559fa.pop();
                let _: Option<char> = acc_0fe559fa.pop();
                acc_0fe559fa
            }
        ))
    }
    fn select_only_created_ids_query_bind<'lifetime>(
        value: &'lifetime Self::CreateForQuery,
        mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& value . field_0 , query) { Ok (value_231618d9) => { query = value_231618d9 ; } Err (error) => { return Err (error) ; } }
        match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& value . field_1 , query) { Ok (value_231618d9) => { query = value_231618d9 ; } Err (error) => { return Err (error) ; } }
        match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& value . field_2 , query) { Ok (value_231618d9) => { query = value_231618d9 ; } Err (error) => { return Err (error) ; } }
        Ok(query)
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlType for ObjectExampleAsNotNullJsonbObject {
    type TableTypeDeclaration = ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration;
    fn create_table_column_query_part(
        column: &dyn std::fmt::Display,
        _: bool,
    ) -> impl std::fmt::Display {
        format!(
            "{column} jsonb not null check (jsonb_matches_schema('{}', {column}))",
            serde_json::to_string(&schemars::schema_for!(
                ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration
            ))
            .expect("59a1654b-cbde-40a6-a958-383d263ee19d")
        )
    }
    type Create = ObjectExampleAsNotNullJsonbObjectCreate;
    fn create_query_part(
        _: &Self::Create,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match postgresql_crud::increment_checked_add_one_returning_increment(increment) {
            Ok(value_7df9eb00) => Ok(format!("${value_7df9eb00}")),
            Err(error) => Err(error),
        }
    }
    fn create_query_bind(
        value: Self::Create,
        mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        if let Err(error) = query
            .try_bind(<Self as postgresql_crud::PostgresqlJsonType>::CreateForQuery::from(value))
        {
            return Err(error.to_string());
        }
        Ok(query)
    }
    type Select = ObjectExampleAsNotNullJsonbObjectSelect;
    fn select_query_part(
        value: &Self::Select,
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_query_part_postgresql_type(column) {
            Ok(value_d91c19a6) => Ok(format!("{value_d91c19a6} as {column}")),
            Err(error) => Err(error),
        }
    }
    type Where = ObjectExampleAsNotNullJsonbObjectWhere;
    type Read = ObjectExampleAsNotNullJsonbObjectRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = ObjectExampleAsNotNullJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match <Self as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(column) {
            Ok(value_e776e9fa) => Ok(format!("{value_e776e9fa} as {column},")),
            Err(error) => Err(error),
        }
    }
    type ReadInner = ObjectExampleAsNotNullJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        <Self as postgresql_crud::PostgresqlJsonType>::into_inner(value)
    }
    type Update = ObjectExampleAsNotNullJsonbObjectUpdate;
    type UpdateForQuery = ObjectExampleAsNotNullJsonbObjectUpdateForQuery;
    fn update_query_part(
        value: &Self::UpdateForQuery,
        jsonb_set_accumulator: &str,
        jsonb_set_target: &str,
        jsonb_set_path: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        <Self as postgresql_crud::PostgresqlJsonType>::update_query_part(
            value,
            jsonb_set_accumulator,
            jsonb_set_target,
            jsonb_set_path,
            increment,
        )
    }
    fn update_query_bind(
        value: Self::UpdateForQuery,
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        <Self as postgresql_crud::PostgresqlJsonType>::update_query_bind(value, query)
    }
    fn select_only_updated_ids_query_part(
        value: &Self::UpdateForQuery,
        column: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_only_updated_ids_query_part(column, increment) {
            Ok(value_f0787243) => Ok(format!(
                "jsonb_build_object('value',{value_f0787243}) as {column},"
            )),
            Err(error) => Err(error),
        }
    }
    fn select_only_updated_ids_query_bind<'lifetime>(
        value: &'lifetime Self::UpdateForQuery,
        query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        <Self as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(
            value, query,
        )
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlJsonTypeTestCases for ObjectExampleAsNotNullJsonbObject {
    type PostgresqlJsonType = Self;
    type Select = ObjectExampleAsNotNullJsonbObjectSelect;
    fn option_vec_create()
    -> Option<Vec<<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create>> {
        Some({
            let mut acc_ccd79a32 = Vec::new();
            if let Some (value_0296b347) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: option_vec_create () { for el_37154498 in value_0296b347 { let value = < Self as postgresql_crud :: PostgresqlJsonType > :: Create :: new (el_37154498 , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ()) ; if ! acc_ccd79a32 . contains (& value) { acc_ccd79a32 . push (value) ; } } }
            if let Some (value_0296b347) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: option_vec_create () { for el_37154498 in value_0296b347 { let value = < Self as postgresql_crud :: PostgresqlJsonType > :: Create :: new (postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , el_37154498 , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ()) ; if ! acc_ccd79a32 . contains (& value) { acc_ccd79a32 . push (value) ; } } }
            if let Some (value_0296b347) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: option_vec_create () { for el_37154498 in value_0296b347 { let value = < Self as postgresql_crud :: PostgresqlJsonType > :: Create :: new (postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , el_37154498) ; if ! acc_ccd79a32 . contains (& value) { acc_ccd79a32 . push (value) ; } } }
            acc_ccd79a32
        })
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(
        read_only_ids : & < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
    ) -> Vec<Vec<<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadInner>>
    {
        let mut acc_ef081dc3 = Vec::new();
        let mut field_0_last = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (read_only_ids . 0 . value . field_0 . clone ()) ;
        let mut field_1_last = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (read_only_ids . 0 . value . field_1 . clone ()) ;
        let mut field_2_last = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (read_only_ids . 0 . value . field_2 . clone ()) ;
        for el_7bf83754 in < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& read_only_ids . 0 . value . field_0) { for el_2720df8a in el_7bf83754 { let field_0_current = Some (postgresql_crud :: Value { value : el_2720df8a }) ; field_0_last . clone_from (& field_0_current) ; acc_ef081dc3 . push (vec ! [ObjectExampleAsNotNullJsonbObjectReadInner { field_0 : field_0_current . clone () , field_1 : field_1_last . clone () , field_2 : field_2_last . clone () }]) ; } }
        for el_7bf83754 in < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& read_only_ids . 0 . value . field_1) { for el_2720df8a in el_7bf83754 { let field_1_current = Some (postgresql_crud :: Value { value : el_2720df8a }) ; field_1_last . clone_from (& field_1_current) ; acc_ef081dc3 . push (vec ! [ObjectExampleAsNotNullJsonbObjectReadInner { field_0 : field_0_last . clone () , field_1 : field_1_current . clone () , field_2 : field_2_last . clone () }]) ; } }
        for el_7bf83754 in < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& read_only_ids . 0 . value . field_2) { for el_2720df8a in el_7bf83754 { let field_2_current = Some (postgresql_crud :: Value { value : el_2720df8a }) ; field_2_last . clone_from (& field_2_current) ; acc_ef081dc3 . push (vec ! [ObjectExampleAsNotNullJsonbObjectReadInner { field_0 : field_0_last . clone () , field_1 : field_1_last . clone () , field_2 : field_2_current . clone () }]) ; } }
        acc_ef081dc3
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        value: ObjectExampleAsNotNullJsonbObjectReadInner,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlType > :: Read :: try_new (value . field_0 . map (| value_8ff65e09 | postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value_8ff65e09 . value) }) , value . field_1 . map (| value_8ff65e09 | postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value_8ff65e09 . value) }) , value . field_2 . map (| value_8ff65e09 | postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value_8ff65e09 . value) })) . expect ("3aeeabba-3ffc-4df2-a3bf-e389c40ec566")
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        value: ObjectExampleAsNotNullJsonbObjectReadInner,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update {
        < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlType > :: Update :: new (postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_ebea163e = Vec :: new () ; acc_ebea163e . extend (value . field_0 . map (| el_23bdfe1e | { ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field0 (postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped (el_23bdfe1e . value) }) })) ; acc_ebea163e . extend (value . field_1 . map (| el_23bdfe1e | { ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field1 (postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped (el_23bdfe1e . value) }) })) ; acc_ebea163e . extend (value . field_2 . map (| el_23bdfe1e | { ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field2 (postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped (el_23bdfe1e . value) }) })) ; acc_ebea163e }) . expect ("a06dbdc5-296c-48a8-ba00-913e1fe82ebf"))
    }
    fn read_only_ids_into_option_value_read_inner(
        value: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadInner,
        >,
    > {
        Some (postgresql_crud :: Value { value : ObjectExampleAsNotNullJsonbObjectReadInner { field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (value . 0. value . field_0) . map_or_else (|| Some (postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (< < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el ()) }) , Some) , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (value . 0. value . field_1) . map_or_else (|| Some (postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (< < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el ()) }) , Some) , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (value . 0. value . field_2) . map_or_else (|| Some (postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (< < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el ()) }) , Some) } })
    }
    fn update_to_read_only_ids(
        value: &<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        let mut field_0 = None;
        let mut field_1 = None;
        let mut field_2 = None;
        for el_b3974846 in value.0.to_vec() {
            match el_b3974846 {
                ObjectExampleAsNotNullJsonbObjectUpdateElement::Field0(value_0f4d677e) => {
                    field_0 = Some (< postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: update_to_read_only_ids (& value_0f4d677e . value)) ;
                }
                ObjectExampleAsNotNullJsonbObjectUpdateElement::Field1(value_0f4d677e) => {
                    field_1 = Some (< postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: update_to_read_only_ids (& value_0f4d677e . value)) ;
                }
                ObjectExampleAsNotNullJsonbObjectUpdateElement::Field2(value_0f4d677e) => {
                    field_2 = Some (< postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: update_to_read_only_ids (& value_0f4d677e . value)) ;
                }
            }
        }
        ObjectExampleAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: ObjectExampleAsNotNullJsonbObjectReadOnlyIdsHandle {
                field_0: field_0.expect("106f16f2-ae03-48b3-9239-f4b1b60746d5"),
                field_1: field_1.expect("106f16f2-ae03-48b3-9239-f4b1b60746d5"),
                field_2: field_2.expect("106f16f2-ae03-48b3-9239-f4b1b60746d5"),
            },
        })
    }
    fn read_only_ids_to_option_value_read_default_option_some_vec_one_el(
        value: &<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        >,
    > {
        Some (postgresql_crud :: Value { value : ObjectExampleAsNotNullJsonbObjectRead :: try_new (< postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (& value . 0. value . field_0) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (& value . 0. value . field_1) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (& value . 0. value . field_2)) . expect ("57820868-38ac-4f77-aa0f-dc734399d8b2") })
    }
    fn previous_read_merged_with_option_update_into_read(
        read: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        option_update: Option<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update,
        >,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        match option_update {
            Some(value_e5377436) => {
                let mut field_0 = None;
                let mut field_1 = None;
                let mut field_2 = None;
                for el_629b1544 in value_e5377436.0.into_vec() {
                    match el_629b1544 {
                        ObjectExampleAsNotNullJsonbObjectUpdateElement::Field0(value) => {
                            field_0 = Some(value.value);
                        }
                        ObjectExampleAsNotNullJsonbObjectUpdateElement::Field1(value) => {
                            field_1 = Some(value.value);
                        }
                        ObjectExampleAsNotNullJsonbObjectUpdateElement::Field2(value) => {
                            field_2 = Some(value.value);
                        }
                    }
                }
                ObjectExampleAsNotNullJsonbObjectRead { field_0 : Some (postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (read . field_0 . expect ("a2d26e36-48f9-4739-aabe-adc0f0593570") . value , field_0) }) , field_1 : Some (postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (read . field_1 . expect ("a2d26e36-48f9-4739-aabe-adc0f0593570") . value , field_1) }) , field_2 : Some (postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (read . field_2 . expect ("a2d26e36-48f9-4739-aabe-adc0f0593570") . value , field_2) }) }
            }
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        ObjectExampleAsNotNullJsonbObjectRead :: try_new (< postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids . 0. value . field_0 , create . field_0) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids . 0. value . field_1 , create . field_1) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids . 0. value . field_2 , create . field_2)) . expect ("52ad3994-8392-4fc5-8427-4ca42d87d726")
    }
    fn read_only_ids_merged_with_create_into_option_value_read(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        >,
    > {
        Some (postgresql_crud :: Value { value : < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_read (read_only_ids , create) })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration
    {
        ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration :: new (< postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids . 0. value . field_0 , create . field_0) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids . 0. value . field_1 , create . field_1) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids . 0. value . field_2 , create . field_2))
    }
    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where {
        ObjectExampleAsNotNullJsonbObjectWhere :: Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration :: new (< postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids . 0. value . field_0 , create . field_0) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids . 0. value . field_1 , create . field_1) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids . 0. value . field_2 , create . field_2)) })
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
    > {
        postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids . 0. value . field_0 , create . field_0))) , ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids . 0. value . field_1 , create . field_1))) , ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids . 0. value . field_2 , create . field_2)))]) . expect ("ba9c52c1-6fb6-4fb7-bb5a-b4998b7a2ed2")
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_to_json_field(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
    > {
        postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_89ec072c = Vec :: new () ; for el_d830c061 in < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids . 0. value . field_0 , create . field_0) . into_vec () { acc_89ec072c . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: Or , vec ! [el_d830c061] ,) . expect ("0c6ccad1-6ffc-451f-9b16-0731010fee9f") ,)) ; } for el_d830c061 in < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids . 0. value . field_1 , create . field_1) . into_vec () { acc_89ec072c . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: Or , vec ! [el_d830c061] ,) . expect ("0c6ccad1-6ffc-451f-9b16-0731010fee9f") ,)) ; } for el_d830c061 in < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids . 0. value . field_2 , create . field_2) . into_vec () { acc_89ec072c . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: Or , vec ! [el_d830c061] ,) . expect ("0c6ccad1-6ffc-451f-9b16-0731010fee9f") ,)) ; } acc_89ec072c }) . expect ("9c50391c-001e-4a4f-aac0-14bb614de456")
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_2fe1cca8 = Vec::new();
            if let Some (value_2bbd2c96) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids . 0. value . field_0 , create . field_0) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            if let Some (value_2bbd2c96) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids . 0. value . field_1 , create . field_1) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            if let Some (value_2bbd2c96) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids . 0. value . field_2 , create . field_2) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            acc_2fe1cca8
        }) {
            Ok(value_a5fa471d) => Some(value_a5fa471d),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("89e719cf-3a6d-4250-95fc-237aaf46659b")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_2fe1cca8 = Vec::new();
            if let Some (value_2bbd2c96) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids . 0. value . field_0 , create . field_0) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            if let Some (value_2bbd2c96) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids . 0. value . field_1 , create . field_1) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            if let Some (value_2bbd2c96) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids . 0. value . field_2 , create . field_2) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            acc_2fe1cca8
        }) {
            Ok(value_a5fa471d) => Some(value_a5fa471d),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("89e719cf-3a6d-4250-95fc-237aaf46659b")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_2fe1cca8 = Vec::new();
            if let Some (value_2bbd2c96) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids . 0. value . field_0 , create . field_0) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            if let Some (value_2bbd2c96) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids . 0. value . field_1 , create . field_1) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            if let Some (value_2bbd2c96) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids . 0. value . field_2 , create . field_2) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            acc_2fe1cca8
        }) {
            Ok(value_a5fa471d) => Some(value_a5fa471d),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("89e719cf-3a6d-4250-95fc-237aaf46659b")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_2fe1cca8 = Vec::new();
            if let Some (value_2bbd2c96) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids . 0. value . field_0 , create . field_0) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            if let Some (value_2bbd2c96) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids . 0. value . field_1 , create . field_1) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            if let Some (value_2bbd2c96) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids . 0. value . field_2 , create . field_2) { for el_84537322 in value_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_84537322]) . expect ("9a25e058-6701-430f-a1d1-729aa5e8058a"))) ; } let value_c45bab0d = ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& value_c45bab0d) { acc_2fe1cca8 . push (value_c45bab0d) ; } }
            acc_2fe1cca8
        }) {
            Ok(value_a5fa471d) => Some(value_a5fa471d),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("89e719cf-3a6d-4250-95fc-237aaf46659b")
                }
            },
        }
    }
    fn create_into_postgresql_json_type_option_vec_where_length_equal(
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_587bf907 = Vec::new();
            if let Some (value_927601a4) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create . field_0) { for el_194a660a in value_927601a4 . clone () . into_vec () { acc_587bf907 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_194a660a]) . expect ("2f437949-0c13-4b15-83dd-8ef0399b7d61"))) ; } let value_84ea8e4c = ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_927601a4)) ; if ! acc_587bf907 . contains (& value_84ea8e4c) { acc_587bf907 . push (value_84ea8e4c) ; } }
            if let Some (value_927601a4) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create . field_1) { for el_194a660a in value_927601a4 . clone () . into_vec () { acc_587bf907 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_194a660a]) . expect ("2f437949-0c13-4b15-83dd-8ef0399b7d61"))) ; } let value_84ea8e4c = ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_927601a4)) ; if ! acc_587bf907 . contains (& value_84ea8e4c) { acc_587bf907 . push (value_84ea8e4c) ; } }
            if let Some (value_927601a4) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create . field_2) { for el_194a660a in value_927601a4 . clone () . into_vec () { acc_587bf907 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_194a660a]) . expect ("2f437949-0c13-4b15-83dd-8ef0399b7d61"))) ; } let value_84ea8e4c = ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_927601a4)) ; if ! acc_587bf907 . contains (& value_84ea8e4c) { acc_587bf907 . push (value_84ea8e4c) ; } }
            acc_587bf907
        }) {
            Ok(value_ea661a62) => Some(value_ea661a62),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("7786dfd4-66c1-4d63-acce-794ef80d8bb6")
                }
            },
        }
    }
    fn create_into_postgresql_json_type_option_vec_where_length_greater_than(
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_f5866fb6 = Vec::new();
            if let Some (value_3432b965) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create . field_0) { for el_9bbf8527 in value_3432b965 . clone () . into_vec () { acc_f5866fb6 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_9bbf8527]) . expect ("479db858-6f36-48ba-9ab0-741b7df7956c"))) ; } let el_4a00ab02 = ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_3432b965)) ; if ! acc_f5866fb6 . contains (& el_4a00ab02) { acc_f5866fb6 . push (el_4a00ab02) ; } }
            if let Some (value_3432b965) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create . field_1) { for el_9bbf8527 in value_3432b965 . clone () . into_vec () { acc_f5866fb6 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_9bbf8527]) . expect ("479db858-6f36-48ba-9ab0-741b7df7956c"))) ; } let el_4a00ab02 = ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_3432b965)) ; if ! acc_f5866fb6 . contains (& el_4a00ab02) { acc_f5866fb6 . push (el_4a00ab02) ; } }
            if let Some (value_3432b965) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create . field_2) { for el_9bbf8527 in value_3432b965 . clone () . into_vec () { acc_f5866fb6 . push (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_9bbf8527]) . expect ("479db858-6f36-48ba-9ab0-741b7df7956c"))) ; } let el_4a00ab02 = ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_3432b965)) ; if ! acc_f5866fb6 . contains (& el_4a00ab02) { acc_f5866fb6 . push (el_4a00ab02) ; } }
            acc_f5866fb6
        }) {
            Ok(value_c4c01cd9) => Some(value_c4c01cd9),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("91d713b5-fcb1-4876-ab05-70a52a91bce8")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_a94bd7fb = Vec::new();
            if let Some (value_a2900ac9) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids . 0. value . field_0 , create . field_0) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids . 0. value . field_1 , create . field_1) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids . 0. value . field_2 , create . field_2) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            acc_a94bd7fb
        }) {
            Ok(value_ebe930f0) => Some(value_ebe930f0),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("b877e9c0-7d1e-47e6-9d23-c9bd080753fc")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_a94bd7fb = Vec::new();
            if let Some (value_a2900ac9) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids . 0. value . field_0 , create . field_0) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids . 0. value . field_1 , create . field_1) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids . 0. value . field_2 , create . field_2) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            acc_a94bd7fb
        }) {
            Ok(value_ebe930f0) => Some(value_ebe930f0),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("b877e9c0-7d1e-47e6-9d23-c9bd080753fc")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_a94bd7fb = Vec::new();
            if let Some (value_a2900ac9) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids . 0. value . field_0 , create . field_0) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids . 0. value . field_1 , create . field_1) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids . 0. value . field_2 , create . field_2) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            acc_a94bd7fb
        }) {
            Ok(value_ebe930f0) => Some(value_ebe930f0),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("b877e9c0-7d1e-47e6-9d23-c9bd080753fc")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_a94bd7fb = Vec::new();
            if let Some (value_a2900ac9) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids . 0. value . field_0 , create . field_0) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids . 0. value . field_1 , create . field_1) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids . 0. value . field_2 , create . field_2) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            acc_a94bd7fb
        }) {
            Ok(value_ebe930f0) => Some(value_ebe930f0),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("b877e9c0-7d1e-47e6-9d23-c9bd080753fc")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_a94bd7fb = Vec::new();
            if let Some (value_a2900ac9) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids . 0. value . field_0 , create . field_0) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids . 0. value . field_1 , create . field_1) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids . 0. value . field_2 , create . field_2) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            acc_a94bd7fb
        }) {
            Ok(value_ebe930f0) => Some(value_ebe930f0),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("b877e9c0-7d1e-47e6-9d23-c9bd080753fc")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_a94bd7fb = Vec::new();
            if let Some (value_a2900ac9) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids . 0. value . field_0 , create . field_0) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids . 0. value . field_1 , create . field_1) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            if let Some (value_a2900ac9) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids . 0. value . field_2 , create . field_2) { let and = postgresql_crud :: LogicalOperator :: And ; for el_3e86d33d in value_a2900ac9 . clone () . into_vec () { match el_3e86d33d { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2635ede5-e733-4793-a2b5-110dda258c90")))) ; } , } } let value_3e75a2f2 = postgresql_crud :: SingleOrMultiple :: Single (ObjectExampleAsNotNullJsonbObjectWhere :: Field2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , value_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , postgresql_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab-fca8-4443-bbad-26d92d0a4667"))) ; if ! acc_a94bd7fb . contains (& value_3e75a2f2) { acc_a94bd7fb . push (value_3e75a2f2) ; } }
            acc_a94bd7fb
        }) {
            Ok(value_ebe930f0) => Some(value_ebe930f0),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("b877e9c0-7d1e-47e6-9d23-c9bd080753fc")
                }
            },
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlTypeTestCases for ObjectExampleAsNotNullJsonbObject {
    type PostgresqlType = Self;
    type Select = ObjectExampleAsNotNullJsonbObjectSelect;
    fn option_vec_create()
    -> Option<Vec<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create>> {
        <Self as postgresql_crud::PostgresqlJsonTypeTestCases>::option_vec_create()
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(
        read_only_ids: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
    ) -> Vec<Vec<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadInner>> {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (read_only_ids)
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        value: ObjectExampleAsNotNullJsonbObjectReadInner,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        value: ObjectExampleAsNotNullJsonbObjectReadInner,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped (value)
    }
    fn update_to_read_only_ids(
        value: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        <Self as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(value)
    }
    fn read_only_ids_to_option_value_read_default_option_some_vec_one_el(
        value: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read>,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (value)
    }
    fn previous_read_merged_with_option_update_into_read(
        read: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read,
        option_update: Option<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update>,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (read , option_update)
    }
    fn read_only_ids_merged_with_create_into_read(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_read (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_option_value_read(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::Value<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read>,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::TableTypeDeclaration {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        Some (< Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids , create))
    }
    fn create_into_postgresql_type_option_vec_where_dimension_one_equal(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        None
    }
    fn postgresql_type_option_vec_where_greater_than_test() -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::PostgresqlTypeGreaterThanTest<Self::PostgresqlType>,
        >,
    > {
        None
    }
    fn read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than(
        greater_than_variant: postgresql_crud::PostgresqlTypeGreaterThanVariant,
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        table_type_declaration : < Self :: PostgresqlType as postgresql_crud :: PostgresqlType > :: TableTypeDeclaration,
    ) -> Option<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where> {
        None
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids , create)
    }
    fn create_into_postgresql_json_type_option_vec_where_length_equal(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create)
    }
    fn create_into_postgresql_json_type_option_vec_where_length_greater_than(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids , create)
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlTypeNotPrimaryKey for ObjectExampleAsNotNullJsonbObject {
    type PostgresqlType = Self;
    type Create = ObjectExampleAsNotNullJsonbObjectCreate;
}
#[derive(Debug, Clone, Copy)]
pub struct OptionObjectExampleAsNullableJsonbObject;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct OptionObjectExampleAsNullableJsonbObjectTableTypeDeclaration(
    Option<ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration>,
);
impl OptionObjectExampleAsNullableJsonbObjectTableTypeDeclaration {
    #[must_use]
    pub const fn new(value: Option<ObjectExampleAsNotNullJsonbObjectTableTypeDeclaration>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for OptionObjectExampleAsNullableJsonbObjectTableTypeDeclaration
{
    fn default_option_some_vec_one_el() -> Self {
        Self(Some(
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for OptionObjectExampleAsNullableJsonbObjectTableTypeDeclaration
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectTableTypeDeclaration {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct OptionObjectExampleAsNullableJsonbObjectCreate(
    Option<ObjectExampleAsNotNullJsonbObjectCreate>,
);
impl OptionObjectExampleAsNullableJsonbObjectCreate {
    #[must_use]
    pub const fn new(value: Option<ObjectExampleAsNotNullJsonbObjectCreate>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl for OptionObjectExampleAsNullableJsonbObjectCreate {
    fn default_option_some_vec_one_el() -> Self {
        Self(Some(
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectCreate {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectCreate {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl std::fmt::Display for OptionObjectExampleAsNullableJsonbObjectCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionObjectExampleAsNullableJsonbObjectCreate {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionObjectExampleAsNullableJsonbObjectCreateForQuery(
    Option<
        <ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::CreateForQuery,
    >,
);
impl From<OptionObjectExampleAsNullableJsonbObjectCreate>
    for OptionObjectExampleAsNullableJsonbObjectCreateForQuery
{
    fn from(value: OptionObjectExampleAsNullableJsonbObjectCreate) -> Self {
        Self (value . 0 . map (< ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery :: from))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectCreateForQuery {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectCreateForQuery {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct OptionObjectExampleAsNullableJsonbObjectSelect(
    Option<<ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select>,
);
impl OptionObjectExampleAsNullableJsonbObjectSelect {
    #[must_use]
    pub fn new(
        value: Option<
            postgresql_crud::NotEmptyUniqueVec<ObjectExampleAsNotNullJsonbObjectSelectElement>,
        >,
    ) -> Self {
        Self(value.map(
            <ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select::new,
        ))
    }
    fn select_query_part_postgresql_type(
        &self,
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let value = self . 0 . as_ref () . map_or_else (< < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: Select as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el , Clone :: clone) ;
        match value.select_query_part_postgresql_type(column) {
            Ok(value_c69f1ffe) => Ok(format!(
                "case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({value_c69f1ffe}) end"
            )),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectSelect {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl for OptionObjectExampleAsNullableJsonbObjectSelect {
    fn default_option_some_vec_one_el() -> Self {
        Self(Some(
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneElMaxPageSize
    for OptionObjectExampleAsNullableJsonbObjectSelect
{
    fn default_option_some_vec_one_el_max_page_size() -> Self {
        Self (Some (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ()))
    }
}
pub type OptionObjectExampleAsNullableJsonbObjectWhere =
    postgresql_crud::NullableJsonObjectPostgresqlTypeWhereFilter<
        <ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Where,
    >;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct OptionObjectExampleAsNullableJsonbObjectRead(
    Option<<ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>,
);
impl OptionObjectExampleAsNullableJsonbObjectRead {
    #[must_use]
    pub const fn new(
        value: Option<
            <ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Read,
        >,
    ) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl for OptionObjectExampleAsNullableJsonbObjectRead {
    fn default_option_some_vec_one_el() -> Self {
        Self(Some(
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectRead {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectRead {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionObjectExampleAsNullableJsonbObjectReadOnlyIds(
    postgresql_crud::Value<Option<ObjectExampleAsNotNullJsonbObjectReadOnlyIds>>,
);
impl sqlx::Decode<'_, sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionObjectExampleAsNullableJsonbObjectReadOnlyIds {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
pub type OptionObjectExampleAsNullableJsonbObjectReadInner =
    Option<<ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadInner>;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct OptionObjectExampleAsNullableJsonbObjectUpdate(
    Option<<ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update>,
);
impl OptionObjectExampleAsNullableJsonbObjectUpdate {
    #[must_use]
    pub const fn new(
        value: Option<
            <ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update,
        >,
    ) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl for OptionObjectExampleAsNullableJsonbObjectUpdate {
    fn default_option_some_vec_one_el() -> Self {
        Self(Some(
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionObjectExampleAsNullableJsonbObjectUpdateForQuery(
    Option<
        <ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::UpdateForQuery,
    >,
);
impl OptionObjectExampleAsNullableJsonbObjectUpdateForQuery {
    #[allow(clippy::single_call_fn)]
    fn select_only_updated_ids_query_part(
        &self,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok(match &self.0 {
            Some(value_9570957e) => {
                let mut acc_f7537df2 = String::default();
                for el_97687be3 in value_9570957e.0.to_vec() {
                    match & el_97687be3 { ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field0 (value_92d002a5) => match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_part (& value_92d002a5 . value , "field_0" , column_name_and_maybe_field_getter , increment) { Ok (mut value) => { let _ : Option < char > = value . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_f7537df2 , "jsonb_build_object({value})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field1 (value_92d002a5) => match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_part (& value_92d002a5 . value , "field_1" , column_name_and_maybe_field_getter , increment) { Ok (mut value) => { let _ : Option < char > = value . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_f7537df2 , "jsonb_build_object({value})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field2 (value_92d002a5) => match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_part (& value_92d002a5 . value , "field_2" , column_name_and_maybe_field_getter , increment) { Ok (mut value) => { let _ : Option < char > = value . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_f7537df2 , "jsonb_build_object({value})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } } }
                }
                let _: Option<char> = acc_f7537df2.pop();
                let _: Option<char> = acc_f7537df2.pop();
                format!("jsonb_build_object('value',{acc_f7537df2})")
            }
            None => "'null'::jsonb".to_owned(),
        })
    }
}
impl From < < OptionObjectExampleAsNullableJsonbObject as postgresql_crud :: PostgresqlJsonType > :: Update > for < OptionObjectExampleAsNullableJsonbObject as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery { fn from (value : < OptionObjectExampleAsNullableJsonbObject as postgresql_crud :: PostgresqlJsonType > :: Update) -> Self { Self (value . 0 . map (< ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery :: from)) } }
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlJsonType for OptionObjectExampleAsNullableJsonbObject {
    type TableTypeDeclaration = OptionObjectExampleAsNullableJsonbObjectTableTypeDeclaration;
    type Create = OptionObjectExampleAsNullableJsonbObjectCreate;
    type CreateForQuery = OptionObjectExampleAsNullableJsonbObjectCreateForQuery;
    type Select = OptionObjectExampleAsNullableJsonbObjectSelect;
    fn select_query_part(
        value: &Self::Select,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        column_name_and_maybe_field_getter_for_error_message: &str,
        is_postgresql_type: bool,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let column_name_and_maybe_field_getter_field_ident =
            format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        let value_46039f0e = value . 0 . as_ref () . map_or_else (< < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: Select as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el , Clone :: clone) ;
        match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: select_query_part (& value_46039f0e , field_ident , & column_name_and_maybe_field_getter_field_ident , column_name_and_maybe_field_getter_for_error_message , true) { Ok (value_1f8de96a) => Ok (format ! ("jsonb_build_object('{field_ident}',jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident}) = 'null' then 'null'::jsonb else ({value_1f8de96a}) end))")) , Err (error) => Err (error) }
    }
    type Where = OptionObjectExampleAsNullableJsonbObjectWhere;
    type Read = OptionObjectExampleAsNullableJsonbObjectRead;
    type ReadOnlyIds = OptionObjectExampleAsNullableJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(
        column_name_and_maybe_field_getter: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: select_only_ids_query_part (column_name_and_maybe_field_getter) { Ok (value_21000130) => Ok (format ! ("jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter})='null' then 'null'::jsonb else {value_21000130} end)")) , Err (error) => Err (error) }
    }
    type ReadInner = OptionObjectExampleAsNullableJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.map(
            <ObjectExampleAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner,
        )
    }
    type Update = OptionObjectExampleAsNullableJsonbObjectUpdate;
    type UpdateForQuery = OptionObjectExampleAsNullableJsonbObjectUpdateForQuery;
    fn update_query_part(
        value: &Self::UpdateForQuery,
        jsonb_set_accumulator: &str,
        jsonb_set_target: &str,
        jsonb_set_path: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match & value . 0 { Some (value_92f34435) => < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: update_query_part (value_92f34435 , jsonb_set_accumulator , jsonb_set_target , jsonb_set_path , increment ,) , None => match postgresql_crud :: increment_checked_add_one_returning_increment (increment) { Ok (value_27b8537f) => Ok (format ! ("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${value_27b8537f})")) , Err (error) => Err (error) , } }
    }
    fn update_query_bind(
        value: Self::UpdateForQuery,
        mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        match value . 0 { Some (value_269a0d34) => < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: update_query_bind (value_269a0d34 , query) , None => if let Err (error) = query . try_bind (sqlx :: types :: Json (< Self as postgresql_crud :: PostgresqlJsonType > :: Update :: new (None))) { Err (error . to_string ()) } else { Ok (query) } , }
    }
    fn select_only_updated_ids_query_part(
        value: &Self::UpdateForQuery,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_only_updated_ids_query_part(
            &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
            increment,
        ) {
            Ok(value_e137951b) => Ok(format!(
                "'{field_ident}',jsonb_build_object('value',{value_e137951b}),"
            )),
            Err(error) => Err(error),
        }
    }
    fn select_only_updated_ids_query_bind<'lifetime>(
        value: &'lifetime Self::UpdateForQuery,
        mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        if let Some(value_6334d77d) = &value.0 {
            match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_bind (value_6334d77d , query) { Ok (value_0bd3ba6f) => { query = value_0bd3ba6f ; } , Err (error) => { return Err (error) ; } }
        }
        Ok(query)
    }
    fn select_only_created_ids_query_part(
        value: &Self::CreateForQuery,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!(
            "'{field_ident}',jsonb_build_object('value',{}),",
            match &value.0 {
                Some(value_90219286) => format!("jsonb_build_object('value',{})", {
                    let mut acc_0e9170a3 = String::new();
                    match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& value_90219286 . field_0 , "field_0" , & format ! ("{column_name_and_maybe_field_getter}->'field_0'") , increment) { Ok (mut value_93015133) => { let _ : Option < char > = value_93015133 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0e9170a3 , "jsonb_build_object({value_93015133})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } }
                    match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& value_90219286 . field_1 , "field_1" , & format ! ("{column_name_and_maybe_field_getter}->'field_1'") , increment) { Ok (mut value_93015133) => { let _ : Option < char > = value_93015133 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0e9170a3 , "jsonb_build_object({value_93015133})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } }
                    match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& value_90219286 . field_2 , "field_2" , & format ! ("{column_name_and_maybe_field_getter}->'field_2'") , increment) { Ok (mut value_93015133) => { let _ : Option < char > = value_93015133 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0e9170a3 , "jsonb_build_object({value_93015133})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } }
                    let _: Option<char> = acc_0e9170a3.pop();
                    let _: Option<char> = acc_0e9170a3.pop();
                    acc_0e9170a3
                }),
                None => "'null'::jsonb".to_owned(),
            }
        ))
    }
    fn select_only_created_ids_query_bind<'lifetime>(
        value: &'lifetime Self::CreateForQuery,
        mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        if let Some(value_a1ccd526) = &value.0 {
            match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (value_a1ccd526 , query) { Ok (value_70ed6013) => { query = value_70ed6013 ; } Err (error) => { return Err (error) ; } }
        }
        Ok(query)
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlType for OptionObjectExampleAsNullableJsonbObject {
    type TableTypeDeclaration = OptionObjectExampleAsNullableJsonbObjectTableTypeDeclaration;
    fn create_table_column_query_part(
        column: &dyn std::fmt::Display,
        _: bool,
    ) -> impl std::fmt::Display {
        format!(
            "{column} jsonb not null check (jsonb_matches_schema('{}', {column}))",
            serde_json::to_string(&schemars::schema_for!(
                OptionObjectExampleAsNullableJsonbObjectTableTypeDeclaration
            ))
            .expect("59a1654b-cbde-40a6-a958-383d263ee19d")
        )
    }
    type Create = OptionObjectExampleAsNullableJsonbObjectCreate;
    fn create_query_part(
        _: &Self::Create,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match postgresql_crud::increment_checked_add_one_returning_increment(increment) {
            Ok(value_7df9eb00) => Ok(format!("${value_7df9eb00}")),
            Err(error) => Err(error),
        }
    }
    fn create_query_bind(
        value: Self::Create,
        mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        if let Err(error) = query
            .try_bind(<Self as postgresql_crud::PostgresqlJsonType>::CreateForQuery::from(value))
        {
            return Err(error.to_string());
        }
        Ok(query)
    }
    type Select = OptionObjectExampleAsNullableJsonbObjectSelect;
    fn select_query_part(
        value: &Self::Select,
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_query_part_postgresql_type(column) {
            Ok(value_d91c19a6) => Ok(format!("{value_d91c19a6} as {column}")),
            Err(error) => Err(error),
        }
    }
    type Where = OptionObjectExampleAsNullableJsonbObjectWhere;
    type Read = OptionObjectExampleAsNullableJsonbObjectRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = OptionObjectExampleAsNullableJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match <Self as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(column) {
            Ok(value_e776e9fa) => Ok(format!("{value_e776e9fa} as {column},")),
            Err(error) => Err(error),
        }
    }
    type ReadInner = OptionObjectExampleAsNullableJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        <Self as postgresql_crud::PostgresqlJsonType>::into_inner(value)
    }
    type Update = OptionObjectExampleAsNullableJsonbObjectUpdate;
    type UpdateForQuery = OptionObjectExampleAsNullableJsonbObjectUpdateForQuery;
    fn update_query_part(
        value: &Self::UpdateForQuery,
        jsonb_set_accumulator: &str,
        jsonb_set_target: &str,
        jsonb_set_path: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match & value . 0 { Some (value_92f34435) => < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: update_query_part (value_92f34435 , jsonb_set_accumulator , jsonb_set_target , jsonb_set_path , increment ,) , None => match postgresql_crud :: increment_checked_add_one_returning_increment (increment) { Ok (value_27b8537f) => Ok (format ! ("${value_27b8537f}")) , Err (error) => Err (error) , } }
    }
    fn update_query_bind(
        value: Self::UpdateForQuery,
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        <Self as postgresql_crud::PostgresqlJsonType>::update_query_bind(value, query)
    }
    fn select_only_updated_ids_query_part(
        value: &Self::UpdateForQuery,
        column: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_only_updated_ids_query_part(column, increment) {
            Ok(value_f0787243) => Ok(format!(
                "jsonb_build_object('value',{value_f0787243}) as {column},"
            )),
            Err(error) => Err(error),
        }
    }
    fn select_only_updated_ids_query_bind<'lifetime>(
        value: &'lifetime Self::UpdateForQuery,
        query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        <Self as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(
            value, query,
        )
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlJsonTypeTestCases for OptionObjectExampleAsNullableJsonbObject {
    type PostgresqlJsonType = Self;
    type Select = OptionObjectExampleAsNullableJsonbObjectSelect;
    fn option_vec_create()
    -> Option<Vec<<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create>> {
        Some({
            let mut acc_ccd79a32 = Vec::new();
            if let Some (value_399e6a50) = < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: option_vec_create () { for el_e2767811 in value_399e6a50 { let value = < Self as postgresql_crud :: PostgresqlJsonType > :: Create :: new (Some (el_e2767811)) ; if ! acc_ccd79a32 . contains (& value) { acc_ccd79a32 . push (value) ; } } }
            {
                let value = <Self as postgresql_crud::PostgresqlJsonType>::Create::new(None);
                if !acc_ccd79a32.contains(&value) {
                    acc_ccd79a32.push(value);
                }
            }
            acc_ccd79a32
        })
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(
        read_only_ids : & < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
    ) -> Vec<Vec<<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadInner>>
    {
        read_only_ids . 0. value . as_ref () . into_iter () . flat_map (| value_5fa0668c | { < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (value_5fa0668c) . into_iter () . flat_map (| element0 | { element0 . into_iter () . map (| element1 | vec ! [Some (element1)]) }) }) . chain (std :: iter :: once (vec ! [None])) . collect ()
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        value: OptionObjectExampleAsNullableJsonbObjectReadInner,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlType > :: Read :: new (value . map (< ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped))
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        value: OptionObjectExampleAsNullableJsonbObjectReadInner,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update {
        < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlType > :: Update :: new (value . map (< ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped))
    }
    fn read_only_ids_into_option_value_read_inner(
        value: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadInner,
        >,
    > {
        Some (postgresql_crud :: Value { value : value . 0. value . and_then (| value_5d7e3961 | match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (value_5d7e3961) { Some (value_cfca0099) => Some (value_cfca0099 . value) , None => None , }) })
    }
    fn update_to_read_only_ids(
        value: &<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        OptionObjectExampleAsNullableJsonbObjectReadOnlyIds (postgresql_crud :: Value { value : value . 0 . as_ref () . map (< ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: update_to_read_only_ids) })
    }
    fn read_only_ids_to_option_value_read_default_option_some_vec_one_el(
        value: &<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        >,
    > {
        Some (postgresql_crud :: Value { value : OptionObjectExampleAsNullableJsonbObjectRead :: new (value . 0. value . as_ref () . and_then (| value_dfa7815e | match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (value_dfa7815e) { Some (value_02cef266) => Some (value_02cef266 . value) , None => None , })) })
    }
    fn previous_read_merged_with_option_update_into_read(
        read: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        option_update: Option<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update,
        >,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        match option_update { Some (value_fca601b5) => OptionObjectExampleAsNullableJsonbObjectRead (match value_fca601b5 . 0 { Some (value_8d7747f1) => Some (< ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (read . 0 . unwrap_or_else (postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el) , Some (value_8d7747f1) ,)) , None => None , }) , None => read , }
    }
    fn read_only_ids_merged_with_create_into_read(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        OptionObjectExampleAsNullableJsonbObjectRead :: new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_2b2ab8a1) , Some (create_4a1adaa3)) => { Some (< ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_2b2ab8a1 , create_4a1adaa3) . expect ("56ac4450-0feb-4ea7-aca7-6f51c8f4893c") . value) } , (Some (_) , None) => panic ! ("75be9ae0-ca9f-4251-bfff-2156a90b10c6") , (None , Some (_)) => panic ! ("6a95d7ae-54f5-4e04-9217-223ba156b799") , (None , None) => None , })
    }
    fn read_only_ids_merged_with_create_into_option_value_read(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        >,
    > {
        Some (postgresql_crud :: Value { value : < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_read (read_only_ids , create) })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration
    {
        OptionObjectExampleAsNullableJsonbObjectTableTypeDeclaration :: new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_fb2ec2e4) , Some (create_2f615d4f)) => { Some (< ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_fb2ec2e4 , create_2f615d4f)) } , (Some (_) , None) => panic ! ("9349dcd5-3ed3-4157-b1ef-14c51d55262f") , (None , Some (_)) => panic ! ("45f8e70a-ffca-41b6-ac70-96f101ac3c80") , (None , None) => None , })
    }
    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where {
        postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_ce30c0fe) , Some (create_8fd81ed8)) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [< ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_ce30c0fe , create_8fd81ed8)]) { Ok (value_7a9cd49b) => Some (value_7a9cd49b) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("463769fc-19da-49dc-9b79-8f6ed360fd2b") } } , (Some (_) , None) => panic ! ("1a2b314c-289e-4dc7-bec8-654c60966abf") , (None , Some (_)) => panic ! ("9faea0f9-78ef-4241-98fc-2acde83d07ce") , (None , None) => None , })
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
    > {
        postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_2898c440) , Some (create_f1c4667c)) => Some (< ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids_2898c440 , create_f1c4667c)) , (Some (_) , None) => panic ! ("49e4c289-b37d-4365-96e3-5d896d6860f7") , (None , Some (_)) => panic ! ("ad71caa2-2503-4f9a-952c-e796abf5bbbe") , (None , None) => None , })]) . expect ("ba9c52c1-6fb6-4fb7-bb5a-b4998b7a2ed2")
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_to_json_field(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
    > {
        postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_12b6f16d = Vec :: new () ; match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_2f024927) , Some (create_120c1dad)) => { for el_a8b181a0 in < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids_2f024927 , create_120c1dad) . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_a8b181a0]) { Ok (value_8e72cfd7) => { acc_12b6f16d . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_8e72cfd7))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("2a88b17f-cf3d-4793-a221-d6fc5922b218") } } } } , (Some (_) , None) => panic ! ("b4507b4c-5282-4d91-9a50-190b2d789849") , (None , Some (_)) => panic ! ("8f458c1d-a286-404f-b3b7-cd8f7b4c8bed") , (None , None) => { acc_12b6f16d . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)) ; } , } acc_12b6f16d }) . expect ("7efc9aae-4b7c-4821-b916-72f5eb2fbd6b")
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud :: NotEmptyUniqueVec :: try_new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_cdcb6239) , Some (create_fdd53941)) => match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids_cdcb6239 , create_fdd53941) { Some (value_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el_6739e82f in value_d6124e21 . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_6739e82f]) { Ok (value_7ed84f3b) => { acc_bd78dc08 . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_7ed84f3b))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23dca12f-65c0-4c0e-addd-cc392c663733") } } } let value_e48110ec = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_d6124e21)) ; if ! acc_bd78dc08 . contains (& value_e48110ec) { acc_bd78dc08 . push (value_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b-2ba2-4eb1-a21e-2f9d30b21e98") , (None , Some (_)) => panic ! ("a2761cd2-27ff-4db0-ae81-948aa04573a6") , (None , None) => vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)] }) { Ok (value_55f2dc3d) => Some (value_55f2dc3d) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("88912e24-3bee-4dc4-a373-6d96d260170f") } }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud :: NotEmptyUniqueVec :: try_new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_cdcb6239) , Some (create_fdd53941)) => match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids_cdcb6239 , create_fdd53941) { Some (value_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el_6739e82f in value_d6124e21 . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_6739e82f]) { Ok (value_7ed84f3b) => { acc_bd78dc08 . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_7ed84f3b))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23dca12f-65c0-4c0e-addd-cc392c663733") } } } let value_e48110ec = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_d6124e21)) ; if ! acc_bd78dc08 . contains (& value_e48110ec) { acc_bd78dc08 . push (value_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b-2ba2-4eb1-a21e-2f9d30b21e98") , (None , Some (_)) => panic ! ("a2761cd2-27ff-4db0-ae81-948aa04573a6") , (None , None) => vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)] }) { Ok (value_55f2dc3d) => Some (value_55f2dc3d) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("88912e24-3bee-4dc4-a373-6d96d260170f") } }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud :: NotEmptyUniqueVec :: try_new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_cdcb6239) , Some (create_fdd53941)) => match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids_cdcb6239 , create_fdd53941) { Some (value_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el_6739e82f in value_d6124e21 . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_6739e82f]) { Ok (value_7ed84f3b) => { acc_bd78dc08 . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_7ed84f3b))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23dca12f-65c0-4c0e-addd-cc392c663733") } } } let value_e48110ec = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_d6124e21)) ; if ! acc_bd78dc08 . contains (& value_e48110ec) { acc_bd78dc08 . push (value_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b-2ba2-4eb1-a21e-2f9d30b21e98") , (None , Some (_)) => panic ! ("a2761cd2-27ff-4db0-ae81-948aa04573a6") , (None , None) => vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)] }) { Ok (value_55f2dc3d) => Some (value_55f2dc3d) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("88912e24-3bee-4dc4-a373-6d96d260170f") } }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud :: NotEmptyUniqueVec :: try_new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_cdcb6239) , Some (create_fdd53941)) => match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids_cdcb6239 , create_fdd53941) { Some (value_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el_6739e82f in value_d6124e21 . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_6739e82f]) { Ok (value_7ed84f3b) => { acc_bd78dc08 . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_7ed84f3b))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23dca12f-65c0-4c0e-addd-cc392c663733") } } } let value_e48110ec = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_d6124e21)) ; if ! acc_bd78dc08 . contains (& value_e48110ec) { acc_bd78dc08 . push (value_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b-2ba2-4eb1-a21e-2f9d30b21e98") , (None , Some (_)) => panic ! ("a2761cd2-27ff-4db0-ae81-948aa04573a6") , (None , None) => vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)] }) { Ok (value_55f2dc3d) => Some (value_55f2dc3d) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("88912e24-3bee-4dc4-a373-6d96d260170f") } }
    }
    fn create_into_postgresql_json_type_option_vec_where_length_equal(
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud :: NotEmptyUniqueVec :: try_new (match create . 0 { Some (create_09a81dae) => match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create_09a81dae) { Some (value_3680a4c9) => { let mut acc_5c441d3a = Vec :: new () ; for el_a8b181a0 in value_3680a4c9 . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_a8b181a0]) { Ok (value_15097b27) => { acc_5c441d3a . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_15097b27))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("6c4da72e-e16c-4c17-a939-9a52195e89a9") } } } let value_84ea8e4c = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_3680a4c9)) ; if ! acc_5c441d3a . contains (& value_84ea8e4c) { acc_5c441d3a . push (value_84ea8e4c) ; } acc_5c441d3a } , None => { return None ; } } , None => vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)] , }) { Ok (value_72dbefbc) => Some (value_72dbefbc) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("d41bcbca-5d4c-436c-a465-4920c9da6a43") } }
    }
    fn create_into_postgresql_json_type_option_vec_where_length_greater_than(
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        create . 0 . map_or_else (|| None , | create_612f2a61 | < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create_612f2a61) . map_or_else (|| None , | value_1ea95b5d | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_87f84b5c = Vec :: new () ; for el_9bbf8527 in value_1ea95b5d . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_9bbf8527]) { Ok (value_1d0202fc) => { acc_87f84b5c . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_1d0202fc))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("bdb0a112-6f75-481c-ad28-f540252d8525") , } , } } let value_4e4cfda3 = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_1ea95b5d)) ; if ! acc_87f84b5c . contains (& value_4e4cfda3) { acc_87f84b5c . push (value_4e4cfda3) ; } acc_87f84b5c }) { Ok (value_ea4ca151) => Some (value_ea4ca151) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("c7ecc36f-d510-40ff-a740-e796e112eee5") , } , } ,))
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlTypeTestCases for OptionObjectExampleAsNullableJsonbObject {
    type PostgresqlType = Self;
    type Select = OptionObjectExampleAsNullableJsonbObjectSelect;
    fn option_vec_create()
    -> Option<Vec<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create>> {
        <Self as postgresql_crud::PostgresqlJsonTypeTestCases>::option_vec_create()
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(
        read_only_ids: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
    ) -> Vec<Vec<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadInner>> {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (read_only_ids)
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        value: OptionObjectExampleAsNullableJsonbObjectReadInner,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        value: OptionObjectExampleAsNullableJsonbObjectReadInner,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped (value)
    }
    fn update_to_read_only_ids(
        value: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        <Self as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(value)
    }
    fn read_only_ids_to_option_value_read_default_option_some_vec_one_el(
        value: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read>,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (value)
    }
    fn previous_read_merged_with_option_update_into_read(
        read: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read,
        option_update: Option<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update>,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (read , option_update)
    }
    fn read_only_ids_merged_with_create_into_read(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_read (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_option_value_read(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::Value<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read>,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::TableTypeDeclaration {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        Some (< Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids , create))
    }
    fn create_into_postgresql_type_option_vec_where_dimension_one_equal(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        None
    }
    fn postgresql_type_option_vec_where_greater_than_test() -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::PostgresqlTypeGreaterThanTest<Self::PostgresqlType>,
        >,
    > {
        None
    }
    fn read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than(
        greater_than_variant: postgresql_crud::PostgresqlTypeGreaterThanVariant,
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        table_type_declaration : < Self :: PostgresqlType as postgresql_crud :: PostgresqlType > :: TableTypeDeclaration,
    ) -> Option<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where> {
        None
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids , create)
    }
    fn create_into_postgresql_json_type_option_vec_where_length_equal(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create)
    }
    fn create_into_postgresql_json_type_option_vec_where_length_greater_than(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids , create)
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlTypeNotPrimaryKey for OptionObjectExampleAsNullableJsonbObject {
    type PostgresqlType = Self;
    type Create = OptionObjectExampleAsNullableJsonbObjectCreate;
}
#[derive(Debug, Clone, Copy)]
pub struct VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration(
    Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration>,
);
impl VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    #[must_use]
    pub const fn new(
        value: Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration>,
    ) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration
{
    fn default_option_some_vec_one_el() -> Self {
        Self(vec![
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ])
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate(
    Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate>,
);
impl VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    #[must_use]
    pub const fn new(value: Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate
{
    fn default_option_some_vec_one_el() -> Self {
        Self(vec![
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ])
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl std::fmt::Display for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate
{
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreateForQuery(
    Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreateForQuery>,
);
impl From<VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreateForQuery
{
    fn from(value: VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreateForQuery::from)
                .collect(),
        )
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreateForQuery
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreateForQuery
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    object_example_with_id_as_not_null_jsonb_object_with_id_select:
        ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelect,
    dimension1_pagination: postgresql_crud::PaginationStartsWithZero,
}
impl VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    #[must_use]
    pub const fn new(
        object_example_with_id_as_not_null_jsonb_object_with_id_select : ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelect,
        dimension1_pagination: postgresql_crud::PaginationStartsWithZero,
    ) -> Self {
        Self {
            object_example_with_id_as_not_null_jsonb_object_with_id_select,
            dimension1_pagination,
        }
    }
    fn select_query_part_postgresql_type(
        &self,
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let object_example_with_id_as_not_null_jsonb_object_with_id_select = {
            let mut acc_399d9786 = String::default();
            for el_0127bf54 in self
                .object_example_with_id_as_not_null_jsonb_object_with_id_select
                .0
                .to_vec()
            {
                if { use std :: fmt :: Write as _ ; write ! (acc_399d9786 , "{}||" , match el_0127bf54 { ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement :: Id (value_3c8acf6a) => match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "id" , "value" , column , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } , ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement :: Field0 (value_3c8acf6a) => match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "field_0" , "value" , column , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } , ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement :: Field1 (value_3c8acf6a) => match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "field_1" , "value" , column , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } , ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement :: Field2 (value_3c8acf6a) => match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "field_2" , "value" , column , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } }) } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
            }
            let _: Option<char> = acc_399d9786.pop();
            let _: Option<char> = acc_399d9786.pop();
            acc_399d9786
        };
        let dimension1_start = self.dimension1_pagination.start();
        let dimension1_end = self.dimension1_pagination.end();
        Ok(format!(
            "(case when (jsonb_array_length({column}) = 0) then '[]'::jsonb else (select jsonb_agg(({object_example_with_id_as_not_null_jsonb_object_with_id_select})) from jsonb_array_elements((select {column})) with ordinality where ordinality between {dimension1_start} and {dimension1_end}) end)"
        ))
    }
}
impl sqlx::Type<sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect
{
    fn default_option_some_vec_one_el() -> Self {
        Self {
            object_example_with_id_as_not_null_jsonb_object_with_id_select:
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            dimension1_pagination:
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneElMaxPageSize
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect
{
    fn default_option_some_vec_one_el_max_page_size() -> Self {
        Self { object_example_with_id_as_not_null_jsonb_object_with_id_select : postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size () , dimension1_pagination : postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size () , }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub enum VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere {
    Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration >) , DimensionOneEqual (postgresql_crud :: PostgresqlJsonTypeWhereDimensionOneEqual < ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration >) , LengthEqual (postgresql_crud :: PostgresqlJsonTypeWhereLengthEqual) , LengthGreaterThan (postgresql_crud :: PostgresqlJsonTypeWhereLengthGreaterThan) , In (postgresql_crud :: PostgresqlJsonTypeWhereIn < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: TableTypeDeclaration >) , DimensionOneIn (postgresql_crud :: PostgresqlJsonTypeWhereDimensionOneIn < ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration >) , ContainsAllElementsOfArray (postgresql_crud :: PostgresqlJsonTypeWhereContainsAllElementsOfArray < ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration >) , OverlapsWithArray (postgresql_crud :: PostgresqlJsonTypeWhereOverlapsWithArray < ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration >) , ElementId (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Where >) , ElementField0 (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Where >) , ElementField1 (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Where >) , ElementField2 (postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Where >) }
#[allow(clippy::arbitrary_source_item_ordering)]
impl<'lifetime> postgresql_crud::PostgresqlTypeWhereFilter<'lifetime>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere
{
    fn query_part(
        &self,
        increment: &mut u64,
        column: &dyn std::fmt::Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut generate_el_query =
            |logical_operator: &postgresql_crud::LogicalOperator,
             value: &dyn postgresql_crud::PostgresqlTypeWhereFilter<'_>,
             field: &str|
             -> Result<String, postgresql_crud::QueryPartErrorNamed> {
                let logical_operator_query_part =
                    logical_operator.to_query_part(is_need_to_add_logical_operator);
                let elem = "elem";
                let value_9696ee60 = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                    value,
                    increment,
                    &format!("{elem}->'{field}'"),
                    false,
                ) {
                    Ok(value_c7ec4e53) => value_c7ec4e53,
                    Err(error) => {
                        return Err(error);
                    }
                };
                Ok(format!(
                    "{logical_operator_query_part}(exists (select 1 from jsonb_array_elements({column}) as {elem} where {value_9696ee60}))"
                ))
            };
        match &self {
            Self::Equal(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                column,
                is_need_to_add_logical_operator,
            ),
            Self::DimensionOneEqual(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                    value,
                    increment,
                    column,
                    is_need_to_add_logical_operator,
                )
            }
            Self::LengthEqual(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                column,
                is_need_to_add_logical_operator,
            ),
            Self::LengthGreaterThan(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                    value,
                    increment,
                    column,
                    is_need_to_add_logical_operator,
                )
            }
            Self::In(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                column,
                is_need_to_add_logical_operator,
            ),
            Self::DimensionOneIn(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                value,
                increment,
                column,
                is_need_to_add_logical_operator,
            ),
            Self::ContainsAllElementsOfArray(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                    value,
                    increment,
                    column,
                    is_need_to_add_logical_operator,
                )
            }
            Self::OverlapsWithArray(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                    value,
                    increment,
                    column,
                    is_need_to_add_logical_operator,
                )
            }
            Self::ElementId(value) => generate_el_query(value.get_logical_operator(), value, "id"),
            Self::ElementField0(value) => {
                generate_el_query(value.get_logical_operator(), value, "field_0")
            }
            Self::ElementField1(value) => {
                generate_el_query(value.get_logical_operator(), value, "field_1")
            }
            Self::ElementField2(value) => {
                generate_el_query(value.get_logical_operator(), value, "field_2")
            }
        }
    }
    fn query_bind(
        self,
        query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        match self {
            Self::Equal(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::DimensionOneEqual(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::LengthEqual(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::LengthGreaterThan(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::In(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIn(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::ContainsAllElementsOfArray(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::OverlapsWithArray(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::ElementId(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::ElementField0(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::ElementField1(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
            Self::ElementField2(value) => {
                postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query)
            }
        }
    }
}
impl error_occurence_lib::ToStdStringString
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere
{
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultOptionSomeVecOneEl
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere
{
    fn all_variants_default_option_some_vec_one_el() -> Vec<Self> {
        vec![
            Self::Equal(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::DimensionOneEqual(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::LengthEqual(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::LengthGreaterThan(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::In(postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el()),
            Self::DimensionOneIn(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::ContainsAllElementsOfArray(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::OverlapsWithArray(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::ElementId(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::ElementField0(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::ElementField1(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
            Self::ElementField2(
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ),
        ]
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead(
    Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead>,
);
impl VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    #[must_use]
    pub const fn new(value: Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead
{
    fn default_option_some_vec_one_el() -> Self {
        Self(vec![
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ])
    }
}
impl sqlx::Type<sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds(
    postgresql_crud::Value<Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIds>>,
);
impl sqlx::Decode<'_, sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
pub type VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner =
    Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadInner>;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema,
)]
pub struct VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate { # [serde (skip_serializing_if = "Vec::is_empty")] create : Vec < ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate > , update : postgresql_crud :: UniqueVec :: < ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement > , # [serde (skip_serializing_if = "Vec::is_empty")] delete : Vec < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Update > , }
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed {
    CreateUpdateDeleteAreEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    IdsAreNotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        duplicate: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdInJsonDeleteArray {
        #[eo_to_std_string_string_serialize_deserialize]
        error: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdInJsonUpdateAndDeleteArrays {
        #[eo_to_std_string_string_serialize_deserialize]
        error: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
    pub fn try_new(
        create: Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate>,
        update: postgresql_crud::UniqueVec<
            ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement,
        >,
        delete : Vec < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Update >,
    ) -> Result<
        Self,
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed,
    > {
        if create.is_empty() && update.is_empty() && delete.is_empty() {
            return Err (VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed :: CreateUpdateDeleteAreEmpty { code_occurence : error_occurence_lib :: code_occurence ! () }) ;
        }
        {
            let mut acc_2bf4e098 = Vec::new();
            for el_dff7634c in update.to_vec() {
                if acc_2bf4e098.contains(&&el_dff7634c.id) {
                    return Err (VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed :: IdsAreNotUnique { duplicate : < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Update as error_occurence_lib :: ToStdStringString > :: to_std_string_string (& el_dff7634c . id) , code_occurence : error_occurence_lib :: code_occurence ! () }) ;
                }
                acc_2bf4e098.push(&el_dff7634c.id);
            }
            for el_2b0181e6 in &delete {
                if acc_2bf4e098.contains(&el_2b0181e6) {
                    return Err (VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed :: IdsAreNotUnique { duplicate : < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Update as error_occurence_lib :: ToStdStringString > :: to_std_string_string (el_2b0181e6) , code_occurence : error_occurence_lib :: code_occurence ! () }) ;
                }
                acc_2bf4e098.push(el_2b0181e6);
            }
        }
        {
            let update_acc = update . to_vec () . iter () . map (| el_b6af219f | & el_b6af219f . id) . collect :: < Vec < & < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Update >> () ;
            let delete_acc = {
                let mut delete_acc = Vec::new();
                for el_2ecc509c in &delete {
                    if delete_acc.contains(&el_2ecc509c) {
                        return Err (VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed :: NotUniqueIdInJsonDeleteArray { error : format ! ("custom serde error deserializing VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate: not unique id in json delete array: {}" , < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: get_inner (& el_2ecc509c . clone () . into ())) , code_occurence : error_occurence_lib :: code_occurence ! () }) ;
                    }
                    delete_acc.push(el_2ecc509c);
                }
                delete_acc
            };
            for el_fefe9816 in update_acc {
                if delete_acc.contains(&el_fefe9816) {
                    return Err (VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed :: NotUniqueIdInJsonUpdateAndDeleteArrays { error : format ! ("custom serde error deserializing VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate: not unique id in json update and delete arrays: {}" , < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: get_inner (& el_fefe9816 . clone () . into ())) , code_occurence : error_occurence_lib :: code_occurence ! () }) ;
                }
            }
        }
        Ok(Self {
            create,
            update,
            delete,
        })
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl<'de> serde::Deserialize<'de>
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate
{
    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __ignore,
        }
        #[doc(hidden)]
        struct __FieldVisitor;
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(
                &self,
                __f: &mut serde::__private228::Formatter<'_>,
            ) -> serde::__private228::fmt::Result {
                serde::__private228::Formatter::write_str(__f, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => Ok(__Field::__field0),
                    1u64 => Ok(__Field::__field1),
                    2u64 => Ok(__Field::__field2),
                    _ => Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "create" => Ok(__Field::__field0),
                    "update" => Ok(__Field::__field1),
                    "delete" => Ok(__Field::__field2),
                    _ => Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"create" => Ok(__Field::__field0),
                    b"update" => Ok(__Field::__field1),
                    b"delete" => Ok(__Field::__field2),
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
            marker: serde::__private228::PhantomData<
                VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate,
            >,
            lifetime: serde::__private228::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
            fn expecting(
                &self,
                __f: &mut serde::__private228::Formatter<'_>,
            ) -> serde::__private228::fmt::Result {
                serde::__private228::Formatter::write_str(
                    __f,
                    "tuple struct VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate",
                )
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0_value = serde::de::SeqAccess::next_element::<
                    Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate>,
                >(&mut __seq)?
                .unwrap_or_default();
                let __field1_value = serde::de::SeqAccess::next_element::<
                    postgresql_crud::UniqueVec<
                        ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement,
                    >,
                >(&mut __seq)?
                .unwrap_or_default();
                let __field2_value = serde :: de :: SeqAccess :: next_element :: < Vec < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Update > > (& mut __seq) ? . unwrap_or_default () ;
                match VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate :: try_new (__field0_value , __field1_value , __field2_value) { Ok (value) => Ok (value) , Err (error) => Err (serde :: de :: Error :: custom (format ! ("{error:?}"))) }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: Option<Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate>> =
                    None;
                let mut __field1: Option<
                    postgresql_crud::UniqueVec<
                        ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement,
                    >,
                > = None;
                let mut __field2 : Option < Vec < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Update > > = None ;
                while let Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "create",
                                ));
                            }
                            __field0 = Some(serde::de::MapAccess::next_value::<
                                Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate>,
                            >(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if Option::is_some(&__field1) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "update",
                                ));
                            }
                            __field1 = Some(serde::de::MapAccess::next_value::<
                                postgresql_crud::UniqueVec<
                                    ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement,
                                >,
                            >(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if Option::is_some(&__field2) {
                                return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                    "delete",
                                ));
                            }
                            __field2 = Some (serde :: de :: MapAccess :: next_value :: < Vec < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Update > > (& mut __map) ?) ;
                        }
                        __Field::__ignore => {
                            let _: serde::de::IgnoredAny =
                                serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                        }
                    }
                }
                let __field0_value = __field0.unwrap_or_default();
                let __field1_value = __field1.unwrap_or_default();
                let __field2_value = __field2.unwrap_or_default();
                match VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate :: try_new (__field0_value , __field1_value , __field2_value) { Ok (value) => Ok (value) , Err (error) => Err (serde :: de :: Error :: custom (format ! ("{error:?}"))) }
            }
        }
        #[doc(hidden)]
        const FIELDS: &[&str] = &["create", "update", "delete"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate",
            FIELDS,
            __Visitor {
                marker: serde::__private228::PhantomData::<Self>,
                lifetime: serde::__private228::PhantomData,
            },
        )
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate
{
    fn default_option_some_vec_one_el() -> Self {
        Self {
            create: vec![
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ],
            update: postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            delete: vec![
                postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            ],
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateForQuery { # [serde (skip_serializing_if = "Vec::is_empty")] create : Vec < ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreateForQuery > , update : postgresql_crud :: UniqueVec :: < ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateForQueryElement > , # [serde (skip_serializing_if = "Vec::is_empty")] delete : Vec < < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery > , }
impl VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateForQuery {
    #[allow(clippy::single_call_fn)]
    fn select_only_updated_ids_query_part(
        &self,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!(
            "(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))",
            {
                let mut acc_57cd0744 = String::new();
                for el_d7561f40 in self.update.to_vec() {
                    let mut acc_892857b1 = String::new();
                    match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_part (& el_d7561f40 . id , "id" , "elem" , increment) { Ok (mut value) => { let _ : Option < char > = value . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_892857b1 , "jsonb_build_object({value})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } }
                    for el_738b2a83 in el_d7561f40.fields.0.to_vec() {
                        match & el_738b2a83 { ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field0 (value) => match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_part (& value . value , "field_0" , "elem" , increment) { Ok (mut value_33d3b52e) => { let _ : Option < char > = value_33d3b52e . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_892857b1 , "jsonb_build_object({value_33d3b52e})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field1 (value) => match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_part (& value . value , "field_1" , "elem" , increment) { Ok (mut value_33d3b52e) => { let _ : Option < char > = value_33d3b52e . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_892857b1 , "jsonb_build_object({value_33d3b52e})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } } , ObjectExampleAsNotNullJsonbObjectUpdateForQueryElement :: Field2 (value) => match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_part (& value . value , "field_2" , "elem" , increment) { Ok (mut value_33d3b52e) => { let _ : Option < char > = value_33d3b52e . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_892857b1 , "jsonb_build_object({value_33d3b52e})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } } }
                    }
                    let _: Option<char> = acc_892857b1.pop();
                    let _: Option<char> = acc_892857b1.pop();
                    if {
                        use std::fmt::Write as _;
                        write!(acc_57cd0744, "{acc_892857b1}||")
                    }
                    .is_err()
                    {
                        return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
                for el_b1359d90 in &self.create {
                    match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_b1359d90 . id , "id" , "elem" , increment) { Ok (mut value) => { let _ : Option < char > = value . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_57cd0744 , "jsonb_build_object({value})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } }
                    match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_b1359d90 . field_0 , "field_0" , "elem" , increment) { Ok (mut value) => { let _ : Option < char > = value . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_57cd0744 , "jsonb_build_object({value})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } }
                    match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_b1359d90 . field_1 , "field_1" , "elem" , increment) { Ok (mut value) => { let _ : Option < char > = value . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_57cd0744 , "jsonb_build_object({value})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } }
                    match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_b1359d90 . field_2 , "field_2" , "elem" , increment) { Ok (mut value) => { let _ : Option < char > = value . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_57cd0744 , "jsonb_build_object({value})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } , Err (error) => { return Err (error) ; } }
                }
                let _: Option<char> = acc_57cd0744.pop();
                let _: Option<char> = acc_57cd0744.pop();
                format!("jsonb_build_object('value',{acc_57cd0744})")
            },
            column_name_and_maybe_field_getter,
            {
                let mut acc_d497e8a5 = String::new();
                for _ in self.update.to_vec() {
                    match postgresql_crud::increment_checked_add_one_returning_increment(increment)
                    {
                        Ok(value_c31cb081) => {
                            if {
                                use std::fmt::Write as _;
                                write!(acc_d497e8a5, "${value_c31cb081},")
                            }
                            .is_err()
                            {
                                return Err(
                                    postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    },
                                );
                            }
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                for _ in &self.create {
                    match postgresql_crud::increment_checked_add_one_returning_increment(increment)
                    {
                        Ok(value_b52c3fe1) => {
                            if {
                                use std::fmt::Write as _;
                                write!(acc_d497e8a5, "${value_b52c3fe1},")
                            }
                            .is_err()
                            {
                                return Err(
                                    postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    },
                                );
                            }
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                let _: Option<char> = acc_d497e8a5.pop();
                acc_d497e8a5
            }
        ))
    }
}
impl From < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Update > for < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery { fn from (value : < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Update) -> Self { Self { create : value . create . into_iter () . map (ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreateForQuery :: from) . collect () , update : postgresql_crud :: UniqueVec :: from_t1_impl_from_t2 (value . update) , delete : value . delete . into_iter () . map (Into :: into) . collect () , } } }
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlJsonType
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId
{
    type TableTypeDeclaration =
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate;
    type CreateForQuery =
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreateForQuery;
    type Select = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(
        value: &Self::Select,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        column_name_and_maybe_field_getter_for_error_message: &str,
        is_postgresql_type: bool,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let object_example_with_id_as_not_null_jsonb_object_with_id_select = {
            let mut acc_41dea548 = String::default();
            for el_0127bf54 in value
                .object_example_with_id_as_not_null_jsonb_object_with_id_select
                .0
                .to_vec()
            {
                if { use std :: fmt :: Write as _ ; write ! (acc_41dea548 , "{}||" , match el_0127bf54 { ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement :: Id (value_3c8acf6a) => match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "id" , "value" , "value" , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } , ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement :: Field0 (value_3c8acf6a) => match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "field_0" , "value" , "value" , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } , ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement :: Field1 (value_3c8acf6a) => match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "field_1" , "value" , "value" , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } , ObjectExampleWithIdAsNotNullJsonbObjectWithIdSelectElement :: Field2 (value_3c8acf6a) => match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_query_part (value_3c8acf6a , "field_2" , "value" , "value" , false ,) { Ok (value_d54cf786) => value_d54cf786 , Err (error) => { return Err (error) ; } } }) } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
            }
            let _: Option<char> = acc_41dea548.pop();
            let _: Option<char> = acc_41dea548.pop();
            acc_41dea548
        };
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        Ok(format!(
            "jsonb_build_object('{field_ident}',jsonb_build_object('value',case when (jsonb_array_length({column_name_and_maybe_field_getter}->'{field_ident}') = 0) then '[]'::jsonb else (select jsonb_agg(({object_example_with_id_as_not_null_jsonb_object_with_id_select})) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {dimension1_start} and {dimension1_end}) end ))"
        ))
    }
    type Where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere;
    type Read = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead;
    type ReadOnlyIds = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(
        column_name_and_maybe_field_getter: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!(
            "jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({column_name_and_maybe_field_getter}) as elem))",
            {
                let mut acc_2912b128 = String::default();
                if { use std :: fmt :: Write as _ ; write ! (acc_2912b128 , "jsonb_build_object('id',{})||" , match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: select_only_ids_query_part ("elem->'id'") { Ok (value_2317e0af) => value_2317e0af , Err (error) => { return Err (error) ; } }) } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
                if { use std :: fmt :: Write as _ ; write ! (acc_2912b128 , "jsonb_build_object('field_0',{})||" , match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_ids_query_part ("elem->'field_0'") { Ok (value_2317e0af) => value_2317e0af , Err (error) => { return Err (error) ; } }) } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
                if { use std :: fmt :: Write as _ ; write ! (acc_2912b128 , "jsonb_build_object('field_1',{})||" , match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_ids_query_part ("elem->'field_1'") { Ok (value_2317e0af) => value_2317e0af , Err (error) => { return Err (error) ; } }) } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
                if { use std :: fmt :: Write as _ ; write ! (acc_2912b128 , "jsonb_build_object('field_2',{})||" , match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_ids_query_part ("elem->'field_2'") { Ok (value_2317e0af) => value_2317e0af , Err (error) => { return Err (error) ; } }) } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
                let _: Option<char> = acc_2912b128.pop();
                let _: Option<char> = acc_2912b128.pop();
                format!("jsonb_build_object('value',{acc_2912b128})")
            }
        ))
    }
    type ReadInner = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value . 0 . into_iter () . map (| el_34d57236 | ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadInner { id : el_34d57236 . id . map (| value_6e5af985 | postgresql_crud :: Value { value : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: into_inner (value_6e5af985 . value) }) , field_0 : el_34d57236 . field_0 . map (| value_6e5af985 | postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (value_6e5af985 . value) }) , field_1 : el_34d57236 . field_1 . map (| value_6e5af985 | postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (value_6e5af985 . value) }) , field_2 : el_34d57236 . field_2 . map (| value_6e5af985 | postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (value_6e5af985 . value) }) }) . collect ()
    }
    type Update = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
    type UpdateForQuery =
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateForQuery;
    fn update_query_part(
        value: &Self::UpdateForQuery,
        jsonb_set_accumulator: &str,
        jsonb_set_target: &str,
        jsonb_set_path: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let value_58d685d3 = value;
        let update_query_part_acc = {
            if value_58d685d3.update.is_empty() {
                String::from("elem")
            } else {
                let mut acc_2e2ad041 = String::default();
                for el_a0a61823 in value_58d685d3.update.to_vec() {
                    let ident_with_id_handle = {
                        let id_increment = match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: increment_checked_add_one (increment) { Ok (value_15b44b54) => value_15b44b54 , Err (error) => { return Err (error) ; } } ;
                        match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: update_query_part (& el_a0a61823 . fields , "" , "elem" , "" , increment) { Ok (value_56c44461) => Ok (format ! ("when elem->>'id' = ${id_increment} then {value_56c44461} ")) , Err (error) => Err (error) }
                    };
                    match ident_with_id_handle {
                        Ok(value_8333f8f4) => {
                            if {
                                use std::fmt::Write as _;
                                write!(acc_2e2ad041, "{value_8333f8f4}")
                            }
                            .is_err()
                            {
                                return Err(
                                    postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    },
                                );
                            }
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                let _: Option<char> = acc_2e2ad041.pop();
                format!("case {acc_2e2ad041} else elem end")
            }
        };
        let delete_query_part_acc = {
            let mut acc_5b4cd920 = String::default();
            for _ in &value_58d685d3.delete {
                let increment_cb6ba4a7 = match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: increment_checked_add_one (increment) { Ok (value_110650cc) => value_110650cc , Err (error) => { return Err (error) ; } } ;
                let maybe_space_and_space = if acc_5b4cd920.is_empty() { "" } else { " and " };
                if {
                    use std::fmt::Write as _;
                    write!(
                        acc_5b4cd920,
                        "{maybe_space_and_space}elem->>'id' <> ${increment_cb6ba4a7}"
                    )
                }
                .is_err()
                {
                    return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
            acc_5b4cd920
        };
        let create_query_part_acc = {
            let mut acc_8554f572 = String::default();
            for _ in &value_58d685d3.create {
                let increment_5bbc4961 = match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: increment_checked_add_one (increment) { Ok (value_27487842) => value_27487842 , Err (error) => { return Err (error) ; } } ;
                if {
                    use std::fmt::Write as _;
                    write!(acc_8554f572, "${increment_5bbc4961},")
                }
                .is_err()
                {
                    return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
            let _: Option<char> = acc_8554f572.pop();
            acc_8554f572
        };
        let maybe_where = if value_58d685d3.delete.is_empty() {
            String::default()
        } else {
            format!(" where {delete_query_part_acc}")
        };
        let maybe_jsonb_build_array = if value_58d685d3.create.is_empty() {
            String::default()
        } else {
            format!(" || jsonb_build_array({create_query_part_acc})")
        };
        Ok(format!(
            "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})"
        ))
    }
    fn update_query_bind(
        value: Self::UpdateForQuery,
        mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        for el_30541f69 in value.update.into_vec() {
            match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: query_bind_string_as_postgresql_text_update_for_query (el_30541f69 . id , query) { Ok (value_7633dc9b) => { query = value_7633dc9b ; } , Err (error) => { return Err (error) ; } }
            match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: update_query_bind (el_30541f69 . fields , query) { Ok (value_2073f07a) => { query = value_2073f07a ; } , Err (error) => { return Err (error) ; } }
        }
        for el_4b6f8c01 in value.delete {
            match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: query_bind_string_as_postgresql_text_update_for_query (el_4b6f8c01 , query) { Ok (value_31262d92) => { query = value_31262d92 ; } , Err (error) => { return Err (error) ; } }
        }
        for el_a44eb132 in value.create {
            if let Err(error) = query.try_bind(sqlx::types::Json(el_a44eb132)) {
                return Err(error.to_string());
            }
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(
        value: &Self::UpdateForQuery,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_only_updated_ids_query_part(
            &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
            increment,
        ) {
            Ok(value_e137951b) => Ok(format!(
                "'{field_ident}',jsonb_build_object('value',{value_e137951b}),"
            )),
            Err(error) => Err(error),
        }
    }
    fn select_only_updated_ids_query_bind<'lifetime>(
        value: &'lifetime Self::UpdateForQuery,
        mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        for el_e5af9b26 in value.update.to_vec() {
            match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_bind (& el_e5af9b26 . id , query) { Ok (value_0fd735de) => { query = value_0fd735de ; } , Err (error) => { return Err (error) ; } }
            match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_bind (& el_e5af9b26 . fields , query) { Ok (value_4b52fa4f) => { query = value_4b52fa4f ; } , Err (error) => { return Err (error) ; } }
        }
        for el_5fba4c1f in &value.create {
            match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& el_5fba4c1f . id , query) { Ok (value_cb81ec2c) => { query = value_cb81ec2c ; } Err (error) => { return Err (error) ; } }
            match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& el_5fba4c1f . field_0 , query) { Ok (value_cb81ec2c) => { query = value_cb81ec2c ; } Err (error) => { return Err (error) ; } }
            match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& el_5fba4c1f . field_1 , query) { Ok (value_cb81ec2c) => { query = value_cb81ec2c ; } Err (error) => { return Err (error) ; } }
            match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& el_5fba4c1f . field_2 , query) { Ok (value_cb81ec2c) => { query = value_cb81ec2c ; } Err (error) => { return Err (error) ; } }
        }
        for el_d9eff5ca in value.update.to_vec() {
            match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: query_bind_string_as_postgresql_text_update_for_query (el_d9eff5ca . id . clone () , query) { Ok (value_b0da764b) => { query = value_b0da764b ; } Err (error) => { return Err (error) ; } }
        }
        for el_ae971756 in &value.create {
            match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: query_bind_string_as_postgresql_text_create_for_query (el_ae971756 . id . clone () , query) { Ok (value_dd8932e8) => { query = value_dd8932e8 ; } Err (error) => { return Err (error) ; } }
        }
        Ok(query)
    }
    fn select_only_created_ids_query_part(
        value: &Self::CreateForQuery,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!(
            "'{field_ident}',jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))),",
            {
                let mut acc_0f2b92d0 = String::new();
                for el_3c1dab62 in &value.0 {
                    match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_3c1dab62 . id , "id" , "elem" , increment) { Ok (mut value_6d76c065) => { let _ : Option < char > = value_6d76c065 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0f2b92d0 , "jsonb_build_object({value_6d76c065})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } }
                    match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_3c1dab62 . field_0 , "field_0" , "elem" , increment) { Ok (mut value_6d76c065) => { let _ : Option < char > = value_6d76c065 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0f2b92d0 , "jsonb_build_object({value_6d76c065})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } }
                    match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_3c1dab62 . field_1 , "field_1" , "elem" , increment) { Ok (mut value_6d76c065) => { let _ : Option < char > = value_6d76c065 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0f2b92d0 , "jsonb_build_object({value_6d76c065})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } }
                    match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_3c1dab62 . field_2 , "field_2" , "elem" , increment) { Ok (mut value_6d76c065) => { let _ : Option < char > = value_6d76c065 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0f2b92d0 , "jsonb_build_object({value_6d76c065})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } }
                }
                let _: Option<char> = acc_0f2b92d0.pop();
                let _: Option<char> = acc_0f2b92d0.pop();
                format!("jsonb_build_object('value',{acc_0f2b92d0})")
            },
            &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
            {
                let mut acc_44b1f772 = String::new();
                for _ in &value.0 {
                    match postgresql_crud::increment_checked_add_one_returning_increment(increment)
                    {
                        Ok(value_73b58d3a) => {
                            if {
                                use std::fmt::Write as _;
                                write!(acc_44b1f772, "${value_73b58d3a},")
                            }
                            .is_err()
                            {
                                return Err(
                                    postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    },
                                );
                            }
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                let _: Option<char> = acc_44b1f772.pop();
                acc_44b1f772
            }
        ))
    }
    fn select_only_created_ids_query_bind<'lifetime>(
        value: &'lifetime Self::CreateForQuery,
        mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        for el_9bdcd847 in &value.0 {
            match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& el_9bdcd847 . id , query) { Ok (value_ade27463) => { query = value_ade27463 ; } Err (error) => { return Err (error) ; } }
            match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& el_9bdcd847 . field_0 , query) { Ok (value_ade27463) => { query = value_ade27463 ; } Err (error) => { return Err (error) ; } }
            match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& el_9bdcd847 . field_1 , query) { Ok (value_ade27463) => { query = value_ade27463 ; } Err (error) => { return Err (error) ; } }
            match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (& el_9bdcd847 . field_2 , query) { Ok (value_ade27463) => { query = value_ade27463 ; } Err (error) => { return Err (error) ; } }
        }
        for el_b191a891 in &value.0 {
            match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: query_bind_string_as_postgresql_text_create_for_query (el_b191a891 . id . clone () , query) { Ok (value_a3749ea8) => { query = value_a3749ea8 ; } Err (error) => { return Err (error) ; } }
        }
        Ok(query)
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlType
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId
{
    type TableTypeDeclaration =
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    fn create_table_column_query_part(
        column: &dyn std::fmt::Display,
        _: bool,
    ) -> impl std::fmt::Display {
        format ! ("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))" , serde_json :: to_string (& schemars :: schema_for ! (VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration)) . expect ("59a1654b-cbde-40a6-a958-383d263ee19d"))
    }
    type Create = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate;
    fn create_query_part(
        _: &Self::Create,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match postgresql_crud::increment_checked_add_one_returning_increment(increment) {
            Ok(value_7df9eb00) => Ok(format!("${value_7df9eb00}")),
            Err(error) => Err(error),
        }
    }
    fn create_query_bind(
        value: Self::Create,
        mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        if let Err(error) = query
            .try_bind(<Self as postgresql_crud::PostgresqlJsonType>::CreateForQuery::from(value))
        {
            return Err(error.to_string());
        }
        Ok(query)
    }
    type Select = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(
        value: &Self::Select,
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_query_part_postgresql_type(column) {
            Ok(value_d91c19a6) => Ok(format!("{value_d91c19a6} as {column}")),
            Err(error) => Err(error),
        }
    }
    type Where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere;
    type Read = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match <Self as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(column) {
            Ok(value_e776e9fa) => Ok(format!("{value_e776e9fa} as {column},")),
            Err(error) => Err(error),
        }
    }
    type ReadInner = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        <Self as postgresql_crud::PostgresqlJsonType>::into_inner(value)
    }
    type Update = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
    type UpdateForQuery =
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateForQuery;
    fn update_query_part(
        value: &Self::UpdateForQuery,
        jsonb_set_accumulator: &str,
        jsonb_set_target: &str,
        jsonb_set_path: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let value_58d685d3 = value;
        let update_query_part_acc = {
            if value_58d685d3.update.is_empty() {
                String::from("elem")
            } else {
                let mut acc_2e2ad041 = String::default();
                for el_a0a61823 in value_58d685d3.update.to_vec() {
                    let ident_with_id_handle = {
                        let id_increment = match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: increment_checked_add_one (increment) { Ok (value_15b44b54) => value_15b44b54 , Err (error) => { return Err (error) ; } } ;
                        match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: update_query_part (& el_a0a61823 . fields , "" , "elem" , "" , increment) { Ok (value_56c44461) => Ok (format ! ("when elem->>'id' = ${id_increment} then {value_56c44461} ")) , Err (error) => Err (error) }
                    };
                    match ident_with_id_handle {
                        Ok(value_8333f8f4) => {
                            if {
                                use std::fmt::Write as _;
                                write!(acc_2e2ad041, "{value_8333f8f4}")
                            }
                            .is_err()
                            {
                                return Err(
                                    postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    },
                                );
                            }
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                let _: Option<char> = acc_2e2ad041.pop();
                format!("case {acc_2e2ad041} else elem end")
            }
        };
        let delete_query_part_acc = {
            let mut acc_5b4cd920 = String::default();
            for _ in &value_58d685d3.delete {
                let increment_cb6ba4a7 = match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: increment_checked_add_one (increment) { Ok (value_110650cc) => value_110650cc , Err (error) => { return Err (error) ; } } ;
                let maybe_space_and_space = if acc_5b4cd920.is_empty() { "" } else { " and " };
                if {
                    use std::fmt::Write as _;
                    write!(
                        acc_5b4cd920,
                        "{maybe_space_and_space}elem->>'id' <> ${increment_cb6ba4a7}"
                    )
                }
                .is_err()
                {
                    return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
            acc_5b4cd920
        };
        let create_query_part_acc = {
            let mut acc_8554f572 = String::default();
            for _ in &value_58d685d3.create {
                let increment_5bbc4961 = match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: increment_checked_add_one (increment) { Ok (value_27487842) => value_27487842 , Err (error) => { return Err (error) ; } } ;
                if {
                    use std::fmt::Write as _;
                    write!(acc_8554f572, "${increment_5bbc4961},")
                }
                .is_err()
                {
                    return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
            let _: Option<char> = acc_8554f572.pop();
            acc_8554f572
        };
        let maybe_where = if value_58d685d3.delete.is_empty() {
            String::default()
        } else {
            format!(" where {delete_query_part_acc}")
        };
        let maybe_jsonb_build_array = if value_58d685d3.create.is_empty() {
            String::default()
        } else {
            format!(" || jsonb_build_array({create_query_part_acc})")
        };
        Ok(format!(
            "((select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) {maybe_jsonb_build_array})"
        ))
    }
    fn update_query_bind(
        value: Self::UpdateForQuery,
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        <Self as postgresql_crud::PostgresqlJsonType>::update_query_bind(value, query)
    }
    fn select_only_updated_ids_query_part(
        value: &Self::UpdateForQuery,
        column: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_only_updated_ids_query_part(column, increment) {
            Ok(value_f0787243) => Ok(format!(
                "jsonb_build_object('value',{value_f0787243}) as {column},"
            )),
            Err(error) => Err(error),
        }
    }
    fn select_only_updated_ids_query_bind<'lifetime>(
        value: &'lifetime Self::UpdateForQuery,
        query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        <Self as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(
            value, query,
        )
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlJsonTypeTestCases
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId
{
    type PostgresqlJsonType = Self;
    type Select = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect;
    fn option_vec_create()
    -> Option<Vec<<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create>> {
        Some({
            let mut acc_ccd79a32 = Vec::new();
            if let Some (vec_create) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: option_vec_create () { let mut acc_6a886d56 = Vec :: new () ; let option_additional = { let mut option_additional = None ; for el_37154498 in & vec_create { if option_additional . is_none () { let value = ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate :: new (el_37154498 . clone () , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ()) ; option_additional = Some ((VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate :: new (vec ! [value . clone ()]) , VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate :: new (vec ! [value . clone () , value]))) ; } else { break ; } } option_additional } ; let has_len_greater_than_one = vec_create . len () > 1 ; for el_37154498 in vec_create { acc_6a886d56 . push (ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate :: new (el_37154498 , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ())) ; } { let value_07c0c08c = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate :: new (acc_6a886d56) ; if ! acc_ccd79a32 . contains (& value_07c0c08c) { acc_ccd79a32 . push (value_07c0c08c) ; } } if let Some (value_f6686d5d) = option_additional { if has_len_greater_than_one { let value_60116463 = value_f6686d5d . 0 ; if ! acc_ccd79a32 . contains (& value_60116463) { acc_ccd79a32 . push (value_60116463) ; } } else { let value_7a843059 = value_f6686d5d . 1 ; if ! acc_ccd79a32 . contains (& value_7a843059) { acc_ccd79a32 . push (value_7a843059) ; } } } }
            if let Some (vec_create) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: option_vec_create () { let mut acc_6a886d56 = Vec :: new () ; let option_additional = { let mut option_additional = None ; for el_37154498 in & vec_create { if option_additional . is_none () { let value = ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate :: new (postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , el_37154498 . clone () , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ()) ; option_additional = Some ((VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate :: new (vec ! [value . clone ()]) , VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate :: new (vec ! [value . clone () , value]))) ; } else { break ; } } option_additional } ; let has_len_greater_than_one = vec_create . len () > 1 ; for el_37154498 in vec_create { acc_6a886d56 . push (ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate :: new (postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , el_37154498 , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ())) ; } { let value_07c0c08c = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate :: new (acc_6a886d56) ; if ! acc_ccd79a32 . contains (& value_07c0c08c) { acc_ccd79a32 . push (value_07c0c08c) ; } } if let Some (value_f6686d5d) = option_additional { if has_len_greater_than_one { let value_60116463 = value_f6686d5d . 0 ; if ! acc_ccd79a32 . contains (& value_60116463) { acc_ccd79a32 . push (value_60116463) ; } } else { let value_7a843059 = value_f6686d5d . 1 ; if ! acc_ccd79a32 . contains (& value_7a843059) { acc_ccd79a32 . push (value_7a843059) ; } } } }
            if let Some (vec_create) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: option_vec_create () { let mut acc_6a886d56 = Vec :: new () ; let option_additional = { let mut option_additional = None ; for el_37154498 in & vec_create { if option_additional . is_none () { let value = ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate :: new (postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , el_37154498 . clone ()) ; option_additional = Some ((VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate :: new (vec ! [value . clone ()]) , VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate :: new (vec ! [value . clone () , value]))) ; } else { break ; } } option_additional } ; let has_len_greater_than_one = vec_create . len () > 1 ; for el_37154498 in vec_create { acc_6a886d56 . push (ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate :: new (postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () , el_37154498)) ; } { let value_07c0c08c = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate :: new (acc_6a886d56) ; if ! acc_ccd79a32 . contains (& value_07c0c08c) { acc_ccd79a32 . push (value_07c0c08c) ; } } if let Some (value_f6686d5d) = option_additional { if has_len_greater_than_one { let value_60116463 = value_f6686d5d . 0 ; if ! acc_ccd79a32 . contains (& value_60116463) { acc_ccd79a32 . push (value_60116463) ; } } else { let value_7a843059 = value_f6686d5d . 1 ; if ! acc_ccd79a32 . contains (& value_7a843059) { acc_ccd79a32 . push (value_7a843059) ; } } } }
            acc_ccd79a32
        })
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(
        read_only_ids : & < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
    ) -> Vec<Vec<<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadInner>>
    {
        read_only_ids . 0. value . iter () . map (| el_49a5bb62 | { let mut acc_00b3df88 = Vec :: new () ; for el_4b4da5aa in < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& el_49a5bb62 . 0. value . field_0 . clone ()) { for el_18d1f553 in el_4b4da5aa { acc_00b3df88 . push (vec ! [ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadInner { id : Some (postgresql_crud :: Value { value : el_49a5bb62 . 0. value . id . 0. value }) , field_0 : Some (postgresql_crud :: Value { value : el_18d1f553 }) , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (el_49a5bb62 . 0. value . field_1 . clone ()) , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (el_49a5bb62 . 0. value . field_2 . clone ()) }]) ; } } for el_4b4da5aa in < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& el_49a5bb62 . 0. value . field_1 . clone ()) { for el_18d1f553 in el_4b4da5aa { acc_00b3df88 . push (vec ! [ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadInner { id : Some (postgresql_crud :: Value { value : el_49a5bb62 . 0. value . id . 0. value }) , field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (el_49a5bb62 . 0. value . field_0 . clone ()) , field_1 : Some (postgresql_crud :: Value { value : el_18d1f553 }) , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (el_49a5bb62 . 0. value . field_2 . clone ()) }]) ; } } for el_4b4da5aa in < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& el_49a5bb62 . 0. value . field_2 . clone ()) { for el_18d1f553 in el_4b4da5aa { acc_00b3df88 . push (vec ! [ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadInner { id : Some (postgresql_crud :: Value { value : el_49a5bb62 . 0. value . id . 0. value }) , field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (el_49a5bb62 . 0. value . field_0 . clone ()) , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (el_49a5bb62 . 0. value . field_1 . clone ()) , field_2 : Some (postgresql_crud :: Value { value : el_18d1f553 }) }]) ; } } acc_00b3df88 }) . collect ()
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        value: VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead :: new (value . into_iter () . map (| el_ffed1bfc | ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead { id : el_ffed1bfc . id . map (| value_3ac52220 | postgresql_crud :: Value { value : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value_3ac52220 . value) }) , field_0 : el_ffed1bfc . field_0 . map (| value_3ac52220 | postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value_3ac52220 . value) }) , field_1 : el_ffed1bfc . field_1 . map (| value_3ac52220 | postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value_3ac52220 . value) }) , field_2 : el_ffed1bfc . field_2 . map (| value_3ac52220 | postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value_3ac52220 . value) }) }) . collect ())
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        value: VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update {
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate :: try_new (Vec :: new () , postgresql_crud :: UniqueVec :: < ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement > :: try_new (value . into_iter () . map (| el_ffed1bfc | ObjectExampleWithIdAsNotNullJsonbObjectWithIdUpdateElement { id : postgresql_crud :: UuidUuidAsNotNullJsonbStringUpdate :: new (el_ffed1bfc . id . clone () . expect ("f04a2c6d-bc0b-4693-b7e5-ede348cb229e") . value) , fields : < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped (ObjectExampleAsNotNullJsonbObjectReadInner { field_0 : el_ffed1bfc . field_0 , field_1 : el_ffed1bfc . field_1 , field_2 : el_ffed1bfc . field_2 }) , }) . collect () ,) . expect ("ca51d559-d3d1-4855-8d9a-0f799cccd3e4") , Vec :: new () ,) . expect ("0449fe82-4acc-4c83-9753-18f313b278d1")
    }
    fn read_only_ids_into_option_value_read_inner(
        value: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadInner,
        >,
    > {
        Some (postgresql_crud :: Value { value : value . 0. value . into_iter () . fold (Vec :: new () , | mut acc_cf4743b1 , el_6603f209 | { acc_cf4743b1 . push (ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadInner { id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (el_6603f209 . 0. value . id) . map_or_else (|| Some (postgresql_crud :: Value { value : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: into_inner (< < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: Read as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el ()) }) , Some) , field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (el_6603f209 . 0. value . field_0) . map_or_else (|| Some (postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (< < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el ()) }) , Some) , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (el_6603f209 . 0. value . field_1) . map_or_else (|| Some (postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (< < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el ()) }) , Some) , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (el_6603f209 . 0. value . field_2) . map_or_else (|| Some (postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: into_inner (< < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: Read as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el ()) }) , Some) }) ; acc_cf4743b1 }) })
    }
    fn update_to_read_only_ids(
        value: &<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds (postgresql_crud :: Value { value : value . update . to_vec () . iter () . map (| el_4634bb8a | { let mut field_0 = None ; let mut field_1 = None ; let mut field_2 = None ; for el_da177c5a in el_4634bb8a . fields . 0 . to_vec () { match & el_da177c5a { ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field0 (value_d2a6daf8) => { field_0 = Some (value_d2a6daf8 . value . clone ()) ; } , ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field1 (_) | ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field2 (_) => () , } } for el_da177c5a in el_4634bb8a . fields . 0 . to_vec () { match & el_da177c5a { ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field1 (value_d2a6daf8) => { field_1 = Some (value_d2a6daf8 . value . clone ()) ; } , ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field0 (_) | ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field2 (_) => () , } } for el_da177c5a in el_4634bb8a . fields . 0 . to_vec () { match & el_da177c5a { ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field2 (value_d2a6daf8) => { field_2 = Some (value_d2a6daf8 . value . clone ()) ; } , ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field0 (_) | ObjectExampleAsNotNullJsonbObjectUpdateElement :: Field1 (_) => () , } } ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIds (postgresql_crud :: Value { value : ObjectExampleWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle { id : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: update_to_read_only_ids (& el_4634bb8a . id) , field_0 : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: update_to_read_only_ids (& field_0 . expect ("a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d")) , field_1 : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: update_to_read_only_ids (& field_1 . expect ("a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d")) , field_2 : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: update_to_read_only_ids (& field_2 . expect ("a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d")) } }) }) . collect () })
    }
    fn read_only_ids_to_option_value_read_default_option_some_vec_one_el(
        value: &<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        >,
    > {
        Some(postgresql_crud::Value {
            value: VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead::new({
                let mut acc_5f587d35 = value . 0. value . clone () . into_iter () . map (| el_629b1544 | { ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead :: try_new (< postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (& el_629b1544 . 0. value . id) , < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (& el_629b1544 . 0. value . field_0) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (& el_629b1544 . 0. value . field_1) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (& el_629b1544 . 0. value . field_2)) . expect ("8f6fb6b6-6e84-4819-b9c6-526d39e1ac88") }) . collect :: < Vec < ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead >> () ;
                acc_5f587d35 . sort_by (| first , second | { if let (Some (value_first) , Some (value_second)) = (& first . id , & second . id) { < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: into_inner (value_first . value . clone ()) . cmp (& < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: into_inner (value_second . value . clone ())) } else { panic ! ("0bdf0f44-0012-4cda-9a27-3a89804d871b") ; } }) ;
                acc_5f587d35
            }),
        })
    }
    fn previous_read_merged_with_option_update_into_read(
        read: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        option_update: Option<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update,
        >,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        match option_update {
            Some(value_47f5a20b) => {
                VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead({
                    let mut acc_04a67ef2 = Vec::new();
                    for el_377d1bb4 in value_47f5a20b.update.into_vec() {
                        let mut option_read_element = None;
                        for el_d2daee30 in &read.0 {
                            if * < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: get_inner (& el_377d1bb4 . id . clone () . into ()) == < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: into_inner (el_d2daee30 . id . clone () . expect ("df2413fe-e703-451b-ab75-add67da716f7") . value) { option_read_element = Some (el_d2daee30 . clone ()) ; break ; }
                        }
                        let found_read_element =
                            option_read_element.expect("139882b9-d38f-4cb5-98ea-af0ab23ec474");
                        let mut field_0 = None;
                        let mut field_1 = None;
                        let mut field_2 = None;
                        for el_629b1544 in el_377d1bb4.fields.0.into_vec() {
                            match el_629b1544 {
                                ObjectExampleAsNotNullJsonbObjectUpdateElement::Field0(value) => {
                                    field_0 = Some(value.value);
                                }
                                ObjectExampleAsNotNullJsonbObjectUpdateElement::Field1(value) => {
                                    field_1 = Some(value.value);
                                }
                                ObjectExampleAsNotNullJsonbObjectUpdateElement::Field2(value) => {
                                    field_2 = Some(value.value);
                                }
                            }
                        }
                        acc_04a67ef2 . push (ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead { id : found_read_element . id , field_0 : Some (postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (found_read_element . field_0 . expect ("2e8229f7-38d6-48c1-93c9-e77631a3e155") . value , field_0) }) , field_1 : Some (postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (found_read_element . field_1 . expect ("2e8229f7-38d6-48c1-93c9-e77631a3e155") . value , field_1) }) , field_2 : Some (postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (found_read_element . field_2 . expect ("2e8229f7-38d6-48c1-93c9-e77631a3e155") . value , field_2) }) }) ;
                    }
                    acc_04a67ef2
                })
            }
            None => read,
        }
    }
    fn read_only_ids_merged_with_create_into_read(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead::new({
            assert_eq!(
                read_only_ids.0.value.len(),
                create.0.len(),
                "90d33ddd-e08d-488c-8577-c75febe97301"
            );
            let mut acc_37909420 = Vec::new();
            for (read_only_ids_225e2b76, create_3c660445) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                acc_37909420 . push (ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead :: try_new (< postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_225e2b76 . 0. value . id , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () ,) , < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_225e2b76 . 0. value . field_0 , create_3c660445 . field_0 ,) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_225e2b76 . 0. value . field_1 , create_3c660445 . field_1 ,) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_225e2b76 . 0. value . field_2 , create_3c660445 . field_2 ,)) . expect ("1330ac8d-ebf2-4c79-b25e-6394d58de927")) ;
            }
            acc_37909420
        })
    }
    fn read_only_ids_merged_with_create_into_option_value_read(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        >,
    > {
        Some (postgresql_crud :: Value { value : < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_read (read_only_ids , create) })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration
    {
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration::new({
            assert_eq!(
                read_only_ids.0.value.len(),
                create.0.len(),
                "7776a146-06a8-4972-8e16-371d41ee164c"
            );
            let mut acc_319e1fb1 = Vec::new();
            for (read_only_ids_94b49496, create_24629087) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                acc_319e1fb1 . push (ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration :: new (< postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_94b49496 . 0. value . id , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el () ,) , < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_94b49496 . 0. value . field_0 , create_24629087 . field_0 ,) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_94b49496 . 0. value . field_1 , create_24629087 . field_1 ,) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_94b49496 . 0. value . field_2 , create_24629087 . field_2 ,))) ;
            }
            acc_319e1fb1
        })
    }
    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where {
        VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: And , value : VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration :: new ({ let mut acc_321b3fcd = Vec :: new () ; for (read_only_ids_ea32954c , create_3cbe8967) in read_only_ids . 0. value . into_iter () . zip (create . 0 . into_iter ()) { acc_321b3fcd . push (ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration :: new (< postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_ea32954c . 0. value . id , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ()) , < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_ea32954c . 0. value . field_0 , create_3cbe8967 . field_0) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_ea32954c . 0. value . field_1 , create_3cbe8967 . field_1) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_ea32954c . 0. value . field_2 , create_3cbe8967 . field_2))) ; } acc_321b3fcd }) })
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
    > {
        postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: And , value : VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration :: new ({ let mut acc_97ebf7d6 = Vec :: new () ; for (read_only_ids_319c9e78 , create_00ae06d2) in read_only_ids . 0. value . into_iter () . zip (create . 0 . into_iter ()) { acc_97ebf7d6 . push (ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration :: new (< postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_319c9e78 . 0. value . id , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ()) , < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_319c9e78 . 0. value . field_0 , create_00ae06d2 . field_0) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_319c9e78 . 0. value . field_1 , create_00ae06d2 . field_1) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_319c9e78 . 0. value . field_2 , create_00ae06d2 . field_2))) ; } acc_97ebf7d6 }) })]) . expect ("ba9c52c1-6fb6-4fb7-bb5a-b4998b7a2ed2")
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_to_json_field(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_dd377eb1 = Vec::new();
            for (index_47620dcf, (read_only_ids_420d38ca, create_76f032c1)) in read_only_ids
                .0
                .value
                .into_iter()
                .zip(create.0.into_iter())
                .enumerate()
            {
                if let Some (value_bf84026e) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids_420d38ca . 0. value . field_0 . clone () , create_76f032c1 . field_0 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
                if let Some (value_bf84026e) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids_420d38ca . 0. value . field_1 . clone () , create_76f032c1 . field_1 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
                if let Some (value_bf84026e) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids_420d38ca . 0. value . field_2 . clone () , create_76f032c1 . field_2 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
                acc_dd377eb1 . push (VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: DimensionOneEqual (postgresql_crud :: PostgresqlJsonTypeWhereDimensionOneEqual { logical_operator : postgresql_crud :: LogicalOperator :: And , dimensions : postgresql_crud :: BoundedStdVecVec :: try_from (vec ! [postgresql_crud :: UnsignedPartOfStdPrimitiveI32 :: try_from (i32 :: try_from (index_47620dcf) . expect ("5341936f-ce9e-4e14-ae30-765f04c12e14")) . expect ("76906f3c-4472-4ac0-a605-1b02f02fd680")]) . expect ("8a624c70-3701-4907-b361-5637c5361e1f") , value : ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration :: new (< postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_420d38ca . 0. value . id , postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el ()) , < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_420d38ca . 0. value . field_0 , create_76f032c1 . field_0) , < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_420d38ca . 0. value . field_1 , create_76f032c1 . field_1) , < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_420d38ca . 0. value . field_2 , create_76f032c1 . field_2)) , })) ;
            }
            acc_dd377eb1
        }) {
            Ok(value_dfac36e4) => Some(value_dfac36e4),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("93390f1a-8860-4bf5-b01d-41a6cea3c494")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_dd377eb1 = Vec::new();
            for (read_only_ids_420d38ca, create_76f032c1) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                if let Some (value_bf84026e) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids_420d38ca . 0. value . field_0 . clone () , create_76f032c1 . field_0 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
                if let Some (value_bf84026e) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids_420d38ca . 0. value . field_1 . clone () , create_76f032c1 . field_1 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
                if let Some (value_bf84026e) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids_420d38ca . 0. value . field_2 . clone () , create_76f032c1 . field_2 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
            }
            acc_dd377eb1
        }) {
            Ok(value_dfac36e4) => Some(value_dfac36e4),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("93390f1a-8860-4bf5-b01d-41a6cea3c494")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_dd377eb1 = Vec::new();
            for (read_only_ids_420d38ca, create_76f032c1) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                if let Some (value_bf84026e) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids_420d38ca . 0. value . field_0 . clone () , create_76f032c1 . field_0 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
                if let Some (value_bf84026e) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids_420d38ca . 0. value . field_1 . clone () , create_76f032c1 . field_1 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
                if let Some (value_bf84026e) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids_420d38ca . 0. value . field_2 . clone () , create_76f032c1 . field_2 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
            }
            acc_dd377eb1
        }) {
            Ok(value_dfac36e4) => Some(value_dfac36e4),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("93390f1a-8860-4bf5-b01d-41a6cea3c494")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_dd377eb1 = Vec::new();
            for (read_only_ids_420d38ca, create_76f032c1) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                if let Some (value_bf84026e) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids_420d38ca . 0. value . field_0 . clone () , create_76f032c1 . field_0 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
                if let Some (value_bf84026e) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids_420d38ca . 0. value . field_1 . clone () , create_76f032c1 . field_1 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
                if let Some (value_bf84026e) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids_420d38ca . 0. value . field_2 . clone () , create_76f032c1 . field_2 . clone ()) { for el_f6be06df in value_bf84026e . clone () . into_vec () { let value_592e6b5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_f6be06df]) . expect ("1f7ae335-461f-4215-8fb5-ee7cf2f32881")) ; if ! acc_dd377eb1 . contains (& value_592e6b5f) { acc_dd377eb1 . push (value_592e6b5f) ; } } let value_03205172 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_bf84026e)) ; if ! acc_dd377eb1 . contains (& value_03205172) { acc_dd377eb1 . push (value_03205172) ; } }
            }
            acc_dd377eb1
        }) {
            Ok(value_dfac36e4) => Some(value_dfac36e4),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("93390f1a-8860-4bf5-b01d-41a6cea3c494")
                }
            },
        }
    }
    fn create_into_postgresql_json_type_option_vec_where_length_equal(
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_480d72e5 = Vec::new();
            for create_e06a9fe2 in create.0.clone() {
                if let Some (value_ee015fcc) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create_e06a9fe2 . field_0) { for el_63008daa in value_ee015fcc . clone () . into_vec () { let value_0ae29f5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_63008daa]) . expect ("38ca88dc-ab40-4a76-8bcd-223df66a1f81") ,) ; if ! acc_480d72e5 . contains (& value_0ae29f5f) { acc_480d72e5 . push (value_0ae29f5f) ; } } let value_4e4cfda3 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_ee015fcc)) ; if ! acc_480d72e5 . contains (& value_4e4cfda3) { acc_480d72e5 . push (value_4e4cfda3) ; } }
            }
            for create_e06a9fe2 in create.0.clone() {
                if let Some (value_ee015fcc) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create_e06a9fe2 . field_1) { for el_63008daa in value_ee015fcc . clone () . into_vec () { let value_0ae29f5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_63008daa]) . expect ("38ca88dc-ab40-4a76-8bcd-223df66a1f81") ,) ; if ! acc_480d72e5 . contains (& value_0ae29f5f) { acc_480d72e5 . push (value_0ae29f5f) ; } } let value_4e4cfda3 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_ee015fcc)) ; if ! acc_480d72e5 . contains (& value_4e4cfda3) { acc_480d72e5 . push (value_4e4cfda3) ; } }
            }
            for create_e06a9fe2 in create.0.clone() {
                if let Some (value_ee015fcc) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create_e06a9fe2 . field_2) { for el_63008daa in value_ee015fcc . clone () . into_vec () { let value_0ae29f5f = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_63008daa]) . expect ("38ca88dc-ab40-4a76-8bcd-223df66a1f81") ,) ; if ! acc_480d72e5 . contains (& value_0ae29f5f) { acc_480d72e5 . push (value_0ae29f5f) ; } } let value_4e4cfda3 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_ee015fcc)) ; if ! acc_480d72e5 . contains (& value_4e4cfda3) { acc_480d72e5 . push (value_4e4cfda3) ; } }
            }
            acc_480d72e5.push(
                VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere::LengthEqual(
                    postgresql_crud::PostgresqlJsonTypeWhereLengthEqual {
                        logical_operator: postgresql_crud::LogicalOperator::And,
                        value: postgresql_crud::UnsignedPartOfStdPrimitiveI32::try_from(
                            i32::try_from(create.0.len())
                                .expect("1811faf7-c0a5-4e05-b866-546affd441df"),
                        )
                        .expect("a590f39b-ad2c-4002-afac-f7c18156362e"),
                    },
                ),
            );
            acc_480d72e5
        }) {
            Ok(value_cc01db9a) => Some(value_cc01db9a),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("bad01dd0-163c-4ea5-99c0-a8594a4066e1")
                }
            },
        }
    }
    fn create_into_postgresql_json_type_option_vec_where_length_greater_than(
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_acceb7eb = Vec::new();
            for create_34a1e540 in create.0.clone() {
                if let Some (value_51fe384b) = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create_34a1e540 . field_0) { for el_4a00ab02 in value_51fe384b . clone () . into_vec () { let el_938f8b34 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_4a00ab02]) . expect ("955c6c27-863d-4b9b-9d88-e71f11161b3e") ,) ; if ! acc_acceb7eb . contains (& el_938f8b34) { acc_acceb7eb . push (el_938f8b34) ; } } let el_e17d9fba = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_51fe384b)) ; if ! acc_acceb7eb . contains (& el_e17d9fba) { acc_acceb7eb . push (el_e17d9fba) ; } }
            }
            for create_34a1e540 in create.0.clone() {
                if let Some (value_51fe384b) = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create_34a1e540 . field_1) { for el_4a00ab02 in value_51fe384b . clone () . into_vec () { let el_938f8b34 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_4a00ab02]) . expect ("955c6c27-863d-4b9b-9d88-e71f11161b3e") ,) ; if ! acc_acceb7eb . contains (& el_938f8b34) { acc_acceb7eb . push (el_938f8b34) ; } } let el_e17d9fba = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_51fe384b)) ; if ! acc_acceb7eb . contains (& el_e17d9fba) { acc_acceb7eb . push (el_e17d9fba) ; } }
            }
            for create_34a1e540 in create.0.clone() {
                if let Some (value_51fe384b) = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create_34a1e540 . field_2) { for el_4a00ab02 in value_51fe384b . clone () . into_vec () { let el_938f8b34 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [el_4a00ab02]) . expect ("955c6c27-863d-4b9b-9d88-e71f11161b3e") ,) ; if ! acc_acceb7eb . contains (& el_938f8b34) { acc_acceb7eb . push (el_938f8b34) ; } } let el_e17d9fba = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , value_51fe384b)) ; if ! acc_acceb7eb . contains (& el_e17d9fba) { acc_acceb7eb . push (el_e17d9fba) ; } }
            }
            acc_acceb7eb . push (VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: LengthGreaterThan (postgresql_crud :: PostgresqlJsonTypeWhereLengthGreaterThan { logical_operator : postgresql_crud :: LogicalOperator :: And , value : postgresql_crud :: UnsignedPartOfStdPrimitiveI32 :: try_from (i32 :: try_from (create . 0 . len () . checked_sub (1) . unwrap_or_else (|| { panic ! ("e411b8ca-9419-4c9f-b555-2b6a661fed62") ; })) . expect ("1fbbd897-2fae-4271-9fac-4b4007aecf32")) . expect ("0eb5d915-bbbe-44c0-9096-d3d858d3a937") , })) ;
            acc_acceb7eb
        }) {
            Ok(value_a889de37) => Some(value_a889de37),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("a9e99f81-aa41-4535-ac15-56f1beb0eb49")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_359c0b3f = Vec::new();
            for (read_only_ids_629675e2, create_82796400) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                let and = postgresql_crud::LogicalOperator::And;
                let id = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementId (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [postgresql_crud :: UuidUuidAsNotNullJsonbStringWhere :: Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: UuidUuidAsNotNullJsonbStringTableTypeDeclaration :: new (read_only_ids_629675e2 . 0. value . id . 0. value) , })] ,) . expect ("31db8e1e-28cd-44f7-9f32-a41cc6675660") ,) ;
                let field_0 = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids_629675e2 . 0. value . field_0 , create_82796400 . field_0) ;
                let field_1 = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids_629675e2 . 0. value . field_1 , create_82796400 . field_1) ;
                let field_2 = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids_629675e2 . 0. value . field_2 , create_82796400 . field_2) ;
                if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                    let mut all_fields_acc = vec![];
                    if let Some(value_f190793e) = field_0 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_1 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_2 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    match postgresql_crud::NotEmptyUniqueVec::try_new({
                        all_fields_acc.push(id);
                        all_fields_acc
                    }) {
                        Ok(value_80199720) => {
                            acc_359c0b3f
                                .push(postgresql_crud::SingleOrMultiple::Multiple(value_80199720));
                        }
                        Err(error) => match error {
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                ..
                            } => (),
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                ..
                            } => panic!("32a3da97-c772-44d7-91f9-2916759034e0"),
                        },
                    }
                }
            }
            acc_359c0b3f
        }) {
            Ok(value_752f0e8d) => Some(value_752f0e8d),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("76542a11-ed6f-4cdc-954f-42c48a81acfd")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_359c0b3f = Vec::new();
            for (read_only_ids_629675e2, create_82796400) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                let and = postgresql_crud::LogicalOperator::And;
                let id = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementId (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [postgresql_crud :: UuidUuidAsNotNullJsonbStringWhere :: Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: UuidUuidAsNotNullJsonbStringTableTypeDeclaration :: new (read_only_ids_629675e2 . 0. value . id . 0. value) , })] ,) . expect ("31db8e1e-28cd-44f7-9f32-a41cc6675660") ,) ;
                let field_0 = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids_629675e2 . 0. value . field_0 , create_82796400 . field_0) ;
                let field_1 = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids_629675e2 . 0. value . field_1 , create_82796400 . field_1) ;
                let field_2 = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids_629675e2 . 0. value . field_2 , create_82796400 . field_2) ;
                if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                    let mut all_fields_acc = vec![];
                    if let Some(value_f190793e) = field_0 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_1 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_2 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    match postgresql_crud::NotEmptyUniqueVec::try_new({
                        all_fields_acc.push(id);
                        all_fields_acc
                    }) {
                        Ok(value_80199720) => {
                            acc_359c0b3f
                                .push(postgresql_crud::SingleOrMultiple::Multiple(value_80199720));
                        }
                        Err(error) => match error {
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                ..
                            } => (),
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                ..
                            } => panic!("32a3da97-c772-44d7-91f9-2916759034e0"),
                        },
                    }
                }
            }
            acc_359c0b3f
        }) {
            Ok(value_752f0e8d) => Some(value_752f0e8d),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("76542a11-ed6f-4cdc-954f-42c48a81acfd")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_359c0b3f = Vec::new();
            for (read_only_ids_629675e2, create_82796400) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                let and = postgresql_crud::LogicalOperator::And;
                let id = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementId (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [postgresql_crud :: UuidUuidAsNotNullJsonbStringWhere :: Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: UuidUuidAsNotNullJsonbStringTableTypeDeclaration :: new (read_only_ids_629675e2 . 0. value . id . 0. value) , })] ,) . expect ("31db8e1e-28cd-44f7-9f32-a41cc6675660") ,) ;
                let field_0 = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids_629675e2 . 0. value . field_0 , create_82796400 . field_0) ;
                let field_1 = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids_629675e2 . 0. value . field_1 , create_82796400 . field_1) ;
                let field_2 = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids_629675e2 . 0. value . field_2 , create_82796400 . field_2) ;
                if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                    let mut all_fields_acc = vec![];
                    if let Some(value_f190793e) = field_0 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_1 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_2 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    match postgresql_crud::NotEmptyUniqueVec::try_new({
                        all_fields_acc.push(id);
                        all_fields_acc
                    }) {
                        Ok(value_80199720) => {
                            acc_359c0b3f
                                .push(postgresql_crud::SingleOrMultiple::Multiple(value_80199720));
                        }
                        Err(error) => match error {
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                ..
                            } => (),
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                ..
                            } => panic!("32a3da97-c772-44d7-91f9-2916759034e0"),
                        },
                    }
                }
            }
            acc_359c0b3f
        }) {
            Ok(value_752f0e8d) => Some(value_752f0e8d),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("76542a11-ed6f-4cdc-954f-42c48a81acfd")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_359c0b3f = Vec::new();
            for (read_only_ids_629675e2, create_82796400) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                let and = postgresql_crud::LogicalOperator::And;
                let id = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementId (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [postgresql_crud :: UuidUuidAsNotNullJsonbStringWhere :: Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: UuidUuidAsNotNullJsonbStringTableTypeDeclaration :: new (read_only_ids_629675e2 . 0. value . id . 0. value) , })] ,) . expect ("31db8e1e-28cd-44f7-9f32-a41cc6675660") ,) ;
                let field_0 = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids_629675e2 . 0. value . field_0 , create_82796400 . field_0) ;
                let field_1 = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids_629675e2 . 0. value . field_1 , create_82796400 . field_1) ;
                let field_2 = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids_629675e2 . 0. value . field_2 , create_82796400 . field_2) ;
                if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                    let mut all_fields_acc = vec![];
                    if let Some(value_f190793e) = field_0 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_1 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_2 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    match postgresql_crud::NotEmptyUniqueVec::try_new({
                        all_fields_acc.push(id);
                        all_fields_acc
                    }) {
                        Ok(value_80199720) => {
                            acc_359c0b3f
                                .push(postgresql_crud::SingleOrMultiple::Multiple(value_80199720));
                        }
                        Err(error) => match error {
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                ..
                            } => (),
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                ..
                            } => panic!("32a3da97-c772-44d7-91f9-2916759034e0"),
                        },
                    }
                }
            }
            acc_359c0b3f
        }) {
            Ok(value_752f0e8d) => Some(value_752f0e8d),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("76542a11-ed6f-4cdc-954f-42c48a81acfd")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_359c0b3f = Vec::new();
            for (read_only_ids_629675e2, create_82796400) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                let and = postgresql_crud::LogicalOperator::And;
                let id = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementId (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [postgresql_crud :: UuidUuidAsNotNullJsonbStringWhere :: Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: UuidUuidAsNotNullJsonbStringTableTypeDeclaration :: new (read_only_ids_629675e2 . 0. value . id . 0. value) , })] ,) . expect ("31db8e1e-28cd-44f7-9f32-a41cc6675660") ,) ;
                let field_0 = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids_629675e2 . 0. value . field_0 , create_82796400 . field_0) ;
                let field_1 = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids_629675e2 . 0. value . field_1 , create_82796400 . field_1) ;
                let field_2 = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids_629675e2 . 0. value . field_2 , create_82796400 . field_2) ;
                if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                    let mut all_fields_acc = vec![];
                    if let Some(value_f190793e) = field_0 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_1 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_2 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    match postgresql_crud::NotEmptyUniqueVec::try_new({
                        all_fields_acc.push(id);
                        all_fields_acc
                    }) {
                        Ok(value_80199720) => {
                            acc_359c0b3f
                                .push(postgresql_crud::SingleOrMultiple::Multiple(value_80199720));
                        }
                        Err(error) => match error {
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                ..
                            } => (),
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                ..
                            } => panic!("32a3da97-c772-44d7-91f9-2916759034e0"),
                        },
                    }
                }
            }
            acc_359c0b3f
        }) {
            Ok(value_752f0e8d) => Some(value_752f0e8d),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("76542a11-ed6f-4cdc-954f-42c48a81acfd")
                }
            },
        }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match postgresql_crud::NotEmptyUniqueVec::try_new({
            let mut acc_359c0b3f = Vec::new();
            for (read_only_ids_629675e2, create_82796400) in
                read_only_ids.0.value.into_iter().zip(create.0.into_iter())
            {
                let and = postgresql_crud::LogicalOperator::And;
                let id = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementId (postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [postgresql_crud :: UuidUuidAsNotNullJsonbStringWhere :: Equal (postgresql_crud :: PostgresqlJsonTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: UuidUuidAsNotNullJsonbStringTableTypeDeclaration :: new (read_only_ids_629675e2 . 0. value . id . 0. value) , })] ,) . expect ("31db8e1e-28cd-44f7-9f32-a41cc6675660") ,) ;
                let field_0 = < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids_629675e2 . 0. value . field_0 , create_82796400 . field_0) ;
                let field_1 = < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids_629675e2 . 0. value . field_1 , create_82796400 . field_1) ;
                let field_2 = < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids_629675e2 . 0. value . field_2 , create_82796400 . field_2) ;
                if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                    let mut all_fields_acc = vec![];
                    if let Some(value_f190793e) = field_0 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField0 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_1 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField1 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    if let Some(value_f190793e) = field_2 {
                        for el_22ac4087 in value_f190793e.clone().into_vec() {
                            let current_where = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (match el_22ac4087 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => postgresql_crud :: PostgresqlTypeWhere :: new (and , multiple . clone ()) , postgresql_crud :: SingleOrMultiple :: Single (single) => postgresql_crud :: PostgresqlTypeWhere :: try_new (and , vec ! [single]) . expect ("2ed4dc5e-b893-4bd9-b05c-ffd3bab797cd") , }) ;
                            all_fields_acc.push(current_where.clone());
                            match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [id . clone () , current_where]) { Ok (value_fdd1b3eb) => { let multiple_current_where_with_id = postgresql_crud :: SingleOrMultiple :: Multiple (value_fdd1b3eb) ; if ! acc_359c0b3f . contains (& multiple_current_where_with_id) { acc_359c0b3f . push (multiple_current_where_with_id) ; } } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("f0e3d01b-ac0c-43d4-b31b-45f02e274696") } }
                        }
                        match postgresql_crud::NotEmptyUniqueVec::try_new(
                            value_f190793e
                                .into_vec()
                                .into_iter()
                                .flat_map(|el_6df4f0be| match el_6df4f0be {
                                    postgresql_crud::SingleOrMultiple::Multiple(multiple) => {
                                        multiple.into_vec()
                                    }
                                    postgresql_crud::SingleOrMultiple::Single(single) => {
                                        std::iter::once(single).collect()
                                    }
                                })
                                .fold(Vec::new(), |mut acc_01265629, el_9a7c960d| {
                                    if !acc_01265629.contains(&el_9a7c960d) {
                                        acc_01265629.push(el_9a7c960d);
                                    }
                                    acc_01265629
                                }),
                        ) {
                            Ok(value_a4000d70) => {
                                let value_d6218307 = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhere :: ElementField2 (postgresql_crud :: PostgresqlTypeWhere :: new (and , value_a4000d70)) ;
                                if !all_fields_acc.contains(&value_d6218307) {
                                    all_fields_acc.push(value_d6218307);
                                }
                            }
                            Err(error) => match error {
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                    ..
                                } => (),
                                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                    ..
                                } => panic!("f8fcc434-f952-4f73-9e94-1e5d73516fd7"),
                            },
                        }
                    }
                    match postgresql_crud::NotEmptyUniqueVec::try_new({
                        all_fields_acc.push(id);
                        all_fields_acc
                    }) {
                        Ok(value_80199720) => {
                            acc_359c0b3f
                                .push(postgresql_crud::SingleOrMultiple::Multiple(value_80199720));
                        }
                        Err(error) => match error {
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                                ..
                            } => (),
                            postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                                ..
                            } => panic!("32a3da97-c772-44d7-91f9-2916759034e0"),
                        },
                    }
                }
            }
            acc_359c0b3f
        }) {
            Ok(value_752f0e8d) => Some(value_752f0e8d),
            Err(error) => match error {
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { .. } => None,
                postgresql_crud::NotEmptyUniqueVecTryNewErrorNamed::NotUnique { .. } => {
                    panic!("76542a11-ed6f-4cdc-954f-42c48a81acfd")
                }
            },
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlTypeTestCases
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId
{
    type PostgresqlType = Self;
    type Select = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect;
    fn option_vec_create()
    -> Option<Vec<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create>> {
        <Self as postgresql_crud::PostgresqlJsonTypeTestCases>::option_vec_create()
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(
        read_only_ids: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
    ) -> Vec<Vec<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadInner>> {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (read_only_ids)
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        value: VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        value: VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped (value)
    }
    fn update_to_read_only_ids(
        value: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        <Self as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(value)
    }
    fn read_only_ids_to_option_value_read_default_option_some_vec_one_el(
        value: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read>,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (value)
    }
    fn previous_read_merged_with_option_update_into_read(
        read: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read,
        option_update: Option<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update>,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (read , option_update)
    }
    fn read_only_ids_merged_with_create_into_read(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_read (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_option_value_read(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::Value<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read>,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::TableTypeDeclaration {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        Some (< Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids , create))
    }
    fn create_into_postgresql_type_option_vec_where_dimension_one_equal(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        None
    }
    fn postgresql_type_option_vec_where_greater_than_test() -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::PostgresqlTypeGreaterThanTest<Self::PostgresqlType>,
        >,
    > {
        None
    }
    fn read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than(
        greater_than_variant: postgresql_crud::PostgresqlTypeGreaterThanVariant,
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        table_type_declaration : < Self :: PostgresqlType as postgresql_crud :: PostgresqlType > :: TableTypeDeclaration,
    ) -> Option<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where> {
        None
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids , create)
    }
    fn create_into_postgresql_json_type_option_vec_where_length_equal(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create)
    }
    fn create_into_postgresql_json_type_option_vec_where_length_greater_than(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids , create)
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlTypeNotPrimaryKey
    for VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId
{
    type PostgresqlType = Self;
    type Create = VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate;
}
#[derive(Debug, Clone, Copy)]
pub struct OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithId;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration(
    Option<VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration>,
);
impl OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    #[must_use]
    pub fn new(
        value: Option<Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration>>,
    ) -> Self {
        Self (value . map (VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration :: new))
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration
{
    fn default_option_some_vec_one_el() -> Self {
        Self(Some(
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate(
    Option<VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate>,
);
impl OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate {
    #[must_use]
    pub fn new(value: Option<Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdCreate>>) -> Self {
        Self(value.map(VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate::new))
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate
{
    fn default_option_some_vec_one_el() -> Self {
        Self(Some(
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl std::fmt::Display
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate
{
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreateForQuery (Option < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery >) ;
impl From<OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreateForQuery
{
    fn from(
        value: OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate,
    ) -> Self {
        Self (value . 0 . map (< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: CreateForQuery :: from))
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreateForQuery
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Type<sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreateForQuery
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect (Option < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Select >) ;
impl OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect {
    #[must_use]
    pub const fn new(
        value : Option < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Select >,
    ) -> Self {
        Self(value)
    }
    fn select_query_part_postgresql_type(
        &self,
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let value = self . 0 . as_ref () . map_or_else (< < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Select as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el , Clone :: clone) ;
        match value.select_query_part_postgresql_type(column) {
            Ok(value_c2ca032e) => Ok(format!(
                "case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({value_c2ca032e}) end"
            )),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect
{
    fn default_option_some_vec_one_el() -> Self {
        Self(Some(
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneElMaxPageSize
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect
{
    fn default_option_some_vec_one_el_max_page_size() -> Self {
        Self (Some (postgresql_crud :: DefaultOptionSomeVecOneElMaxPageSize :: default_option_some_vec_one_el_max_page_size ()))
    }
}
pub type OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdWhere = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Where > ;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead (Option < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Read >) ;
impl OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead {
    #[must_use]
    pub fn new(value: Option<Vec<ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead>>) -> Self {
        Self (value . map (< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Read :: new))
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead
{
    fn default_option_some_vec_one_el() -> Self {
        Self(Some(
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))
    }
}
impl sqlx::Type<sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead
{
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds(
    postgresql_crud::Value<
        Option<VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds>,
    >,
);
impl sqlx::Decode<'_, sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds
{
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value_147c3532) => Ok(value_147c3532.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres>
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds
{
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
pub type OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner = Option < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: ReadInner > ;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdate (Option < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Update >) ;
impl OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdate {
    #[must_use]
    pub const fn new(
        value : Option < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Update >,
    ) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultOptionSomeVecOneEl
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdate
{
    fn default_option_some_vec_one_el() -> Self {
        Self(Some(
            postgresql_crud::DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ))
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdateForQuery (Option < < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery >) ;
impl OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdateForQuery {
    #[allow(clippy::single_call_fn)]
    fn select_only_updated_ids_query_part(
        &self,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok (match & self . 0 { Some (value_bc509c9a) => format ! ("jsonb_build_object('value',{})" , match VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateForQuery :: select_only_updated_ids_query_part (value_bc509c9a , column_name_and_maybe_field_getter , increment) { Ok (value_1e016751) => value_1e016751 , Err (error) => { return Err (error) ; } }) , None => "'null'::jsonb" . to_owned () , })
    }
}
impl From < < OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Update > for < OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery { fn from (value : < OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Update) -> Self { Self (value . 0 . map (< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: UpdateForQuery :: from)) } }
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlJsonType
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithId
{
    type TableTypeDeclaration =
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate;
    type CreateForQuery =
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreateForQuery;
    type Select = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(
        value: &Self::Select,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        column_name_and_maybe_field_getter_for_error_message: &str,
        is_postgresql_type: bool,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let value_174d33cd = value . 0 . as_ref () . map_or_else (< < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: Select as postgresql_crud :: DefaultOptionSomeVecOneEl > :: default_option_some_vec_one_el , Clone :: clone) ;
        match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: select_query_part (& value_174d33cd , field_ident , column_name_and_maybe_field_getter , column_name_and_maybe_field_getter_for_error_message , true) { Ok (value_d7bbd03c) => Ok (format ! ("case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('{field_ident}',jsonb_build_object('value','null'::jsonb)) else ({value_d7bbd03c}) end")) , Err (error) => Err (error) }
    }
    type Where = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdWhere;
    type Read = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead;
    type ReadOnlyIds =
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(
        column_name_and_maybe_field_getter: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: select_only_ids_query_part (column_name_and_maybe_field_getter) { Ok (value_21000130) => Ok (format ! ("jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter})='null' then 'null'::jsonb else {value_21000130} end)")) , Err (error) => Err (error) }
    }
    type ReadInner =
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value . 0 . map (< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: into_inner)
    }
    type Update = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdate;
    type UpdateForQuery =
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdateForQuery;
    fn update_query_part(
        value: &Self::UpdateForQuery,
        jsonb_set_accumulator: &str,
        jsonb_set_target: &str,
        jsonb_set_path: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match & value . 0 { Some (value_3245b79f) => < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: update_query_part (value_3245b79f , jsonb_set_accumulator , jsonb_set_target , jsonb_set_path , increment ,) , None => match postgresql_crud :: increment_checked_add_one_returning_increment (increment) { Ok (value_87e08bec) => Ok (format ! ("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${value_87e08bec})")) , Err (error) => Err (error) } }
    }
    fn update_query_bind(
        value: Self::UpdateForQuery,
        mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        match value . 0 { Some (value_a2156b3e) => < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: update_query_bind (value_a2156b3e , query) , None => if let Err (error) = query . try_bind (sqlx :: types :: Json (< Self as postgresql_crud :: PostgresqlJsonType > :: Update :: new (None))) { Err (error . to_string ()) } else { Ok (query) } , }
    }
    fn select_only_updated_ids_query_part(
        value: &Self::UpdateForQuery,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_only_updated_ids_query_part(
            &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
            increment,
        ) {
            Ok(value_e137951b) => Ok(format!(
                "'{field_ident}',jsonb_build_object('value',{value_e137951b}),"
            )),
            Err(error) => Err(error),
        }
    }
    fn select_only_updated_ids_query_bind<'lifetime>(
        value: &'lifetime Self::UpdateForQuery,
        mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        if let Some(value_107e6639) = &value.0 {
            match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: select_only_updated_ids_query_bind (value_107e6639 , query) { Ok (value_ecf1b8de) => { query = value_ecf1b8de ; } , Err (error) => { return Err (error) ; } }
        }
        Ok(query)
    }
    fn select_only_created_ids_query_part(
        value: &Self::CreateForQuery,
        field_ident: &str,
        column_name_and_maybe_field_getter: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!(
            "'{field_ident}',jsonb_build_object('value',{}),",
            match &value.0 {
                Some(value_3c415c92) =>
                    format!(
                        "jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({})))",
                        {
                            let mut acc_1a91bdc7 = String::new();
                            for el_9bdcd847 in &value_3c415c92.0 {
                                match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_9bdcd847 . id , "id" , "elem" , increment) { Ok (mut value_d49fe9d8) => { let _ : Option < char > = value_d49fe9d8 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_1a91bdc7 , "jsonb_build_object({value_d49fe9d8})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } }
                                match < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_9bdcd847 . field_0 , "field_0" , "elem" , increment) { Ok (mut value_d49fe9d8) => { let _ : Option < char > = value_d49fe9d8 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_1a91bdc7 , "jsonb_build_object({value_d49fe9d8})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } }
                                match < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_9bdcd847 . field_1 , "field_1" , "elem" , increment) { Ok (mut value_d49fe9d8) => { let _ : Option < char > = value_d49fe9d8 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_1a91bdc7 , "jsonb_build_object({value_d49fe9d8})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } }
                                match < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_part (& el_9bdcd847 . field_2 , "field_2" , "elem" , increment) { Ok (mut value_d49fe9d8) => { let _ : Option < char > = value_d49fe9d8 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_1a91bdc7 , "jsonb_build_object({value_d49fe9d8})||") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error) => { return Err (error) ; } }
                            }
                            let _: Option<char> = acc_1a91bdc7.pop();
                            let _: Option<char> = acc_1a91bdc7.pop();
                            format!("jsonb_build_object('value',{acc_1a91bdc7})")
                        },
                        &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"),
                        {
                            let mut acc_857ce631 = String::new();
                            for _ in &value_3c415c92.0 {
                                match postgresql_crud::increment_checked_add_one_returning_increment(
                                    increment,
                                ) {
                                    Ok(value_7f11bec0) => {
                                        if {
                                            use std::fmt::Write as _;
                                            write!(acc_857ce631, "${value_7f11bec0},")
                                        }
                                        .is_err()
                                        {
                                            return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ;
                                        }
                                    }
                                    Err(error) => {
                                        return Err(error);
                                    }
                                }
                            }
                            let _: Option<char> = acc_857ce631.pop();
                            acc_857ce631
                        }
                    ),
                None => "'null'::jsonb".to_owned(),
            }
        ))
    }
    fn select_only_created_ids_query_bind<'lifetime>(
        value: &'lifetime Self::CreateForQuery,
        mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        if let Some(value_0b55a65a) = &value.0 {
            match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonType > :: select_only_created_ids_query_bind (value_0b55a65a , query) { Ok (value_ad6a1ac5) => { query = value_ad6a1ac5 ; } Err (error) => { return Err (error) ; } }
        }
        Ok(query)
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlType
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithId
{
    type TableTypeDeclaration =
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    fn create_table_column_query_part(
        column: &dyn std::fmt::Display,
        _: bool,
    ) -> impl std::fmt::Display {
        format ! ("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))" , serde_json :: to_string (& schemars :: schema_for ! (OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration)) . expect ("59a1654b-cbde-40a6-a958-383d263ee19d"))
    }
    type Create = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate;
    fn create_query_part(
        _: &Self::Create,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match postgresql_crud::increment_checked_add_one_returning_increment(increment) {
            Ok(value_7df9eb00) => Ok(format!("${value_7df9eb00}")),
            Err(error) => Err(error),
        }
    }
    fn create_query_bind(
        value: Self::Create,
        mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        if let Err(error) = query
            .try_bind(<Self as postgresql_crud::PostgresqlJsonType>::CreateForQuery::from(value))
        {
            return Err(error.to_string());
        }
        Ok(query)
    }
    type Select = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(
        value: &Self::Select,
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_query_part_postgresql_type(column) {
            Ok(value_d91c19a6) => Ok(format!("{value_d91c19a6} as {column}")),
            Err(error) => Err(error),
        }
    }
    type Where = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdWhere;
    type Read = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds =
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(
        column: &str,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match <Self as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(column) {
            Ok(value_e776e9fa) => Ok(format!("{value_e776e9fa} as {column},")),
            Err(error) => Err(error),
        }
    }
    type ReadInner =
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        <Self as postgresql_crud::PostgresqlJsonType>::into_inner(value)
    }
    type Update = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdate;
    type UpdateForQuery =
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdateForQuery;
    fn update_query_part(
        value: &Self::UpdateForQuery,
        jsonb_set_accumulator: &str,
        jsonb_set_target: &str,
        jsonb_set_path: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value_58d685d3) => {
                let update_query_part_acc = {
                    if value_58d685d3.update.is_empty() {
                        String::from("elem")
                    } else {
                        let mut acc_2e2ad041 = String::default();
                        for el_a0a61823 in value_58d685d3.update.to_vec() {
                            let ident_with_id_handle = {
                                let id_increment = match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: increment_checked_add_one (increment) { Ok (value_15b44b54) => value_15b44b54 , Err (error) => { return Err (error) ; } } ;
                                match < ObjectExampleAsNotNullJsonbObject as postgresql_crud :: PostgresqlJsonType > :: update_query_part (& el_a0a61823 . fields , "" , "elem" , "" , increment) { Ok (value_56c44461) => Ok (format ! ("when elem->>'id' = ${id_increment} then {value_56c44461} ")) , Err (error) => Err (error) }
                            };
                            match ident_with_id_handle {
                                Ok(value_8333f8f4) => {
                                    if {
                                        use std::fmt::Write as _;
                                        write!(acc_2e2ad041, "{value_8333f8f4}")
                                    }
                                    .is_err()
                                    {
                                        return Err(
                                            postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            },
                                        );
                                    }
                                }
                                Err(error) => {
                                    return Err(error);
                                }
                            }
                        }
                        let _: Option<char> = acc_2e2ad041.pop();
                        format!("case {acc_2e2ad041} else elem end")
                    }
                };
                let delete_query_part_acc = {
                    let mut acc_5b4cd920 = String::default();
                    for _ in &value_58d685d3.delete {
                        let increment_cb6ba4a7 = match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: increment_checked_add_one (increment) { Ok (value_110650cc) => value_110650cc , Err (error) => { return Err (error) ; } } ;
                        let maybe_space_and_space =
                            if acc_5b4cd920.is_empty() { "" } else { " and " };
                        if {
                            use std::fmt::Write as _;
                            write!(
                                acc_5b4cd920,
                                "{maybe_space_and_space}elem->>'id' <> ${increment_cb6ba4a7}"
                            )
                        }
                        .is_err()
                        {
                            return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                    acc_5b4cd920
                };
                let create_query_part_acc = {
                    let mut acc_8554f572 = String::default();
                    for _ in &value_58d685d3.create {
                        let increment_5bbc4961 = match < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeObjectVecElementId > :: increment_checked_add_one (increment) { Ok (value_27487842) => value_27487842 , Err (error) => { return Err (error) ; } } ;
                        if {
                            use std::fmt::Write as _;
                            write!(acc_8554f572, "${increment_5bbc4961},")
                        }
                        .is_err()
                        {
                            return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                    }
                    let _: Option<char> = acc_8554f572.pop();
                    acc_8554f572
                };
                let maybe_where = if value_58d685d3.delete.is_empty() {
                    String::default()
                } else {
                    format!(" where {delete_query_part_acc}")
                };
                let maybe_jsonb_build_array = if value_58d685d3.create.is_empty() {
                    String::default()
                } else {
                    format!(" || jsonb_build_array({create_query_part_acc})")
                };
                Ok(format!(
                    "(case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})"
                ))
            }
            None => match postgresql_crud::increment_checked_add_one_returning_increment(increment)
            {
                Ok(value_d31ab6f0) => Ok(format!("${value_d31ab6f0}")),
                Err(error) => Err(error),
            },
        }
    }
    fn update_query_bind(
        value: Self::UpdateForQuery,
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        <Self as postgresql_crud::PostgresqlJsonType>::update_query_bind(value, query)
    }
    fn select_only_updated_ids_query_part(
        value: &Self::UpdateForQuery,
        column: &str,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match value.select_only_updated_ids_query_part(column, increment) {
            Ok(value_f0787243) => Ok(format!(
                "jsonb_build_object('value',{value_f0787243}) as {column},"
            )),
            Err(error) => Err(error),
        }
    }
    fn select_only_updated_ids_query_bind<'lifetime>(
        value: &'lifetime Self::UpdateForQuery,
        query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        <Self as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(
            value, query,
        )
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlJsonTypeTestCases
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithId
{
    type PostgresqlJsonType = Self;
    type Select = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect;
    fn option_vec_create()
    -> Option<Vec<<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create>> {
        Some({
            let mut acc_ccd79a32 = Vec::new();
            if let Some (value_399e6a50) = < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: option_vec_create () { for el_e2767811 in value_399e6a50 { let value = < Self as postgresql_crud :: PostgresqlJsonType > :: Create :: new (Some (el_e2767811 . 0)) ; if ! acc_ccd79a32 . contains (& value) { acc_ccd79a32 . push (value) ; } } }
            {
                let value = <Self as postgresql_crud::PostgresqlJsonType>::Create::new(None);
                if !acc_ccd79a32.contains(&value) {
                    acc_ccd79a32.push(value);
                }
            }
            acc_ccd79a32
        })
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(
        read_only_ids : & < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
    ) -> Vec<Vec<<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadInner>>
    {
        let mut acc_fb5111f1 = Vec::new();
        if let Some(value_6ee5750e) = &read_only_ids.0.value {
            for el_4a5a4b09 in < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (value_6ee5750e) { for el_264980ec in el_4a5a4b09 { acc_fb5111f1 . push (vec ! [Some (el_264980ec)]) ; } }
        }
        acc_fb5111f1.push(vec![None]);
        acc_fb5111f1
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        value: OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlType > :: Read :: new (value . map (| value_189e3c07 | value_189e3c07 . into_iter () . map (| el_ffed1bfc | ObjectExampleWithIdAsNotNullJsonbObjectWithIdRead { id : el_ffed1bfc . id . as_ref () . map (| el_5c1f7f63 | postgresql_crud :: Value { value : < postgresql_crud :: UuidUuidAsNotNullJsonbString as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (el_5c1f7f63 . value . clone ()) }) , field_0 : el_ffed1bfc . field_0 . as_ref () . map (| el_5c1f7f63 | postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (el_5c1f7f63 . value . clone ()) }) , field_1 : el_ffed1bfc . field_1 . as_ref () . map (| el_5c1f7f63 | postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (el_5c1f7f63 . value . clone ()) }) , field_2 : el_ffed1bfc . field_2 . as_ref () . map (| el_5c1f7f63 | postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (el_5c1f7f63 . value . clone ()) }) }) . collect ()))
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        value: OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update {
        < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlType > :: Update :: new (value . map (< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped))
    }
    fn read_only_ids_into_option_value_read_inner(
        value: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadInner,
        >,
    > {
        Some (postgresql_crud :: Value { value : value . 0. value . and_then (| value_f816032d | match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_into_option_value_read_inner (value_f816032d) { Some (value_d0549423) => Some (value_d0549423 . value) , None => None , }) })
    }
    fn update_to_read_only_ids(
        value: &<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds (postgresql_crud :: Value { value : value . 0 . as_ref () . map (< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: update_to_read_only_ids) })
    }
    fn read_only_ids_to_option_value_read_default_option_some_vec_one_el(
        value: &<Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        >,
    > {
        Some (postgresql_crud :: Value { value : OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead :: new (value . 0. value . as_ref () . and_then (| value_16ab4136 | match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (value_16ab4136) { Some (value_71a66429) => Some (value_71a66429 . value . 0) , None => None , })) })
    }
    fn previous_read_merged_with_option_update_into_read(
        read: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        option_update: Option<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Update,
        >,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        match option_update { Some (value_fca601b5) => OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead (match value_fca601b5 . 0 { Some (value_8d7747f1) => Some (< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (read . 0 . unwrap_or_else (postgresql_crud :: DefaultOptionSomeVecOneEl :: default_option_some_vec_one_el) , Some (value_8d7747f1) ,)) , None => None , }) , None => read , }
    }
    fn read_only_ids_merged_with_create_into_read(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read {
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead :: new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_2b2ab8a1) , Some (create_4a1adaa3)) => { Some (< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_2b2ab8a1 , create_4a1adaa3) . expect ("56ac4450-0feb-4ea7-aca7-6f51c8f4893c") . value . 0) } , (Some (_) , None) => panic ! ("75be9ae0-ca9f-4251-bfff-2156a90b10c6") , (None , Some (_)) => panic ! ("6a95d7ae-54f5-4e04-9217-223ba156b799") , (None , None) => None , })
    }
    fn read_only_ids_merged_with_create_into_option_value_read(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::Value<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Read,
        >,
    > {
        Some (postgresql_crud :: Value { value : < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_read (read_only_ids , create) })
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration
    {
        OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration :: new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_fb2ec2e4) , Some (create_2f615d4f)) => { Some (< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids_fb2ec2e4 , create_2f615d4f) . 0) } , (Some (_) , None) => panic ! ("9349dcd5-3ed3-4157-b1ef-14c51d55262f") , (None , Some (_)) => panic ! ("45f8e70a-ffca-41b6-ac70-96f101ac3c80") , (None , None) => None , })
    }
    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where {
        postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_ce30c0fe) , Some (create_8fd81ed8)) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_ce30c0fe , create_8fd81ed8)]) { Ok (value_7a9cd49b) => Some (value_7a9cd49b) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("463769fc-19da-49dc-9b79-8f6ed360fd2b") } } , (Some (_) , None) => panic ! ("1a2b314c-289e-4dc7-bec8-654c60966abf") , (None , Some (_)) => panic ! ("9faea0f9-78ef-4241-98fc-2acde83d07ce") , (None , None) => None , })
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
    > {
        postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_2898c440) , Some (create_f1c4667c)) => Some (< VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids_2898c440 , create_f1c4667c)) , (Some (_) , None) => panic ! ("49e4c289-b37d-4365-96e3-5d896d6860f7") , (None , Some (_)) => panic ! ("ad71caa2-2503-4f9a-952c-e796abf5bbbe") , (None , None) => None , })]) . expect ("ba9c52c1-6fb6-4fb7-bb5a-b4998b7a2ed2")
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_to_json_field(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud :: NotEmptyUniqueVec :: try_new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_cdcb6239) , Some (create_fdd53941)) => match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids_cdcb6239 , create_fdd53941) { Some (value_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el_6739e82f in value_d6124e21 . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_6739e82f]) { Ok (value_7ed84f3b) => { acc_bd78dc08 . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_7ed84f3b))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23dca12f-65c0-4c0e-addd-cc392c663733") } } } let value_e48110ec = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_d6124e21)) ; if ! acc_bd78dc08 . contains (& value_e48110ec) { acc_bd78dc08 . push (value_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b-2ba2-4eb1-a21e-2f9d30b21e98") , (None , Some (_)) => panic ! ("a2761cd2-27ff-4db0-ae81-948aa04573a6") , (None , None) => vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)] }) { Ok (value_55f2dc3d) => Some (value_55f2dc3d) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("88912e24-3bee-4dc4-a373-6d96d260170f") } }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud :: NotEmptyUniqueVec :: try_new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_cdcb6239) , Some (create_fdd53941)) => match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids_cdcb6239 , create_fdd53941) { Some (value_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el_6739e82f in value_d6124e21 . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_6739e82f]) { Ok (value_7ed84f3b) => { acc_bd78dc08 . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_7ed84f3b))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23dca12f-65c0-4c0e-addd-cc392c663733") } } } let value_e48110ec = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_d6124e21)) ; if ! acc_bd78dc08 . contains (& value_e48110ec) { acc_bd78dc08 . push (value_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b-2ba2-4eb1-a21e-2f9d30b21e98") , (None , Some (_)) => panic ! ("a2761cd2-27ff-4db0-ae81-948aa04573a6") , (None , None) => vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)] }) { Ok (value_55f2dc3d) => Some (value_55f2dc3d) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("88912e24-3bee-4dc4-a373-6d96d260170f") } }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud :: NotEmptyUniqueVec :: try_new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_cdcb6239) , Some (create_fdd53941)) => match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids_cdcb6239 , create_fdd53941) { Some (value_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el_6739e82f in value_d6124e21 . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_6739e82f]) { Ok (value_7ed84f3b) => { acc_bd78dc08 . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_7ed84f3b))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23dca12f-65c0-4c0e-addd-cc392c663733") } } } let value_e48110ec = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_d6124e21)) ; if ! acc_bd78dc08 . contains (& value_e48110ec) { acc_bd78dc08 . push (value_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b-2ba2-4eb1-a21e-2f9d30b21e98") , (None , Some (_)) => panic ! ("a2761cd2-27ff-4db0-ae81-948aa04573a6") , (None , None) => vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)] }) { Ok (value_55f2dc3d) => Some (value_55f2dc3d) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("88912e24-3bee-4dc4-a373-6d96d260170f") } }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud :: NotEmptyUniqueVec :: try_new (match (read_only_ids . 0. value , create . 0) { (Some (read_only_ids_cdcb6239) , Some (create_fdd53941)) => match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids_cdcb6239 , create_fdd53941) { Some (value_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el_6739e82f in value_d6124e21 . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_6739e82f]) { Ok (value_7ed84f3b) => { acc_bd78dc08 . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_7ed84f3b))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23dca12f-65c0-4c0e-addd-cc392c663733") } } } let value_e48110ec = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_d6124e21)) ; if ! acc_bd78dc08 . contains (& value_e48110ec) { acc_bd78dc08 . push (value_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b-2ba2-4eb1-a21e-2f9d30b21e98") , (None , Some (_)) => panic ! ("a2761cd2-27ff-4db0-ae81-948aa04573a6") , (None , None) => vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)] }) { Ok (value_55f2dc3d) => Some (value_55f2dc3d) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("88912e24-3bee-4dc4-a373-6d96d260170f") } }
    }
    fn create_into_postgresql_json_type_option_vec_where_length_equal(
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        match postgresql_crud :: NotEmptyUniqueVec :: try_new (match create . 0 { Some (create_09a81dae) => match < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create_09a81dae) { Some (value_3680a4c9) => { let mut acc_5c441d3a = Vec :: new () ; for el_a8b181a0 in value_3680a4c9 . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_a8b181a0]) { Ok (value_15097b27) => { acc_5c441d3a . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_15097b27))) ; } , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("6c4da72e-e16c-4c17-a939-9a52195e89a9") } } } let value_84ea8e4c = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_3680a4c9)) ; if ! acc_5c441d3a . contains (& value_84ea8e4c) { acc_5c441d3a . push (value_84ea8e4c) ; } acc_5c441d3a } , None => { return None ; } } , None => vec ! [postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (None)] , }) { Ok (value_72dbefbc) => Some (value_72dbefbc) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("d41bcbca-5d4c-436c-a465-4920c9da6a43") } }
    }
    fn create_into_postgresql_json_type_option_vec_where_length_greater_than(
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
        >,
    > {
        create . 0 . map_or_else (|| None , | create_612f2a61 | < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create_612f2a61) . map_or_else (|| None , | value_1ea95b5d | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_87f84b5c = Vec :: new () ; for el_9bbf8527 in value_1ea95b5d . clone () . into_vec () { match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [el_9bbf8527]) { Ok (value_1d0202fc) => { acc_87f84b5c . push (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_1d0202fc))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("bdb0a112-6f75-481c-ad28-f540252d8525") , } , } } let value_4e4cfda3 = postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_1ea95b5d)) ; if ! acc_87f84b5c . contains (& value_4e4cfda3) { acc_87f84b5c . push (value_4e4cfda3) ; } acc_87f84b5c }) { Ok (value_ea4ca151) => Some (value_ea4ca151) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("c7ecc36f-d510-40ff-a740-e796e112eee5") , } , } ,))
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression(
        read_only_ids : < Self :: PostgresqlJsonType as postgresql_crud :: PostgresqlJsonType > :: ReadOnlyIds,
        create: <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlJsonType as postgresql_crud::PostgresqlJsonType>::Where,
            >,
        >,
    > {
        match (read_only_ids . 0 . value , create . 0) { (Some (read_only_ids_3e2e30c8) , Some (create_79039a2f)) => < VecOfObjectExampleWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids_3e2e30c8 , create_79039a2f) . map_or_else (|| None , | value_35662b3a | match postgresql_crud :: NotEmptyUniqueVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el_4632f100 in value_35662b3a . into_vec () { match el_4632f100 { postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (multiple)))) ; } , postgresql_crud :: SingleOrMultiple :: Single (single) => match postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [single]) { Ok (value_4ce6ecd3) => { acc_e0d72451 . push (postgresql_crud :: SingleOrMultiple :: Single (postgresql_crud :: NullableJsonObjectPostgresqlTypeWhereFilter (Some (value_4ce6ecd3)))) ; } Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => () , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("626ffa77-f81a-46ce-b5a0-44663fe1f182") , } , } , } } acc_e0d72451 }) { Ok (value_5d381053) => Some (value_5d381053) , Err (error) => match error { postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: IsEmpty { .. } => None , postgresql_crud :: NotEmptyUniqueVecTryNewErrorNamed :: NotUnique { .. } => panic ! ("23a17416-0bac-4a1b-90df-cfd9d61ae86c") , } , }) , (Some (_) , None) => panic ! ("994082bf-aa95-45ea-9f80-ce91ae8661fc") , (None , Some (_)) => panic ! ("04f4d016-619e-4326-a260-cdec59c23907") , (None , None) => None , }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlTypeTestCases
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithId
{
    type PostgresqlType = Self;
    type Select = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect;
    fn option_vec_create()
    -> Option<Vec<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create>> {
        <Self as postgresql_crud::PostgresqlJsonTypeTestCases>::option_vec_create()
    }
    fn read_only_ids_to_two_dimensional_vec_read_inner(
        read_only_ids: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
    ) -> Vec<Vec<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadInner>> {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (read_only_ids)
    }
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        value: OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_read_with_new_or_try_new_unwraped (value)
    }
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        value: OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped (value)
    }
    fn update_to_read_only_ids(
        value: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        <Self as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(value)
    }
    fn read_only_ids_to_option_value_read_default_option_some_vec_one_el(
        value: &<Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
    ) -> Option<
        postgresql_crud::Value<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read>,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_to_option_value_read_default_option_some_vec_one_el (value)
    }
    fn previous_read_merged_with_option_update_into_read(
        read: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read,
        option_update: Option<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Update>,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: previous_read_merged_with_option_update_into_read (read , option_update)
    }
    fn read_only_ids_merged_with_create_into_read(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_read (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_option_value_read(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::Value<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Read>,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_table_type_declaration(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::TableTypeDeclaration {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_table_type_declaration (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_where_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_vec_where_equal_using_fields(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> postgresql_crud::NotEmptyUniqueVec<
        <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        Some (< Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_to_json_field (read_only_ids , create))
    }
    fn create_into_postgresql_type_option_vec_where_dimension_one_equal(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        None
    }
    fn postgresql_type_option_vec_where_greater_than_test() -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::PostgresqlTypeGreaterThanTest<Self::PostgresqlType>,
        >,
    > {
        None
    }
    fn read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than(
        greater_than_variant: postgresql_crud::PostgresqlTypeGreaterThanVariant,
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        table_type_declaration : < Self :: PostgresqlType as postgresql_crud :: PostgresqlType > :: TableTypeDeclaration,
    ) -> Option<<Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where> {
        None
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids , create)
    }
    fn create_into_postgresql_json_type_option_vec_where_length_equal(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (create)
    }
    fn create_into_postgresql_json_type_option_vec_where_length_greater_than(
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than (read_only_ids , create)
    }
    fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression(
        read_only_ids: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::ReadOnlyIds,
        create: <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Create,
    ) -> Option<
        postgresql_crud::NotEmptyUniqueVec<
            postgresql_crud::SingleOrMultiple<
                <Self::PostgresqlType as postgresql_crud::PostgresqlType>::Where,
            >,
        >,
    > {
        < Self as postgresql_crud :: PostgresqlJsonTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression (read_only_ids , create)
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl postgresql_crud::PostgresqlTypeNotPrimaryKey
    for OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithId
{
    type PostgresqlType = Self;
    type Create = OptionVecOfObjectExampleWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate;
}
