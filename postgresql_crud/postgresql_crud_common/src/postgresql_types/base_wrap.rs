#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseWrapTypeTokens
)]
pub struct StdPrimitiveBoolAsPostgresqlBool(crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool);
impl crate::CreateTableQueryPart for StdPrimitiveBoolAsPostgresqlBool {
    fn create_table_query_part() -> impl std::fmt::Display {
        "BOOL"
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseWrapTypeTokens
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(crate::postgresql_types::base::StdPrimitiveBool);
impl crate::CreateTableQueryPart for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        "BOOL NOT NULL"
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseWrapTypeTokens
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNull(crate::postgresql_types::base::StdPrimitiveI64);
impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        "BIGSERIAL"
    }
}
//exception for offset and limit for now
const _: () = {
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
};
/////////////////////////////////
//+
impl std::fmt::Display for StdPrimitiveBoolAsPostgresqlBool {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
//+
impl error_occurence_lib::ToStdStringString for StdPrimitiveBoolAsPostgresqlBool {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
//+
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBool {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
//+
impl crate::BindQuerySecond<'_> for StdPrimitiveBoolAsPostgresqlBool {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
//+
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveBoolAsPostgresqlBoolColumn;
//+
impl crate::generate_postgresql_query_part::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolColumn {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![]
    }
}
//+
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveBoolAsPostgresqlBoolToCreate(crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool);
//+
impl crate::BindQuerySecond<'_> for StdPrimitiveBoolAsPostgresqlBoolToCreate {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
//-
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolToCreate {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
//-
// impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolToCreate {
//     fn type_info() -> sqlx::postgres::PgTypeInfo {
//         <crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
// }
//+
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolToCreate {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
//+
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveBoolAsPostgresqlBoolToRead(crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool);
//-
// impl crate::BindQuerySecond<'_> for StdPrimitiveBoolAsPostgresqlBoolToRead {
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
//     }
//     fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::BindQuerySecond::bind_value_to_query(self.0, query)
//     }
// }
//+
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolToRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
//+
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolToRead {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
//-
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolToRead {
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
//+
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveBoolAsPostgresqlBoolToUpdate(crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool);
//+
impl crate::BindQuerySecond<'_> for StdPrimitiveBoolAsPostgresqlBoolToUpdate {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
// -
// impl std::fmt::Display for StdPrimitiveBoolAsPostgresqlBoolToUpdate {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", self.0)
//     }
// }
//-
// impl error_occurence_lib::ToStdStringString for StdPrimitiveBoolAsPostgresqlBoolToUpdate {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self}")
//     }
// }
//-
// impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolToUpdate {
//     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
//         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
//     }
// }
//-
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolToUpdate {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
//-
// impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolToUpdate {
//     fn type_info() -> sqlx::postgres::PgTypeInfo {
//         <crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
// }
//+
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolToUpdate {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
//---
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct StdPrimitiveBoolAsPostgresqlBoolToDelete(crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool);
// impl crate::BindQuerySecond<'_> for StdPrimitiveBoolAsPostgresqlBoolToDelete {
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
//     }
//     fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::BindQuerySecond::bind_value_to_query(self.0, query)
//     }
// }
// impl std::fmt::Display for StdPrimitiveBoolAsPostgresqlBoolToDelete {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", self.0)
//     }
// }
// impl error_occurence_lib::ToStdStringString for StdPrimitiveBoolAsPostgresqlBoolToDelete {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self}")
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolToDelete {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBoolAsPostgresqlBoolToDelete {
//     fn type_info() -> sqlx::postgres::PgTypeInfo {
//         <crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
// }
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolToDelete {
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
//+
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveBoolAsPostgresqlBoolWhere {
    pub value: StdPrimitiveBoolAsPostgresqlBool,
    pub conjunctive_operator: crate::ConjunctiveOperator,
}
//+
impl crate::BindQuerySecond<'_> for StdPrimitiveBoolAsPostgresqlBoolWhere {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.value, increment)
    }
    fn bind_value_to_query(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.value, query)
    }
}
//+
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolWhere {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            conjunctive_operator: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
