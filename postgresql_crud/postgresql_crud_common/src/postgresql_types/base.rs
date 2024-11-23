#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    // postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl,
    // postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
)]
pub(crate) struct StdPrimitiveBool(std::primitive::bool); //todo maybe make it private? //todo column "std_primitive_bool_as_postgresql_bool" is of type boolean but expression is of type bigint
impl crate::AsPostgresqlBool for StdPrimitiveBool {}
impl crate::PostgresqlOrder for StdPrimitiveBool {}
impl crate::AsPostgresqlBool for StdOptionOptionStdPrimitiveBool {}
impl crate::PostgresqlOrder for StdOptionOptionStdPrimitiveBool {}


////////////////
impl std::fmt::Display for StdPrimitiveBool {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveBool {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl StdPrimitiveBool {
    pub(crate) fn into_inner(self) -> std::primitive::bool {
        self.0
    }
}
impl std::convert::From<StdPrimitiveBool> for std::primitive::bool {
    fn from(value: StdPrimitiveBool) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBool {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl crate::CheckSupportedPostgresqlColumnType for StdPrimitiveBool {
    fn check_supported_postgresql_column_type() {}
}
impl std::convert::From<StdPrimitiveBool> for crate::SupportedSqlxPostgresType {
    fn from(_value: StdPrimitiveBool) -> Self {
        Self::StdPrimitiveBool
    }
}
impl StdPrimitiveBool {
    pub(crate) fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::bool> {
        value.into_iter().map(Self::into_inner).collect()
    }
}
impl crate::BindQuery<'_> for StdPrimitiveBool {
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
        let mut increments = std::string::String::default();
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                increments.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(increments)
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.0);
        query
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveBool {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::postgres::PgHasArrayType for StdPrimitiveBool {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <std::primitive::bool as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[derive(Debug, PartialEq, Clone, serde :: Serialize, serde :: Deserialize, Eq)]
pub(crate) struct StdOptionOptionStdPrimitiveBool(std::option::Option<StdPrimitiveBool>);
impl sqlx::Encode<'_, sqlx::Postgres> for StdOptionOptionStdPrimitiveBool {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl std::fmt::Display for StdOptionOptionStdPrimitiveBool {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:?}",
            match &self.0 {
                Some(value) => Some(&value.0),
                None => None,
            }
        )
    }
}
impl StdOptionOptionStdPrimitiveBool {
    pub(crate) fn into_inner(self) -> std::option::Option<std::primitive::bool> {
        match self.0 {
            Some(value) => Some(value.0),
            None => None,
        }
    }
}
impl std::convert::From<StdOptionOptionStdPrimitiveBool> for std::option::Option<std::primitive::bool> {
    fn from(value: StdOptionOptionStdPrimitiveBool) -> Self {
        match value.0 {
            Some(value) => Some(value.0),
            None => None,
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdOptionOptionStdPrimitiveBool {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<std::primitive::bool> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::option::Option<std::primitive::bool> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl crate::CheckSupportedPostgresqlColumnType for StdOptionOptionStdPrimitiveBool {
    fn check_supported_postgresql_column_type() {}
}
impl std::convert::From<StdOptionOptionStdPrimitiveBool> for crate::SupportedSqlxPostgresType {
    fn from(_value: StdOptionOptionStdPrimitiveBool) -> Self {
        crate::SupportedSqlxPostgresType::StdPrimitiveBool
    }
}
impl StdOptionOptionStdPrimitiveBool {
    pub(crate) fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::option::Option<std::primitive::bool>> {
        value.into_iter().map(|element| element.into_inner()).collect()
    }
}
impl crate::BindQuery<'_> for StdOptionOptionStdPrimitiveBool {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(())
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut increments = std::string::String::default();
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                increments.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(increments)
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(match self.0 {
            Some(value) => Some(value.0),
            None => None,
        });
        query
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdPrimitiveBool {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
// #[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize, Eq)]
// pub struct StdOptionOptionStdPrimitiveBoolWhere {
//     value: StdOptionOptionStdPrimitiveBool,
//     conjuctive_operator: crate::ConjunctiveOperator,
// }
// impl std::fmt::Display for StdOptionOptionStdPrimitiveBoolWhere {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
//     }
// }
// impl crate::BindQuery<'_> for StdOptionOptionStdPrimitiveBoolWhere {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
//         increment.checked_add(1).map_or_else(
//             || Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             |incr| {
//                 *increment = incr;
//                 Ok(())
//             },
//         )
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 Ok(format!("${increment}"))
//             }
//             None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(match self.value.0 {
//             Some(value) => Some(value.0),
//             None => None,
//         });
//         query
//     }
// }
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdPrimitiveBoolWhere {
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             conjuctive_operator: crate::ConjunctiveOperator::default(),
//         }
//     }
// }


impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBool {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveBool {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::primitive::bool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
}
////////////////



#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::CommonWithEqImpl,
    // postgresql_crud_types_macro_logic_reuse::GenerateStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementFullTypePath,
)]
pub(crate) struct StdPrimitiveI64(std::primitive::i64);
impl crate::AsPostgresqlBigInt for StdPrimitiveI64 {}
impl crate::AsPostgresqlBigSerial for StdPrimitiveI64 {}
impl crate::AsPostgresqlInt8 for StdPrimitiveI64 {}
impl crate::PostgresqlOrder for StdPrimitiveI64 {}

impl std::fmt::Display for StdPrimitiveI64 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64 {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl StdPrimitiveI64 {
    pub(crate) fn into_inner(self) -> std::primitive::i64 {
        self.0
    }
}
impl std::convert::From<StdPrimitiveI64> for std::primitive::i64 {
    fn from(value: StdPrimitiveI64) -> Self {
        value.0
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl crate::CheckSupportedPostgresqlColumnType for StdPrimitiveI64 {
    fn check_supported_postgresql_column_type() {}
}
impl std::convert::From<StdPrimitiveI64> for crate::SupportedSqlxPostgresType {
    fn from(_value: StdPrimitiveI64) -> Self {
        Self::StdPrimitiveI64
    }
}
impl StdPrimitiveI64 {
    pub(crate) fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::i64> {
        value.into_iter().map(Self::into_inner).collect()
    }
}
impl crate::BindQuery<'_> for StdPrimitiveI64 {
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
        let mut increments = std::string::String::default();
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                increments.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(increments)
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.0);
        query
    }
}
/////////
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI64 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::postgres::PgHasArrayType for StdPrimitiveI64 {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <std::primitive::i64 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
////////
#[derive(Debug, PartialEq, Clone, serde :: Serialize, serde :: Deserialize, Eq)]
pub(crate) struct StdOptionOptionStdPrimitiveI64(std::option::Option<StdPrimitiveI64>);
impl std::fmt::Display for StdOptionOptionStdPrimitiveI64 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:?}",
            match &self.0 {
                Some(value) => Some(&value.0),
                None => None,
            }
        )
    }
}
impl StdOptionOptionStdPrimitiveI64 {
    pub(crate) fn into_inner(self) -> std::option::Option<std::primitive::i64> {
        match self.0 {
            Some(value) => Some(value.0),
            None => None,
        }
    }
}
impl std::convert::From<StdOptionOptionStdPrimitiveI64> for std::option::Option<std::primitive::i64> {
    fn from(value: StdOptionOptionStdPrimitiveI64) -> Self {
        match value.0 {
            Some(value) => Some(value.0),
            None => None,
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdOptionOptionStdPrimitiveI64 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<std::primitive::i64> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::option::Option<std::primitive::i64> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl crate::CheckSupportedPostgresqlColumnType for StdOptionOptionStdPrimitiveI64 {
    fn check_supported_postgresql_column_type() {}
}
impl std::convert::From<StdOptionOptionStdPrimitiveI64> for crate::SupportedSqlxPostgresType {
    fn from(_value: StdOptionOptionStdPrimitiveI64) -> Self {
        crate::SupportedSqlxPostgresType::StdPrimitiveI64
    }
}
impl StdOptionOptionStdPrimitiveI64 {
    pub(crate) fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::option::Option<std::primitive::i64>> {
        value.into_iter().map(|element| element.into_inner()).collect()
    }
}
impl crate::BindQuery<'_> for StdOptionOptionStdPrimitiveI64 {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(())
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut increments = std::string::String::default();
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                increments.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(increments)
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(match self.0 {
            Some(value) => Some(value.0),
            None => None,
        });
        query
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdPrimitiveI64 {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
// #[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize, Eq)]
// pub struct StdOptionOptionStdPrimitiveI64Where {
//     value: StdOptionOptionStdPrimitiveI64,
//     conjuctive_operator: crate::ConjunctiveOperator,
// }
// impl std::fmt::Display for StdOptionOptionStdPrimitiveI64Where {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
//     }
// }
// impl crate::BindQuery<'_> for StdOptionOptionStdPrimitiveI64Where {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
//         increment.checked_add(1).map_or_else(
//             || Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             |incr| {
//                 *increment = incr;
//                 Ok(())
//             },
//         )
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 Ok(format!("${increment}"))
//             }
//             None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(match self.value.0 {
//             Some(value) => Some(value.0),
//             None => None,
//         });
//         query
//     }
// }
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdPrimitiveI64Where {
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             conjuctive_operator: crate::ConjunctiveOperator::default(),
//         }
//     }
// }

impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64 {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::primitive::i64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error)
        }
    }
}