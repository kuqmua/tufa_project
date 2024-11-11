/////////////////////////////here
// todo maybe inner value must be pub
// #[derive(
//     Debug,
//     Clone,
// )]
// pub struct StdPrimitiveBoolAsPostgresqlBool(pub StdOptionOptionStdPrimitiveBool);

#[derive(
    Debug,
    Clone,
    Copy,
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(pub StdPrimitiveBool);

///////////////////////////////////
//new type pattern
// sqlx::Encode impl was copied from https://docs.rs/sqlx/0.7.3/sqlx/trait.Encode.html
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
pub struct StdPrimitiveBool(pub std::primitive::bool); //todo maybe make it private? //todo column "std_primitive_bool_as_postgresql_bool" is of type boolean but expression is of type bigint
// impl AsPostgresqlBool for StdPrimitiveBool {}
// impl PostgresqlOrder for StdPrimitiveBool {}
// impl AsPostgresqlBool for StdOptionOptionStdPrimitiveBool {}
// impl PostgresqlOrder for StdOptionOptionStdPrimitiveBool {}

        impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBool {
            #[inline]
            fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                Self(::core::default::Default::default())
            }
        }

// impl std::fmt::Display for StdPrimitiveBool {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", self.0)
//     }
// }
// impl error_occurence_lib::ToStdStringString for StdPrimitiveBool {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self}")
//     }
// }
// impl StdPrimitiveBool {
//     pub fn into_inner(self) -> std::primitive::bool {
//         self.0
//     }
// }
// impl std::convert::From<StdPrimitiveBool> for std::primitive::bool {
//     fn from(value: StdPrimitiveBool) -> Self {
//         value.0
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBool {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl CheckSupportedPostgresqlColumnType for StdPrimitiveBool {
//     fn check_supported_postgresql_column_type() {}
// }
// impl std::convert::From<StdPrimitiveBool> for SupportedSqlxPostgresType {
//     fn from(_value: StdPrimitiveBool) -> Self {
//         Self::StdPrimitiveBool
//     }
// }
// impl StdPrimitiveBool {
//     pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::bool> {
//         value.into_iter().map(Self::into_inner).collect()
//     }
// }
// impl BindQuery<'_> for StdPrimitiveBool {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         increment.checked_add(1).map_or_else(
//             || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             |incr| {
//                 *increment = incr;
//                 Ok(())
//             },
//         )
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::default();
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("${increment}"));
//             }
//             None => {
//                 return Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         Ok(increments)
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(self.0);
//         query
//     }
// }
// #[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize, Eq)]
// pub struct WhereStdPrimitiveBool {
//     pub value: StdPrimitiveBool,
//     pub conjuctive_operator: ConjunctiveOperator,
// }
// impl std::fmt::Display for WhereStdPrimitiveBool {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
//     }
// }
// impl BindQuery<'_> for WhereStdPrimitiveBool {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         increment.checked_add(1).map_or_else(
//             || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             |incr| {
//                 *increment = incr;
//                 Ok(())
//             },
//         )
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
//         increment.checked_add(1).map_or_else(
//             || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             |incr| {
//                 *increment = incr;
//                 Ok(format!("${increment}"))
//             },
//         )
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(self.value.0);
//         query
//     }
// }
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for WhereStdPrimitiveBool {
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             conjuctive_operator: ConjunctiveOperator::default(),
//         }
//     }
// }
// #[derive(Debug, PartialEq, Clone, serde :: Serialize, serde :: Deserialize, Eq)]
// pub struct StdOptionOptionStdPrimitiveBool(pub std::option::Option<StdPrimitiveBool>);
// impl std::fmt::Display for StdOptionOptionStdPrimitiveBool {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             formatter,
//             "{:?}",
//             match &self.0 {
//                 Some(value) => Some(&value.0),
//                 None => None,
//             }
//         )
//     }
// }
// impl StdOptionOptionStdPrimitiveBool {
//     pub fn into_inner(self) -> std::option::Option<std::primitive::bool> {
//         match self.0 {
//             Some(value) => Some(value.0),
//             None => None,
//         }
//     }
// }
// impl std::convert::From<StdOptionOptionStdPrimitiveBool> for std::option::Option<std::primitive::bool> {
//     fn from(value: StdOptionOptionStdPrimitiveBool) -> Self {
//         match value.0 {
//             Some(value) => Some(value.0),
//             None => None,
//         }
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for StdOptionOptionStdPrimitiveBool {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <std::option::Option<std::primitive::bool> as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <std::option::Option<std::primitive::bool> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl CheckSupportedPostgresqlColumnType for StdOptionOptionStdPrimitiveBool {
//     fn check_supported_postgresql_column_type() {}
// }
// impl std::convert::From<StdOptionOptionStdPrimitiveBool> for SupportedSqlxPostgresType {
//     fn from(_value: StdOptionOptionStdPrimitiveBool) -> Self {
//         SupportedSqlxPostgresType::StdPrimitiveBool
//     }
// }
// impl StdOptionOptionStdPrimitiveBool {
//     pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::option::Option<std::primitive::bool>> {
//         value.into_iter().map(|element| element.into_inner()).collect()
//     }
// }
// impl BindQuery<'_> for StdOptionOptionStdPrimitiveBool {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 Ok(())
//             }
//             None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::default();
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("${increment}"));
//             }
//             None => {
//                 return Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         Ok(increments)
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(match self.0 {
//             Some(value) => Some(value.0),
//             None => None,
//         });
//         query
//     }
// }
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdPrimitiveBool {
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(Some(
//             crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//         ))
//     }
// }
// #[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize, Eq)]
// pub struct WhereStdOptionOptionStdPrimitiveBool {
//     pub value: StdOptionOptionStdPrimitiveBool,
//     pub conjuctive_operator: ConjunctiveOperator,
// }
// impl std::fmt::Display for WhereStdOptionOptionStdPrimitiveBool {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
//     }
// }
// impl BindQuery<'_> for WhereStdOptionOptionStdPrimitiveBool {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         increment.checked_add(1).map_or_else(
//             || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             |incr| {
//                 *increment = incr;
//                 Ok(())
//             },
//         )
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 Ok(format!("${increment}"))
//             }
//             None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
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
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for WhereStdOptionOptionStdPrimitiveBool {
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             conjuctive_operator: ConjunctiveOperator::default(),
//         }
//     }
// }
// impl std::fmt::Display for StdPrimitiveBool {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", self.0)
//     }
// }
// impl error_occurence_lib::ToStdStringString for StdPrimitiveBool {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self}")
//     }
// }
// impl StdPrimitiveBool {
//     pub fn into_inner(self) -> std::primitive::bool {
//         self.0
//     }
// }
// impl std::convert::From<StdPrimitiveBool> for std::primitive::bool {
//     fn from(value: StdPrimitiveBool) -> Self {
//         value.0
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for StdPrimitiveBool {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl CheckSupportedPostgresqlColumnType for StdPrimitiveBool {
//     fn check_supported_postgresql_column_type() {}
// }
// impl std::convert::From<StdPrimitiveBool> for SupportedSqlxPostgresType {
//     fn from(_value: StdPrimitiveBool) -> Self {
//         Self::StdPrimitiveBool
//     }
// }
// impl StdPrimitiveBool {
//     pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::bool> {
//         value.into_iter().map(Self::into_inner).collect()
//     }
// }
// impl BindQuery<'_> for StdPrimitiveBool {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         increment.checked_add(1).map_or_else(
//             || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             |incr| {
//                 *increment = incr;
//                 Ok(())
//             },
//         )
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::default();
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("${increment}"));
//             }
//             None => {
//                 return Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         Ok(increments)
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(self.0);
//         query
//     }
// }
// #[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize, Eq)]
// pub struct WhereStdPrimitiveBool {
//     pub value: StdPrimitiveBool,
//     pub conjuctive_operator: ConjunctiveOperator,
// }
// impl std::fmt::Display for WhereStdPrimitiveBool {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
//     }
// }
// impl BindQuery<'_> for WhereStdPrimitiveBool {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         increment.checked_add(1).map_or_else(
//             || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             |incr| {
//                 *increment = incr;
//                 Ok(())
//             },
//         )
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
//         increment.checked_add(1).map_or_else(
//             || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             |incr| {
//                 *increment = incr;
//                 Ok(format!("${increment}"))
//             },
//         )
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(self.value.0);
//         query
//     }
// }
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for WhereStdPrimitiveBool {
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             conjuctive_operator: ConjunctiveOperator::default(),
//         }
//     }
// }
// #[derive(Debug, PartialEq, Clone, serde :: Serialize, serde :: Deserialize, Eq)]
// pub struct StdOptionOptionStdPrimitiveBool(pub std::option::Option<StdPrimitiveBool>);
// impl std::fmt::Display for StdOptionOptionStdPrimitiveBool {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             formatter,
//             "{:?}",
//             match &self.0 {
//                 Some(value) => Some(&value.0),
//                 None => None,
//             }
//         )
//     }
// }
// impl StdOptionOptionStdPrimitiveBool {
//     pub fn into_inner(self) -> std::option::Option<std::primitive::bool> {
//         match self.0 {
//             Some(value) => Some(value.0),
//             None => None,
//         }
//     }
// }
// impl std::convert::From<StdOptionOptionStdPrimitiveBool> for std::option::Option<std::primitive::bool> {
//     fn from(value: StdOptionOptionStdPrimitiveBool) -> Self {
//         match value.0 {
//             Some(value) => Some(value.0),
//             None => None,
//         }
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for StdOptionOptionStdPrimitiveBool {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <std::option::Option<std::primitive::bool> as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
//         <std::option::Option<std::primitive::bool> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl CheckSupportedPostgresqlColumnType for StdOptionOptionStdPrimitiveBool {
//     fn check_supported_postgresql_column_type() {}
// }
// impl std::convert::From<StdOptionOptionStdPrimitiveBool> for SupportedSqlxPostgresType {
//     fn from(_value: StdOptionOptionStdPrimitiveBool) -> Self {
//         SupportedSqlxPostgresType::StdPrimitiveBool
//     }
// }
// impl StdOptionOptionStdPrimitiveBool {
//     pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::option::Option<std::primitive::bool>> {
//         value.into_iter().map(|element| element.into_inner()).collect()
//     }
// }
// impl BindQuery<'_> for StdOptionOptionStdPrimitiveBool {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 Ok(())
//             }
//             None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
//         let mut increments = std::string::String::default();
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 increments.push_str(&format!("${increment}"));
//             }
//             None => {
//                 return Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
//         Ok(increments)
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(match self.0 {
//             Some(value) => Some(value.0),
//             None => None,
//         });
//         query
//     }
// }
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdPrimitiveBool {
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(Some(
//             crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//         ))
//     }
// }
// #[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize, Eq)]
// pub struct WhereStdOptionOptionStdPrimitiveBool {
//     pub value: StdOptionOptionStdPrimitiveBool,
//     pub conjuctive_operator: ConjunctiveOperator,
// }
// impl std::fmt::Display for WhereStdOptionOptionStdPrimitiveBool {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
//     }
// }
// impl BindQuery<'_> for WhereStdOptionOptionStdPrimitiveBool {
//     fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
//         increment.checked_add(1).map_or_else(
//             || Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             |incr| {
//                 *increment = incr;
//                 Ok(())
//             },
//         )
//     }
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(incr) => {
//                 *increment = incr;
//                 Ok(format!("${increment}"))
//             }
//             None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
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
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for WhereStdOptionOptionStdPrimitiveBool {
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             conjuctive_operator: ConjunctiveOperator::default(),
//         }
//     }
// }
