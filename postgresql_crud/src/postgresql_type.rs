// generate_postgresql_types::generate_postgresql_types!("All");
generate_postgresql_types::generate_postgresql_types!({
    "Concrete": [
        // {
        //     "postgresql_type": "StdPrimitiveI64AsInt8",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdStringStringAsText",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // }
        // ,
        // {
        //     "postgresql_type": "StdPrimitiveI32AsInt4",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI16AsInt2",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeTimeAsTime",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "Nullable",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "Nullable"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdPrimitiveI64AsBigSerialInitializedByPostgresql",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": "Standart"
        // },
        // {
        //     "postgresql_type": "StdStringStringAsText",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesTimeDateAsDate",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveDateTimeAsTimestamp",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxTypesChronoNaiveTimeAsTime",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "StdVecVecStdPrimitiveU8AsBytea",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // }
        // ,
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI32AsInt4Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        // {
        //     "postgresql_type": "SqlxPostgresTypesPgRangeStdPrimitiveI64AsInt8Range",
        //     "not_null_or_nullable": "NotNull",
        //     "postgresql_type_pattern": {
        //         "ArrayDimension1": {
        //             "dimension1_not_null_or_nullable": "NotNull"
        //         }
        //     }
        // },
        {
            "postgresql_type": "SqlxTypesUuidUuidAsUuidV4InitializedByPostgresql",
            "not_null_or_nullable": "NotNull",
            "postgresql_type_pattern": "Standart"
        }
    ]
});


#[derive(Debug)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTz;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin(sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>);
impl SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin {
    pub fn new(value: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>) -> Self {
        Self(value)
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>) -> Self {
        Self::new(value)
    }
}
impl std::fmt::Display for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::postgres::PgHasArrayType for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} timestamptz not null")
    }
}
pub type SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzTableTypeDeclaration = SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin;
pub type SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzCreate = SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin;
#[derive(Debug, Default, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzWhereElement {
    Equal(crate::where_element_filters::PostgresqlTypeWhereElementEqual<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin>),
    Before(crate::where_element_filters::PostgresqlTypeWhereElementBefore<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzTableTypeDeclaration>),
    Between(crate::where_element_filters::PostgresqlTypeWhereElementBetween<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzTableTypeDeclaration>),
}
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Before(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Before(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Before(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRead(SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin);
impl SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRead {
    pub fn new(value: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>) -> Self {
        Self(SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin::new(value))
    }
    pub fn new_or_try_new_unwraped_for_test(value: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>) -> Self {
        Self(SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin::new_or_try_new_unwraped_for_test(value))
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRead {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRead {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzReadInner = sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>;
pub type SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzUpdate = SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzOrigin;
impl crate::PostgresqlType for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTz {
    type TableTypeDeclaration = SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzTableTypeDeclaration;
    type Create = SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzCreate;
    fn create_query_part(_: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                acc.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(acc)
    }
    fn create_query_bind(value: Self::Create, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(value);
        query
    }
    type Select = SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzSelect;
    fn select_query_part(_: &Self::Select, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type WhereElement = SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzWhereElement;
    type Read = SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRead;
    type ReadInner = SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.0.0
    }
    type Update = SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzUpdate;
    fn update_query_part(_: &Self::Update, _: &std::primitive::str, _: &std::primitive::str, _: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                acc.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(acc)
    }
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(value);
        query
    }
}
impl crate::tests::PostgresqlTypeTestCases for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTz {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as crate::PostgresqlType>::ReadInner> {
        vec![
            sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::UNIX_EPOCH
            ,
            sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
                chrono::NaiveDateTime::new(
                    chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
                    // NaiveTime
                    crate::SqlxTypesChronoNaiveTime::try_new(
                        crate::Hour::try_new(14).unwrap(),
                        crate::Minute::try_new(22).unwrap(),
                        crate::Second::try_new(33).unwrap(),
                        crate::Microsecond::try_new(123_456).unwrap(),
                    ).unwrap().into(),
                ),
                sqlx::types::chrono::Utc
            )
        ]
    }
}
