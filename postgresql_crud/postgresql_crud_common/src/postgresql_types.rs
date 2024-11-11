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
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct PostgresqlBoolNotNull(std::primitive::bool);
impl std::fmt::Display for PostgresqlBoolNotNull {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlBoolNotNull {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl error_occurence_lib::ToStdStringString for PostgresqlBoolNotNull {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{}", self.0)
    }
}
// impl PostgresqlBoolNotNull {
//     pub fn into_inner(self) -> std::primitive::bool {
//         self.0
//     }
// }
// impl std::convert::From<StdPrimitiveBool> for std::primitive::bool {
//     fn from(value: StdPrimitiveBool) -> Self {
//         value.0
//     }
// }
impl sqlx::Type<sqlx::Postgres> for PostgresqlBoolNotNull {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::bool as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
// impl CheckSupportedPostgresqlColumnType for PostgresqlBoolNotNull {
//     fn check_supported_postgresql_column_type() {}
// }
// impl std::convert::From<StdPrimitiveBool> for SupportedSqlxPostgresType {
//     fn from(_value: StdPrimitiveBool) -> Self {
//         Self::StdPrimitiveBool
//     }
// }
// impl PostgresqlBoolNotNull {
//     pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::primitive::bool> {
//         value.into_iter().map(Self::into_inner).collect()
//     }
// }
impl crate::BindQuery<'_> for PostgresqlBoolNotNull {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(())
            },
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                code_occurence: error_occurence_lib::code_occurence!()
            })
        }
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                code_occurence: error_occurence_lib::code_occurence!()
            })
        }
        
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.0);
        query
    }
}
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Eq)]
pub struct WherePostgresqlBoolNotNull {
    pub value: PostgresqlBoolNotNull,
    pub conjuctive_operator: crate::ConjunctiveOperator,
}
impl std::fmt::Display for WherePostgresqlBoolNotNull {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
    }
}
impl crate::BindQuery<'_> for WherePostgresqlBoolNotNull {
    fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(())
            },
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                code_occurence: error_occurence_lib::code_occurence!()
            })
        }
    }
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            },
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                code_occurence: error_occurence_lib::code_occurence!()
            })
        }
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value.0);
        query
    }
}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for WherePostgresqlBoolNotNull {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            value: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            conjuctive_operator: crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        }
    }
}
///////////////////////////////////
