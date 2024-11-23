
// #[derive(Debug, Clone, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
// pub struct StdPrimitiveBoolAsPostgresqlBool(pub StdOptionOptionStdPrimitiveBool);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(crate::postgresql_types::base::StdPrimitiveBool);

impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::postgres::PgHasArrayType for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveBool as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
//

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNullColumn;
impl crate::generate_postgresql_query_part::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolNotNullColumn {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![]
    }
}

impl crate::CreateTableQueryPart for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        "BOOL NOT NULL"
    }
}
impl std::fmt::Display for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}



//todo maybe refactor later
#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNullToCreate(crate::postgresql_types::base::StdPrimitiveBool);
impl crate::BindQuery<'_> for StdPrimitiveBoolAsPostgresqlBoolNotNullToCreate {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_increment(&self.0, increment)
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(self.0, query)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNullToCreate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_types::base::StdPrimitiveBool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNullToCreate {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolNotNullToCreate {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNullToRead(crate::postgresql_types::base::StdPrimitiveBool);
impl crate::BindQuery<'_> for StdPrimitiveBoolAsPostgresqlBoolNotNullToRead {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_increment(&self.0, increment)
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(self.0, query)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNullToRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_types::base::StdPrimitiveBool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNullToRead {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolNotNullToRead {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate(crate::postgresql_types::base::StdPrimitiveBool);
impl crate::BindQuery<'_> for StdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_increment(&self.0, increment)
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(self.0, query)
    }
}
impl std::fmt::Display for StdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_types::base::StdPrimitiveBool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNullToDelete(crate::postgresql_types::base::StdPrimitiveBool);
impl crate::BindQuery<'_> for StdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_increment(&self.0, increment)
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(self.0, query)
    }
}
impl std::fmt::Display for StdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_types::base::StdPrimitiveBool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
#[derive(Debug, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNullWhere {
    pub value: StdPrimitiveBoolAsPostgresqlBoolNotNull,
    pub conjuctive_operator: crate::ConjunctiveOperator,
}
impl std::fmt::Display for StdPrimitiveBoolAsPostgresqlBoolNotNullWhere {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
    }
}
impl crate::BindQuery<'_> for StdPrimitiveBoolAsPostgresqlBoolNotNullWhere {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        increment.checked_add(1).map_or_else(
            || Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            |incr| {
                *increment = incr;
                Ok(())
            },
        )
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        increment.checked_add(1).map_or_else(
            || Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            |incr| {
                *increment = incr;
                Ok(format!("${increment}"))
            },
        )
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = crate::BindQuery::bind_value_to_query(self.value.0, query);
        query
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolNotNullWhere {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            conjuctive_operator: crate::ConjunctiveOperator::default(),
        }
    }
}




















// #[derive(Debug, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
// pub struct StdPrimitiveI64AsPostgresqlBigSerial(pub StdOptionOptionStdPrimitiveI64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNull(crate::postgresql_types::base::StdPrimitiveI64);
// #[derive(Debug, Clone, Copy, postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon)]
// pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(pub StdPrimitiveI64);


//just for the limit and offset types. maybe refactor later
impl crate::BindQuery<'_> for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_increment(&self.0, increment)
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(self.0, query)
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}

impl crate::CheckSupportedRustAndPostgresqlColumnType for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn check_supported_rust_and_postgresql_column_type() {}
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::postgres::PgHasArrayType for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}




#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullColumn;
impl crate::generate_postgresql_query_part::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialNotNullColumn {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![]
    }
}

impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        "BIGSERIAL"
    }
}
impl std::fmt::Display for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}




//todo maybe refactor later
#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate(crate::postgresql_types::base::StdPrimitiveI64);
impl crate::BindQuery<'_> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_increment(&self.0, increment)
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(self.0, query)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}

////////////
#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead(crate::postgresql_types::base::StdPrimitiveI64);
impl crate::BindQuery<'_> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_increment(&self.0, increment)
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(self.0, query)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialNotNullToRead {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
///////////////////
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate(crate::postgresql_types::base::StdPrimitiveI64);
impl crate::BindQuery<'_> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_increment(&self.0, increment)
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(self.0, query)
    }
}
impl std::fmt::Display for StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::AsPostgresqlCommon
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete(crate::postgresql_types::base::StdPrimitiveI64);
impl crate::BindQuery<'_> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_increment(&self.0, increment)
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(self.0, query)
    }
}
impl std::fmt::Display for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
#[derive(Debug, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    pub value: StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    pub conjuctive_operator: crate::ConjunctiveOperator,
}
impl std::fmt::Display for StdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
    }
}
impl crate::BindQuery<'_> for StdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        increment.checked_add(1).map_or_else(
            || Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            |incr| {
                *increment = incr;
                Ok(())
            },
        )
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        increment.checked_add(1).map_or_else(
            || Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            |incr| {
                *increment = incr;
                Ok(format!("${increment}"))
            },
        )
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = crate::BindQuery::bind_value_to_query(self.value.0, query);
        query
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            conjuctive_operator: crate::ConjunctiveOperator::default(),
        }
    }
}