generate_postgresql_json_types::generate_postgresql_json_types!([
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    },
    {
        "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
        "not_null_or_nullable": "Nullable",
        "postgresql_json_type_pattern": "Standart"
    },
    // {
    //     "postgresql_json_type": "StdPrimitiveI8AsJsonbNumber",
    //     "not_null_or_nullable": "Nullable",
    //     "postgresql_json_type_pattern": {
    //         "ArrayDimension4": {
    //             "dimension1_not_null_or_nullable": "Nullable",
    //             "dimension2_not_null_or_nullable": "Nullable",
    //             "dimension3_not_null_or_nullable": "Nullable",
    //             "dimension4_not_null_or_nullable": "Nullable"
    //         }
    //     }
    // },
    {
        "postgresql_json_type": "StdPrimitiveI16AsJsonbNumber",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    },
    {
        "postgresql_json_type": "UuidUuidAsJsonbString",
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart"
    }
]);
//////////
//////////
//////////
#[derive(Debug)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumber;
#[derive(Debug, Clone, PartialEq, PartialOrd, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberOrigin(std::option::Option<StdPrimitiveI8AsNotNullJsonbNumberOrigin>);
impl OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    pub fn new(value: std::option::Option<std::primitive::i8>) -> Self {
        Self(match value {
            Some(value) => Some(StdPrimitiveI8AsNotNullJsonbNumberOrigin::new(value)),
            None => None,
        })
    }
}
impl std::fmt::Display for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<std::primitive::i8> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<std::primitive::i8> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for OptionStdPrimitiveI8AsNullableJsonbNumberOrigin {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self.0.clone()), buf)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberSelect;
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionStdPrimitiveI8AsNullableJsonbNumberSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {}
    }
}
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
// pub enum OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement {
//     Equal(crate::where_element_filters::PostgresqlJsonTypeWhereElementEqual<OptionStdPrimitiveI8AsNullableJsonbNumberOrigin>),
//     GreaterThan(crate::where_element_filters::PostgresqlJsonTypeWhereElementGreaterThan<StdPrimitiveI8AsNotNullJsonbNumberOrigin>),
//     Between(crate::where_element_filters::PostgresqlJsonTypeWhereElementBetween<StdPrimitiveI8AsNotNullJsonbNumberOrigin>),
//     In(crate::where_element_filters::PostgresqlJsonTypeWhereElementIn<OptionStdPrimitiveI8AsNullableJsonbNumberOrigin>),
// }
// impl<'a> crate::PostgresqlTypeWhereFilter<'a> for OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement {
//     fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
//         match &self {
//             Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//             Self::In(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
//         }
//     }
//     fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         match self {
//             Self::Equal(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::GreaterThan(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::Between(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
//             Self::In(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
//         }
//     }
// }
// impl error_occurence_lib::ToStdStringString for OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         vec![
//             Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::GreaterThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::Between(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//             Self::In(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//         ]
//     }
// }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement(std::option::Option<crate::UniqueVec<StdPrimitiveI8AsNotNullJsonbNumberWhereElement>>);
impl<'a> crate::PostgresqlTypeWhereFilter<'a> for OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self.0 {
            Some(value) => crate::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            None => Ok(format!("{column} = 'null'")), //todo fix
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self.0 {
            Some(value) => crate::PostgresqlTypeWhereFilter::query_bind(value, query),
            None => query, //todo maybe wrong
        }
    }
}
impl error_occurence_lib::ToStdStringString for OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self(Some(
                crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
            )),
            Self(None)
        ]
    }
}
///////////
// #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
// pub struct NullableJsonObjectPostgresqlTypeWhereFilter(pub std::option::Option<UniqueVec<T>>);
// impl<'a, T> PostgresqlTypeWhereFilter<'a> for NullableJsonObjectPostgresqlTypeWhereFilter<T>
// {
//     fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, QueryPartErrorNamed> {
//         match &self.0 {
//             Some(value) => value.query_part(increment, column, is_need_to_add_logical_operator),
//             None => Ok(format!("{column} = 'null'")), //todo fix
//         }
//     }
//     fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         match self.0 {
//             Some(value) => value.query_bind(query),
//             None => query, //todo maybe wrong
//         }
//     }
// }
// impl<T> error_occurence_lib::ToStdStringString for NullableJsonObjectPostgresqlTypeWhereFilter<T>
// {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl<T> crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for NullableJsonObjectPostgresqlTypeWhereFilter<T>
// {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         vec![Self(Some(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())), Self(None)]
//     }
// }

///////////







impl crate::PostgresqlJsonType for OptionStdPrimitiveI8AsNullableJsonbNumber {
    type TableTypeDeclaration = OptionStdPrimitiveI8AsNullableJsonbNumberOrigin;
    type Create = OptionStdPrimitiveI8AsNullableJsonbNumberOrigin;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn create_query_bind(value: Self::Create, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(value);
        query
    }
    type Select = OptionStdPrimitiveI8AsNullableJsonbNumberSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}')='null' then null else ({column_name_and_maybe_field_getter}->'{field_ident}') end)))")
    }
    type WhereElement = OptionStdPrimitiveI8AsNullableJsonbNumberWhereElement;
    type Read = OptionStdPrimitiveI8AsNullableJsonbNumberOrigin;
    type Update = OptionStdPrimitiveI8AsNullableJsonbNumberOrigin;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn update_query_bind(value: Self::Update, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(value);
        query
    }
}
